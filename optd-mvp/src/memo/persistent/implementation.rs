//! This module contains the implementation of [`PersistentMemo`].
//!
//! TODO For parallelism, almost all of these methods need to be under transactions.
//! TODO Write more docs.
//! TODO Remove dead code.

#![allow(dead_code)]

use super::{PersistentMemo, PersistentMemoTransaction};
use crate::{
    entities::*,
    expression::{LogicalExpression, PhysicalExpression},
    memo::{GroupId, GroupStatus, LogicalExpressionId, MemoError, PhysicalExpressionId},
    OptimizerResult, DATABASE_URL,
};
use sea_orm::{
    entity::{prelude::*, IntoActiveModel, NotSet, Set},
    Database, DatabaseTransaction, TransactionTrait,
};
use std::{collections::HashSet, marker::PhantomData};

impl<L, P> PersistentMemo<L, P>
where
    L: LogicalExpression,
    P: PhysicalExpression,
{
    /// Creates a new `PersistentMemo` struct by connecting to a database defined at
    /// [`DATABASE_URL`].
    ///
    /// # Panics
    ///
    /// Panics if unable to create a databse connection to [`DATABASE_URL`].
    pub async fn new() -> Self {
        Self {
            db: Database::connect(DATABASE_URL).await.unwrap(),
            _phantom_logical: PhantomData,
            _phantom_physical: PhantomData,
        }
    }

    /// Starts a new database transaction.
    ///
    /// # Errors
    ///
    /// Returns a [`DbErr`] if unable to create a new transaction.
    pub async fn begin(&self) -> OptimizerResult<PersistentMemoTransaction<L, P>> {
        Ok(PersistentMemoTransaction::new(self.db.begin().await?).await)
    }

    /// Deletes all objects in the backing database.
    ///
    /// Since there is no asynchronous drop yet in Rust, in order to drop all objects in the
    /// database, the user must call this manually.
    ///
    /// # Panics
    ///
    /// May panic if unable to delete entities from any table.
    pub async fn cleanup(&self) {
        /// Simple private macro to teardown all tables in the database.
        /// Note that these have to be specified manually, so when adding a new table to the
        /// database, we must make sure to add that table here.
        macro_rules! delete_all {
            ($($module: ident),+ $(,)?) => {
                $(
                    $module::Entity::delete_many()
                        .exec(&self.db)
                        .await
                        .unwrap();
                )+
            };
        }

        delete_all! {
            group,
            fingerprint,
            logical_expression,
            logical_children,
            physical_expression,
            physical_children
        };
    }
}

impl<L, P> PersistentMemoTransaction<L, P>
where
    L: LogicalExpression,
    P: PhysicalExpression,
{
    /// Creates a new transaction object.
    pub async fn new(txn: DatabaseTransaction) -> Self {
        Self {
            txn,
            _phantom_logical: PhantomData,
            _phantom_physical: PhantomData,
        }
    }

    /// Commits the transaction.
    ///
    /// # Errors
    ///
    /// Returns a [`DbErr`] if unable to commit the transaction.
    pub async fn commit(self) -> OptimizerResult<()> {
        Ok(self.txn.commit().await?)
    }

    /// Rolls back the transaction.
    ///
    /// # Errors
    ///
    /// Returns a [`DbErr`] if unable to roll back the transaction.
    pub async fn rollback(self) -> OptimizerResult<()> {
        Ok(self.txn.rollback().await?)
    }

    /// Retrieves a [`group::Model`] given its ID.
    ///
    /// FIXME: use an in-memory representation of a group instead.
    ///
    /// # Errors
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    pub async fn get_group(&self, group_id: GroupId) -> OptimizerResult<group::Model> {
        Ok(group::Entity::find_by_id(group_id.0)
            .one(&self.txn)
            .await?
            .ok_or(MemoError::UnknownGroup(group_id))?)
    }

    /// Retrieves the root / canonical group ID of the given group ID.
    ///
    /// The groups form a union find / disjoint set parent pointer forest, where group merging
    /// causes two trees to merge.
    ///
    /// This function uses the path compression optimization, which amortizes the cost to a single
    /// lookup (theoretically in constant time, but we must be wary of the I/O roundtrip).
    ///
    /// # Errors
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error. This function
    /// also performs path compression pointer updates, so any of those updates can fail with a
    /// [`DbErr`].
    pub async fn get_root_group(&self, group_id: GroupId) -> OptimizerResult<GroupId> {
        let curr_group = self.get_group(group_id).await?;

        // If we have no parent, then we are at the root.
        let Some(parent_id) = curr_group.parent_id else {
            return Ok(GroupId(curr_group.id));
        };

        // Recursively find the root group ID.
        let root_id = Box::pin(self.get_root_group(GroupId(parent_id))).await?;

        // Path Compression Optimization:
        // For every group along the path that we walked, set their parent id pointer to the root.
        // This allows for an amortized O(1) cost for `get_root_group`.
        let mut active_group = curr_group.into_active_model();

        // Update the group to point to the new parent.
        active_group.parent_id = Set(Some(root_id.0));
        active_group.update(&self.txn).await?;

        Ok(GroupId(root_id.0))
    }

    /// Retrieves every group ID of groups that share the same root group with the input group.
    ///
    /// The group records form a union-find data structure that also maintains a circular linked
    /// list in every set that allows us to iterate over all elements in a set in linear time.
    ///
    /// # Errors
    ///
    /// If the input group does not exist, or if any pointer along the path is invalid, returns a
    /// [`MemoError::UnknownGroup`] error.
    ///
    /// # Panics
    ///
    /// Panics if the embedded union-find data structure is malformed.
    pub async fn get_group_set(&self, group_id: GroupId) -> OptimizerResult<Vec<GroupId>> {
        // Iterate over the circular linked list until we reach ourselves again.
        let base_group = self.get_group(group_id).await?;

        // The only case when `next_id` is set to `None` is if the current group is a root, which
        // means that this group is the only group in the set.
        if base_group.next_id.is_none() {
            assert!(base_group.parent_id.is_none());
            return Ok(vec![group_id]);
        }

        // Iterate over the circular linked list until we see ourselves again, collecting nodes
        // along the way.
        let mut set = vec![group_id];
        let mut next_id = base_group
            .next_id
            .expect("next pointer cannot be null if it is in a cycle");
        loop {
            let curr_group = self.get_group(GroupId(next_id)).await?;

            if curr_group.id == group_id.0 {
                break;
            }

            set.push(GroupId(curr_group.id));
            next_id = curr_group
                .next_id
                .expect("next pointer cannot be null if it is in a cycle");
        }

        Ok(set)
    }

    /// Retrieves a [`physical_expression::Model`] given a [`PhysicalExpressionId`].
    ///
    /// # Errors
    ///
    /// If the physical expression does not exist, returns a
    /// [`MemoError::UnknownPhysicalExpression`] error.
    pub async fn get_physical_expression(
        &self,
        physical_expression_id: PhysicalExpressionId,
    ) -> OptimizerResult<(GroupId, P)> {
        // Lookup the entity in the database via the unique expression ID.
        let model = physical_expression::Entity::find_by_id(physical_expression_id.0)
            .one(&self.txn)
            .await?
            .ok_or(MemoError::UnknownPhysicalExpression(physical_expression_id))?;

        let group_id = GroupId(model.group_id);
        let expr = model.into();

        Ok((group_id, expr))
    }

    /// Retrieves a [`logical_expression::Model`] given its [`LogicalExpressionId`].
    ///
    /// # Errors
    ///
    /// If the logical expression does not exist, returns a [`MemoError::UnknownLogicalExpression`]
    /// error.
    pub async fn get_logical_expression(
        &self,
        logical_expression_id: LogicalExpressionId,
    ) -> OptimizerResult<(GroupId, L)> {
        // Lookup the entity in the database via the unique expression ID.
        let model = logical_expression::Entity::find_by_id(logical_expression_id.0)
            .one(&self.txn)
            .await?
            .ok_or(MemoError::UnknownLogicalExpression(logical_expression_id))?;

        let group_id = GroupId(model.group_id);
        let expr = model.into();

        Ok((group_id, expr))
    }

    /// Retrieves all of the logical expression "children" IDs of a group.
    ///
    /// FIXME: `find_related` does not work for some reason, have to use manual `filter`.
    ///
    /// # Errors
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error. Can also return
    /// a [`DbErr`] if the something goes wrong with the filter scan.
    pub async fn get_logical_children(
        &self,
        group_id: GroupId,
    ) -> OptimizerResult<Vec<LogicalExpressionId>> {
        // First ensure that the group exists.
        let _ = self.get_group(group_id).await?;

        // Search for expressions that have the given parent group ID.
        let children = logical_expression::Entity::find()
            .filter(logical_expression::Column::GroupId.eq(group_id.0))
            .all(&self.txn)
            .await?
            .into_iter()
            .map(|m| LogicalExpressionId(m.id))
            .collect();

        Ok(children)
    }

    /// Retrieves all of the physical expression "children" IDs of a group.
    ///
    /// FIXME: `find_related` does not work for some reason, have to use manual `filter`.
    ///
    /// # Errors
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error. Can also return
    /// a [`DbErr`] if the something goes wrong with the filter scan.
    pub async fn get_physical_children(
        &self,
        group_id: GroupId,
    ) -> OptimizerResult<Vec<PhysicalExpressionId>> {
        // First ensure that the group exists.
        let _ = self.get_group(group_id).await?;

        // Search for expressions that have the given parent group ID.
        let children = physical_expression::Entity::find()
            .filter(physical_expression::Column::GroupId.eq(group_id.0))
            .all(&self.txn)
            .await?
            .into_iter()
            .map(|m| PhysicalExpressionId(m.id))
            .collect();

        Ok(children)
    }

    /// Updates / replaces a group's status. Returns the previous group status.
    ///
    /// # Errors
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error. Can also return a
    /// [`DbErr`] if the update fails.
    pub async fn update_group_status(
        &mut self,
        group_id: GroupId,
        status: GroupStatus,
    ) -> OptimizerResult<GroupStatus> {
        // First retrieve the group record.
        let mut group = self.get_group(group_id).await?.into_active_model();

        // Update the group's status.
        let old_status = group.status;
        group.status = Set(status as u8 as i8);
        group.update(&self.txn).await?;

        let old_status = match old_status.unwrap() {
            0 => GroupStatus::InProgress,
            1 => GroupStatus::Explored,
            2 => GroupStatus::Optimized,
            _ => unreachable!("encountered an invalid group status"),
        };

        Ok(old_status)
    }

    /// Updates / replaces a group's best physical plan (winner). Optionally returns the previous
    /// winner's physical expression ID.
    ///
    /// FIXME: In the future, this should first check that we aren't overwriting a winner that was
    /// updated from another thread by comparing against the cost of the plan.
    ///
    /// # Errors
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error. Can also return a
    /// [`DbErr`] if the update fails.
    pub async fn update_group_winner(
        &mut self,
        group_id: GroupId,
        physical_expression_id: PhysicalExpressionId,
    ) -> OptimizerResult<Option<PhysicalExpressionId>> {
        // First retrieve the group record.
        let mut group = self.get_group(group_id).await?.into_active_model();

        // Update the group to point to the new winner.
        let old_id = group.winner;
        group.winner = Set(Some(physical_expression_id.0));
        group.update(&self.txn).await?;

        // Note that the `unwrap` here is unwrapping the `ActiveValue`, not the `Option`.
        let old_id = old_id.unwrap().map(PhysicalExpressionId);
        Ok(old_id)
    }

    /// Adds a logical expression to an existing group via its ID.
    ///
    /// The caller is required to pass in a slice of [`GroupId`] that represent the child groups of
    /// the input expression.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    ///
    /// If the memo table detects that the input logical expression is a duplicate expression, this
    /// function will **not** insert the expression into the memo table. Instead, it will return an
    /// `Ok(Err(expression_id))`, which is a unique identifier of the expression that the input is a
    /// duplicate of. The caller can use this ID to retrieve the group the original belongs to.
    ///
    /// If the memo table detects that the input is unique, it will insert the expression into the
    /// input group and return an `Ok(Ok(expression_id))`.
    ///
    /// # Errors
    ///
    /// Note that the return value is a [`Result`] wrapped in an [`OptimizerResult`]. The outer
    /// result is used for raising [`DbErr`] or other database/IO-related errors. The inner result
    /// is used for notifying the caller if the expression that they attempted to insert was a
    /// duplicate expression or not.
    pub async fn add_logical_expression_to_group(
        &mut self,
        group_id: GroupId,
        logical_expression: L,
        children: &[GroupId],
    ) -> OptimizerResult<Result<LogicalExpressionId, (GroupId, LogicalExpressionId)>> {
        // Check if the expression already exists anywhere in the memo table.
        if let Some(existing_id) = self
            .is_duplicate_logical_expression(&logical_expression, children)
            .await?
        {
            return Ok(Err(existing_id));
        }

        // Check if the group actually exists.
        let _ = self.get_group(group_id).await?;

        // Insert the expression.
        let model: logical_expression::Model = logical_expression.into();
        let mut active_model = model.into_active_model();
        active_model.group_id = Set(group_id.0);
        active_model.id = NotSet;
        let new_model = active_model.insert(&self.txn).await?;

        let expr_id = new_model.id;

        // Insert the child groups of the expression into the junction / children table.
        logical_children::Entity::insert_many(children.iter().copied().map(|child_id| {
            logical_children::ActiveModel {
                logical_expression_id: Set(expr_id),
                group_id: Set(child_id.0),
            }
        }))
        .on_empty_do_nothing()
        .exec(&self.txn)
        .await?;

        // Finally, insert the fingerprint of the logical expression as well.
        let new_expr: L = new_model.into();
        let kind = new_expr.kind();

        // In order to calculate a correct fingerprint, we will want to use the IDs of the root
        // groups of the children instead of the child ID themselves.
        let mut rewrites = vec![];
        for &child_id in children {
            let root_id = self.get_root_group(child_id).await?;
            rewrites.push((child_id, root_id));
        }
        let hash = new_expr.rewrite(&rewrites).fingerprint();

        let fingerprint = fingerprint::ActiveModel {
            id: NotSet,
            logical_expression_id: Set(expr_id),
            kind: Set(kind),
            hash: Set(hash),
        };
        fingerprint::Entity::insert(fingerprint)
            .exec(&self.txn)
            .await?;

        Ok(Ok(LogicalExpressionId(expr_id)))
    }

    /// Adds a physical expression to an existing group via its ID.
    ///
    /// The caller is required to pass in a slice of [`GroupId`] that represent the child groups of
    /// the input expression.
    ///
    /// On successful insertion, returns the ID of the physical expression.
    ///
    /// # Errors
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error. Can also fail if
    /// insertion of the new physical expression or any of its child junction entries are not able
    /// to be inserted.
    pub async fn add_physical_expression_to_group(
        &mut self,
        group_id: GroupId,
        physical_expression: P,
        children: &[GroupId],
    ) -> OptimizerResult<PhysicalExpressionId> {
        // Check if the group actually exists.
        let _ = self.get_group(group_id).await?;

        // Insert the expression.
        let model: physical_expression::Model = physical_expression.into();
        let mut active_model = model.into_active_model();
        active_model.group_id = Set(group_id.0);
        active_model.id = NotSet;
        let new_model = active_model.insert(&self.txn).await?;

        // Insert the child groups of the expression into the junction / children table.
        physical_children::Entity::insert_many(children.iter().copied().map(|child_id| {
            physical_children::ActiveModel {
                physical_expression_id: Set(new_model.id),
                group_id: Set(child_id.0),
            }
        }))
        .on_empty_do_nothing()
        .exec(&self.txn)
        .await?;

        Ok(PhysicalExpressionId(new_model.id))
    }

    /// Checks if the given logical expression is a duplicate / already exists in the memo table.
    ///
    /// In order to prevent a large amount of duplicate work, the memo table must support duplicate
    /// expression detection.
    ///
    /// Returns `Some((group_id, expression_id))` if the memo table detects that the expression
    /// already exists, and `None` otherwise.
    ///
    /// This function assumes that the child groups of the expression are currently roots of their
    /// group sets. For example, if G1 and G2 should be merged, and G1 is the root, then the input
    /// expression should _not_ have G2 as a child, and should be replaced with G1.
    ///
    /// # Errors
    ///
    /// Returns a [`DbErr`] when a database operation fails.
    pub async fn is_duplicate_logical_expression(
        &self,
        logical_expression: &L,
        children: &[GroupId],
    ) -> OptimizerResult<Option<(GroupId, LogicalExpressionId)>> {
        let model: logical_expression::Model = logical_expression.clone().into();

        // Lookup all expressions that have the same fingerprint and kind. There may be false
        // positives, but we will check for those next.
        let kind = model.kind;

        // In order to calculate a correct fingerprint, we will want to use the IDs of the root
        // groups of the children instead of the child ID themselves.
        let mut rewrites = vec![];
        for &child_id in children {
            let root_id = self.get_root_group(child_id).await?;
            rewrites.push((child_id, root_id));
        }
        let fingerprint = logical_expression.rewrite(&rewrites).fingerprint();

        // Filter first by the fingerprint, and then the kind.
        // FIXME: The kind is already embedded into the fingerprint, so we may not actually need the
        // second filter?
        let potential_matches = fingerprint::Entity::find()
            .filter(fingerprint::Column::Hash.eq(fingerprint))
            .filter(fingerprint::Column::Kind.eq(kind))
            .all(&self.txn)
            .await?;

        if potential_matches.is_empty() {
            return Ok(None);
        }

        // Now that we have all of the expressions that match the given fingerprint, we need to
        // filter out all of the expressions that might have had the same fingerprint but are not
        // actually equivalent (hash collisions).
        let mut match_id = None;
        for potential_match in potential_matches {
            let expr_id = LogicalExpressionId(potential_match.logical_expression_id);
            let (group_id, expr) = self.get_logical_expression(expr_id).await?;

            // We need to add the root groups of the new expression to the rewrites vector.
            // TODO make this much more efficient by making rewrites a hash map, potentially im::HashMap.
            let mut rewrites = rewrites.clone();
            for child_id in expr.children() {
                let root_id = self.get_root_group(child_id).await?;
                rewrites.push((child_id, root_id));
            }

            // Check for an exact match after rewrites.
            if logical_expression
                .rewrite(&rewrites)
                .is_duplicate(&expr.rewrite(&rewrites))
            {
                match_id = Some((group_id, expr_id));

                // There should be at most one duplicate expression, so we can break here.
                break;
            }
        }

        Ok(match_id)
    }

    /// Adds a new group into the memo table via a logical expression, creating a new group if the
    /// logical expression does not already exist.
    ///
    /// The caller is required to pass in a slice of [`GroupId`] that represent the child groups of
    /// the input expression.
    ///
    /// If the expression already exists, then this function will return the [`GroupId`] of the
    /// parent group and the corresponding (already existing) [`LogicalExpressionId`]. It will also
    /// completely ignore the group ID field of the input expression as well as ignore the input
    /// slice of child groups.
    ///
    /// If the expression does not exist, this function will create a new group and a new
    /// expression, returning brand new IDs for both.
    ///
    /// # Errors
    ///
    /// Note that the return value is a [`Result`] wrapped in an [`OptimizerResult`]. The outer
    /// result is used for raising [`DbErr`] or other database/IO-related errors. The inner result
    /// is used for notifying the caller if the expression/group that they attempted to insert was a
    /// duplicate expression or not.
    pub async fn add_group(
        &mut self,
        logical_expression: L,
        children: &[GroupId],
    ) -> OptimizerResult<Result<(GroupId, LogicalExpressionId), (GroupId, LogicalExpressionId)>>
    {
        // Check if the expression already exists in the memo table.
        if let Some((group_id, existing_id)) = self
            .is_duplicate_logical_expression(&logical_expression, children)
            .await?
        {
            return Ok(Err((group_id, existing_id)));
        }

        // The expression does not exist yet, so we need to create a new group and new expression.
        let group = group::ActiveModel {
            status: Set(0), // `GroupStatus::InProgress` status.
            ..Default::default()
        };

        // Create the new group.
        let group_res = group::Entity::insert(group).exec(&self.txn).await?;
        let group_id = group_res.last_insert_id;

        // Insert the input expression into the newly created group.
        let expression: logical_expression::Model = logical_expression.clone().into();
        let mut active_expression = expression.into_active_model();
        active_expression.group_id = Set(group_id);
        active_expression.id = NotSet;
        let new_expression = active_expression.insert(&self.txn).await?;

        let group_id = new_expression.group_id;
        let expr_id = new_expression.id;

        // Insert the child groups of the expression into the junction / children table.
        logical_children::Entity::insert_many(children.iter().copied().map(|child_id| {
            logical_children::ActiveModel {
                logical_expression_id: Set(new_expression.id),
                group_id: Set(child_id.0),
            }
        }))
        .on_empty_do_nothing()
        .exec(&self.txn)
        .await?;

        // Finally, insert the fingerprint of the logical expression as well.
        let new_logical_expression: L = new_expression.into();
        let kind = new_logical_expression.kind();

        // In order to calculate a correct fingerprint, we will want to use the IDs of the root
        // groups of the children instead of the child ID themselves.
        let mut rewrites = vec![];
        for &child_id in children {
            let root_id = self.get_root_group(child_id).await?;
            rewrites.push((child_id, root_id));
        }
        let hash = new_logical_expression.rewrite(&rewrites).fingerprint();

        let fingerprint = fingerprint::ActiveModel {
            id: NotSet,
            logical_expression_id: Set(expr_id),
            kind: Set(kind),
            hash: Set(hash),
        };
        fingerprint::Entity::insert(fingerprint)
            .exec(&self.txn)
            .await?;

        Ok(Ok((GroupId(group_id), LogicalExpressionId(expr_id))))
    }

    /// Merges two groups sets together.
    ///
    /// If either of the input groups do not exist, returns a [`MemoError::UnknownGroup`] error.
    ///
    /// TODO write docs.
    /// TODO highly inefficient, need to understand metrics and performance testing.
    /// TODO Optimization: add rank / size into data structure
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn merge_groups(
        &mut self,
        left_group_id: GroupId,
        right_group_id: GroupId,
    ) -> OptimizerResult<GroupId> {
        // Without a rank / size field, we have no way of determining which set is better to merge
        // into the other. So we will arbitrarily choose to merge the left group into the right
        // group here. If rank is added in the future, then merge the smaller set into the larger.

        let left_root_id = self.get_root_group(left_group_id).await?;
        let left_root = self.get_group(left_root_id).await?;
        // A `None` next pointer means it should technically be pointing to itself.
        let left_next = left_root.next_id.unwrap_or(left_root_id.0);
        let mut active_left_root = left_root.into_active_model();

        let right_root_id = self.get_root_group(right_group_id).await?;
        let right_root = self.get_group(right_root_id).await?;
        // A `None` next pointer means it should technically be pointing to itself.
        let right_next = right_root.next_id.unwrap_or(right_root_id.0);
        let mut active_right_root = right_root.into_active_model();

        // Before we actually update the group records, We first need to generate new fingerprints
        // for every single expression that has a child group in the left set.
        // TODO make this more efficient, this code is doing double work from `get_group_set`.
        let group_set_ids = self.get_group_set(left_group_id).await?;
        let mut left_group_models = Vec::with_capacity(group_set_ids.len());
        for &group_id in &group_set_ids {
            left_group_models.push(self.get_group(group_id).await?);
        }

        // Retrieve every single expression that has a child group in the left set.
        let left_group_expressions: Vec<Vec<logical_expression::Model>> = left_group_models
            .load_many_to_many(
                logical_expression::Entity,
                logical_children::Entity,
                &self.txn,
            )
            .await?;

        // Need to replace every single occurrence of groups in the set with the new root.
        let rewrites: Vec<(GroupId, GroupId)> = group_set_ids
            .iter()
            .map(|&group_id| (group_id, right_root_id))
            .collect();

        // For each expression, generate a new fingerprint.
        let mut seen = HashSet::new();
        for model in left_group_expressions.into_iter().flatten() {
            let expr_id = model.id;

            // There may be duplicates in the expressions list.
            if seen.contains(&expr_id) {
                continue;
            } else {
                seen.insert(expr_id);
            }

            let logical_expression: L = model.into();
            let hash = logical_expression.rewrite(&rewrites).fingerprint();

            let fingerprint = fingerprint::ActiveModel {
                id: NotSet,
                logical_expression_id: Set(expr_id),
                kind: Set(logical_expression.kind()),
                hash: Set(hash),
            };
            fingerprint::Entity::insert(fingerprint)
                .exec(&self.txn)
                .await?;
        }

        // Update the left group root to point to the right group root.
        active_left_root.parent_id = Set(Some(right_root_id.0));

        // Swap the next pointers of each root to maintain the circular linked list.
        active_left_root.next_id = Set(Some(right_next));
        active_right_root.next_id = Set(Some(left_next));

        active_left_root.update(&self.txn).await?;
        active_right_root.update(&self.txn).await?;

        Ok(right_root_id)
    }
}

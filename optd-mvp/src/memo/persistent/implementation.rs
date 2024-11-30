//! This module contains the implementation of [`PersistentMemo`].
//!
//! TODO For parallelism, almost all of these methods need to be under transactions.
//! TODO Write more docs.
//! TODO Remove dead code.

#![allow(dead_code)]

use super::PersistentMemo;
use crate::{
    entities::*,
    expression::{LogicalExpression, PhysicalExpression},
    memo::{GroupId, LogicalExpressionId, MemoError, PhysicalExpressionId},
    OptimizerResult, DATABASE_URL,
};
use sea_orm::{
    entity::prelude::*,
    entity::{IntoActiveModel, NotSet, Set},
    Database,
};

impl PersistentMemo {
    /// Creates a new `PersistentMemo` struct by connecting to a database defined at
    /// [`DATABASE_URL`].
    pub async fn new() -> Self {
        Self {
            db: Database::connect(DATABASE_URL).await.unwrap(),
        }
    }

    /// Deletes all objects in the backing database.
    ///
    /// Since there is no asynchronous drop yet in Rust, in order to drop all objects in the
    /// database, the user must call this manually.
    pub async fn cleanup(&self) {
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
            cascades_group,
            fingerprint,
            logical_expression,
            logical_children,
            physical_expression,
            physical_children
        };
    }

    /// Retrieves a [`cascades_group::Model`] given its ID.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    ///
    /// FIXME: use an in-memory representation of a group instead.
    pub async fn get_group(&self, group_id: GroupId) -> OptimizerResult<cascades_group::Model> {
        Ok(cascades_group::Entity::find_by_id(group_id.0)
            .one(&self.db)
            .await?
            .ok_or(MemoError::UnknownGroup(group_id))?)
    }

    /// Retrieves a [`physical_expression::Model`] given a [`PhysicalExpressionId`].
    ///
    /// If the physical expression does not exist, returns a
    /// [`MemoError::UnknownPhysicalExpression`] error.
    pub async fn get_physical_expression(
        &self,
        physical_expression_id: PhysicalExpressionId,
    ) -> OptimizerResult<(GroupId, PhysicalExpression)> {
        // Lookup the entity in the database via the unique expression ID.
        let model = physical_expression::Entity::find_by_id(physical_expression_id.0)
            .one(&self.db)
            .await?
            .ok_or(MemoError::UnknownPhysicalExpression(physical_expression_id))?;

        let group_id = GroupId(model.group_id);
        let expr = model.into();

        Ok((group_id, expr))
    }

    /// Retrieves a [`logical_expression::Model`] given its [`LogicalExpressionId`].
    ///
    /// If the logical expression does not exist, returns a [`MemoError::UnknownLogicalExpression`]
    /// error.
    pub async fn get_logical_expression(
        &self,
        logical_expression_id: LogicalExpressionId,
    ) -> OptimizerResult<(GroupId, LogicalExpression)> {
        // Lookup the entity in the database via the unique expression ID.
        let model = logical_expression::Entity::find_by_id(logical_expression_id.0)
            .one(&self.db)
            .await?
            .ok_or(MemoError::UnknownLogicalExpression(logical_expression_id))?;

        let group_id = GroupId(model.group_id);
        let expr = model.into();

        Ok((group_id, expr))
    }

    /// Retrieves all of the logical expression "children" IDs of a group.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    ///
    /// FIXME: `find_related` does not work for some reason, have to use manual `filter`.
    pub async fn get_logical_children(
        &self,
        group_id: GroupId,
    ) -> OptimizerResult<Vec<LogicalExpressionId>> {
        // Search for expressions that have the given parent group ID.
        let children = logical_expression::Entity::find()
            .filter(logical_expression::Column::GroupId.eq(group_id.0))
            .all(&self.db)
            .await?
            .into_iter()
            .map(|m| LogicalExpressionId(m.id))
            .collect();

        Ok(children)
    }

    /// Retrieves all of the physical expression "children" IDs of a group.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    pub async fn get_physical_children(
        &self,
        group_id: GroupId,
    ) -> OptimizerResult<Vec<PhysicalExpressionId>> {
        // Search for expressions that have the given parent group ID.
        let children = physical_expression::Entity::find()
            .filter(physical_expression::Column::GroupId.eq(group_id.0))
            .all(&self.db)
            .await?
            .into_iter()
            .map(|m| PhysicalExpressionId(m.id))
            .collect();

        Ok(children)
    }

    /// Updates / replaces a group's best physical plan (winner). Optionally returns the previous
    /// winner's physical expression ID.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    ///
    /// FIXME: In the future, this should first check that we aren't overwriting a winner that was
    /// updated from another thread by comparing against the cost of the plan.
    pub async fn update_group_winner(
        &self,
        group_id: GroupId,
        physical_expression_id: PhysicalExpressionId,
    ) -> OptimizerResult<Option<PhysicalExpressionId>> {
        // First retrieve the group record.
        let mut group = self.get_group(group_id).await?.into_active_model();

        // Update the group to point to the new winner.
        let old_id = group.winner;
        group.winner = Set(Some(physical_expression_id.0));
        group.update(&self.db).await?;

        // Note that the `unwrap` here is unwrapping the `ActiveValue`, not the `Option`.
        let old = old_id.unwrap().map(PhysicalExpressionId);
        Ok(old)
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
    /// FIXME Check that all of the children are reduced groups?
    pub async fn add_logical_expression_to_group(
        &self,
        group_id: GroupId,
        logical_expression: LogicalExpression,
        children: &[GroupId],
    ) -> OptimizerResult<Result<LogicalExpressionId, LogicalExpressionId>> {
        // Check if the expression already exists anywhere in the memo table.
        if let Some(existing_id) = self
            .is_duplicate_logical_expression(&logical_expression)
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
        let new_model = active_model.insert(&self.db).await?;

        let expr_id = new_model.id;

        // Insert the child groups of the expression into the junction / children table.
        logical_children::Entity::insert_many(children.iter().copied().map(|child_id| {
            logical_children::ActiveModel {
                logical_expression_id: Set(expr_id),
                group_id: Set(child_id.0),
            }
        }))
        .on_empty_do_nothing()
        .exec(&self.db)
        .await?;

        // Finally, insert the fingerprint of the logical expression as well.
        let new_expr: LogicalExpression = new_model.into();
        let kind = new_expr.kind();
        let hash = new_expr.fingerprint();

        let fingerprint = fingerprint::ActiveModel {
            id: NotSet,
            logical_expression_id: Set(expr_id),
            kind: Set(kind),
            hash: Set(hash),
        };
        let _ = fingerprint::Entity::insert(fingerprint)
            .exec(&self.db)
            .await?;

        Ok(Ok(LogicalExpressionId(expr_id)))
    }

    /// Adds a physical expression to an existing group via its ID.
    ///
    /// The caller is required to pass in a slice of [`GroupId`] that represent the child groups of
    /// the input expression.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    ///
    /// On successful insertion, returns the ID of the physical expression.
    pub async fn add_physical_expression_to_group(
        &self,
        group_id: GroupId,
        physical_expression: PhysicalExpression,
        children: &[GroupId],
    ) -> OptimizerResult<PhysicalExpressionId> {
        // Check if the group actually exists.
        let _ = self.get_group(group_id).await?;

        // Insert the expression.
        let model: physical_expression::Model = physical_expression.into();
        let mut active_model = model.into_active_model();
        active_model.group_id = Set(group_id.0);
        active_model.id = NotSet;
        let new_model = active_model.insert(&self.db).await?;

        // Insert the child groups of the expression into the junction / children table.
        physical_children::Entity::insert_many(children.iter().copied().map(|child_id| {
            physical_children::ActiveModel {
                physical_expression_id: Set(new_model.id),
                group_id: Set(child_id.0),
            }
        }))
        .on_empty_do_nothing()
        .exec(&self.db)
        .await?;

        Ok(PhysicalExpressionId(new_model.id))
    }

    /// Checks if the given logical expression is a duplicate / already exists in the memo table.
    ///
    /// In order to prevent a large amount of duplicate work, the memo table must support duplicate
    /// expression detection.
    ///
    /// Returns `Some(expression_id)` if the memo table detects that the expression already exists,
    /// and `None` otherwise.
    ///
    /// This function assumes that the child groups of the expression are currently roots of their
    /// group sets. For example, if G1 and G2 should be merged, and G1 is the root, then the input
    /// expression should _not_ have G2 as a child, and should be replaced with G1.
    ///
    /// TODO Check that all of the children are root groups? How to do this?
    pub async fn is_duplicate_logical_expression(
        &self,
        logical_expression: &LogicalExpression,
    ) -> OptimizerResult<Option<LogicalExpressionId>> {
        let model: logical_expression::Model = logical_expression.clone().into();

        // Lookup all expressions that have the same fingerprint and kind. There may be false
        // positives, but we will check for those next.
        let kind = model.kind;
        let fingerprint = logical_expression.fingerprint();

        // Filter first by the fingerprint, and then the kind.
        // FIXME: The kind is already embedded into the fingerprint, so we may not actually need the
        // second filter?
        let potential_matches = fingerprint::Entity::find()
            .filter(fingerprint::Column::Hash.eq(fingerprint))
            .filter(fingerprint::Column::Kind.eq(kind))
            .all(&self.db)
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
            let (_, expr) = self.get_logical_expression(expr_id).await?;

            // Check for an exact match.
            if &expr == logical_expression {
                match_id = Some(expr_id);

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
    /// FIXME Check that all of the children are reduced groups?
    pub async fn add_group(
        &self,
        logical_expression: LogicalExpression,
        children: &[GroupId],
    ) -> OptimizerResult<Result<(GroupId, LogicalExpressionId), (GroupId, LogicalExpressionId)>>
    {
        // Check if the expression already exists in the memo table.
        if let Some(existing_id) = self
            .is_duplicate_logical_expression(&logical_expression)
            .await?
        {
            let (group_id, _expr) = self.get_logical_expression(existing_id).await?;
            return Ok(Err((group_id, existing_id)));
        }

        // The expression does not exist yet, so we need to create a new group and new expression.
        let group = cascades_group::ActiveModel {
            winner: Set(None),
            is_optimized: Set(false),
            ..Default::default()
        };

        // Create the new group.
        let res = cascades_group::Entity::insert(group).exec(&self.db).await?;

        // Insert the input expression into the newly created group.
        let model: logical_expression::Model = logical_expression.clone().into();
        let mut active_model = model.into_active_model();
        active_model.group_id = Set(res.last_insert_id);
        active_model.id = NotSet;
        let new_model = active_model.insert(&self.db).await?;

        let group_id = new_model.group_id;
        let expr_id = new_model.id;

        // Insert the child groups of the expression into the junction / children table.
        logical_children::Entity::insert_many(children.iter().copied().map(|child_id| {
            logical_children::ActiveModel {
                logical_expression_id: Set(new_model.id),
                group_id: Set(child_id.0),
            }
        }))
        .on_empty_do_nothing()
        .exec(&self.db)
        .await?;

        // Finally, insert the fingerprint of the logical expression as well.
        let new_expr: LogicalExpression = new_model.into();
        let kind = new_expr.kind();
        let hash = new_expr.fingerprint();

        let fingerprint = fingerprint::ActiveModel {
            id: NotSet,
            logical_expression_id: Set(expr_id),
            kind: Set(kind),
            hash: Set(hash),
        };
        let _ = fingerprint::Entity::insert(fingerprint)
            .exec(&self.db)
            .await?;

        Ok(Ok((GroupId(group_id), LogicalExpressionId(expr_id))))
    }
}

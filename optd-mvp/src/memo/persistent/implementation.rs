//! This module contains the implementation of the [`Memo`] trait for [`PersistentMemo`].

use super::*;
use crate::{
    hash_expression,
    memo::{Memo, MemoError},
    OptimizerResult,
};

impl Memo for PersistentMemo {
    type Group = cascades_group::Model;
    type GroupId = i32;
    type LogicalExpression = logical_expression::Model;
    type LogicalExpressionId = i32;
    type PhysicalExpression = physical_expression::Model;
    type PhysicalExpressionId = i32;

    async fn get_group(&self, group_id: Self::GroupId) -> OptimizerResult<Self::Group> {
        Ok(CascadesGroup::find_by_id(group_id)
            .one(&self.db)
            .await?
            .ok_or(MemoError::UnknownGroup(group_id))?)
    }

    async fn get_logical_expression(
        &self,
        logical_expression_id: Self::LogicalExpressionId,
    ) -> OptimizerResult<Self::LogicalExpression> {
        Ok(LogicalExpression::find_by_id(logical_expression_id)
            .one(&self.db)
            .await?
            .ok_or(MemoError::UnknownLogicalExpression(logical_expression_id))?)
    }

    async fn get_physical_expression(
        &self,
        physical_expression_id: Self::PhysicalExpressionId,
    ) -> OptimizerResult<Self::PhysicalExpression> {
        Ok(PhysicalExpression::find_by_id(physical_expression_id)
            .one(&self.db)
            .await?
            .ok_or(MemoError::UnknownPhysicalExpression(physical_expression_id))?)
    }

    async fn get_logical_children(
        &self,
        group_id: Self::GroupId,
    ) -> OptimizerResult<Vec<Self::LogicalExpressionId>> {
        // First retrieve the group record, and then find all related logical expressions.
        Ok(self
            .get_group(group_id)
            .await?
            .find_related(LogicalChildren)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|m| m.logical_expression_id)
            .collect())
    }

    async fn get_physical_children(
        &self,
        group_id: Self::GroupId,
    ) -> OptimizerResult<Vec<Self::PhysicalExpressionId>> {
        // First retrieve the group record, and then find all related physical expressions.
        Ok(self
            .get_group(group_id)
            .await?
            .find_related(PhysicalChildren)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|m| m.physical_expression_id)
            .collect())
    }

    /// FIXME Check that all of the children are root groups?
    async fn is_duplicate_logical_expression(
        &self,
        logical_expression: &Self::LogicalExpression,
    ) -> OptimizerResult<Option<Self::LogicalExpressionId>> {
        // Lookup all expressions that have the same fingerprint and kind. There may be false
        // positives, but we will check for those next.
        let kind = logical_expression.kind;
        let fingerprint = hash_expression(kind, &logical_expression.data);

        let potential_matches = Fingerprint::find()
            .filter(fingerprint::Column::Hash.eq(fingerprint))
            .filter(fingerprint::Column::Kind.eq(kind))
            .all(&self.db)
            .await?;

        if potential_matches.is_empty() {
            return Ok(None);
        }

        let mut match_id = None;
        for potential_match in potential_matches {
            let expr_id = potential_match.logical_expression_id;
            let expr = self.get_logical_expression(expr_id).await?;

            if expr.data == logical_expression.data {
                // There should be at most one duplicate expression.
                match_id = Some(expr_id);
                break;
            }
        }

        Ok(match_id)
    }

    /// FIXME: In the future, this should first check that we aren't overwriting a winner that was
    /// updated from another thread.
    async fn update_group_winner(
        &self,
        group_id: Self::GroupId,
        physical_expression_id: Self::PhysicalExpressionId,
    ) -> OptimizerResult<Option<Self::PhysicalExpressionId>> {
        // First retrieve the group record, and then use an `ActiveModel` to update it.
        let mut group = self.get_group(group_id).await?.into_active_model();
        let old_id = group.winner;

        group.winner = Set(Some(physical_expression_id));
        group.update(&self.db).await?;

        // The old value must be set (`None` still means it has been set).
        let old = old_id.unwrap();
        Ok(old)
    }

    async fn add_physical_expression_to_group(
        &self,
        group_id: Self::GroupId,
        physical_expression: Self::PhysicalExpression,
        children: &[Self::GroupId],
    ) -> OptimizerResult<Self::PhysicalExpressionId> {
        if physical_expression.group_id != group_id {
            Err(MemoError::InvalidExpression)?
        }

        // Check if the group actually exists.
        let _ = self.get_group(group_id).await?;

        // Insert the child groups of the expression into the junction / children table.
        if !children.is_empty() {
            PhysicalChildren::insert_many(children.iter().copied().map(|group_id| {
                physical_children::ActiveModel {
                    physical_expression_id: Set(physical_expression.id),
                    group_id: Set(group_id),
                }
            }))
            .exec(&self.db)
            .await?;
        }

        // Insert the expression.
        let res = physical_expression
            .into_active_model()
            .insert(&self.db)
            .await?;

        Ok(res.id)
    }

    /// FIXME Check that all of the children are reduced groups?
    async fn add_logical_expression_to_group(
        &self,
        group_id: Self::GroupId,
        logical_expression: Self::LogicalExpression,
        children: &[Self::GroupId],
    ) -> OptimizerResult<Result<Self::LogicalExpressionId, Self::LogicalExpressionId>> {
        if logical_expression.group_id != group_id {
            Err(MemoError::InvalidExpression)?
        }

        // Check if the expression already exists in the memo table.
        if let Some(existing_id) = self
            .is_duplicate_logical_expression(&logical_expression)
            .await?
        {
            return Ok(Err(existing_id));
        }

        // Check if the group actually exists.
        let _ = self.get_group(group_id).await?;

        // Insert the child groups of the expression into the junction / children table.
        if !children.is_empty() {
            LogicalChildren::insert_many(children.iter().copied().map(|group_id| {
                logical_children::ActiveModel {
                    logical_expression_id: Set(logical_expression.id),
                    group_id: Set(group_id),
                }
            }))
            .exec(&self.db)
            .await?;
        }

        // Insert the expression.
        let res = logical_expression
            .into_active_model()
            .insert(&self.db)
            .await?;

        Ok(Ok(res.id))
    }

    /// FIXME Check that all of the children are reduced groups?
    async fn add_logical_expression(
        &self,
        logical_expression: Self::LogicalExpression,
        children: &[Self::GroupId],
    ) -> OptimizerResult<
        Result<
            (Self::GroupId, Self::LogicalExpressionId),
            (Self::GroupId, Self::LogicalExpressionId),
        >,
    > {
        // Check if the expression already exists in the memo table.
        if let Some(existing_id) = self
            .is_duplicate_logical_expression(&logical_expression)
            .await?
        {
            let expr = self.get_logical_expression(existing_id).await?;
            return Ok(Err((expr.group_id, expr.id)));
        }

        // The expression does not exist yet, so we need to create a new group and new expression.
        let group = cascades_group::ActiveModel {
            winner: Set(None),
            is_optimized: Set(false),
            ..Default::default()
        };

        // Create a new group.
        let res = cascades_group::Entity::insert(group).exec(&self.db).await?;

        // Insert the input expression with the correct `group_id`.
        let mut new_expr = logical_expression.into_active_model();
        new_expr.group_id = Set(res.last_insert_id);
        new_expr.id = NotSet;
        let new_expr = new_expr.insert(&self.db).await?;

        // Insert the child groups of the expression into the junction / children table.
        if !children.is_empty() {
            LogicalChildren::insert_many(children.iter().copied().map(|group_id| {
                logical_children::ActiveModel {
                    logical_expression_id: Set(new_expr.id),
                    group_id: Set(group_id),
                }
            }))
            .exec(&self.db)
            .await?;
        }

        // Insert the fingerprint of the logical expression.
        let hash = hash_expression(new_expr.kind, &new_expr.data);
        let fingerprint = fingerprint::ActiveModel {
            id: NotSet,
            logical_expression_id: Set(new_expr.id),
            kind: Set(new_expr.kind),
            hash: Set(hash),
        };
        let _ = fingerprint::Entity::insert(fingerprint)
            .exec(&self.db)
            .await?;

        Ok(Ok((new_expr.group_id, new_expr.id)))
    }
}

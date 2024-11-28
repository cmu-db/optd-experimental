use crate::{
    entities::{prelude::*, *},
    memo::{Memo, MemoError},
    OptimizerResult, DATABASE_URL,
};
use sea_orm::*;

/// A persistent memo table, backed by a database on disk.
///
/// TODO more docs.
pub struct PersistentMemo {
    /// This `PersistentMemo` is reliant on the SeaORM [`DatabaseConnection`] that stores all of the
    /// objects needed for query optimization.
    db: DatabaseConnection,
}

impl PersistentMemo {
    /// TODO remove dead code and write docs.
    #[allow(dead_code)]
    pub async fn new() -> Self {
        Self {
            db: Database::connect(DATABASE_URL).await.unwrap(),
        }
    }
}

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

    async fn add_logical_expression_to_group(
        &self,
        group_id: Self::GroupId,
        logical_expression: Self::LogicalExpression,
        children: &[Self::GroupId],
    ) -> OptimizerResult<()> {
        if logical_expression.group_id != group_id {
            Err(MemoError::InvalidExpression)?
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
        let _ = logical_expression
            .into_active_model()
            .insert(&self.db)
            .await?;

        Ok(())
    }

    async fn add_physical_expression_to_group(
        &self,
        group_id: Self::GroupId,
        physical_expression: Self::PhysicalExpression,
        children: &[Self::GroupId],
    ) -> OptimizerResult<()> {
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
        let _ = physical_expression
            .into_active_model()
            .insert(&self.db)
            .await?;

        Ok(())
    }

    async fn add_logical_expression(
        &self,
        logical_expression: Self::LogicalExpression,
        children: &[Self::GroupId],
    ) -> OptimizerResult<(Self::GroupId, Self::LogicalExpressionId)> {
        // Lookup all expressions that have the same fingerprint. There may be false positives, but
        // we will check for those later.
        let fingerprint = logical_expression.fingerprint;
        let potential_matches = LogicalExpression::find()
            .filter(logical_expression::Column::Fingerprint.eq(fingerprint))
            .all(&self.db)
            .await?;

        // Of the expressions that have the same fingerprint, check if there already exists an
        // expression that is exactly identical to the input expression.
        let mut matches: Vec<_> = potential_matches
            .into_iter()
            .filter(|expr| expr == &logical_expression)
            .collect();
        assert!(
            matches.len() <= 1,
            "there cannot be more than 1 exact logical expression match"
        );

        // The expression already exists, so return its data.
        if !matches.is_empty() {
            let existing_expression = matches
                .pop()
                .expect("we just checked that an element exists");

            return Ok((existing_expression.group_id, existing_expression.id));
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

        Ok((new_expr.group_id, new_expr.id))
    }
}

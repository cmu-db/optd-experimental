use crate::{
    entities::{prelude::*, *},
    BackendManager, {Memo, MemoError, StorageResult},
};
use sea_orm::*;

impl Memo for BackendManager {
    type Group = cascades_group::Model;
    type GroupId = i32;
    type LogicalExpression = logical_expression::Model;
    type LogicalExpressionId = i32;
    type PhysicalExpression = physical_expression::Model;
    type PhysicalExpressionId = i32;

    async fn get_group(&self, group_id: Self::GroupId) -> StorageResult<Self::Group> {
        Ok(CascadesGroup::find_by_id(group_id)
            .one(&self.db)
            .await?
            .ok_or(MemoError::UnknownGroup)?)
    }

    async fn get_all_groups(&self) -> StorageResult<Vec<Self::Group>> {
        Ok(CascadesGroup::find().all(&self.db).await?)
    }

    async fn get_logical_expression(
        &self,
        logical_expression_id: Self::LogicalExpressionId,
    ) -> StorageResult<Self::LogicalExpression> {
        Ok(LogicalExpression::find_by_id(logical_expression_id)
            .one(&self.db)
            .await?
            .ok_or(MemoError::UnknownLogicalExpression)?)
    }

    async fn get_physical_expression(
        &self,
        physical_expression_id: Self::PhysicalExpressionId,
    ) -> StorageResult<Self::PhysicalExpression> {
        Ok(PhysicalExpression::find_by_id(physical_expression_id)
            .one(&self.db)
            .await?
            .ok_or(MemoError::UnknownPhysicalExpression)?)
    }

    async fn get_group_from_logical_expression(
        &self,
        logical_expression_id: Self::LogicalExpressionId,
    ) -> StorageResult<Self::GroupId> {
        // Find the logical expression and then look up the field.
        Ok(self
            .get_logical_expression(logical_expression_id)
            .await?
            .group_id)
    }

    async fn get_group_from_physical_expression(
        &self,
        physical_expression_id: Self::PhysicalExpressionId,
    ) -> StorageResult<Self::GroupId> {
        Ok(self
            .get_physical_expression(physical_expression_id)
            .await?
            .group_id)
    }

    async fn get_group_logical_expressions(
        &self,
        group_id: Self::GroupId,
    ) -> StorageResult<Vec<Self::LogicalExpression>> {
        // First retrieve the group record, and then find all related logical expressions.
        Ok(self
            .get_group(group_id)
            .await?
            .find_related(LogicalExpression)
            .all(&self.db)
            .await?)
    }

    async fn get_group_physical_expressions(
        &self,
        group_id: Self::GroupId,
    ) -> StorageResult<Vec<Self::PhysicalExpression>> {
        // First retrieve the group record, and then find all related physical expressions.
        Ok(self
            .get_group(group_id)
            .await?
            .find_related(PhysicalExpression)
            .all(&self.db)
            .await?)
    }

    async fn get_winner(
        &self,
        group_id: Self::GroupId,
    ) -> StorageResult<Option<Self::PhysicalExpressionId>> {
        Ok(self.get_group(group_id).await?.latest_winner)
    }

    async fn update_group_winner(
        &self,
        group_id: Self::GroupId,
        physical_expression_id: Self::PhysicalExpressionId,
    ) -> StorageResult<Option<Self::PhysicalExpressionId>> {
        // First retrieve the group record, and then use an `ActiveModel` to update it.
        let mut group = self.get_group(group_id).await?.into_active_model();
        let old_id = group.latest_winner;

        group.latest_winner = Set(Some(physical_expression_id));
        group.update(&self.db).await?;

        // The old value must be set (`None` still means it has been set).
        let old = old_id.unwrap();
        Ok(old)
    }

    async fn add_logical_expression_to_group(
        &self,
        group_id: Self::GroupId,
        logical_expression: Self::LogicalExpression,
        children: Vec<Self::LogicalExpressionId>,
    ) -> StorageResult<()> {
        if logical_expression.group_id != group_id {
            Err(MemoError::InvalidExpression)?
        }

        // Check if the group actually exists.
        let _ = self.get_group(group_id).await?;

        // Insert the expression.
        let _ = logical_expression
            .into_active_model()
            .insert(&self.db)
            .await?;

        todo!("add the children of the logical expression into the children table")
    }

    async fn add_physical_expression_to_group(
        &self,
        group_id: Self::GroupId,
        physical_expression: Self::PhysicalExpression,
        children: Vec<Self::LogicalExpressionId>,
    ) -> StorageResult<()> {
        if physical_expression.group_id != group_id {
            Err(MemoError::InvalidExpression)?
        }

        // Check if the group actually exists.
        let _ = self.get_group(group_id).await?;

        // Insert the expression.
        let _ = physical_expression
            .into_active_model()
            .insert(&self.db)
            .await?;

        todo!("add the children of the logical expression into the children table")
    }

    /// Note that in this function, we ignore the group ID that the logical expression contains.
    async fn add_logical_expression(
        &self,
        expression: Self::LogicalExpression,
        children: Vec<Self::LogicalExpressionId>,
    ) -> StorageResult<(Self::GroupId, Self::LogicalExpressionId)> {
        // Lookup all expressions that have the same fingerprint. There may be false positives, but
        // we will check for those later.
        let fingerprint = expression.fingerprint;
        let potential_matches = LogicalExpression::find()
            .filter(logical_expression::Column::Fingerprint.eq(fingerprint))
            .all(&self.db)
            .await?;

        // Of the expressions that have the same fingerprint, check if there already exists an
        // expression that is exactly identical to the input expression.
        let mut matches: Vec<_> = potential_matches
            .into_iter()
            .filter(|expr| expr == &expression)
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
            latest_winner: Set(None),
            in_progress: Set(false),
            is_optimized: Set(false),
            ..Default::default()
        };

        // Insert a new group.
        let res = cascades_group::Entity::insert(group).exec(&self.db).await?;

        // Insert the input expression with the correct `group_id`.
        let mut new_expr = expression.into_active_model();
        new_expr.group_id = Set(res.last_insert_id);
        let new_expr = new_expr.insert(&self.db).await?;

        Ok((new_expr.group_id, new_expr.id))
    }
}

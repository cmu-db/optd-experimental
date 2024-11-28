//! This module defines the [`Memo`] trait, which defines shared behavior of all memo table that can
//! be used for query optimization in the Cascades framework.

use crate::OptimizerResult;
use thiserror::Error;

/// The different kinds of errors that might occur while running operations on a memo table.
#[derive(Error, Debug)]
pub enum MemoError {
    #[error("unknown group ID {0}")]
    UnknownGroup(i32),
    #[error("unknown logical expression ID {0}")]
    UnknownLogicalExpression(i32),
    #[error("unknown physical expression ID {0}")]
    UnknownPhysicalExpression(i32),
    #[error("invalid expression encountered")]
    InvalidExpression,
}

/// A trait representing an implementation of a memoization table.
///
/// Note that we use [`trait_variant`] here in order to add bounds on every method.
/// See this [blog post](
/// https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#async-fn-in-public-traits)
/// for more information.
///
/// TODO remove dead code.
#[allow(dead_code)]
#[trait_variant::make(Send)]
pub trait Memo {
    /// A type representing a group in the Cascades framework.
    type Group;
    /// A type representing a unique identifier for a group.
    type GroupId;
    /// A type representing a logical expression.
    type LogicalExpression;
    /// A type representing a unique identifier for a logical expression.
    type LogicalExpressionId;
    /// A type representing a physical expression.
    type PhysicalExpression;
    /// A type representing a unique identifier for a physical expression.
    type PhysicalExpressionId;

    /// Retrieves a [`Self::Group`] given a [`Self::GroupId`].
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    async fn get_group(&self, group_id: Self::GroupId) -> OptimizerResult<Self::Group>;

    /// Retrieves a [`Self::LogicalExpression`] given a [`Self::LogicalExpressionId`].
    ///
    /// If the logical expression does not exist, returns a [`MemoError::UnknownLogicalExpression`]
    /// error.
    async fn get_logical_expression(
        &self,
        logical_expression_id: Self::LogicalExpressionId,
    ) -> OptimizerResult<Self::LogicalExpression>;

    /// Retrieves a [`Self::PhysicalExpression`] given a [`Self::PhysicalExpressionId`].
    ///
    /// If the physical expression does not exist, returns a
    /// [`MemoError::UnknownPhysicalExpression`] error.
    async fn get_physical_expression(
        &self,
        physical_expression_id: Self::PhysicalExpressionId,
    ) -> OptimizerResult<Self::PhysicalExpression>;

    /// Retrieves all of the logical expression "children" IDs of a group.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    async fn get_logical_children(
        &self,
        group_id: Self::GroupId,
    ) -> OptimizerResult<Vec<Self::LogicalExpressionId>>;

    /// Retrieves all of the physical expression "children" IDs of a group.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    async fn get_physical_children(
        &self,
        group_id: Self::GroupId,
    ) -> OptimizerResult<Vec<Self::PhysicalExpressionId>>;

    /// Checks if a given logical expression is a duplicate / already exists in the memo table.
    ///
    /// In order to prevent a large amount of duplicate work, the memo table must support duplicate
    /// expression detection.
    ///
    /// Returns `Some(expression_id)` if the memo table detects that the expression already exists,
    /// and `None` otherwise.
    async fn is_duplicate_logical_expression(
        &self,
        logical_expression: &Self::LogicalExpression,
    ) -> OptimizerResult<Option<Self::LogicalExpressionId>>;

    /// Updates / replaces a group's best physical plan (winner). Optionally returns the previous
    /// winner's physical expression ID.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    async fn update_group_winner(
        &self,
        group_id: Self::GroupId,
        physical_expression_id: Self::PhysicalExpressionId,
    ) -> OptimizerResult<Option<Self::PhysicalExpressionId>>;

    /// Adds a physical expression to an existing group via its [`Self::GroupId`].
    ///
    /// The caller is required to pass in a slice of `GroupId` that represent the child groups of
    /// the input expression.
    ///
    /// The caller is also required to set the `group_id` field of the input `physical_expression`
    /// to be equal to `group_id`, otherwise this function will return a
    /// [`MemoError::InvalidExpression`] error.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    ///
    /// On successful insertion, returns the ID of the physical expression.
    async fn add_physical_expression_to_group(
        &self,
        group_id: Self::GroupId,
        physical_expression: Self::PhysicalExpression,
        children: &[Self::GroupId],
    ) -> OptimizerResult<Self::PhysicalExpressionId>;

    /// Adds a logical expression to an existing group via its [`Self::GroupId`].
    ///
    /// The caller is required to pass in a slice of `GroupId` that represent the child groups of
    /// the input expression.
    ///
    /// The caller is also required to set the `group_id` field of the input `logical_expression`
    /// to be equal to `group_id`, otherwise this function will return a
    /// [`MemoError::InvalidExpression`] error.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    ///
    /// If the memo table detects that the input logical expression is a duplicate expression, it
    /// will **not** insert the expression into the memo table. Instead, it will return an
    /// `Ok(Err(expression_id))`, which is a unique identifier of the expression that the input is a
    /// duplicate of. The caller can use this ID to retrieve the group the original belongs to.
    ///
    /// If the memo table detects that the input is unique, it will insert the expression into the
    /// input group and return an `Ok(Ok(expression_id))`.
    async fn add_logical_expression_to_group(
        &self,
        group_id: Self::GroupId,
        logical_expression: Self::LogicalExpression,
        children: &[Self::GroupId],
    ) -> OptimizerResult<Result<Self::LogicalExpressionId, Self::LogicalExpressionId>>;

    /// Adds a new logical expression into the memo table, creating a new group if the expression
    /// does not already exist.
    ///
    /// The caller is required to pass in a slice of `GroupId` that represent the child groups of
    /// the input expression.
    ///
    /// The [`Self::LogicalExpression`] type should have some sort of mechanism for checking if
    /// the expression has been seen before, and if it has already been created, then the parent
    /// group ID should also be retrievable.
    ///
    /// If the expression already exists, then this function will return the [`Self::GroupId`] of
    /// the parent group and the corresponding (already existing) [`Self::LogicalExpressionId`]. It
    /// will also completely ignore the group ID field of the input expression as well as ignore the
    /// input slice of child groups.
    ///
    /// If the expression does not exist, this function will create a new group and a new
    /// expression, returning brand new IDs for both.
    async fn add_logical_expression(
        &self,
        expression: Self::LogicalExpression,
        children: &[Self::LogicalExpressionId],
    ) -> OptimizerResult<
        Result<
            (Self::GroupId, Self::LogicalExpressionId),
            (Self::GroupId, Self::LogicalExpressionId),
        >,
    >;
}

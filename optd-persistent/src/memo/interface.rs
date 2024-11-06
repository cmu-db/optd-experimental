use crate::StorageResult;

/// A trait representing an implementation of a memoization table.
///
/// Note that we use [`trait_variant`] here in order to add bounds on every method.
/// See this [blog post](
/// https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#async-fn-in-public-traits)
/// for more information.
///
/// TODO Figure out for each when to get the ID of a record or the entire record itself.
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
    async fn get_group(&self, group_id: Self::GroupId) -> StorageResult<Self::Group>;

    /// Retrieves all group IDs that are stored in the memo table.
    async fn get_all_groups(&self) -> StorageResult<Vec<Self::Group>>;

    /// Retrieves a [`Self::LogicalExpression`] given a [`Self::LogicalExpressionId`].
    ///
    /// If the logical expression does not exist, returns a [`MemoError::UnknownLogicalExpression`]
    /// error.
    async fn get_logical_expression(
        &self,
        logical_expression_id: Self::LogicalExpressionId,
    ) -> StorageResult<Self::LogicalExpression>;

    /// Retrieves a [`Self::PhysicalExpression`] given a [`Self::PhysicalExpressionId`].
    ///
    /// If the physical expression does not exist, returns a
    /// [`MemoError::UnknownPhysicalExpression`] error.
    async fn get_physical_expression(
        &self,
        physical_expression_id: Self::PhysicalExpressionId,
    ) -> StorageResult<Self::PhysicalExpression>;

    /// Retrieves the parent group ID of a logical expression given its expression ID.
    ///
    /// If the logical expression does not exist, returns a [`MemoError::UnknownLogicalExpression`]
    /// error.
    async fn get_group_from_logical_expression(
        &self,
        logical_expression_id: Self::LogicalExpressionId,
    ) -> StorageResult<Self::GroupId>;

    /// Retrieves the parent group ID of a logical expression given its expression ID.
    ///
    /// If the physical expression does not exist, returns a
    /// [`MemoError::UnknownPhysicalExpression`] error.
    async fn get_group_from_physical_expression(
        &self,
        physical_expression_id: Self::PhysicalExpressionId,
    ) -> StorageResult<Self::GroupId>;

    /// Retrieves all of the logical expression "children" of a group.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    async fn get_group_logical_expressions(
        &self,
        group_id: Self::GroupId,
    ) -> StorageResult<Vec<Self::LogicalExpression>>;

    /// Retrieves all of the physical expression "children" of a group.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    async fn get_group_physical_expressions(
        &self,
        group_id: Self::GroupId,
    ) -> StorageResult<Vec<Self::PhysicalExpression>>;

    /// Retrieves the best physical query plan (winner) for a given group.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    async fn get_winner(
        &self,
        group_id: Self::GroupId,
    ) -> StorageResult<Option<Self::PhysicalExpressionId>>;

    /// Updates / replaces a group's best physical plan (winner). Optionally returns the previous
    /// winner's physical expression ID.
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    async fn update_group_winner(
        &self,
        group_id: Self::GroupId,
        physical_expression_id: Self::PhysicalExpressionId,
    ) -> StorageResult<Option<Self::PhysicalExpressionId>>;

    /// Adds a logical expression to an existing group via its [`Self::GroupId`].
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    async fn add_logical_expression_to_group(
        &self,
        group_id: Self::GroupId,
        logical_expression: Self::LogicalExpression,
    ) -> StorageResult<()>;

    /// Adds a physical expression to an existing group via its [`Self::GroupId`].
    ///
    /// If the group does not exist, returns a [`MemoError::UnknownGroup`] error.
    async fn add_physical_expression_to_group(
        &self,
        group_id: Self::GroupId,
        physical_expression: Self::PhysicalExpression,
    ) -> StorageResult<()>;

    /// Adds a new logical expression into the memo table, creating a new group if the expression
    /// does not already exist.
    ///
    /// The [`Self::LogicalExpression`] type should have some sort of mechanism for checking if
    /// the expression has been seen before, and if it has already been created, then the parent
    /// group ID should also be retrievable.
    ///
    /// If the expression already exists, then this function will return the [`Self::GroupId`] of
    /// the parent group and the corresponding (already existing) [`Self::LogicalExpressionId`].
    ///
    /// If the expression does not exist, this function will create a new group and a new
    /// expression, returning brand new IDs for both.
    async fn add_logical_expression(
        &self,
        expression: Self::LogicalExpression,
    ) -> StorageResult<(Self::GroupId, Self::LogicalExpressionId)>;
}

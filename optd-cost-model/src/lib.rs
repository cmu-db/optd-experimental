use common::{
    nodes::{ArcPredicateNode, PhysicalNodeType},
    types::{AttrId, EpochId, ExprId, GroupId, TableId},
};
use optd_persistent::{
    cost_model::interface::{Stat, StatType},
    BackendError,
};

pub mod common;
pub mod cost;
pub mod cost_model;
pub mod memo_ext;
pub mod stats;
pub mod storage;
pub mod utils;

pub enum StatValue {
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct ComputeCostContext {
    pub group_id: GroupId,
    pub expr_id: ExprId,
    pub children_group_ids: Vec<GroupId>,
}

#[derive(Default, Clone, Debug, PartialOrd, PartialEq)]
pub struct Cost(pub Vec<f64>);

/// Estimated statistic calculated by the cost model.
/// It is the estimated output row count of the targeted expression.
pub struct EstimatedStatistic(pub u64);

pub type CostModelResult<T> = Result<T, CostModelError>;

#[derive(Debug)]
pub enum SemanticError {
    // TODO: Add more error types
    UnknownStatisticType,
    VersionedStatisticNotFound,
    AttributeNotFound(TableId, i32), // (table_id, attribute_base_index)
}

#[derive(Debug)]
pub enum CostModelError {
    // TODO: Add more error types
    ORMError(BackendError),
    SemanticError(SemanticError),
}

impl From<BackendError> for CostModelError {
    fn from(err: BackendError) -> Self {
        CostModelError::ORMError(err)
    }
}

pub trait CostModel: 'static + Send + Sync {
    /// TODO: documentation
    fn compute_operation_cost(
        &self,
        node: &PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_stats: &[Option<&EstimatedStatistic>],
        context: ComputeCostContext,
    ) -> CostModelResult<Cost>;

    /// TODO: documentation
    /// It is for cardinality estimation. The output should be the estimated
    /// statistic calculated by the cost model.
    /// TODO: Consider make it a helper function, so we can store Cost in the
    /// ORM more easily.
    fn derive_statistics(
        &self,
        node: PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_statistics: &[Option<&EstimatedStatistic>],
        context: ComputeCostContext,
    ) -> CostModelResult<EstimatedStatistic>;

    /// TODO: documentation
    /// It is for **REAL** statistic updates, not for estimated statistics.
    /// TODO: Change data from String to other types.
    fn update_statistics(
        &self,
        stats: Vec<Stat>,
        source: String,
        data: String,
    ) -> CostModelResult<()>;

    /// TODO: documentation
    fn get_table_statistic_for_analysis(
        &self,
        table_id: TableId,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>>;

    /// TODO: documentation
    fn get_attribute_statistic_for_analysis(
        &self,
        attr_ids: Vec<AttrId>,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>>;

    /// TODO: documentation
    fn get_cost_for_analysis(
        &self,
        expr_id: ExprId,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<Cost>>;
}

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
pub mod test_utils;
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
pub struct Cost {
    pub compute_cost: f64,
    pub io_cost: f64,
}

impl From<Cost> for optd_persistent::cost_model::interface::Cost {
    fn from(c: Cost) -> optd_persistent::cost_model::interface::Cost {
        Self {
            compute_cost: c.compute_cost,
            io_cost: c.io_cost,
        }
    }
}

impl From<optd_persistent::cost_model::interface::Cost> for Cost {
    fn from(c: optd_persistent::cost_model::interface::Cost) -> Cost {
        Self {
            compute_cost: c.compute_cost,
            io_cost: c.io_cost,
        }
    }
}

/// Estimated statistic calculated by the cost model.
/// It is the estimated output row count of the targeted expression.
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct EstimatedStatistic(pub f64);

impl From<EstimatedStatistic> for f32 {
    fn from(e: EstimatedStatistic) -> f32 {
        e.0 as f32
    }
}

impl From<EstimatedStatistic> for f64 {
    fn from(e: EstimatedStatistic) -> f64 {
        e.0
    }
}

impl From<f32> for EstimatedStatistic {
    fn from(f: f32) -> EstimatedStatistic {
        Self(f as f64)
    }
}

pub type CostModelResult<T> = Result<T, CostModelError>;

#[derive(Debug)]
pub enum SemanticError {
    // TODO: Add more error types
    UnknownStatisticType,
    VersionedStatisticNotFound,
    AttributeNotFound(TableId, u64), // (table_id, attribute_base_index)
    // FIXME: not sure if this should be put here
    InvalidPredicate(String),
}

#[derive(Debug)]
pub enum CostModelError {
    ORMError(BackendError),
    SemanticError(SemanticError),
    SerdeError(serde_json::Error),
}

impl From<BackendError> for CostModelError {
    fn from(err: BackendError) -> Self {
        CostModelError::ORMError(err)
    }
}

impl From<SemanticError> for CostModelError {
    fn from(err: SemanticError) -> Self {
        CostModelError::SemanticError(err)
    }
}

impl From<serde_json::Error> for CostModelError {
    fn from(err: serde_json::Error) -> Self {
        CostModelError::SerdeError(err)
    }
}

#[async_trait::async_trait]
pub trait CostModel: 'static + Send + Sync {
    /// TODO: documentation
    async fn compute_operation_cost(
        &self,
        node: &PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_stats: &[EstimatedStatistic],
        context: ComputeCostContext,
    ) -> CostModelResult<Cost>;

    /// TODO: documentation
    /// It is for cardinality estimation. The output should be the estimated
    /// statistic calculated by the cost model.
    /// TODO: Consider make it a helper function, so we can store Cost in the
    /// ORM more easily.
    async fn derive_statistics(
        &self,
        node: PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_stats: &[EstimatedStatistic],
        context: ComputeCostContext,
    ) -> CostModelResult<EstimatedStatistic>;

    /// TODO: documentation
    /// It is for **REAL** statistic updates, not for estimated statistics.
    /// TODO: Change data from String to other types.
    async fn update_statistics(
        &self,
        stats: Vec<Stat>,
        source: String,
        data: String,
    ) -> CostModelResult<()>;

    /// TODO: documentation
    async fn get_table_statistic_for_analysis(
        &self,
        table_id: TableId,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>>;

    /// TODO: documentation
    async fn get_attribute_statistic_for_analysis(
        &self,
        attr_ids: Vec<AttrId>,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>>;

    /// TODO: documentation
    async fn get_cost_for_analysis(
        &self,
        expr_id: ExprId,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<Cost>>;
}

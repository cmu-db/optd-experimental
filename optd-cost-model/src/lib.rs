use common::{
    nodes::{ArcPredicateNode, PhysicalNodeType},
    types::{ExprId, GroupId},
};
use optd_persistent::cost_model::interface::{Stat, StatType};

pub mod common;
pub mod cost;
pub mod cost_model;

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
pub enum CostModelError {
    // TODO: Add more error types
    ORMError,
}

pub trait CostModel: 'static + Send + Sync {
    /// TODO: documentation
    /// It is for cardinality estimation. The output should be the estimated
    /// statistic calculated by the cost model.
    fn derive_statistics(
        &self,
        node: PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_statistics: &[Option<&EstimatedStatistic>],
        context: Option<ComputeCostContext>,
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

    fn get_table_statistic_for_analysis(
        &self,
        // TODO: i32 should be changed to TableId.
        table_id: i32,
        stat_type: StatType,
        epoch_id: Option<i32>,
    ) -> CostModelResult<Option<StatValue>>;

    fn get_attribute_statistic_for_analysis(
        &self,
        // TODO: i32 should be changed to AttrId or EpochId.
        attr_ids: Vec<i32>,
        stat_type: StatType,
        epoch_id: Option<i32>,
    ) -> CostModelResult<Option<StatValue>>;

    fn get_cost_for_analysis(
        &self,
        expr_id: ExprId,
        epoch_id: Option<i32>,
    ) -> CostModelResult<Option<Cost>>;
}

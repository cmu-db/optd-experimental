#![allow(dead_code, unused_imports, unused_variables)]

use std::sync::Arc;

use optd_persistent::{
    cost_model::interface::{CatalogSource, Stat, StatType},
    CostModelStorageLayer,
};

use crate::{
    common::{
        nodes::{ArcPredicateNode, PhysicalNodeType},
        types::{AttrId, EpochId, ExprId, TableId},
    },
    stats::AttributeCombValueStats,
    storage::CostModelStorageManager,
    ComputeCostContext, Cost, CostModel, CostModelResult, EstimatedStatistic, StatValue,
};

/// TODO: documentation
pub struct CostModelImpl<S: CostModelStorageManager> {
    pub storage_manager: S,
    pub default_catalog_source: CatalogSource,
}

impl<S: CostModelStorageManager> CostModelImpl<S> {
    /// TODO: documentation
    pub fn new(storage_manager: S, default_catalog_source: CatalogSource) -> Self {
        Self {
            storage_manager,
            default_catalog_source,
        }
    }
}

impl<S: CostModelStorageManager + std::marker::Sync + std::marker::Send + 'static> CostModel
    for CostModelImpl<S>
{
    fn compute_operation_cost(
        &self,
        node: &PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_stats: &[Option<&EstimatedStatistic>],
        context: Option<ComputeCostContext>,
    ) -> CostModelResult<Cost> {
        todo!()
    }

    fn derive_statistics(
        &self,
        node: PhysicalNodeType,
        predicates: &[ArcPredicateNode],
        children_statistics: &[Option<&EstimatedStatistic>],
        context: Option<ComputeCostContext>,
    ) -> CostModelResult<EstimatedStatistic> {
        todo!()
    }

    fn update_statistics(
        &self,
        stats: Vec<Stat>,
        source: String,
        data: String,
    ) -> CostModelResult<()> {
        todo!()
    }

    fn get_table_statistic_for_analysis(
        &self,
        table_id: TableId,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>> {
        todo!()
    }

    fn get_attribute_statistic_for_analysis(
        &self,
        attr_ids: Vec<AttrId>,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<StatValue>> {
        todo!()
    }

    fn get_cost_for_analysis(
        &self,
        expr_id: ExprId,
        epoch_id: Option<EpochId>,
    ) -> CostModelResult<Option<crate::Cost>> {
        todo!()
    }
}

impl<S: CostModelStorageManager> CostModelImpl<S> {
    /// TODO: documentation
    /// TODO: if we have memory cache,
    /// we should add the reference. (&AttributeCombValueStats)
    pub(crate) async fn get_attribute_comb_stats(
        &self,
        table_id: TableId,
        attr_comb: &[usize],
    ) -> CostModelResult<Option<AttributeCombValueStats>> {
        self.storage_manager
            .get_attributes_comb_statistics(table_id, attr_comb)
            .await
    }
}

// /// I thought about using the system's own parser and planner to generate these expression trees,
// /// but this is not currently feasible because it would create a cyclic dependency between
// /// optd-datafusion-bridge and optd-datafusion-repr
// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;

//     use arrow_schema::DataType;
//     use itertools::Itertools;
//     use optd_persistent::BackendManager;
//     use serde::{Deserialize, Serialize};

//     use super::*;
//     pub type TestPerColumnStats = AttributeCombValueStats;
//     pub type TestOptCostModel = CostModelImpl<BackendManager>;

//     pub const TABLE1_NAME: &str = "table1";
//     pub const TABLE2_NAME: &str = "table2";
//     pub const TABLE3_NAME: &str = "table3";
//     pub const TABLE4_NAME: &str = "table4";

//     // one column is sufficient for all filter selectivity tests
//     pub fn create_one_column_cost_model(per_column_stats: TestPerColumnStats) -> TestOptCostModel {
//         AdvStats::new(
//             vec![(
//                 String::from(TABLE1_NAME),
//                 TableStats::new(100, vec![(vec![0], per_column_stats)].into_iter().collect()),
//             )]
//             .into_iter()
//             .collect(),
//         )
//     }

//     /// Create a cost model with two columns, one for each table. Each column has 100 values.
//     pub fn create_two_table_cost_model(
//         tbl1_per_column_stats: TestPerColumnStats,
//         tbl2_per_column_stats: TestPerColumnStats,
//     ) -> TestOptCostModel {
//         create_two_table_cost_model_custom_row_cnts(
//             tbl1_per_column_stats,
//             tbl2_per_column_stats,
//             100,
//             100,
//         )
//     }

//     /// Create a cost model with three columns, one for each table. Each column has 100 values.
//     pub fn create_three_table_cost_model(
//         tbl1_per_column_stats: TestPerColumnStats,
//         tbl2_per_column_stats: TestPerColumnStats,
//         tbl3_per_column_stats: TestPerColumnStats,
//     ) -> TestOptCostModel {
//         AdvStats::new(
//             vec![
//                 (
//                     String::from(TABLE1_NAME),
//                     TableStats::new(
//                         100,
//                         vec![(vec![0], tbl1_per_column_stats)].into_iter().collect(),
//                     ),
//                 ),
//                 (
//                     String::from(TABLE2_NAME),
//                     TableStats::new(
//                         100,
//                         vec![(vec![0], tbl2_per_column_stats)].into_iter().collect(),
//                     ),
//                 ),
//                 (
//                     String::from(TABLE3_NAME),
//                     TableStats::new(
//                         100,
//                         vec![(vec![0], tbl3_per_column_stats)].into_iter().collect(),
//                     ),
//                 ),
//             ]
//             .into_iter()
//             .collect(),
//         )
//     }

//     /// Create a cost model with three columns, one for each table. Each column has 100 values.
//     pub fn create_four_table_cost_model(
//         tbl1_per_column_stats: TestPerColumnStats,
//         tbl2_per_column_stats: TestPerColumnStats,
//         tbl3_per_column_stats: TestPerColumnStats,
//         tbl4_per_column_stats: TestPerColumnStats,
//     ) -> TestOptCostModel {
//         AdvStats::new(
//             vec![
//                 (
//                     String::from(TABLE1_NAME),
//                     TableStats::new(
//                         100,
//                         vec![(vec![0], tbl1_per_column_stats)].into_iter().collect(),
//                     ),
//                 ),
//                 (
//                     String::from(TABLE2_NAME),
//                     TableStats::new(
//                         100,
//                         vec![(vec![0], tbl2_per_column_stats)].into_iter().collect(),
//                     ),
//                 ),
//                 (
//                     String::from(TABLE3_NAME),
//                     TableStats::new(
//                         100,
//                         vec![(vec![0], tbl3_per_column_stats)].into_iter().collect(),
//                     ),
//                 ),
//                 (
//                     String::from(TABLE4_NAME),
//                     TableStats::new(
//                         100,
//                         vec![(vec![0], tbl4_per_column_stats)].into_iter().collect(),
//                     ),
//                 ),
//             ]
//             .into_iter()
//             .collect(),
//         )
//     }

//     /// We need custom row counts because some join algorithms rely on the row cnt
//     pub fn create_two_table_cost_model_custom_row_cnts(
//         tbl1_per_column_stats: TestPerColumnStats,
//         tbl2_per_column_stats: TestPerColumnStats,
//         tbl1_row_cnt: usize,
//         tbl2_row_cnt: usize,
//     ) -> TestOptCostModel {
//         AdvStats::new(
//             vec![
//                 (
//                     String::from(TABLE1_NAME),
//                     TableStats::new(
//                         tbl1_row_cnt,
//                         vec![(vec![0], tbl1_per_column_stats)].into_iter().collect(),
//                     ),
//                 ),
//                 (
//                     String::from(TABLE2_NAME),
//                     TableStats::new(
//                         tbl2_row_cnt,
//                         vec![(vec![0], tbl2_per_column_stats)].into_iter().collect(),
//                     ),
//                 ),
//             ]
//             .into_iter()
//             .collect(),
//         )
//     }

//     pub fn col_ref(idx: u64) -> ArcDfPredNode {
//         // this conversion is always safe because idx was originally a usize
//         let idx_as_usize = idx as usize;
//         ColumnRefPred::new(idx_as_usize).into_pred_node()
//     }

//     pub fn cnst(value: Value) -> ArcDfPredNode {
//         ConstantPred::new(value).into_pred_node()
//     }

//     pub fn cast(child: ArcDfPredNode, cast_type: DataType) -> ArcDfPredNode {
//         CastPred::new(child, cast_type).into_pred_node()
//     }

//     pub fn bin_op(op_type: BinOpType, left: ArcDfPredNode, right: ArcDfPredNode) -> ArcDfPredNode {
//         BinOpPred::new(left, right, op_type).into_pred_node()
//     }

//     pub fn log_op(op_type: LogOpType, children: Vec<ArcDfPredNode>) -> ArcDfPredNode {
//         LogOpPred::new(op_type, children).into_pred_node()
//     }

//     pub fn un_op(op_type: UnOpType, child: ArcDfPredNode) -> ArcDfPredNode {
//         UnOpPred::new(child, op_type).into_pred_node()
//     }

//     pub fn in_list(col_ref_idx: u64, list: Vec<Value>, negated: bool) -> InListPred {
//         InListPred::new(
//             col_ref(col_ref_idx),
//             ListPred::new(list.into_iter().map(cnst).collect_vec()),
//             negated,
//         )
//     }

//     pub fn like(col_ref_idx: u64, pattern: &str, negated: bool) -> LikePred {
//         LikePred::new(
//             negated,
//             false,
//             col_ref(col_ref_idx),
//             cnst(Value::String(pattern.into())),
//         )
//     }

//     /// The reason this isn't an associated function of PerColumnStats is because that would require
//     ///   adding an empty() function to the trait definitions of MostCommonValues and Distribution,
//     ///   which I wanted to avoid
//     pub(crate) fn get_empty_per_col_stats() -> TestPerColumnStats {
//         TestPerColumnStats::new(
//             TestMostCommonValues::empty(),
//             0,
//             0.0,
//             Some(TestDistribution::empty()),
//         )
//     }
// }

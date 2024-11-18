use crate::{
    common::{
        nodes::{ArcPredicateNode, PredicateType, ReprPredicateNode},
        predicates::{attr_ref_pred::AttrRefPred, list_pred::ListPred},
        types::TableId,
    },
    cost_model::CostModelImpl,
    stats::DEFAULT_NUM_DISTINCT,
    storage::CostModelStorageManager,
    CostModelError, CostModelResult, EstimatedStatistic, SemanticError,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    pub async fn get_agg_row_cnt(
        &self,
        group_by: ArcPredicateNode,
    ) -> CostModelResult<EstimatedStatistic> {
        let group_by = ListPred::from_pred_node(group_by).unwrap();
        if group_by.is_empty() {
            Ok(EstimatedStatistic(1))
        } else {
            // Multiply the n-distinct of all the group by columns.
            // TODO: improve with multi-dimensional n-distinct
            let mut row_cnt = 1;

            for node in &group_by.0.children {
                match node.typ {
                    PredicateType::AttrRef => {
                        let attr_ref =
                            AttrRefPred::from_pred_node(node.clone()).ok_or_else(|| {
                                SemanticError::InvalidPredicate(
                                    "Expected AttributeRef predicate".to_string(),
                                )
                            })?;
                        if attr_ref.is_derived() {
                            row_cnt *= DEFAULT_NUM_DISTINCT;
                        } else {
                            let table_id = attr_ref.table_id();
                            let attr_idx = attr_ref.attr_index();
                            // TODO: Only query ndistinct instead of all kinds of stats.
                            let stats_option =
                                self.get_attribute_comb_stats(table_id, &[attr_idx]).await?;

                            let ndistinct = match stats_option {
                                Some(stats) => stats.ndistinct,
                                None => {
                                    // The column type is not supported or stats are missing.
                                    DEFAULT_NUM_DISTINCT
                                }
                            };
                            row_cnt *= ndistinct;
                        }
                    }
                    _ => {
                        // TODO: Consider the case where `GROUP BY 1`.
                        panic!("GROUP BY must have attribute ref predicate");
                    }
                }
            }
            Ok(EstimatedStatistic(row_cnt))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        common::{
            predicates::constant_pred::ConstantType, properties::Attribute, types::TableId,
            values::Value,
        },
        cost_model::tests::{
            attr_ref, cnst, create_cost_model_mock_storage, empty_list, empty_per_attr_stats, list,
            TestPerAttributeStats,
        },
        stats::{utilities::simple_map::SimpleMap, MostCommonValues, DEFAULT_NUM_DISTINCT},
        EstimatedStatistic,
    };

    #[tokio::test]
    async fn test_agg_no_stats() {
        let table_id = TableId(0);
        let attr_infos = HashMap::from([(
            table_id,
            HashMap::from([
                (
                    0,
                    Attribute {
                        name: String::from("attr1"),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                ),
                (
                    1,
                    Attribute {
                        name: String::from("attr2"),
                        typ: ConstantType::Int64,
                        nullable: false,
                    },
                ),
            ]),
        )]);
        let cost_model =
            create_cost_model_mock_storage(vec![table_id], vec![], vec![None], attr_infos);

        // Group by empty list should return 1.
        let group_bys = empty_list();
        assert_eq!(
            cost_model.get_agg_row_cnt(group_bys).await.unwrap(),
            EstimatedStatistic(1)
        );

        // Group by single column should return the default value since there are no stats.
        let group_bys = list(vec![attr_ref(table_id, 0)]);
        assert_eq!(
            cost_model.get_agg_row_cnt(group_bys).await.unwrap(),
            EstimatedStatistic(DEFAULT_NUM_DISTINCT)
        );

        // Group by two columns should return the default value squared since there are no stats.
        let group_bys = list(vec![attr_ref(table_id, 0), attr_ref(table_id, 1)]);
        assert_eq!(
            cost_model.get_agg_row_cnt(group_bys).await.unwrap(),
            EstimatedStatistic(DEFAULT_NUM_DISTINCT * DEFAULT_NUM_DISTINCT)
        );
    }

    #[tokio::test]
    async fn test_agg_with_stats() {
        let table_id = TableId(0);
        let attr1_base_idx = 0;
        let attr2_base_idx = 1;
        let attr3_base_idx = 2;
        let attr_infos = HashMap::from([(
            table_id,
            HashMap::from([
                (
                    attr1_base_idx,
                    Attribute {
                        name: String::from("attr1"),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                ),
                (
                    attr2_base_idx,
                    Attribute {
                        name: String::from("attr2"),
                        typ: ConstantType::Int64,
                        nullable: false,
                    },
                ),
                (
                    attr3_base_idx,
                    Attribute {
                        name: String::from("attr3"),
                        typ: ConstantType::Int64,
                        nullable: false,
                    },
                ),
            ]),
        )]);

        let attr1_ndistinct = 12;
        let attr2_ndistinct = 645;
        let attr1_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::default()),
            None,
            attr1_ndistinct,
            0.0,
        );
        let attr2_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::default()),
            None,
            attr2_ndistinct,
            0.0,
        );

        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([
                (attr1_base_idx, attr1_stats),
                (attr2_base_idx, attr2_stats),
            ])],
            vec![None],
            attr_infos,
        );

        // Group by empty list should return 1.
        let group_bys = empty_list();
        assert_eq!(
            cost_model.get_agg_row_cnt(group_bys).await.unwrap(),
            EstimatedStatistic(1)
        );

        // Group by single column should return the n-distinct of the column.
        let group_bys = list(vec![attr_ref(table_id, attr1_base_idx)]);
        assert_eq!(
            cost_model.get_agg_row_cnt(group_bys).await.unwrap(),
            EstimatedStatistic(attr1_ndistinct)
        );

        // Group by two columns should return the product of the n-distinct of the columns.
        let group_bys = list(vec![
            attr_ref(table_id, attr1_base_idx),
            attr_ref(table_id, attr2_base_idx),
        ]);
        assert_eq!(
            cost_model.get_agg_row_cnt(group_bys).await.unwrap(),
            EstimatedStatistic(attr1_ndistinct * attr2_ndistinct)
        );

        // Group by multiple columns should return the product of the n-distinct of the columns. If one of the columns
        // does not have stats, it should use the default value instead.
        let group_bys = list(vec![
            attr_ref(table_id, attr1_base_idx),
            attr_ref(table_id, attr2_base_idx),
            attr_ref(table_id, attr3_base_idx),
        ]);
        assert_eq!(
            cost_model.get_agg_row_cnt(group_bys).await.unwrap(),
            EstimatedStatistic(attr1_ndistinct * attr2_ndistinct * DEFAULT_NUM_DISTINCT)
        );
    }
}

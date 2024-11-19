use crate::{
    common::{
        nodes::{ArcPredicateNode, PredicateType, ReprPredicateNode},
        predicates::{attr_index_pred::AttrIndexPred, list_pred::ListPred},
        properties::attr_ref::{AttrRef, BaseTableAttrRef},
        types::GroupId,
    },
    cost_model::CostModelImpl,
    stats::DEFAULT_NUM_DISTINCT,
    storage::CostModelStorageManager,
    CostModelError, CostModelResult, EstimatedStatistic, SemanticError,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    pub async fn get_agg_row_cnt(
        &self,
        group_id: GroupId,
        group_by: ArcPredicateNode,
    ) -> CostModelResult<EstimatedStatistic> {
        let group_by = ListPred::from_pred_node(group_by).unwrap();
        if group_by.is_empty() {
            Ok(EstimatedStatistic(1.0))
        } else {
            // Multiply the n-distinct of all the group by columns.
            // TODO: improve with multi-dimensional n-distinct
            let mut row_cnt = 1;

            for node in &group_by.0.children {
                match node.typ {
                    PredicateType::AttrIndex => {
                        let attr_ref =
                            AttrIndexPred::from_pred_node(node.clone()).ok_or_else(|| {
                                SemanticError::InvalidPredicate(
                                    "Expected AttributeRef predicate".to_string(),
                                )
                            })?;
                        if let AttrRef::BaseTableAttrRef(BaseTableAttrRef { table_id, attr_idx }) =
                            self.memo.get_attribute_ref(group_id, attr_ref.attr_index())
                        {
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
                        } else {
                            // TOOD: Handle derived attributes.
                            row_cnt *= DEFAULT_NUM_DISTINCT;
                        }
                    }
                    _ => {
                        // TODO: Consider the case where `GROUP BY 1`.
                        panic!("GROUP BY must have attribute ref predicate");
                    }
                }
            }
            Ok(EstimatedStatistic(row_cnt as f64))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, ops::Deref};

    use crate::{
        common::{
            predicates::constant_pred::ConstantType,
            properties::Attribute,
            types::{GroupId, TableId},
            values::Value,
        },
        cost_model::tests::{
            attr_index, cnst, create_mock_cost_model, create_mock_cost_model_with_attr_types,
            empty_list, empty_per_attr_stats, list, TestPerAttributeStats, TEST_ATTR1_BASE_INDEX,
            TEST_ATTR2_BASE_INDEX, TEST_ATTR3_BASE_INDEX, TEST_GROUP1_ID, TEST_TABLE1_ID,
        },
        stats::{utilities::simple_map::SimpleMap, MostCommonValues, DEFAULT_NUM_DISTINCT},
        EstimatedStatistic,
    };

    #[tokio::test]
    async fn test_agg_no_stats() {
        let cost_model = create_mock_cost_model_with_attr_types(
            vec![TEST_TABLE1_ID],
            vec![],
            vec![HashMap::from([
                (TEST_ATTR1_BASE_INDEX, ConstantType::Int32),
                (TEST_ATTR2_BASE_INDEX, ConstantType::Int32),
            ])],
            vec![None],
        );

        // Group by empty list should return 1.
        let group_bys = empty_list();
        assert_eq!(
            cost_model
                .get_agg_row_cnt(TEST_GROUP1_ID, group_bys)
                .await
                .unwrap(),
            EstimatedStatistic(1.0)
        );

        // Group by single column should return the default value since there are no stats.
        let group_bys = list(vec![attr_index(0)]);
        assert_eq!(
            cost_model
                .get_agg_row_cnt(TEST_GROUP1_ID, group_bys)
                .await
                .unwrap(),
            EstimatedStatistic(DEFAULT_NUM_DISTINCT as f64)
        );

        // Group by two columns should return the default value squared since there are no stats.
        let group_bys = list(vec![attr_index(0), attr_index(1)]);
        assert_eq!(
            cost_model
                .get_agg_row_cnt(TEST_GROUP1_ID, group_bys)
                .await
                .unwrap(),
            EstimatedStatistic((DEFAULT_NUM_DISTINCT * DEFAULT_NUM_DISTINCT) as f64)
        );
    }

    #[tokio::test]
    async fn test_agg_with_stats() {
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

        let cost_model = create_mock_cost_model_with_attr_types(
            vec![TEST_TABLE1_ID],
            vec![HashMap::from([
                (TEST_ATTR1_BASE_INDEX, attr1_stats),
                (TEST_ATTR2_BASE_INDEX, attr2_stats),
            ])],
            vec![HashMap::from([
                (TEST_ATTR1_BASE_INDEX, ConstantType::Int32),
                (TEST_ATTR2_BASE_INDEX, ConstantType::Int32),
                (TEST_ATTR3_BASE_INDEX, ConstantType::Int32),
            ])],
            vec![None],
        );

        // Group by empty list should return 1.
        let group_bys = empty_list();
        assert_eq!(
            cost_model
                .get_agg_row_cnt(TEST_GROUP1_ID, group_bys)
                .await
                .unwrap(),
            EstimatedStatistic(1.0)
        );

        // Group by single column should return the n-distinct of the column.
        let group_bys = list(vec![attr_index(0)]);
        assert_eq!(
            cost_model
                .get_agg_row_cnt(TEST_GROUP1_ID, group_bys)
                .await
                .unwrap(),
            EstimatedStatistic(attr1_ndistinct as f64)
        );

        // Group by two columns should return the product of the n-distinct of the columns.
        let group_bys = list(vec![attr_index(0), attr_index(1)]);
        assert_eq!(
            cost_model
                .get_agg_row_cnt(TEST_GROUP1_ID, group_bys)
                .await
                .unwrap(),
            EstimatedStatistic((attr1_ndistinct * attr2_ndistinct) as f64)
        );

        // Group by multiple columns should return the product of the n-distinct of the columns. If one of the columns
        // does not have stats, it should use the default value instead.
        let group_bys = list(vec![attr_index(0), attr_index(1), attr_index(2)]);
        assert_eq!(
            cost_model
                .get_agg_row_cnt(TEST_GROUP1_ID, group_bys)
                .await
                .unwrap(),
            EstimatedStatistic((attr1_ndistinct * attr2_ndistinct * DEFAULT_NUM_DISTINCT) as f64)
        );
    }
}

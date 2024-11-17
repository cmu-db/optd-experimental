use crate::{
    common::{
        nodes::{ArcPredicateNode, PredicateType, ReprPredicateNode},
        predicates::{in_list_pred::InListPred, like_pred::LikePred, un_op_pred::UnOpType},
    },
    cost_model::CostModelImpl,
    stats::UNIMPLEMENTED_SEL,
    storage::CostModelStorageManager,
    CostModelResult, EstimatedStatistic,
};

impl<S: CostModelStorageManager> CostModelImpl<S> {
    // TODO: is it a good design to pass table_id here? I think it needs to be refactored.
    // Consider to remove table_id.
    pub async fn get_filter_row_cnt(
        &self,
        child_row_cnt: EstimatedStatistic,
        cond: ArcPredicateNode,
    ) -> CostModelResult<EstimatedStatistic> {
        let selectivity = { self.get_filter_selectivity(cond).await? };
        Ok(
            EstimatedStatistic((child_row_cnt.0 as f64 * selectivity) as u64)
                .max(EstimatedStatistic(1)),
        )
    }

    pub async fn get_filter_selectivity(
        &self,
        expr_tree: ArcPredicateNode,
    ) -> CostModelResult<f64> {
        Box::pin(async move {
            match &expr_tree.typ {
                PredicateType::Constant(_) => Ok(Self::get_constant_selectivity(expr_tree)),
                PredicateType::AttributeRef => unimplemented!("check bool type or else panic"),
                PredicateType::UnOp(un_op_typ) => {
                    assert!(expr_tree.children.len() == 1);
                    let child = expr_tree.child(0);
                    match un_op_typ {
                        // not doesn't care about nulls so there's no complex logic. it just reverses
                        // the selectivity for instance, != _will not_ include nulls
                        // but "NOT ==" _will_ include nulls
                        UnOpType::Not => Ok(1.0 - self.get_filter_selectivity(child).await?),
                        UnOpType::Neg => panic!(
                            "the selectivity of operations that return numerical values is undefined"
                        ),
                    }
                }
                PredicateType::BinOp(bin_op_typ) => {
                    assert!(expr_tree.children.len() == 2);
                    let left_child = expr_tree.child(0);
                    let right_child = expr_tree.child(1);

                    if bin_op_typ.is_comparison() {
                        self.get_comp_op_selectivity(*bin_op_typ, left_child, right_child).await
                    } else if bin_op_typ.is_numerical() {
                        panic!(
                            "the selectivity of operations that return numerical values is undefined"
                        )
                    } else {
                        unreachable!("all BinOpTypes should be true for at least one is_*() function")
                    }
                }
                PredicateType::LogOp(log_op_typ) => {
                    self.get_log_op_selectivity(*log_op_typ, &expr_tree.children).await
                }
                PredicateType::Func(_) => unimplemented!("check bool type or else panic"),
                PredicateType::SortOrder(_) => {
                    panic!("the selectivity of sort order expressions is undefined")
                }
                PredicateType::Between => Ok(UNIMPLEMENTED_SEL),
                PredicateType::Cast => unimplemented!("check bool type or else panic"),
                PredicateType::Like => {
                    let like_expr = LikePred::from_pred_node(expr_tree).unwrap();
                    self.get_like_selectivity(&like_expr).await
                }
                PredicateType::DataType(_) => {
                    panic!("the selectivity of a data type is not defined")
                }
                PredicateType::InList => {
                    let in_list_expr = InListPred::from_pred_node(expr_tree).unwrap();
                    self.get_in_list_selectivity(&in_list_expr).await
                }
                _ => unreachable!(
                    "all expression DfPredType were enumerated. this should be unreachable"
                ),
            }
        }).await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        common::{
            predicates::{
                bin_op_pred::BinOpType, constant_pred::ConstantType, log_op_pred::LogOpType,
                un_op_pred::UnOpType,
            },
            types::TableId,
            values::Value,
        },
        cost_model::tests::*,
        stats::{
            utilities::{counter::Counter, simple_map::SimpleMap},
            Distribution, MostCommonValues, DEFAULT_EQ_SEL,
        },
        storage::Attribute,
    };
    use arrow_schema::DataType;

    #[tokio::test]
    async fn test_const() {
        let cost_model = create_cost_model_mock_storage(
            vec![TableId(0)],
            vec![HashMap::from([(0, empty_per_attr_stats())])],
            vec![None],
            HashMap::new(),
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(cnst(Value::Bool(true)))
                .await
                .unwrap(),
            1.0
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(cnst(Value::Bool(false)))
                .await
                .unwrap(),
            0.0
        );
    }

    #[tokio::test]
    async fn test_attr_ref_eq_constint_in_mcv() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![(
                vec![Some(Value::Int32(1))],
                0.3,
            )])),
            None,
            0,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = bin_op(BinOpType::Eq, attr_ref(table_id, 0), cnst(Value::Int32(1)));
        let expr_tree_rev = bin_op(BinOpType::Eq, cnst(Value::Int32(1)), attr_ref(table_id, 0));
        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.3
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            0.3
        );
    }

    #[tokio::test]
    async fn test_attr_ref_eq_constint_not_in_mcv() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![
                (vec![Some(Value::Int32(1))], 0.2),
                (vec![Some(Value::Int32(3))], 0.44),
            ])),
            None,
            5,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = bin_op(BinOpType::Eq, attr_ref(table_id, 0), cnst(Value::Int32(2)));
        let expr_tree_rev = bin_op(BinOpType::Eq, cnst(Value::Int32(2)), attr_ref(table_id, 0));
        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.12
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            0.12
        );
    }

    /// I only have one test for NEQ since I'll assume that it uses the same underlying logic as EQ
    #[tokio::test]
    async fn test_attr_ref_neq_constint_in_mcv() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![(
                vec![Some(Value::Int32(1))],
                0.3,
            )])),
            None,
            0,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = bin_op(BinOpType::Neq, attr_ref(table_id, 0), cnst(Value::Int32(1)));
        let expr_tree_rev = bin_op(BinOpType::Neq, cnst(Value::Int32(1)), attr_ref(table_id, 0));
        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            1.0 - 0.3
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            1.0 - 0.3
        );
    }

    #[tokio::test]
    async fn test_attr_ref_leq_constint_no_mcvs_in_range() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::default()),
            Some(Distribution::SimpleDistribution(SimpleMap::new(vec![(
                Value::Int32(15),
                0.7,
            )]))),
            10,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = bin_op(
            BinOpType::Leq,
            attr_ref(table_id, 0),
            cnst(Value::Int32(15)),
        );
        let expr_tree_rev = bin_op(BinOpType::Gt, cnst(Value::Int32(15)), attr_ref(table_id, 0));
        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.7
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            0.7
        );
    }

    #[tokio::test]
    async fn test_attr_ref_leq_constint_with_mcvs_in_range_not_at_border() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![
                (vec![Some(Value::Int32(6))], 0.05),
                (vec![Some(Value::Int32(10))], 0.1),
                (vec![Some(Value::Int32(17))], 0.08),
                (vec![Some(Value::Int32(25))], 0.07),
            ])),
            Some(Distribution::SimpleDistribution(SimpleMap::new(vec![(
                Value::Int32(15),
                0.7,
            )]))),
            10,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = bin_op(
            BinOpType::Leq,
            attr_ref(table_id, 0),
            cnst(Value::Int32(15)),
        );
        let expr_tree_rev = bin_op(BinOpType::Gt, cnst(Value::Int32(15)), attr_ref(table_id, 0));
        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.85
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            0.85
        );
    }

    #[tokio::test]
    async fn test_attr_ref_leq_constint_with_mcv_at_border() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![
                (vec![Some(Value::Int32(6))], 0.05),
                (vec![Some(Value::Int32(10))], 0.1),
                (vec![Some(Value::Int32(15))], 0.08),
                (vec![Some(Value::Int32(25))], 0.07),
            ])),
            Some(Distribution::SimpleDistribution(SimpleMap::new(vec![(
                Value::Int32(15),
                0.7,
            )]))),
            10,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = bin_op(
            BinOpType::Leq,
            attr_ref(table_id, 0),
            cnst(Value::Int32(15)),
        );
        let expr_tree_rev = bin_op(BinOpType::Gt, cnst(Value::Int32(15)), attr_ref(table_id, 0));
        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.93
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            0.93
        );
    }

    #[tokio::test]
    async fn test_attr_ref_lt_constint_no_mcvs_in_range() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::default()),
            Some(Distribution::SimpleDistribution(SimpleMap::new(vec![(
                Value::Int32(15),
                0.7,
            )]))),
            10,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = bin_op(BinOpType::Lt, attr_ref(table_id, 0), cnst(Value::Int32(15)));
        let expr_tree_rev = bin_op(
            BinOpType::Geq,
            cnst(Value::Int32(15)),
            attr_ref(table_id, 0),
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.6
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            0.6
        );
    }

    #[tokio::test]
    async fn test_attr_ef_lt_constint_with_mcvs_in_range_not_at_border() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![
                (vec![Some(Value::Int32(6))], 0.05),
                (vec![Some(Value::Int32(10))], 0.1),
                (vec![Some(Value::Int32(17))], 0.08),
                (vec![Some(Value::Int32(25))], 0.07),
            ])),
            Some(Distribution::SimpleDistribution(SimpleMap::new(vec![(
                Value::Int32(15),
                0.7,
            )]))),
            11, /* there are 4 MCVs which together add up to 0.3. With 11 total ndistinct, each
                 * remaining value has freq 0.1 */
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = bin_op(BinOpType::Lt, attr_ref(table_id, 0), cnst(Value::Int32(15)));
        let expr_tree_rev = bin_op(
            BinOpType::Geq,
            cnst(Value::Int32(15)),
            attr_ref(table_id, 0),
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.75
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            0.75
        );
    }

    #[tokio::test]
    async fn test_attr_ref_lt_constint_with_mcv_at_border() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![
                (vec![Some(Value::Int32(6))], 0.05),
                (vec![Some(Value::Int32(10))], 0.1),
                (vec![Some(Value::Int32(15))], 0.08),
                (vec![Some(Value::Int32(25))], 0.07),
            ])),
            Some(Distribution::SimpleDistribution(SimpleMap::new(vec![(
                Value::Int32(15),
                0.7,
            )]))),
            11, /* there are 4 MCVs which together add up to 0.3. With 11 total ndistinct, each
                 * remaining value has freq 0.1 */
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = bin_op(BinOpType::Lt, attr_ref(table_id, 0), cnst(Value::Int32(15)));
        let expr_tree_rev = bin_op(
            BinOpType::Geq,
            cnst(Value::Int32(15)),
            attr_ref(table_id, 0),
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.85
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            0.85
        );
    }

    /// I have fewer tests for GT since I'll assume that it uses the same underlying logic as LEQ
    /// The only interesting thing to test is that if there are nulls, those aren't included in GT
    #[tokio::test]
    async fn test_attr_ref_gt_constint() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::default()),
            Some(Distribution::SimpleDistribution(SimpleMap::new(vec![(
                Value::Int32(15),
                0.7,
            )]))),
            10,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = bin_op(BinOpType::Gt, attr_ref(table_id, 0), cnst(Value::Int32(15)));
        let expr_tree_rev = bin_op(
            BinOpType::Leq,
            cnst(Value::Int32(15)),
            attr_ref(table_id, 0),
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            1.0 - 0.7
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            1.0 - 0.7
        );
    }

    #[tokio::test]
    async fn test_attr_ref_geq_constint() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::default()),
            Some(Distribution::SimpleDistribution(SimpleMap::new(vec![(
                Value::Int32(15),
                0.7,
            )]))),
            10,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = bin_op(
            BinOpType::Geq,
            attr_ref(table_id, 0),
            cnst(Value::Int32(15)),
        );
        let expr_tree_rev = bin_op(BinOpType::Lt, cnst(Value::Int32(15)), attr_ref(table_id, 0));

        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            1.0 - 0.6
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            1.0 - 0.6
        );
    }

    #[tokio::test]
    async fn test_and() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![
                (vec![Some(Value::Int32(1))], 0.3),
                (vec![Some(Value::Int32(5))], 0.5),
                (vec![Some(Value::Int32(8))], 0.2),
            ])),
            None,
            0,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let eq1 = bin_op(BinOpType::Eq, attr_ref(table_id, 0), cnst(Value::Int32(1)));
        let eq5 = bin_op(BinOpType::Eq, attr_ref(table_id, 0), cnst(Value::Int32(5)));
        let eq8 = bin_op(BinOpType::Eq, attr_ref(table_id, 0), cnst(Value::Int32(8)));
        let expr_tree = log_op(LogOpType::And, vec![eq1.clone(), eq5.clone(), eq8.clone()]);
        let expr_tree_shift1 = log_op(LogOpType::And, vec![eq5.clone(), eq8.clone(), eq1.clone()]);
        let expr_tree_shift2 = log_op(LogOpType::And, vec![eq8.clone(), eq1.clone(), eq5.clone()]);

        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.03
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_shift1)
                .await
                .unwrap(),
            0.03
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_shift2)
                .await
                .unwrap(),
            0.03
        );
    }

    #[tokio::test]
    async fn test_or() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![
                (vec![Some(Value::Int32(1))], 0.3),
                (vec![Some(Value::Int32(5))], 0.5),
                (vec![Some(Value::Int32(8))], 0.2),
            ])),
            None,
            0,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let eq1 = bin_op(BinOpType::Eq, attr_ref(table_id, 0), cnst(Value::Int32(1)));
        let eq5 = bin_op(BinOpType::Eq, attr_ref(table_id, 0), cnst(Value::Int32(5)));
        let eq8 = bin_op(BinOpType::Eq, attr_ref(table_id, 0), cnst(Value::Int32(8)));
        let expr_tree = log_op(LogOpType::Or, vec![eq1.clone(), eq5.clone(), eq8.clone()]);
        let expr_tree_shift1 = log_op(LogOpType::Or, vec![eq5.clone(), eq8.clone(), eq1.clone()]);
        let expr_tree_shift2 = log_op(LogOpType::Or, vec![eq8.clone(), eq1.clone(), eq5.clone()]);

        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.72
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_shift1)
                .await
                .unwrap(),
            0.72
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_shift2)
                .await
                .unwrap(),
            0.72
        );
    }

    #[tokio::test]
    async fn test_not() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![(
                vec![Some(Value::Int32(1))],
                0.3,
            )])),
            None,
            0,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = un_op(
            UnOpType::Not,
            bin_op(BinOpType::Eq, attr_ref(table_id, 0), cnst(Value::Int32(1))),
        );

        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.7
        );
    }

    // I didn't test any non-unique cases with filter. The non-unique tests without filter should
    // cover that

    #[tokio::test]
    async fn test_attr_ref_eq_cast_value() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![(
                vec![Some(Value::Int32(1))],
                0.3,
            )])),
            None,
            0,
            0.0,
        );
        let table_id = TableId(0);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            HashMap::new(),
        );

        let expr_tree = bin_op(
            BinOpType::Eq,
            attr_ref(table_id, 0),
            cast(cnst(Value::Int64(1)), DataType::Int32),
        );
        let expr_tree_rev = bin_op(
            BinOpType::Eq,
            cast(cnst(Value::Int64(1)), DataType::Int32),
            attr_ref(table_id, 0),
        );

        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.3
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            0.3
        );
    }

    #[tokio::test]
    async fn test_cast_attr_ref_eq_value() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::new(vec![(
                vec![Some(Value::Int32(1))],
                0.3,
            )])),
            None,
            0,
            0.1,
        );
        let table_id = TableId(0);
        let attr_infos = HashMap::from([(
            table_id,
            HashMap::from([(
                0,
                Attribute {
                    name: String::from("attr1"),
                    typ: ConstantType::Int32,
                    nullable: false,
                },
            )]),
        )]);
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            attr_infos,
        );

        let expr_tree = bin_op(
            BinOpType::Eq,
            cast(attr_ref(table_id, 0), DataType::Int64),
            cnst(Value::Int64(1)),
        );
        let expr_tree_rev = bin_op(
            BinOpType::Eq,
            cnst(Value::Int64(1)),
            cast(attr_ref(table_id, 0), DataType::Int64),
        );

        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            0.3
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            0.3
        );
    }

    /// In this case, we should leave the Cast as is.
    ///
    /// Note that the test only checks the selectivity and thus doesn't explicitly test that the
    /// Cast is indeed left as is. However, if get_filter_selectivity() doesn't crash, that's a
    /// pretty good signal that the Cast was left as is.
    #[tokio::test]
    async fn test_cast_attr_ref_eq_attr_ref() {
        let per_attribute_stats = TestPerAttributeStats::new(
            MostCommonValues::SimpleFrequency(SimpleMap::default()),
            None,
            0,
            0.0,
        );
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
        let cost_model = create_cost_model_mock_storage(
            vec![table_id],
            vec![HashMap::from([(0, per_attribute_stats)])],
            vec![None],
            attr_infos,
        );

        let expr_tree = bin_op(
            BinOpType::Eq,
            cast(attr_ref(table_id, 0), DataType::Int64),
            attr_ref(table_id, 1),
        );
        let expr_tree_rev = bin_op(
            BinOpType::Eq,
            attr_ref(table_id, 1),
            cast(attr_ref(table_id, 0), DataType::Int64),
        );

        assert_approx_eq::assert_approx_eq!(
            cost_model.get_filter_selectivity(expr_tree).await.unwrap(),
            DEFAULT_EQ_SEL
        );
        assert_approx_eq::assert_approx_eq!(
            cost_model
                .get_filter_selectivity(expr_tree_rev)
                .await
                .unwrap(),
            DEFAULT_EQ_SEL
        );
    }
}

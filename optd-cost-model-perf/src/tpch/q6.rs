use datafusion_expr::AggregateFunction;
use optd_cost_model::common::nodes::{PhysicalNodeType, PredicateNode, PredicateType};
use optd_cost_model::common::predicates::log_op_pred::LogOpType;
use optd_cost_model::common::predicates::{
    bin_op_pred::BinOpType, constant_pred::ConstantType, func_pred::FuncType,
};
use optd_cost_model::common::properties::attr_ref::{AttrRef, BaseTableAttrRef, GroupAttrRefs};
use optd_cost_model::common::properties::schema::Schema;
use optd_cost_model::common::properties::Attribute;
use optd_cost_model::common::types::{ExprId, GroupId, TableId};
use optd_cost_model::common::values::{SerializableOrderedF64, Value};
use optd_cost_model::test_utils::tests::MemoGroupInfo;

use optd_cost_model::ComputeCostContext;
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::sync::Arc;
use std::vec;

use super::{OperatorNode, LINEITEM_TABLE_ID};

fn create_tpch_q6_memo() -> HashMap<GroupId, MemoGroupInfo> {
    let mut memo = HashMap::new();
    let mut attribute_refs = vec![];
    for i in 0..15 {
        attribute_refs.push(AttrRef::BaseTableAttrRef(BaseTableAttrRef {
            table_id: TableId(LINEITEM_TABLE_ID), // lineitem
            attr_idx: i,
        }));
    }
    let schema_complete = Schema {
        attributes: vec![
            Attribute {
                name: "l_orderkey".to_string(),
                typ: ConstantType::Int32,
                nullable: false,
            },
            Attribute {
                name: "l_partkey".to_string(),
                typ: ConstantType::Int32,
                nullable: false,
            },
            Attribute {
                name: "l_suppkey".to_string(),
                typ: ConstantType::Int32,
                nullable: false,
            },
            Attribute {
                name: "l_linenumber".to_string(),
                typ: ConstantType::Int32,
                nullable: false,
            },
            Attribute {
                name: "l_quantity".to_string(),
                typ: ConstantType::Decimal,
                nullable: false,
            },
            Attribute {
                name: "l_extendedprice".to_string(),
                typ: ConstantType::Decimal,
                nullable: false,
            },
            Attribute {
                name: "l_discount".to_string(),
                typ: ConstantType::Decimal,
                nullable: false,
            },
            Attribute {
                name: "l_tax".to_string(),
                typ: ConstantType::Decimal,
                nullable: false,
            },
            Attribute {
                name: "l_returnflag".to_string(),
                typ: ConstantType::Utf8String,
                nullable: false,
            },
            Attribute {
                name: "l_linestatus".to_string(),
                typ: ConstantType::Utf8String,
                nullable: false,
            },
            Attribute {
                name: "l_shipdate".to_string(),
                typ: ConstantType::Date,
                nullable: false,
            },
            Attribute {
                name: "l_commitdate".to_string(),
                typ: ConstantType::Date,
                nullable: false,
            },
            Attribute {
                name: "l_receiptdate".to_string(),
                typ: ConstantType::Date,
                nullable: false,
            },
            Attribute {
                name: "l_shipinstruct".to_string(),
                typ: ConstantType::Utf8String,
                nullable: false,
            },
            Attribute {
                name: "l_shipmode".to_string(),
                typ: ConstantType::Utf8String,
                nullable: false,
            },
            Attribute {
                name: "l_comment".to_string(),
                typ: ConstantType::Utf8String,
                nullable: false,
            },
        ],
    };

    // Agg node
    memo.insert(
        GroupId(15),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![Attribute {
                    name: "unnamed".to_string(),
                    typ: ConstantType::Binary,
                    nullable: true,
                }],
            },
            attr_refs: GroupAttrRefs::new(vec![AttrRef::Derived], None), // What is the AttrRef in this case??
        },
    );

    // Filter node
    memo.insert(
        GroupId(18),
        MemoGroupInfo {
            schema: schema_complete.clone(),
            attr_refs: GroupAttrRefs::new(attribute_refs.clone(), None),
        },
    );

    // Scan node
    memo.insert(
        GroupId(2),
        MemoGroupInfo {
            schema: schema_complete.clone(),
            attr_refs: GroupAttrRefs::new(attribute_refs.clone(), None),
        },
    );
    memo
}

fn create_tpch_q6_nodes() -> Vec<OperatorNode> {
    let agg_node = OperatorNode {
        typ: PhysicalNodeType::PhysicalAgg,
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::Func(FuncType::Agg(AggregateFunction::Sum)),
                    children: vec![Arc::new(PredicateNode {
                        typ: PredicateType::List,
                        children: vec![Arc::new(PredicateNode {
                            typ: PredicateType::BinOp(BinOpType::Mul),
                            children: vec![
                                Arc::new(PredicateNode {
                                    typ: PredicateType::AttrIndex,
                                    children: vec![],
                                    data: Some(Value::UInt64(0)),
                                }),
                                Arc::new(PredicateNode {
                                    typ: PredicateType::Constant(ConstantType::Int32),
                                    children: vec![],
                                    data: Some(Value::UInt64(1)),
                                }),
                            ],
                            data: None,
                        })],
                        data: None,
                    })],
                    data: None,
                })],
                data: None,
            }),
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![],
                data: None,
            }),
        ],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(15),
            expr_id: ExprId(82),
            children_group_ids: vec![GroupId(11)],
        },
    };

    let projection_node = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(5)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(6)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(11),
            expr_id: ExprId(84),
            children_group_ids: vec![GroupId(18)],
        },
    };

    let filter_node = OperatorNode {
        typ: PhysicalNodeType::PhysicalFilter,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::LogOp(LogOpType::And),
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::BinOp(BinOpType::Geq),
                    children: vec![
                        Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(10)),
                        }),
                        Arc::new(PredicateNode {
                            typ: PredicateType::Constant(ConstantType::Date),
                            children: vec![],
                            data: Some(Value::Int64(9131)),
                        }),
                    ],
                    data: None,
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::BinOp(BinOpType::Lt),
                    children: vec![
                        Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(10)),
                        }),
                        Arc::new(PredicateNode {
                            typ: PredicateType::Constant(ConstantType::Date),
                            children: vec![],
                            data: Some(Value::Int64(9496)),
                        }),
                    ],
                    data: None,
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::BinOp(BinOpType::Geq),
                    children: vec![
                        Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(6)),
                        }),
                        Arc::new(PredicateNode {
                            typ: PredicateType::Constant(ConstantType::Decimal),
                            children: vec![],
                            data: Some(Value::Float(SerializableOrderedF64(OrderedFloat(4.0)))),
                        }),
                    ],
                    data: None,
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::BinOp(BinOpType::Leq),
                    children: vec![
                        Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(6)),
                        }),
                        Arc::new(PredicateNode {
                            typ: PredicateType::Constant(ConstantType::Decimal),
                            children: vec![],
                            data: Some(Value::Float(SerializableOrderedF64(OrderedFloat(6.0)))),
                        }),
                    ],
                    data: None,
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::BinOp(BinOpType::Lt),
                    children: vec![
                        Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(4)),
                        }),
                        Arc::new(PredicateNode {
                            typ: PredicateType::Constant(ConstantType::Decimal),
                            children: vec![],
                            data: Some(Value::Float(SerializableOrderedF64(OrderedFloat(2400.0)))),
                        }),
                    ],
                    data: None,
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(18),
            expr_id: ExprId(86),
            children_group_ids: vec![GroupId(2)],
        },
    };

    let scan_node = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(LINEITEM_TABLE_ID)), // lineitem
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(2),
            expr_id: ExprId(88),
            children_group_ids: vec![],
        },
    };
    vec![scan_node, filter_node, projection_node, agg_node]
}

pub fn init_tpch_q6() -> (
    Vec<TableId>,
    HashMap<GroupId, MemoGroupInfo>,
    Vec<OperatorNode>,
) {
    let memo = create_tpch_q6_memo();
    let nodes = create_tpch_q6_nodes();
    (vec![TableId(LINEITEM_TABLE_ID)], memo, nodes)
}

#[cfg(test)]
pub mod tests {

    use optd_cost_model::{common::types::TableId, CostModel, EstimatedStatistic};
    use std::collections::HashMap;

    use optd_cost_model::test_utils::tests::create_mock_cost_model;

    use crate::tpch::q6::create_tpch_q6_nodes;

    #[tokio::test]
    async fn naive_scan_test() {
        let dummy_row_cnt = 100;
        let cost_model = create_mock_cost_model(
            vec![TableId(0)],
            vec![HashMap::new()],
            vec![Some(dummy_row_cnt)],
        );
        let nodes = create_tpch_q6_nodes();

        let node = nodes[3].clone();
        let res = cost_model
            .derive_statistics(
                node.typ,
                &node.predicates,
                &node.children_stats,
                node.context,
            )
            .await
            .unwrap();
        assert_eq!(res, EstimatedStatistic(dummy_row_cnt as f64));
    }
}

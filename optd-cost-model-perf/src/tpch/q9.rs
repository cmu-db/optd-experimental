use std::sync::Arc;

use datafusion_expr::{AggregateFunction, BuiltinScalarFunction};
use optd_cost_model::{
    common::{
        nodes::{JoinType, PhysicalNodeType, PredicateNode, PredicateType},
        predicates::{
            bin_op_pred::BinOpType, constant_pred::ConstantType, func_pred::FuncType,
            sort_order_pred::SortOrderType,
        },
        types::{ExprId, GroupId},
        values::{SerializableOrderedF64, Value},
    },
    ComputeCostContext,
};
use ordered_float::OrderedFloat;

use super::OperatorNode;
use crate::tpch::{
    LINEITEM_TABLE_ID, NATION_TABLE_ID, ORDERS_TABLE_ID, PARTSUPP_TABLE_ID, PART_TABLE_ID,
    SUPPLIER_TABLE_ID,
};

pub fn create_tpch_q9_nodes() -> Vec<OperatorNode> {
    let node_78 = OperatorNode {
        typ: PhysicalNodeType::PhysicalSort,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::SortOrder(SortOrderType::Asc),
                    children: vec![Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(0)),
                    })],
                    data: None,
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::SortOrder(SortOrderType::Desc),
                    children: vec![Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(1)),
                    })],
                    data: None,
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(78),
            expr_id: ExprId(80),
            children_group_ids: vec![GroupId(75)],
        },
    };

    let node_75 = OperatorNode {
        typ: PhysicalNodeType::PhysicalAgg,
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::Func(FuncType::Agg(AggregateFunction::Sum)),
                    children: vec![Arc::new(PredicateNode {
                        typ: PredicateType::List,
                        children: vec![Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(2)),
                        })],
                        data: None,
                    })],
                    data: None,
                })],
                data: None,
            }),
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![
                    Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(0)),
                    }),
                    Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(1)),
                    }),
                ],
                data: None,
            }),
        ],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(75),
            expr_id: ExprId(12398),
            children_group_ids: vec![GroupId(71)],
        },
    };

    let node_71 = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(7)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::Func(FuncType::Scalar(BuiltinScalarFunction::DatePart)),
                    children: vec![Arc::new(PredicateNode {
                        typ: PredicateType::List,
                        children: vec![
                            Arc::new(PredicateNode {
                                typ: PredicateType::Constant(ConstantType::Utf8String),
                                children: vec![],
                                data: Some(Value::String("YEAR".into())),
                            }),
                            Arc::new(PredicateNode {
                                typ: PredicateType::AttrIndex,
                                children: vec![],
                                data: Some(Value::UInt64(5)),
                            }),
                        ],
                        data: None,
                    })],
                    data: None,
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::BinOp(BinOpType::Sub),
                    children: vec![
                        Arc::new(PredicateNode {
                            typ: PredicateType::BinOp(BinOpType::Mul),
                            children: vec![
                                Arc::new(PredicateNode {
                                    typ: PredicateType::AttrIndex,
                                    children: vec![],
                                    data: Some(Value::UInt64(1)),
                                }),
                                Arc::new(PredicateNode {
                                    typ: PredicateType::BinOp(BinOpType::Sub),
                                    children: vec![
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::Constant(ConstantType::Decimal),
                                            children: vec![],
                                            data: Some(Value::Float(SerializableOrderedF64(
                                                OrderedFloat(1.0),
                                            ))),
                                        }),
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::AttrIndex,
                                            children: vec![],
                                            data: Some(Value::UInt64(2)),
                                        }),
                                    ],
                                    data: None,
                                }),
                            ],
                            data: None,
                        }),
                        Arc::new(PredicateNode {
                            typ: PredicateType::BinOp(BinOpType::Mul),
                            children: vec![
                                Arc::new(PredicateNode {
                                    typ: PredicateType::AttrIndex,
                                    children: vec![],
                                    data: Some(Value::UInt64(4)),
                                }),
                                Arc::new(PredicateNode {
                                    typ: PredicateType::AttrIndex,
                                    children: vec![],
                                    data: Some(Value::UInt64(0)),
                                }),
                            ],
                            data: None,
                        }),
                    ],
                    data: None,
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(71),
            expr_id: ExprId(12400),
            children_group_ids: vec![GroupId(68)],
        },
    };

    let node_68 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(3)),
                })],
                data: None,
            }),
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(0)),
                })],
                data: None,
            }),
        ],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(68),
            expr_id: ExprId(16554),
            children_group_ids: vec![GroupId(59), GroupId(65)],
        },
    };

    let node_65 = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(0)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(1)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(65),
            expr_id: ExprId(12407),
            children_group_ids: vec![GroupId(62)],
        },
    };

    let node_62 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(NATION_TABLE_ID)), // nation,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(62),
            expr_id: ExprId(12409),
            children_group_ids: vec![],
        },
    };

    let node_59 = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(13)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(14)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(15)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(28)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(34)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(36)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(59),
            expr_id: ExprId(15866),
            children_group_ids: vec![GroupId(11775)],
        },
    };

    let node_11775 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(9)),
                })],
                data: None,
            }),
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(0)),
                })],
                data: None,
            }),
        ],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(11775),
            expr_id: ExprId(15869),
            children_group_ids: vec![GroupId(11371), GroupId(53)],
        },
    };

    let node_53 = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(0)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(4)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(53),
            expr_id: ExprId(12416),
            children_group_ids: vec![GroupId(50)],
        },
    };

    let node_50 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(ORDERS_TABLE_ID)), // orders,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(50),
            expr_id: ExprId(12418),
            children_group_ids: vec![],
        },
    };

    let node_11371 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![
                    Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(11)),
                    }),
                    Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(10)),
                    }),
                ],
                data: None,
            }),
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![
                    Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(1)),
                    }),
                    Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(0)),
                    }),
                ],
                data: None,
            }),
        ],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(11371),
            expr_id: ExprId(15347),
            children_group_ids: vec![GroupId(893), GroupId(41)],
        },
    };

    let node_41 = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(0)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(1)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(3)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(41),
            expr_id: ExprId(12425),
            children_group_ids: vec![GroupId(38)],
        },
    };

    let node_38 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(PARTSUPP_TABLE_ID)), // partsupp,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(38),
            expr_id: ExprId(12427),
            children_group_ids: vec![],
        },
    };

    let node_893 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(11)),
                })],
                data: None,
            }),
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(0)),
                })],
                data: None,
            }),
        ],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(893),
            expr_id: ExprId(12922),
            children_group_ids: vec![GroupId(139), GroupId(26)],
        },
    };

    let node_26 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(SUPPLIER_TABLE_ID)), // supplier,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(26),
            expr_id: ExprId(12436),
            children_group_ids: vec![],
        },
    };

    let node_139 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(0)),
                })],
                data: None,
            }),
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(1)),
                })],
                data: None,
            }),
        ],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(139),
            expr_id: ExprId(12472),
            children_group_ids: vec![GroupId(83), GroupId(14)],
        },
    };

    let node_14 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(LINEITEM_TABLE_ID)), // lineitem,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(14),
            expr_id: ExprId(12463),
            children_group_ids: vec![],
        },
    };

    let node_83 = OperatorNode {
        typ: PhysicalNodeType::PhysicalFilter,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Like,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(1)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::Constant(ConstantType::Utf8String),
                    children: vec![],
                    data: Some(Value::String("%forest%".into())),
                }),
            ],
            data: Some(Value::Serialized([0, 0].into())),
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(83),
            expr_id: ExprId(12465),
            children_group_ids: vec![GroupId(2)],
        },
    };

    let node_2 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(PART_TABLE_ID)), // part,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(2),
            expr_id: ExprId(12467),
            children_group_ids: vec![],
        },
    };

    vec![
        node_2, node_83, node_14, node_139, node_26, node_893, node_38, node_41, node_11371,
        node_50, node_53, node_11775, node_59, node_62, node_65, node_68, node_71, node_75,
        node_78,
    ]
}

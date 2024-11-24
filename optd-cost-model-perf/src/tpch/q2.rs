use std::sync::Arc;

use datafusion_expr::AggregateFunction;
use optd_cost_model::{
    common::{
        nodes::{JoinType, PhysicalNodeType, PredicateNode, PredicateType},
        predicates::{
            bin_op_pred::BinOpType, constant_pred::ConstantType, func_pred::FuncType,
            log_op_pred::LogOpType, sort_order_pred::SortOrderType,
        },
        types::{ExprId, GroupId},
        values::Value,
    },
    ComputeCostContext,
};

use crate::tpch::{
    NATION_TABLE_ID, PARTSUPP_TABLE_ID, PART_TABLE_ID, REGION_TABLE_ID, SUPPLIER_TABLE_ID,
};

use super::OperatorNode;

pub fn create_tpch_q2_nodes() -> Vec<OperatorNode> {
    let node_114 = OperatorNode {
        typ: PhysicalNodeType::PhysicalLimit,
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::Constant(ConstantType::UInt64),
                children: vec![],
                data: Some(Value::UInt64(0)),
            }),
            Arc::new(PredicateNode {
                typ: PredicateType::Constant(ConstantType::UInt64),
                children: vec![],
                data: Some(Value::UInt64(100)),
            }),
        ],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(114),
            expr_id: ExprId(117),
            children_group_ids: vec![GroupId(110)],
        },
    };

    let node_110 = OperatorNode {
        typ: PhysicalNodeType::PhysicalSort,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::SortOrder(SortOrderType::Desc),
                    children: vec![Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(0)),
                    })],
                    data: None,
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::SortOrder(SortOrderType::Asc),
                    children: vec![Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(2)),
                    })],
                    data: None,
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::SortOrder(SortOrderType::Asc),
                    children: vec![Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(1)),
                    })],
                    data: None,
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::SortOrder(SortOrderType::Asc),
                    children: vec![Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(3)),
                    })],
                    data: None,
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(110),
            expr_id: ExprId(12508),
            children_group_ids: vec![GroupId(107)],
        },
    };

    let node_107 = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(19)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(15)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(22)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(0)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(2)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(16)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(18)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(20)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(107),
            expr_id: ExprId(16293),
            children_group_ids: vec![GroupId(12190)],
        },
    };

    let node_12190 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
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
                        data: Some(Value::UInt64(12)),
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
            group_id: GroupId(12190),
            expr_id: ExprId(16296),
            children_group_ids: vec![GroupId(11835), GroupId(101)],
        },
    };

    let node_101 = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
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
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(101),
            expr_id: ExprId(12515),
            children_group_ids: vec![GroupId(98)],
        },
    };

    let node_98 = OperatorNode {
        typ: PhysicalNodeType::PhysicalAgg,
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::Func(FuncType::Agg(AggregateFunction::Min)),
                    children: vec![Arc::new(PredicateNode {
                        typ: PredicateType::List,
                        children: vec![Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(1)),
                        })],
                        data: None,
                    })],
                    data: None,
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
            group_id: GroupId(98),
            expr_id: ExprId(12518),
            children_group_ids: vec![GroupId(94)],
        },
    };

    let node_94 = OperatorNode {
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
            group_id: GroupId(94),
            expr_id: ExprId(12520),
            children_group_ids: vec![GroupId(91)],
        },
    };

    let node_91 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(2)),
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
            group_id: GroupId(91),
            expr_id: ExprId(12523),
            children_group_ids: vec![GroupId(84), GroupId(56)],
        },
    };

    let node_84 = OperatorNode {
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
                    data: Some(Value::UInt64(4)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(84),
            expr_id: ExprId(12525),
            children_group_ids: vec![GroupId(81)],
        },
    };

    let node_81 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(2)),
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
            group_id: GroupId(81),
            expr_id: ExprId(12528),
            children_group_ids: vec![GroupId(74), GroupId(78)],
        },
    };

    let node_78 = OperatorNode {
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
                    data: Some(Value::UInt64(2)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(78),
            expr_id: ExprId(12545),
            children_group_ids: vec![GroupId(35)],
        },
    };

    let node_74 = OperatorNode {
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
                    data: Some(Value::UInt64(2)),
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
            group_id: GroupId(74),
            expr_id: ExprId(12530),
            children_group_ids: vec![GroupId(71)],
        },
    };

    let node_71 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(1)),
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
            group_id: GroupId(71),
            expr_id: ExprId(12533),
            children_group_ids: vec![GroupId(17), GroupId(68)],
        },
    };

    let node_68 = OperatorNode {
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
                    data: Some(Value::UInt64(3)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(68),
            expr_id: ExprId(12539),
            children_group_ids: vec![GroupId(26)],
        },
    };

    let node_17 = OperatorNode {
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
            group_id: GroupId(17),
            expr_id: ExprId(12535),
            children_group_ids: vec![GroupId(14)],
        },
    };

    let node_11835 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(23)),
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
            group_id: GroupId(11835),
            expr_id: ExprId(15837),
            children_group_ids: vec![GroupId(11494), GroupId(56)],
        },
    };

    let node_56 = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![Arc::new(PredicateNode {
                typ: PredicateType::AttrIndex,
                children: vec![],
                data: Some(Value::UInt64(0)),
            })],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(56),
            expr_id: ExprId(12551),
            children_group_ids: vec![GroupId(120)],
        },
    };

    let node_120 = OperatorNode {
        typ: PhysicalNodeType::PhysicalFilter,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::BinOp(BinOpType::Eq),
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(1)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::Constant(ConstantType::Utf8String),
                    children: vec![],
                    data: Some(Value::String("EUROPE".into())),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(120),
            expr_id: ExprId(12553),
            children_group_ids: vec![GroupId(47)],
        },
    };

    let node_47 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(REGION_TABLE_ID)), // region,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(47),
            expr_id: ExprId(12555),
            children_group_ids: vec![],
        },
    };

    let node_11494 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(17)),
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
            group_id: GroupId(11494),
            expr_id: ExprId(15396),
            children_group_ids: vec![GroupId(1204), GroupId(38)],
        },
    };

    let node_38 = OperatorNode {
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
                    data: Some(Value::UInt64(2)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(38),
            expr_id: ExprId(12581),
            children_group_ids: vec![GroupId(35)],
        },
    };

    let node_35 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(NATION_TABLE_ID)), // nation,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(35),
            expr_id: ExprId(12547),
            children_group_ids: vec![],
        },
    };

    let node_1204 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(10)),
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
            group_id: GroupId(1204),
            expr_id: ExprId(13159),
            children_group_ids: vec![GroupId(245), GroupId(26)],
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
            expr_id: ExprId(12541),
            children_group_ids: vec![],
        },
    };

    let node_245 = OperatorNode {
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
                    data: Some(Value::UInt64(0)),
                })],
                data: None,
            }),
        ],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(245),
            expr_id: ExprId(12624),
            children_group_ids: vec![GroupId(156), GroupId(14)],
        },
    };

    let node_14 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(PARTSUPP_TABLE_ID)), // partsupp,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(14),
            expr_id: ExprId(12537),
            children_group_ids: vec![],
        },
    };

    let node_156 = OperatorNode {
        typ: PhysicalNodeType::PhysicalFilter,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::LogOp(LogOpType::And),
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::BinOp(BinOpType::Eq),
                    children: vec![
                        Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(5)),
                        }),
                        Arc::new(PredicateNode {
                            typ: PredicateType::Constant(ConstantType::Int32),
                            children: vec![],
                            data: Some(Value::Int32(44)),
                        }),
                    ],
                    data: None,
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::Like,
                    children: vec![
                        Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(4)),
                        }),
                        Arc::new(PredicateNode {
                            typ: PredicateType::Constant(ConstantType::Utf8String),
                            children: vec![],
                            data: Some(Value::String("%TIN".into())),
                        }),
                    ],
                    data: Some(Value::Serialized([0, 0].into())),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(156),
            expr_id: ExprId(12617),
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
            expr_id: ExprId(12619),
            children_group_ids: vec![],
        },
    };

    vec![
        node_2, node_156, node_14, node_245, node_26, node_1204, node_35, node_38, node_11494,
        node_47, node_120, node_56, node_11835, node_17, node_68, node_71, node_74, node_78,
        node_81, node_84, node_91, node_94, node_98, node_101, node_12190, node_107, node_110,
        node_114,
    ]
}

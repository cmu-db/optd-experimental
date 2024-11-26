use std::{collections::HashMap, sync::Arc};

use datafusion_expr::{AggregateFunction, BuiltinScalarFunction};
use optd_cost_model::{
    common::{
        nodes::{JoinType, PhysicalNodeType, PredicateNode, PredicateType},
        predicates::{
            bin_op_pred::BinOpType, constant_pred::ConstantType, func_pred::FuncType,
            log_op_pred::LogOpType, sort_order_pred::SortOrderType,
        },
        properties::{
            attr_ref::{
                AttrRef, BaseTableAttrRef, EqPredicate, GroupAttrRefs, SemanticCorrelation,
            },
            schema::Schema,
            Attribute,
        },
        types::{ExprId, GroupId, TableId},
        values::{SerializableOrderedF64, Value},
    },
    test_utils::tests::MemoGroupInfo,
    ComputeCostContext,
};
use ordered_float::OrderedFloat;

use crate::{
    init_tpch_query,
    tpch::{NATION_TABLE_ID, SUPPLIER_TABLE_ID},
};

use super::{OperatorNode, CUSTOMER_TABLE_ID, LINEITEM_TABLE_ID, ORDERS_TABLE_ID};

pub fn create_tpch_q7_memo() -> HashMap<GroupId, MemoGroupInfo> {
    let mut memo = HashMap::new();

    memo.insert(
        GroupId(2),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "s_suppkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_address".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_nationkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_phone".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_acctbal".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 6,
                    }),
                ],
                None,
            ),
        },
    );

    memo.insert(
        GroupId(8),
        MemoGroupInfo {
            schema: Schema {
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
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 8,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 9,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 10,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 11,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 12,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 13,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 14,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 15,
                    }),
                ],
                None,
            ),
        },
    );

    memo.insert(
        GroupId(96),
        MemoGroupInfo {
            schema: Schema {
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
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 8,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 9,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 10,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 11,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 12,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 13,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 14,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 15,
                    }),
                ],
                None,
            ),
        },
    );

    let mut semantic_correlation_118 = SemanticCorrelation::new();
    let eq_predicates = vec![EqPredicate {
        left: BaseTableAttrRef {
            table_id: TableId(SUPPLIER_TABLE_ID), // supplier
            attr_idx: 0,
        },
        right: BaseTableAttrRef {
            table_id: TableId(LINEITEM_TABLE_ID), // lineitem
            attr_idx: 2,
        },
    }];
    for eq_predicate in eq_predicates {
        semantic_correlation_118.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(118),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "s_suppkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_address".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_nationkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_phone".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_acctbal".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
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
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 8,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 9,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 10,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 11,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 12,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 13,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 14,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 15,
                    }),
                ],
                Some(semantic_correlation_118),
            ),
        },
    );

    memo.insert(
        GroupId(23),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "o_orderkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_custkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderstatus".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_totalprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderdate".to_string(),
                        typ: ConstantType::Date,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderpriority".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_clerk".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_shippriority".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 8,
                    }),
                ],
                None,
            ),
        },
    );

    let mut semantic_correlation_542 = SemanticCorrelation::new();
    let eq_predicates = vec![
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(ORDERS_TABLE_ID), // orders
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_542.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(542),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "s_suppkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_address".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_nationkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_phone".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_acctbal".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
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
                    Attribute {
                        name: "o_orderkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_custkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderstatus".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_totalprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderdate".to_string(),
                        typ: ConstantType::Date,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderpriority".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_clerk".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_shippriority".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 8,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 9,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 10,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 11,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 12,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 13,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 14,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 15,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 8,
                    }),
                ],
                Some(semantic_correlation_542),
            ),
        },
    );

    memo.insert(
        GroupId(35),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "c_custkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "c_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "c_address".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "c_nationkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "c_phone".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "c_acctbal".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "c_mktsegment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "c_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 7,
                    }),
                ],
                None,
            ),
        },
    );

    memo.insert(
        GroupId(38),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 3,
                    }),
                ],
                None,
            ),
        },
    );

    let mut semantic_correlation_11439 = SemanticCorrelation::new();
    let eq_predicates = vec![
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(ORDERS_TABLE_ID), // orders
                attr_idx: 1,
            },
            right: BaseTableAttrRef {
                table_id: TableId(CUSTOMER_TABLE_ID), // customer
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(ORDERS_TABLE_ID), // orders
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_11439.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(11439),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "s_suppkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_address".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_nationkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_phone".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_acctbal".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
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
                    Attribute {
                        name: "o_orderkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_custkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderstatus".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_totalprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderdate".to_string(),
                        typ: ConstantType::Date,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderpriority".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_clerk".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_shippriority".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 8,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 9,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 10,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 11,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 12,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 13,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 14,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 15,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 8,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 3,
                    }),
                ],
                Some(semantic_correlation_11439),
            ),
        },
    );

    memo.insert(
        GroupId(47),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "n_nationkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "n_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "n_regionkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "n_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 3,
                    }),
                ],
                None,
            ),
        },
    );

    memo.insert(
        GroupId(89),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "n_nationkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "n_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "n_regionkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "n_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 3,
                    }),
                ],
                None,
            ),
        },
    );

    memo.insert(
        GroupId(53),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                ],
                None,
            ),
        },
    );

    let mut semantic_correlation_11801 = SemanticCorrelation::new();
    let eq_predicates = vec![
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(ORDERS_TABLE_ID), // orders
                attr_idx: 1,
            },
            right: BaseTableAttrRef {
                table_id: TableId(CUSTOMER_TABLE_ID), // customer
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(ORDERS_TABLE_ID), // orders
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                attr_idx: 3,
            },
            right: BaseTableAttrRef {
                table_id: TableId(NATION_TABLE_ID), // nation
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_11801.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(11801),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "s_suppkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_address".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_nationkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_phone".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_acctbal".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
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
                    Attribute {
                        name: "o_orderkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_custkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderstatus".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_totalprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderdate".to_string(),
                        typ: ConstantType::Date,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderpriority".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_clerk".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_shippriority".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 8,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 9,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 10,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 11,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 12,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 13,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 14,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 15,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 8,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                ],
                Some(semantic_correlation_11801),
            ),
        },
    );

    memo.insert(
        GroupId(82),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "n_nationkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "n_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "n_regionkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "n_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 3,
                    }),
                ],
                None,
            ),
        },
    );

    memo.insert(
        GroupId(64),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                ],
                None,
            ),
        },
    );

    let mut semantic_correlation_12177 = SemanticCorrelation::new();
    let eq_predicates = vec![
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(CUSTOMER_TABLE_ID), // customer
                attr_idx: 3,
            },
            right: BaseTableAttrRef {
                table_id: TableId(NATION_TABLE_ID), // nation
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(ORDERS_TABLE_ID), // orders
                attr_idx: 1,
            },
            right: BaseTableAttrRef {
                table_id: TableId(CUSTOMER_TABLE_ID), // customer
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(ORDERS_TABLE_ID), // orders
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                attr_idx: 3,
            },
            right: BaseTableAttrRef {
                table_id: TableId(NATION_TABLE_ID), // nation
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_12177.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(12177),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "s_suppkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_address".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_nationkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_phone".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_acctbal".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "s_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
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
                    Attribute {
                        name: "o_orderkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_custkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderstatus".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_totalprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderdate".to_string(),
                        typ: ConstantType::Date,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_orderpriority".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_clerk".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_shippriority".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "o_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 8,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 9,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 10,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 11,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 12,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 13,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 14,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 15,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 8,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                ],
                Some(semantic_correlation_12177),
            ),
        },
    );

    let mut semantic_correlation_67 = SemanticCorrelation::new();
    let eq_predicates = vec![
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(CUSTOMER_TABLE_ID), // customer
                attr_idx: 3,
            },
            right: BaseTableAttrRef {
                table_id: TableId(NATION_TABLE_ID), // nation
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(ORDERS_TABLE_ID), // orders
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(ORDERS_TABLE_ID), // orders
                attr_idx: 1,
            },
            right: BaseTableAttrRef {
                table_id: TableId(CUSTOMER_TABLE_ID), // customer
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                attr_idx: 3,
            },
            right: BaseTableAttrRef {
                table_id: TableId(NATION_TABLE_ID), // nation
                attr_idx: 0,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_67.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(67),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                        attr_idx: 10,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(CUSTOMER_TABLE_ID), // customer
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                ],
                Some(semantic_correlation_67),
            ),
        },
    );

    let mut semantic_correlation_70 = SemanticCorrelation::new();
    let eq_predicates = vec![
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(CUSTOMER_TABLE_ID), // customer
                attr_idx: 3,
            },
            right: BaseTableAttrRef {
                table_id: TableId(NATION_TABLE_ID), // nation
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(ORDERS_TABLE_ID), // orders
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(ORDERS_TABLE_ID), // orders
                attr_idx: 1,
            },
            right: BaseTableAttrRef {
                table_id: TableId(CUSTOMER_TABLE_ID), // customer
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                attr_idx: 3,
            },
            right: BaseTableAttrRef {
                table_id: TableId(NATION_TABLE_ID), // nation
                attr_idx: 0,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_70.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(70),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::Derived,
                    AttrRef::Derived,
                ],
                Some(semantic_correlation_70),
            ),
        },
    );

    memo.insert(
        GroupId(74),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::Binary,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::Derived,
                    AttrRef::Derived,
                ],
                None,
            ),
        },
    );

    memo.insert(
        GroupId(77),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::Binary,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                    Attribute {
                        name: "unnamed".to_string(),
                        typ: ConstantType::UInt64,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::Derived,
                    AttrRef::Derived,
                ],
                None,
            ),
        },
    );

    memo
}

fn create_tpch_q7_nodes() -> Vec<OperatorNode> {
    let node_77 = OperatorNode {
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
                        data: Some(Value::UInt64(2)),
                    })],
                    data: None,
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(77),
            expr_id: ExprId(79),
            children_group_ids: vec![GroupId(74)],
        },
    };

    let node_74 = OperatorNode {
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
                            data: Some(Value::UInt64(3)),
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
                    Arc::new(PredicateNode {
                        typ: PredicateType::AttrIndex,
                        children: vec![],
                        data: Some(Value::UInt64(2)),
                    }),
                ],
                data: None,
            }),
        ],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(74),
            expr_id: ExprId(12372),
            children_group_ids: vec![GroupId(70)],
        },
    };

    let node_70 = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(4)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(6)),
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
                                data: Some(Value::UInt64(2)),
                            }),
                        ],
                        data: None,
                    })],
                    data: None,
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::BinOp(BinOpType::Mul),
                    children: vec![
                        Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(0)),
                        }),
                        Arc::new(PredicateNode {
                            typ: PredicateType::BinOp(BinOpType::Sub),
                            children: vec![
                                Arc::new(PredicateNode {
                                    typ: PredicateType::Constant(ConstantType::Decimal),
                                    children: vec![],
                                    data: Some(Value::Float(SerializableOrderedF64(OrderedFloat(
                                        1.0,
                                    )))),
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
                    data: None,
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(70),
            expr_id: ExprId(12374),
            children_group_ids: vec![GroupId(67)],
        },
    };

    let node_67 = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(12)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(13)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(17)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(33)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(35)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(36)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(37)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(67),
            expr_id: ExprId(16078),
            children_group_ids: vec![GroupId(12177)],
        },
    };

    let node_12177 = OperatorNode {
        typ: PhysicalNodeType::PhysicalNestedLoopJoin(JoinType::Inner),
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::LogOp(LogOpType::And),
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::BinOp(BinOpType::Eq),
                    children: vec![
                        Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(33)),
                        }),
                        Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(36)),
                        }),
                    ],
                    data: None,
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::LogOp(LogOpType::Or),
                    children: vec![
                        Arc::new(PredicateNode {
                            typ: PredicateType::LogOp(LogOpType::And),
                            children: vec![
                                Arc::new(PredicateNode {
                                    typ: PredicateType::BinOp(BinOpType::Eq),
                                    children: vec![
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::AttrIndex,
                                            children: vec![],
                                            data: Some(Value::UInt64(35)),
                                        }),
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::Constant(ConstantType::Utf8String),
                                            children: vec![],
                                            data: Some(Value::String("IRAN".into())),
                                        }),
                                    ],
                                    data: None,
                                }),
                                Arc::new(PredicateNode {
                                    typ: PredicateType::BinOp(BinOpType::Eq),
                                    children: vec![
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::AttrIndex,
                                            children: vec![],
                                            data: Some(Value::UInt64(37)),
                                        }),
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::Constant(ConstantType::Utf8String),
                                            children: vec![],
                                            data: Some(Value::String("ETHIOPIA".into())),
                                        }),
                                    ],
                                    data: None,
                                }),
                            ],
                            data: None,
                        }),
                        Arc::new(PredicateNode {
                            typ: PredicateType::LogOp(LogOpType::And),
                            children: vec![
                                Arc::new(PredicateNode {
                                    typ: PredicateType::BinOp(BinOpType::Eq),
                                    children: vec![
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::AttrIndex,
                                            children: vec![],
                                            data: Some(Value::UInt64(35)),
                                        }),
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::Constant(ConstantType::Utf8String),
                                            children: vec![],
                                            data: Some(Value::String("ETHIOPIA".into())),
                                        }),
                                    ],
                                    data: None,
                                }),
                                Arc::new(PredicateNode {
                                    typ: PredicateType::BinOp(BinOpType::Eq),
                                    children: vec![
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::AttrIndex,
                                            children: vec![],
                                            data: Some(Value::UInt64(37)),
                                        }),
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::Constant(ConstantType::Utf8String),
                                            children: vec![],
                                            data: Some(Value::String("IRAN".into())),
                                        }),
                                    ],
                                    data: None,
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
            group_id: GroupId(12177),
            expr_id: ExprId(16080),
            children_group_ids: vec![GroupId(11801), GroupId(64)],
        },
    };

    let node_64 = OperatorNode {
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
            group_id: GroupId(64),
            expr_id: ExprId(12380),
            children_group_ids: vec![GroupId(82)],
        },
    };

    let node_82 = OperatorNode {
        typ: PhysicalNodeType::PhysicalFilter,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::LogOp(LogOpType::Or),
            children: vec![
                Arc::new(PredicateNode {
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
                            data: Some(Value::String("ETHIOPIA".into())),
                        }),
                    ],
                    data: None,
                }),
                Arc::new(PredicateNode {
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
                            data: Some(Value::String("IRAN".into())),
                        }),
                    ],
                    data: None,
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(82),
            expr_id: ExprId(12382),
            children_group_ids: vec![GroupId(47)],
        },
    };

    let node_11801 = OperatorNode {
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
            group_id: GroupId(11801),
            expr_id: ExprId(15708),
            children_group_ids: vec![GroupId(11439), GroupId(53)],
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
                    data: Some(Value::UInt64(1)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(53),
            expr_id: ExprId(12395),
            children_group_ids: vec![GroupId(89)],
        },
    };

    let node_89 = OperatorNode {
        typ: PhysicalNodeType::PhysicalFilter,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::LogOp(LogOpType::Or),
            children: vec![
                Arc::new(PredicateNode {
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
                            data: Some(Value::String("IRAN".into())),
                        }),
                    ],
                    data: None,
                }),
                Arc::new(PredicateNode {
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
                            data: Some(Value::String("ETHIOPIA".into())),
                        }),
                    ],
                    data: None,
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(89),
            expr_id: ExprId(12397),
            children_group_ids: vec![GroupId(47)],
        },
    };

    let node_47 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(NATION_TABLE_ID)), // nation,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(47),
            expr_id: ExprId(12384),
            children_group_ids: vec![],
        },
    };

    let node_11439 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(24)),
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
            group_id: GroupId(11439),
            expr_id: ExprId(15240),
            children_group_ids: vec![GroupId(542), GroupId(38)],
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
                    data: Some(Value::UInt64(3)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(38),
            expr_id: ExprId(12406),
            children_group_ids: vec![GroupId(35)],
        },
    };

    let node_35 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(CUSTOMER_TABLE_ID)), // customer,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(35),
            expr_id: ExprId(12408),
            children_group_ids: vec![],
        },
    };

    let node_542 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(7)),
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
            group_id: GroupId(542),
            expr_id: ExprId(12720),
            children_group_ids: vec![GroupId(118), GroupId(23)],
        },
    };

    let node_23 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(ORDERS_TABLE_ID)), // orders,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(23),
            expr_id: ExprId(12417),
            children_group_ids: vec![],
        },
    };

    let node_118 = OperatorNode {
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
                    data: Some(Value::UInt64(2)),
                })],
                data: None,
            }),
        ],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(118),
            expr_id: ExprId(12449),
            children_group_ids: vec![GroupId(2), GroupId(96)],
        },
    };

    let node_96 = OperatorNode {
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
                    typ: PredicateType::BinOp(BinOpType::Leq),
                    children: vec![
                        Arc::new(PredicateNode {
                            typ: PredicateType::AttrIndex,
                            children: vec![],
                            data: Some(Value::UInt64(10)),
                        }),
                        Arc::new(PredicateNode {
                            typ: PredicateType::Constant(ConstantType::Date),
                            children: vec![],
                            data: Some(Value::Int64(9861)),
                        }),
                    ],
                    data: None,
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(96),
            expr_id: ExprId(12440),
            children_group_ids: vec![GroupId(8)],
        },
    };

    let node_8 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(LINEITEM_TABLE_ID)), // lineitem,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(8),
            expr_id: ExprId(12442),
            children_group_ids: vec![],
        },
    };

    let node_2 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(SUPPLIER_TABLE_ID)), // supplier,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(2),
            expr_id: ExprId(12444),
            children_group_ids: vec![],
        },
    };

    vec![
        node_2, node_8, node_96, node_118, node_23, node_542, node_35, node_38, node_11439,
        node_47, node_89, node_53, node_11801, node_82, node_64, node_12177, node_67, node_70,
        node_74, node_77,
    ]
}

init_tpch_query!(q7);

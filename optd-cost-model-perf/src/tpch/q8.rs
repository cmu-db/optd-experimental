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
    tpch::{
        CUSTOMER_TABLE_ID, LINEITEM_TABLE_ID, NATION_TABLE_ID, ORDERS_TABLE_ID, PART_TABLE_ID,
        REGION_TABLE_ID, SUPPLIER_TABLE_ID,
    },
};

use super::OperatorNode;

pub fn create_tpch_q8_memo() -> HashMap<GroupId, MemoGroupInfo> {
    let mut memo = HashMap::new();

    memo.insert(
        GroupId(26),
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
        GroupId(2),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "p_partkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_mfgr".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_brand".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_type".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_size".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_container".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_retailprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 8,
                    }),
                ],
                None,
            ),
        },
    );

    memo.insert(
        GroupId(162),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "p_partkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_mfgr".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_brand".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_type".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_size".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_container".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_retailprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 8,
                    }),
                ],
                None,
            ),
        },
    );

    memo.insert(
        GroupId(14),
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

    let mut semantic_correlation_218 = SemanticCorrelation::new();
    let eq_predicates = vec![EqPredicate {
        left: BaseTableAttrRef {
            table_id: TableId(PART_TABLE_ID), // part
            attr_idx: 0,
        },
        right: BaseTableAttrRef {
            table_id: TableId(LINEITEM_TABLE_ID), // lineitem
            attr_idx: 1,
        },
    }];
    for eq_predicate in eq_predicates {
        semantic_correlation_218.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(218),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "p_partkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_mfgr".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_brand".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_type".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_size".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_container".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_retailprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_comment".to_string(),
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
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 8,
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
                Some(semantic_correlation_218),
            ),
        },
    );

    let mut semantic_correlation_1004 = SemanticCorrelation::new();
    let eq_predicates = vec![
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(PART_TABLE_ID), // part
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 1,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_1004.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(1004),
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
                        name: "p_partkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_mfgr".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_brand".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_type".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_size".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_container".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_retailprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_comment".to_string(),
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
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 8,
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
                Some(semantic_correlation_1004),
            ),
        },
    );

    memo.insert(
        GroupId(38),
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

    memo.insert(
        GroupId(155),
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

    memo.insert(
        GroupId(44),
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
                        attr_idx: 4,
                    }),
                ],
                None,
            ),
        },
    );

    let mut semantic_correlation_11253 = SemanticCorrelation::new();
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
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(PART_TABLE_ID), // part
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 1,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_11253.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(11253),
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
                        name: "p_partkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_mfgr".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_brand".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_type".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_size".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_container".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_retailprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_comment".to_string(),
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
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 8,
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
                        attr_idx: 4,
                    }),
                ],
                Some(semantic_correlation_11253),
            ),
        },
    );

    memo.insert(
        GroupId(53),
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
        GroupId(56),
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

    let mut semantic_correlation_11657 = SemanticCorrelation::new();
    let eq_predicates = vec![
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
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
                table_id: TableId(PART_TABLE_ID), // part
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 1,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_11657.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(11657),
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
                        name: "p_partkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_mfgr".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_brand".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_type".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_size".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_container".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_retailprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_comment".to_string(),
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
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 8,
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
                        attr_idx: 4,
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
                Some(semantic_correlation_11657),
            ),
        },
    );

    memo.insert(
        GroupId(65),
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
        GroupId(68),
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
                        attr_idx: 2,
                    }),
                ],
                None,
            ),
        },
    );

    let mut semantic_correlation_12075 = SemanticCorrelation::new();
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
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
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
                table_id: TableId(PART_TABLE_ID), // part
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 1,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_12075.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(12075),
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
                        name: "p_partkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_mfgr".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_brand".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_type".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_size".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_container".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_retailprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_comment".to_string(),
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
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 8,
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
                        attr_idx: 4,
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
                        attr_idx: 2,
                    }),
                ],
                Some(semantic_correlation_12075),
            ),
        },
    );

    memo.insert(
        GroupId(78),
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

    let mut semantic_correlation_12507 = SemanticCorrelation::new();
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
                attr_idx: 3,
            },
            right: BaseTableAttrRef {
                table_id: TableId(NATION_TABLE_ID), // nation
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
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
                table_id: TableId(PART_TABLE_ID), // part
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 1,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_12507.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(12507),
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
                        name: "p_partkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_mfgr".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_brand".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_type".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_size".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_container".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_retailprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_comment".to_string(),
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
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 8,
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
                        attr_idx: 4,
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
                        attr_idx: 2,
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
                Some(semantic_correlation_12507),
            ),
        },
    );

    memo.insert(
        GroupId(87),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "r_regionkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "r_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "r_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(REGION_TABLE_ID), // region
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(REGION_TABLE_ID), // region
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(REGION_TABLE_ID), // region
                        attr_idx: 2,
                    }),
                ],
                None,
            ),
        },
    );

    memo.insert(
        GroupId(119),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "r_regionkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "r_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "r_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: true,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(REGION_TABLE_ID), // region
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(REGION_TABLE_ID), // region
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(REGION_TABLE_ID), // region
                        attr_idx: 2,
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
                attributes: vec![Attribute {
                    name: "unnamed".to_string(),
                    typ: ConstantType::UInt64,
                    nullable: true,
                }],
            },
            attr_refs: GroupAttrRefs::new(
                vec![AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                    table_id: TableId(REGION_TABLE_ID), // region
                    attr_idx: 0,
                })],
                None,
            ),
        },
    );

    let mut semantic_correlation_12953 = SemanticCorrelation::new();
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
                attr_idx: 3,
            },
            right: BaseTableAttrRef {
                table_id: TableId(NATION_TABLE_ID), // nation
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
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
                table_id: TableId(NATION_TABLE_ID), // nation
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(REGION_TABLE_ID), // region
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(PART_TABLE_ID), // part
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 1,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_12953.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(12953),
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
                        name: "p_partkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_name".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_mfgr".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_brand".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_type".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_size".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_container".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_retailprice".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "p_comment".to_string(),
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
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 5,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 6,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 7,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PART_TABLE_ID), // part
                        attr_idx: 8,
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
                        attr_idx: 4,
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
                        attr_idx: 2,
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
                        table_id: TableId(REGION_TABLE_ID), // region
                        attr_idx: 0,
                    }),
                ],
                Some(semantic_correlation_12953),
            ),
        },
    );

    let mut semantic_correlation_99 = SemanticCorrelation::new();
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
                table_id: TableId(PART_TABLE_ID), // part
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 1,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
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
                table_id: TableId(NATION_TABLE_ID), // nation
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(REGION_TABLE_ID), // region
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
        semantic_correlation_99.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(99),
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
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 4,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(REGION_TABLE_ID), // region
                        attr_idx: 0,
                    }),
                ],
                Some(semantic_correlation_99),
            ),
        },
    );

    let mut semantic_correlation_102 = SemanticCorrelation::new();
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
                table_id: TableId(PART_TABLE_ID), // part
                attr_idx: 0,
            },
            right: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 1,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(SUPPLIER_TABLE_ID), // supplier
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
                table_id: TableId(NATION_TABLE_ID), // nation
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(REGION_TABLE_ID), // region
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
        semantic_correlation_102.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(102),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![Attribute {
                    name: "unnamed".to_string(),
                    typ: ConstantType::UInt64,
                    nullable: true,
                }],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::Derived,
                    AttrRef::Derived,
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                ],
                Some(semantic_correlation_102),
            ),
        },
    );

    memo.insert(
        GroupId(106),
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
                        typ: ConstantType::Binary,
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
                vec![AttrRef::Derived, AttrRef::Derived, AttrRef::Derived],
                None,
            ),
        },
    );

    memo.insert(
        GroupId(109),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![Attribute {
                    name: "unnamed".to_string(),
                    typ: ConstantType::UInt64,
                    nullable: true,
                }],
            },
            attr_refs: GroupAttrRefs::new(vec![AttrRef::Derived, AttrRef::Derived], None),
        },
    );

    memo.insert(
        GroupId(112),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![Attribute {
                    name: "unnamed".to_string(),
                    typ: ConstantType::UInt64,
                    nullable: true,
                }],
            },
            attr_refs: GroupAttrRefs::new(vec![AttrRef::Derived, AttrRef::Derived], None),
        },
    );

    memo
}

pub fn create_tpch_q8_nodes() -> Vec<OperatorNode> {
    let node_112 = OperatorNode {
        typ: PhysicalNodeType::PhysicalSort,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![Arc::new(PredicateNode {
                typ: PredicateType::SortOrder(SortOrderType::Asc),
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(0)),
                })],
                data: None,
            })],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(112),
            expr_id: ExprId(114),
            children_group_ids: vec![GroupId(109)],
        },
    };

    let node_109 = OperatorNode {
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
                    typ: PredicateType::BinOp(BinOpType::Div),
                    children: vec![
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
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(109),
            expr_id: ExprId(116),
            children_group_ids: vec![GroupId(106)],
        },
    };

    let node_106 = OperatorNode {
        typ: PhysicalNodeType::PhysicalAgg,
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![
                    Arc::new(PredicateNode {
                        typ: PredicateType::Func(FuncType::Agg(AggregateFunction::Sum)),
                        children: vec![Arc::new(PredicateNode {
                            typ: PredicateType::List,
                            children: vec![Arc::new(PredicateNode {
                                typ: PredicateType::Func(FuncType::Case),
                                children: vec![Arc::new(PredicateNode {
                                    typ: PredicateType::List,
                                    children: vec![
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::BinOp(BinOpType::Eq),
                                            children: vec![
                                                Arc::new(PredicateNode {
                                                    typ: PredicateType::AttrIndex,
                                                    children: vec![],
                                                    data: Some(Value::UInt64(2)),
                                                }),
                                                Arc::new(PredicateNode {
                                                    typ: PredicateType::Constant(
                                                        ConstantType::Utf8String,
                                                    ),
                                                    children: vec![],
                                                    data: Some(Value::String("ETHIOPIA".into())),
                                                }),
                                            ],
                                            data: None,
                                        }),
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::AttrIndex,
                                            children: vec![],
                                            data: Some(Value::UInt64(1)),
                                        }),
                                        Arc::new(PredicateNode {
                                            typ: PredicateType::Constant(ConstantType::Decimal),
                                            children: vec![],
                                            data: Some(Value::Float(SerializableOrderedF64(
                                                OrderedFloat(0.0),
                                            ))),
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
                        typ: PredicateType::Func(FuncType::Agg(AggregateFunction::Sum)),
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
                    }),
                ],
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
            group_id: GroupId(106),
            expr_id: ExprId(13163),
            children_group_ids: vec![GroupId(102)],
        },
    };

    let node_102 = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
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
            group_id: GroupId(102),
            expr_id: ExprId(13165),
            children_group_ids: vec![GroupId(99)],
        },
    };

    let node_99 = OperatorNode {
        typ: PhysicalNodeType::PhysicalProjection,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::List,
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(21)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(22)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(34)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(38)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(40)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(41)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(99),
            expr_id: ExprId(18316),
            children_group_ids: vec![GroupId(12953)],
        },
    };

    let node_12953 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(38)),
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
            group_id: GroupId(12953),
            expr_id: ExprId(18319),
            children_group_ids: vec![GroupId(12507), GroupId(96)],
        },
    };

    let node_96 = OperatorNode {
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
            group_id: GroupId(96),
            expr_id: ExprId(13172),
            children_group_ids: vec![GroupId(119)],
        },
    };

    let node_119 = OperatorNode {
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
                    data: Some(Value::String("AFRICA".into())),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(119),
            expr_id: ExprId(13174),
            children_group_ids: vec![GroupId(87)],
        },
    };

    let node_87 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(REGION_TABLE_ID)), // region,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(87),
            expr_id: ExprId(13176),
            children_group_ids: vec![],
        },
    };

    let node_12507 = OperatorNode {
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
            group_id: GroupId(12507),
            expr_id: ExprId(17743),
            children_group_ids: vec![GroupId(12075), GroupId(78)],
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
                    data: Some(Value::UInt64(1)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(78),
            expr_id: ExprId(13195),
            children_group_ids: vec![GroupId(65)],
        },
    };

    let node_12075 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(36)),
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
            group_id: GroupId(12075),
            expr_id: ExprId(17185),
            children_group_ids: vec![GroupId(11657), GroupId(68)],
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
                    data: Some(Value::UInt64(2)),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(68),
            expr_id: ExprId(13204),
            children_group_ids: vec![GroupId(65)],
        },
    };

    let node_65 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(NATION_TABLE_ID)), // nation,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(65),
            expr_id: ExprId(13197),
            children_group_ids: vec![],
        },
    };

    let node_11657 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(33)),
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
            group_id: GroupId(11657),
            expr_id: ExprId(16645),
            children_group_ids: vec![GroupId(11253), GroupId(56)],
        },
    };

    let node_56 = OperatorNode {
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
            group_id: GroupId(56),
            expr_id: ExprId(13211),
            children_group_ids: vec![GroupId(53)],
        },
    };

    let node_53 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(CUSTOMER_TABLE_ID)), // customer,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(53),
            expr_id: ExprId(13213),
            children_group_ids: vec![],
        },
    };

    let node_11253 = OperatorNode {
        typ: PhysicalNodeType::PhysicalHashJoin(JoinType::Inner),
        predicates: vec![
            Arc::new(PredicateNode {
                typ: PredicateType::List,
                children: vec![Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(16)),
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
            group_id: GroupId(11253),
            expr_id: ExprId(16123),
            children_group_ids: vec![GroupId(1004), GroupId(44)],
        },
    };

    let node_44 = OperatorNode {
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
            group_id: GroupId(44),
            expr_id: ExprId(13220),
            children_group_ids: vec![GroupId(155)],
        },
    };

    let node_155 = OperatorNode {
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
                            data: Some(Value::UInt64(4)),
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
                            data: Some(Value::UInt64(4)),
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
            group_id: GroupId(155),
            expr_id: ExprId(13222),
            children_group_ids: vec![GroupId(38)],
        },
    };

    let node_38 = OperatorNode {
        typ: PhysicalNodeType::PhysicalScan,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::Constant(ConstantType::Utf8String),
            children: vec![],
            data: Some(Value::UInt64(ORDERS_TABLE_ID)), // orders,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(38),
            expr_id: ExprId(13224),
            children_group_ids: vec![],
        },
    };

    let node_1004 = OperatorNode {
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
                    data: Some(Value::UInt64(11)),
                })],
                data: None,
            }),
        ],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(1004),
            expr_id: ExprId(13696),
            children_group_ids: vec![GroupId(26), GroupId(218)],
        },
    };

    let node_218 = OperatorNode {
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
            group_id: GroupId(218),
            expr_id: ExprId(13273),
            children_group_ids: vec![GroupId(162), GroupId(14)],
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
            expr_id: ExprId(13264),
            children_group_ids: vec![],
        },
    };

    let node_162 = OperatorNode {
        typ: PhysicalNodeType::PhysicalFilter,
        predicates: vec![Arc::new(PredicateNode {
            typ: PredicateType::BinOp(BinOpType::Eq),
            children: vec![
                Arc::new(PredicateNode {
                    typ: PredicateType::AttrIndex,
                    children: vec![],
                    data: Some(Value::UInt64(4)),
                }),
                Arc::new(PredicateNode {
                    typ: PredicateType::Constant(ConstantType::Utf8String),
                    children: vec![],
                    data: Some(Value::String("SMALL BRUSHED NICKEL".into())),
                }),
            ],
            data: None,
        })],
        children_stats: vec![],
        context: ComputeCostContext {
            group_id: GroupId(162),
            expr_id: ExprId(13266),
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
            expr_id: ExprId(13268),
            children_group_ids: vec![],
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
            expr_id: ExprId(13237),
            children_group_ids: vec![],
        },
    };

    vec![
        node_26, node_2, node_162, node_14, node_218, node_1004, node_38, node_155, node_44,
        node_11253, node_53, node_56, node_11657, node_65, node_68, node_12075, node_78,
        node_12507, node_87, node_119, node_96, node_12953, node_99, node_102, node_106, node_109,
        node_112,
    ]
}

init_tpch_query!(q8);

use std::{collections::HashMap, sync::Arc};

use datafusion_expr::{AggregateFunction, BuiltinScalarFunction};
use optd_cost_model::{
    common::{
        nodes::{JoinType, PhysicalNodeType, PredicateNode, PredicateType},
        predicates::{
            bin_op_pred::BinOpType, constant_pred::ConstantType, func_pred::FuncType,
            sort_order_pred::SortOrderType,
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

use super::OperatorNode;
use crate::{
    init_tpch_query,
    tpch::{
        LINEITEM_TABLE_ID, NATION_TABLE_ID, ORDERS_TABLE_ID, PARTSUPP_TABLE_ID, PART_TABLE_ID,
        SUPPLIER_TABLE_ID,
    },
};

pub fn create_tpch_q9_memo() -> HashMap<GroupId, MemoGroupInfo> {
    let mut memo = HashMap::new();

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
        GroupId(83),
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

    let mut semantic_correlation_139 = SemanticCorrelation::new();
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
        semantic_correlation_139.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(139),
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
                Some(semantic_correlation_139),
            ),
        },
    );

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

    let mut semantic_correlation_893 = SemanticCorrelation::new();
    let eq_predicates = vec![
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
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_893.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(893),
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
                Some(semantic_correlation_893),
            ),
        },
    );

    memo.insert(
        GroupId(38),
        MemoGroupInfo {
            schema: Schema {
                attributes: vec![
                    Attribute {
                        name: "ps_partkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "ps_suppkey".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "ps_availqty".to_string(),
                        typ: ConstantType::Int32,
                        nullable: false,
                    },
                    Attribute {
                        name: "ps_supplycost".to_string(),
                        typ: ConstantType::Decimal,
                        nullable: false,
                    },
                    Attribute {
                        name: "ps_comment".to_string(),
                        typ: ConstantType::Utf8String,
                        nullable: false,
                    },
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 2,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 4,
                    }),
                ],
                None,
            ),
        },
    );

    memo.insert(
        GroupId(41),
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
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 3,
                    }),
                ],
                None,
            ),
        },
    );

    let mut semantic_correlation_11371 = SemanticCorrelation::new();
    let eq_predicates = vec![
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
                attr_idx: 1,
            },
            right: BaseTableAttrRef {
                table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                attr_idx: 0,
            },
        },
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                attr_idx: 1,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_11371.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(11371),
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
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 3,
                    }),
                ],
                Some(semantic_correlation_11371),
            ),
        },
    );

    memo.insert(
        GroupId(50),
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
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 0,
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

    let mut semantic_correlation_11775 = SemanticCorrelation::new();
    let eq_predicates = vec![
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
                attr_idx: 1,
            },
            right: BaseTableAttrRef {
                table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
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
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                attr_idx: 1,
            },
        },
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_11775.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(11775),
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
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 1,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 0,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 4,
                    }),
                ],
                Some(semantic_correlation_11775),
            ),
        },
    );

    let mut semantic_correlation_59 = SemanticCorrelation::new();
    let eq_predicates = vec![
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                attr_idx: 1,
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
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 1,
            },
            right: BaseTableAttrRef {
                table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
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
    ];
    for eq_predicate in eq_predicates {
        semantic_correlation_59.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(59),
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
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 4,
                    }),
                ],
                Some(semantic_correlation_59),
            ),
        },
    );

    memo.insert(
        GroupId(62),
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
        GroupId(65),
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

    let mut semantic_correlation_68 = SemanticCorrelation::new();
    let eq_predicates = vec![
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                attr_idx: 1,
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
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 1,
            },
            right: BaseTableAttrRef {
                table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
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
        semantic_correlation_68.add_predicate(eq_predicate);
    }
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
                        table_id: TableId(SUPPLIER_TABLE_ID), // supplier
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                        attr_idx: 3,
                    }),
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(ORDERS_TABLE_ID), // orders
                        attr_idx: 4,
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
                Some(semantic_correlation_68),
            ),
        },
    );

    let mut semantic_correlation_71 = SemanticCorrelation::new();
    let eq_predicates = vec![
        EqPredicate {
            left: BaseTableAttrRef {
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 2,
            },
            right: BaseTableAttrRef {
                table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
                attr_idx: 1,
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
                table_id: TableId(LINEITEM_TABLE_ID), // lineitem
                attr_idx: 1,
            },
            right: BaseTableAttrRef {
                table_id: TableId(PARTSUPP_TABLE_ID), // partsupp
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
        semantic_correlation_71.add_predicate(eq_predicate);
    }
    memo.insert(
        GroupId(71),
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
                    AttrRef::BaseTableAttrRef(BaseTableAttrRef {
                        table_id: TableId(NATION_TABLE_ID), // nation
                        attr_idx: 1,
                    }),
                    AttrRef::Derived,
                    AttrRef::Derived,
                ],
                Some(semantic_correlation_71),
            ),
        },
    );

    memo.insert(
        GroupId(75),
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
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
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
        GroupId(78),
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
                ],
            },
            attr_refs: GroupAttrRefs::new(
                vec![
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

init_tpch_query!(q9);

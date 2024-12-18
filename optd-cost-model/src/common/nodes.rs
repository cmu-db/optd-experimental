use core::fmt;
use std::{fmt::Display, sync::Arc};

use arrow_schema::DataType;

use super::{
    predicates::{
        bin_op_pred::BinOpType, constant_pred::ConstantType, func_pred::FuncType,
        log_op_pred::LogOpType, sort_order_pred::SortOrderType, un_op_pred::UnOpType,
    },
    values::Value,
};

/// TODO: documentation
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JoinType {
    Inner = 1,
    FullOuter,
    LeftOuter,
    RightOuter,
    Cross,
    LeftSemi,
    RightSemi,
    LeftAnti,
    RightAnti,
}

impl Display for JoinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// TODO: documentation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicalNodeType {
    PhysicalProjection,
    PhysicalFilter,
    PhysicalScan,
    PhysicalSort,
    PhysicalAgg,
    PhysicalHashJoin(JoinType),
    PhysicalNestedLoopJoin(JoinType),
    PhysicalEmptyRelation,
    PhysicalLimit,
}

impl std::fmt::Display for PhysicalNodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// TODO: documentation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PredicateType {
    List,
    Constant(ConstantType),
    AttrIndex,
    UnOp(UnOpType),
    BinOp(BinOpType),
    LogOp(LogOpType),
    Func(FuncType),
    SortOrder(SortOrderType),
    Between,
    Cast,
    Like,
    DataType(DataType),
    InList,
}

impl std::fmt::Display for PredicateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type ArcPredicateNode = Arc<PredicateNode>;

/// TODO: documentation
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PredicateNode {
    /// A generic predicate node type
    pub typ: PredicateType,
    /// Child predicate nodes, always materialized
    pub children: Vec<ArcPredicateNode>,
    /// Data associated with the predicate, if any
    pub data: Option<Value>,
}

impl std::fmt::Display for PredicateNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}", self.typ)?;
        for child in &self.children {
            write!(f, " {}", child)?;
        }
        if let Some(data) = &self.data {
            write!(f, " {}", data)?;
        }
        write!(f, ")")
    }
}

impl PredicateNode {
    pub fn child(&self, idx: usize) -> ArcPredicateNode {
        self.children[idx].clone()
    }

    pub fn unwrap_data(&self) -> Value {
        self.data.clone().unwrap()
    }
}
pub trait ReprPredicateNode: 'static + Clone {
    fn into_pred_node(self) -> ArcPredicateNode;

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self>;
}

impl ReprPredicateNode for ArcPredicateNode {
    fn into_pred_node(self) -> ArcPredicateNode {
        self
    }

    fn from_pred_node(pred_node: ArcPredicateNode) -> Option<Self> {
        Some(pred_node)
    }
}

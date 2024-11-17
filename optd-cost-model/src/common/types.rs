use std::fmt::Display;

/// TODO: Implement from and to methods for the following types to enable conversion
/// to and from their persistent counterparts.

/// TODO: documentation
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct GroupId(pub u64);

/// TODO: documentation
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct ExprId(pub u64);

/// TODO: documentation
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct TableId(pub u64);

/// TODO: documentation
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct AttrId(pub u64);

/// TODO: documentation
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct EpochId(pub u64);

impl Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "!{}", self.0)
    }
}

impl Display for ExprId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for TableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Table#{}", self.0)
    }
}

impl Display for AttrId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Attr#{}", self.0)
    }
}

impl Display for EpochId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Epoch#{}", self.0)
    }
}

impl From<GroupId> for i32 {
    fn from(id: GroupId) -> i32 {
        id.0 as i32
    }
}

impl From<ExprId> for i32 {
    fn from(id: ExprId) -> i32 {
        id.0 as i32
    }
}

impl From<TableId> for i32 {
    fn from(id: TableId) -> i32 {
        id.0 as i32
    }
}

impl From<AttrId> for i32 {
    fn from(id: AttrId) -> i32 {
        id.0 as i32
    }
}

impl From<EpochId> for i32 {
    fn from(id: EpochId) -> i32 {
        id.0 as i32
    }
}

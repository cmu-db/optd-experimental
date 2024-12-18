use crate::common::{
    properties::{
        attr_ref::{AttrRef, GroupAttrRefs},
        schema::Schema,
        Attribute,
    },
    types::GroupId,
};

/// [`MemoExt`] is a trait that provides methods to access the schema, column reference, and attribute
/// information of a group in the memo. The information are used by the cost model to compute the cost of
/// an expression.
///
/// [`MemoExt`] should be implemented by the optimizer core to provide the necessary information to the cost
/// model. All information required here is already present in the memo, so the optimizer core should be able
/// to implement this trait without additional work.
pub trait MemoExt: Send + Sync + 'static {
    /// Get the schema of a group in the memo.
    fn get_schema(&self, group_id: GroupId) -> Schema;
    /// Get the attribute info of a given attribute in a group in the memo.
    fn get_attribute_info(&self, group_id: GroupId, attr_ref_idx: u64) -> Attribute;
    /// Get the attribute reference of a group in the memo.
    fn get_attribute_refs(&self, group_id: GroupId) -> GroupAttrRefs;
    /// Get the attribute reference of a given attribute in a group in the memo.
    fn get_attribute_ref(&self, group_id: GroupId, attr_ref_idx: u64) -> AttrRef;

    // TODO: Figure out what other information is needed to compute the cost...
}

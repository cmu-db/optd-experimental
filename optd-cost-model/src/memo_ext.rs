use crate::common::{
    properties::{attr_ref::GroupAttrRefs, schema::Schema, Attribute},
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
    /// Get the attribute reference of a group in the memo.
    fn get_attribute_ref(&self, group_id: GroupId) -> GroupAttrRefs;
    /// Get the attribute information of a given attribute in a group in the memo.
    fn get_attribute_info(&self, group_id: GroupId, attr_ref_idx: u64) -> Attribute;

    // TODO: Figure out what other information is needed to compute the cost...
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::common::{
        properties::{attr_ref::GroupAttrRefs, schema::Schema, Attribute},
        types::GroupId,
    };

    pub struct MemoGroupInfo {
        pub schema: Schema,
        pub attr_ref: GroupAttrRefs,
    }

    #[derive(Default)]
    pub struct MockMemoExtImpl {
        memo: HashMap<GroupId, MemoGroupInfo>,
    }

    impl super::MemoExt for MockMemoExtImpl {
        fn get_schema(&self, group_id: GroupId) -> Schema {
            self.memo.get(&group_id).unwrap().schema.clone()
        }

        fn get_attribute_ref(&self, group_id: GroupId) -> GroupAttrRefs {
            self.memo.get(&group_id).unwrap().attr_ref.clone()
        }

        fn get_attribute_info(&self, group_id: GroupId, attr_ref_idx: u64) -> Attribute {
            self.memo.get(&group_id).unwrap().schema.attributes[attr_ref_idx as usize].clone()
        }
    }

    impl From<HashMap<GroupId, MemoGroupInfo>> for MockMemoExtImpl {
        fn from(memo: HashMap<GroupId, MemoGroupInfo>) -> Self {
            Self { memo }
        }
    }
}

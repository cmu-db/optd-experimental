use crate::common::{
    properties::{attr_ref::GroupAttrRefs, schema::Schema, Attribute},
    types::GroupId,
};

pub trait MemoExt {
    fn get_schema(&self, group_id: GroupId) -> Schema;
    fn get_column_ref(&self, group_id: GroupId) -> GroupAttrRefs;
    fn get_attribute_info(&self, group_id: GroupId, attr_ref_idx: u64) -> Attribute;
}

use crate::common::{
    properties::{attr_ref::GroupAttrRefs, schema::Schema},
    types::GroupId,
};

pub trait MemoExt {
    fn get_schema_of(&self, group_id: GroupId) -> Schema;
    fn get_column_ref_of(&self, group_id: GroupId) -> GroupAttrRefs;
}

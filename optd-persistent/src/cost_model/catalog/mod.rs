pub mod mock_catalog;

pub enum IndexType {
    BTree,
    Hash,
}

pub enum AttrType {
    Integer,
    Float,
    Varchar,
    Boolean,
}

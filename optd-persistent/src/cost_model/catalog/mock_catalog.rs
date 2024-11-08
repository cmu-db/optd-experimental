use crate::cost_model::interface::{AttrType, IndexType, StatType};

pub struct MockDatabaseMetadata {
    pub id: i32,
    pub name: String,
}

pub struct MockNamespaceMetadata {
    pub id: i32,
    pub name: String,
    pub database_id: i32,
}

pub struct MockTableMetadata {
    pub id: i32,
    pub name: String,
    pub namespace_id: i32,
}

pub struct MockAttribute {
    pub id: i32,
    pub name: String,
    pub attr_index: i32,
    pub table_id: i32,
    pub compression_method: char,
    pub attr_type: i32,
    pub is_not_null: bool,
}

pub struct MockStatistic {
    pub id: i32,
    pub stat_type: i32,
    // TODO(lanlou): what should I use for the value type?
    pub stat_value: String,
    pub attr_ids: Vec<i32>,
    pub table_id: Option<i32>,
    pub name: String,
}

pub struct MockIndex {
    pub id: i32,
    pub name: String,
    pub table_id: i32,
    pub number_of_attributes: i32,
    pub index_type: i32,
    pub is_unique: bool,
    pub nulls_not_distinct: bool,
    pub is_primary: bool,
    pub is_clustered: bool,
    pub is_exclusion: bool,
    pub attr_ids: Vec<i32>,
}

pub struct MockTrigger {
    pub id: i32,
    pub name: String,
    pub table_id: i32,
    pub parent_trigger_id: i32,
    pub function: String,
}

#[derive(Default)]
pub struct MockCatalog {
    pub databases: Vec<MockDatabaseMetadata>,
    pub namespaces: Vec<MockNamespaceMetadata>,
    pub tables: Vec<MockTableMetadata>,
    pub attributes: Vec<MockAttribute>,
    pub statistics: Vec<MockStatistic>,
    pub indexes: Vec<MockIndex>,
    pub triggers: Vec<MockTrigger>,
    // TODO: constraints
}
impl MockCatalog {
    pub fn new() -> Self {
        let databases: Vec<MockDatabaseMetadata> = vec![MockDatabaseMetadata {
            id: 1,
            name: "db1".to_string(),
        }];
        let namespaces: Vec<MockNamespaceMetadata> = vec![MockNamespaceMetadata {
            id: 1,
            name: "ns1".to_string(),
            database_id: 1,
        }];
        let tables: Vec<MockTableMetadata> = vec![MockTableMetadata {
            id: 1,
            name: "table1".to_string(),
            namespace_id: 1,
        }];
        let attributes: Vec<MockAttribute> = vec![
            MockAttribute {
                id: 1,
                name: "attr1".to_string(),
                attr_index: 1,
                table_id: 1,
                compression_method: 'n',
                attr_type: AttrType::Integer as i32,
                is_not_null: true,
            },
            MockAttribute {
                id: 2,
                name: "attr2".to_string(),
                attr_index: 2,
                table_id: 1,
                compression_method: 'n',
                attr_type: AttrType::Integer as i32,
                is_not_null: false,
            },
        ];
        let statistics: Vec<MockStatistic> = vec![
            MockStatistic {
                id: 1,
                stat_type: StatType::Count as i32,
                stat_value: "100".to_string(),
                attr_ids: vec![1],
                table_id: None,
                name: "CountAttr1".to_string(),
            },
            MockStatistic {
                id: 2,
                stat_type: StatType::Count as i32,
                stat_value: "200".to_string(),
                attr_ids: vec![2],
                table_id: None,
                name: "CountAttr2".to_string(),
            },
            MockStatistic {
                id: 3,
                stat_type: StatType::Count as i32,
                stat_value: "300".to_string(),
                attr_ids: vec![],
                table_id: Some(1),
                name: "Table1Count".to_string(),
            },
        ];
        let indexes: Vec<MockIndex> = vec![MockIndex {
            id: 1,
            name: "index1".to_string(),
            table_id: 1,
            number_of_attributes: 1,
            index_type: IndexType::Hash as i32,
            is_unique: false,
            nulls_not_distinct: false,
            is_primary: true,
            is_clustered: false,
            is_exclusion: false,
            attr_ids: vec![1],
        }];

        MockCatalog {
            databases,
            namespaces,
            tables,
            attributes,
            statistics,
            indexes,
            triggers: vec![],
        }
    }
}

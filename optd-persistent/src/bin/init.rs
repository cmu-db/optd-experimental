use optd_persistent::cost_model::interface::AttrType;
use optd_persistent::cost_model::interface::ConstraintType;
use optd_persistent::cost_model::interface::IndexType;
use optd_persistent::cost_model::interface::StatType;
use optd_persistent::entities::*;
use optd_persistent::migrate;
use optd_persistent::TEST_DATABASE_FILENAME;
use optd_persistent::TEST_DATABASE_URL;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::*;
use serde_json::json;

async fn init_all_tables() -> Result<(), sea_orm::error::DbErr> {
    let _ = std::fs::remove_file(TEST_DATABASE_FILENAME);

    let db = Database::connect(TEST_DATABASE_URL.clone())
        .await
        .expect("Unable to connect to the database");

    migrate(&db)
        .await
        .expect("Something went wrong during migration");

    // Inserting into database_metadata
    let database_metadata = database_metadata::ActiveModel {
        id: Set(1),
        name: Set("database1".to_owned()),
        creation_time: Set(Utc::now()),
    };
    database_metadata::Entity::insert(database_metadata)
        .exec(&db)
        .await
        .expect("Unable to insert database metadata");

    // Inserting into namespace_metadata
    let namespace_metadata = namespace_metadata::ActiveModel {
        id: Set(1),
        database_id: Set(1),
        name: Set("default".to_owned()),
        creation_time: Set(Utc::now()),
    };
    namespace_metadata::Entity::insert(namespace_metadata)
        .exec(&db)
        .await
        .expect("Unable to insert namespace metadata");

    // Inserting into table_metadata
    let table_metadata = table_metadata::ActiveModel {
        id: Set(1),
        namespace_id: Set(1),
        name: Set("users".to_owned()),
        creation_time: Set(Utc::now()),
    };
    table_metadata::Entity::insert(table_metadata)
        .exec(&db)
        .await
        .expect("Unable to insert table metadata");

    // Inserting into attribute
    let attribute1 = attribute::ActiveModel {
        id: Set(1),
        table_id: Set(1),
        name: Set("user_id".to_owned()),
        compression_method: Set("N".to_owned()),
        variant_tag: Set(AttrType::Integer as i32),
        base_attribute_number: Set(1),
        is_not_null: Set(true),
    };
    let attribute2 = attribute::ActiveModel {
        id: Set(2),
        table_id: Set(1),
        name: Set("username".to_owned()),
        compression_method: Set("N".to_owned()),
        variant_tag: Set(AttrType::Varchar as i32),
        base_attribute_number: Set(2),
        is_not_null: Set(true),
    };
    attribute::Entity::insert(attribute1)
        .exec(&db)
        .await
        .expect("Unable to insert attribute");
    attribute::Entity::insert(attribute2)
        .exec(&db)
        .await
        .expect("Unable to insert attribute");

    // Inserting into event
    let event = event::ActiveModel {
        epoch_id: Set(1),
        source_variant: Set("execution_engine".to_owned()),
        timestamp: Set(Utc::now()),
        data: Set(json!({"dba": "parpulse"})),
    };
    event::Entity::insert(event)
        .exec(&db)
        .await
        .expect("Unable to insert event");

    // Inserting into statistic

    // Table statistic
    let table_statistic = statistic::ActiveModel {
        id: Set(1),
        name: Set("row_count".to_owned()),
        table_id: Set(Some(1)),
        creation_time: Set(Utc::now()),
        number_of_attributes: Set(0),
        variant_tag: Set(StatType::Count as i32),
        description: Set("".to_owned()),
    };
    let table_versioned_statistic = versioned_statistic::ActiveModel {
        id: Set(1),
        epoch_id: Set(1),
        statistic_id: Set(1),
        statistic_value: Set(json!(0)),
    };
    statistic::Entity::insert(table_statistic)
        .exec(&db)
        .await
        .expect("Unable to insert statistic");
    versioned_statistic::Entity::insert(table_versioned_statistic)
        .exec(&db)
        .await
        .expect("Unable to insert versioned statistic");

    // Single-column attribute statistic
    let single_column_attribute_statistic = statistic::ActiveModel {
        id: Set(2),
        name: Set("cardinality".to_owned()),
        table_id: Set(Some(1)),
        creation_time: Set(Utc::now()),
        number_of_attributes: Set(1),
        variant_tag: Set(StatType::Cardinality as i32),
        description: Set("1".to_owned()),
    };
    let single_column_attribute_versioned_statistic = versioned_statistic::ActiveModel {
        id: Set(2),
        epoch_id: Set(1),
        statistic_id: Set(2),
        statistic_value: Set(json!(0)),
    };
    statistic::Entity::insert(single_column_attribute_statistic)
        .exec(&db)
        .await
        .expect("Unable to insert statistic");
    versioned_statistic::Entity::insert(single_column_attribute_versioned_statistic)
        .exec(&db)
        .await
        .expect("Unable to insert versioned statistic");

    let single_column_statistic_to_attribute_junction =
        statistic_to_attribute_junction::ActiveModel {
            statistic_id: Set(2), // cardinality
            attribute_id: Set(1), // user_id
        };
    statistic_to_attribute_junction::Entity::insert(single_column_statistic_to_attribute_junction)
        .exec(&db)
        .await
        .expect("Unable to insert statistic_to_attribute_junction");

    // Multi-column attribute statistic
    let multi_column_attribute_statistic = statistic::ActiveModel {
        id: Set(3),
        name: Set("joint_cardinality".to_owned()),
        table_id: Set(Some(1)),
        creation_time: Set(Utc::now()),
        number_of_attributes: Set(2),
        variant_tag: Set(StatType::Cardinality as i32),
        description: Set("1,2".to_owned()),
    };
    let multi_column_attribute_versioned_statistic = versioned_statistic::ActiveModel {
        id: Set(3),
        epoch_id: Set(1),
        statistic_id: Set(3),
        statistic_value: Set(json!(0)),
    };
    statistic::Entity::insert(multi_column_attribute_statistic)
        .exec(&db)
        .await
        .expect("Unable to insert statistic");
    versioned_statistic::Entity::insert(multi_column_attribute_versioned_statistic)
        .exec(&db)
        .await
        .expect("Unable to insert versioned statistic");

    let multi_column_statistic_to_attribute_junction1 =
        statistic_to_attribute_junction::ActiveModel {
            statistic_id: Set(3), // joint cardinality
            attribute_id: Set(1), // user_id
        };
    let multi_column_statistic_to_attribute_junction2 =
        statistic_to_attribute_junction::ActiveModel {
            statistic_id: Set(3), // joint cardinality
            attribute_id: Set(2), // username
        };
    statistic_to_attribute_junction::Entity::insert(multi_column_statistic_to_attribute_junction1)
        .exec(&db)
        .await
        .expect("Unable to insert statistic_to_attribute_junction");
    statistic_to_attribute_junction::Entity::insert(multi_column_statistic_to_attribute_junction2)
        .exec(&db)
        .await
        .expect("Unable to insert statistic_to_attribute_junction");

    // Inserting into index_metadata
    let index_metadata = index_metadata::ActiveModel {
        id: Set(1),
        name: Set("user_id_index".to_owned()),
        table_id: Set(1),
        is_unique: Set(true),
        nulls_not_distinct: Set(false),
        is_primary: Set(true),
        is_clustered: Set(false),
        is_exclusion: Set(false),
        variant_tag: Set(IndexType::Hash as i32),
        number_of_attributes: Set(1),
        description: Set("1".to_owned()),
    };
    index_metadata::Entity::insert(index_metadata)
        .exec(&db)
        .await
        .expect("Unable to insert index metadata");

    // Inserting into trigger
    let trigger = trigger::ActiveModel {
        id: Set(1),
        name: Set("after_insert_user".to_owned()),
        table_id: Set(1),
        parent_trigger_id: Set(1),
        function: Set(json!(r#"{"function": "insert"}"#)),
    };
    trigger::Entity::insert(trigger)
        .exec(&db)
        .await
        .expect("Unable to insert trigger");

    // Inserting into constraint_metadata
    let constraint_metadata = constraint_metadata::ActiveModel {
        id: Set(1),
        name: Set("pk_user_id".to_owned()),
        variant_tag: Set(ConstraintType::PrimaryKey as i32),
        table_id: Set(Some(1)),
        index_id: Set(Some(1)),
        foreign_ref_id: Set(None),
        check_src: Set("hello".to_owned()),
    };
    constraint_metadata::Entity::insert(constraint_metadata)
        .exec(&db)
        .await
        .expect("Unable to insert constraint metadata");

    // Inserting into attribute_constraint_junction
    let attribute_constraint_junction = attribute_constraint_junction::ActiveModel {
        attribute_id: Set(1),
        constraint_id: Set(1),
    };
    attribute_constraint_junction::Entity::insert(attribute_constraint_junction)
        .exec(&db)
        .await
        .expect("Unable to insert attribute_constraint_junction");

    // Inserting into attribute_foreign_constraint_junction
    let attribute_foreign_constraint_junction =
        attribute_foreign_constraint_junction::ActiveModel {
            attribute_id: Set(1),
            constraint_id: Set(1),
        };
    attribute_foreign_constraint_junction::Entity::insert(attribute_foreign_constraint_junction)
        .exec(&db)
        .await
        .expect("Unable to insert attribute_foreign_constraint_junction");

    // Inserting into cascades_group
    let cascades_group = cascades_group::ActiveModel {
        id: Set(1),
        latest_winner: Set(None),
        in_progress: Set(true),
        is_optimized: Set(false),
    };
    cascades_group::Entity::insert(cascades_group)
        .exec(&db)
        .await
        .expect("Unable to insert cascades group");

    // Inserting into logical_expression
    let logical_expression = logical_expression::ActiveModel {
        id: Set(1),
        group_id: Set(1),
        fingerprint: Set(12345),
        variant_tag: Set(0),
        data: Set(json!(r#"{"expr": "index_scan"}"#)),
    };
    logical_expression::Entity::insert(logical_expression)
        .exec(&db)
        .await
        .expect("Unable to insert logical expression");

    // Inserting into physical_expression
    let physical_expression = physical_expression::ActiveModel {
        id: Set(1),
        group_id: Set(1),
        fingerprint: Set(12345),
        variant_tag: Set(0),
        data: Set(json!(r#"{"expr": "index_scan"}"#)),
    };
    physical_expression::Entity::insert(physical_expression)
        .exec(&db)
        .await
        .expect("Unable to insert physical expression");

    // Inserting into physical_property
    let physical_property = physical_property::ActiveModel {
        id: Set(1),
        physical_expression_id: Set(1),
        variant_tag: Set(0),
        data: Set(json!(r#"{"property": "indexed"}"#)),
    };
    physical_property::Entity::insert(physical_property)
        .exec(&db)
        .await
        .expect("Unable to insert physical property");

    // Inserting into logical_property
    let logical_property = logical_property::ActiveModel {
        id: Set(1),
        group_id: Set(1),
        variant_tag: Set(0),
        data: Set(json!(r#"{"property": "indexed"}"#)),
    };
    logical_property::Entity::insert(logical_property)
        .exec(&db)
        .await
        .expect("Unable to insert logical property");

    let logical_children = logical_children::ActiveModel {
        logical_expression_id: Set(1),
        group_id: Set(1),
    };
    logical_children::Entity::insert(logical_children)
        .exec(&db)
        .await
        .expect("Unable to insert logical children");

    let physical_children = physical_children::ActiveModel {
        physical_expression_id: Set(1),
        group_id: Set(1),
    };
    physical_children::Entity::insert(physical_children)
        .exec(&db)
        .await
        .expect("Unable to insert physical children");

    // Inserting into plan_cost
    let plan_cost = plan_cost::ActiveModel {
        id: Set(1),
        physical_expression_id: Set(1),
        epoch_id: Set(1),
        cost: Set(10),
        is_valid: Set(true),
    };
    plan_cost::Entity::insert(plan_cost)
        .exec(&db)
        .await
        .expect("Unable to insert plan cost");

    // Inserting into physical_expression_to_statistic_junction
    let physical_expression_to_statistic_junction =
        physical_expression_to_statistic_junction::ActiveModel {
            physical_expression_id: Set(1),
            statistic_id: Set(1),
        };
    physical_expression_to_statistic_junction::Entity::insert(
        physical_expression_to_statistic_junction,
    )
    .exec(&db)
    .await
    .expect("Unable to insert physical_expression_to_statistic_junction");

    // Inserting into group_winner
    let group_winner = group_winner::ActiveModel {
        id: Set(1),
        group_id: Set(1),
        physical_expression_id: Set(1),
        cost_id: Set(1),
        epoch_id: Set(1),
    };
    group_winner::Entity::insert(group_winner)
        .exec(&db)
        .await
        .expect("Unable to insert group winner");

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = init_all_tables().await {
        eprintln!("Error initializing database: {}", e);
        std::process::exit(1);
    }

    println!("Database initialized successfully");
}

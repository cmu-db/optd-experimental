use optd_persistent::entities::*;
use optd_persistent::migrate;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::*;
use serde_json::json;

async fn init_all_tables(db_file: &str) -> Result<(), sea_orm::error::DbErr> {
    let database_url = format!("sqlite:./{}?mode=rwc", db_file);
    let database_file = format!("./{}", db_file);
    let _ = std::fs::remove_file(database_file);

    let db = Database::connect(database_url.clone())
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
        ..Default::default()
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
        ..Default::default()
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
        ..Default::default()
    };
    table_metadata::Entity::insert(table_metadata)
        .exec(&db)
        .await
        .expect("Unable to insert table metadata");

    // Inserting into attribute
    let attribute = attribute::ActiveModel {
        id: Set(1),
        table_id: Set(1),
        name: Set("user_id".to_owned()),
        compression_method: Set("N".to_owned()),
        variant_tag: Set(1),
        base_attribute_number: Set(1),
        is_not_null: Set(true),
        ..Default::default()
    };
    attribute::Entity::insert(attribute)
        .exec(&db)
        .await
        .expect("Unable to insert attribute");

    // Inserting into statistic
    let statistic = statistic::ActiveModel {
        id: Set(1),
        name: Set("row_count".to_owned()),
        table_id: Set(Some(1)),
        creation_time: Set(Utc::now()),
        number_of_attributes: Set(0),
        variant_tag: Set(1),
        description: Set("Total rows".to_owned()),
        ..Default::default()
    };
    statistic::Entity::insert(statistic)
        .exec(&db)
        .await
        .expect("Unable to insert statistic");
    // Inserting into event
    let event = event::ActiveModel {
        epoch_id: Set(1),
        source_variant: Set("insert".to_owned()),
        timestamp: Set(Utc::now()),
        data: Set(json!(r#"{"user_id": 1}"#)),
        ..Default::default()
    };

    event::Entity::insert(event)
        .exec(&db)
        .await
        .expect("Unable to insert event");

    // Inserting into versioned_statistic
    let versioned_statistic = versioned_statistic::ActiveModel {
        id: Set(1),
        epoch_id: Set(1),
        statistic_id: Set(1),
        statistic_value: Set(json!(r#"{"row_count": 0}"#)),
        ..Default::default()
    };
    versioned_statistic::Entity::insert(versioned_statistic)
        .exec(&db)
        .await
        .expect("Unable to insert versioned statistic");

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
        variant_tag: Set(1),
        number_of_attributes: Set(1),
        description: Set("random".to_owned()),
        ..Default::default()
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
        ..Default::default()
    };
    trigger::Entity::insert(trigger)
        .exec(&db)
        .await
        .expect("Unable to insert trigger");

    // Inserting into constraint_metadata
    let constraint_metadata = constraint_metadata::ActiveModel {
        id: Set(1),
        name: Set("pk_user_id".to_owned()),
        variant_tag: Set(1),
        table_id: Set(Some(1)),
        index_id: Set(Some(1)),
        foreign_ref_id: Set(None),
        check_src: Set("hello".to_owned()),
        ..Default::default()
    };
    constraint_metadata::Entity::insert(constraint_metadata)
        .exec(&db)
        .await
        .expect("Unable to insert constraint metadata");

    // Inserting into attribute_constraint_junction
    let attribute_constraint_junction = attribute_constraint_junction::ActiveModel {
        attribute_id: Set(1),
        constraint_id: Set(1),
        ..Default::default()
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
            ..Default::default()
        };
    attribute_foreign_constraint_junction::Entity::insert(attribute_foreign_constraint_junction)
        .exec(&db)
        .await
        .expect("Unable to insert attribute_foreign_constraint_junction");

    // Inserting into statistic_to_attribute_junction
    let statistic_to_attribute_junction = statistic_to_attribute_junction::ActiveModel {
        statistic_id: Set(1),
        attribute_id: Set(1),
        ..Default::default()
    };
    statistic_to_attribute_junction::Entity::insert(statistic_to_attribute_junction)
        .exec(&db)
        .await
        .expect("Unable to insert statistic_to_attribute_junction");

    // Inserting into cascades_group
    let cascades_group = cascades_group::ActiveModel {
        id: Set(1),
        latest_winner: Set(None),
        in_progress: Set(true),
        is_optimized: Set(false),
        ..Default::default()
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
        variant_tag: Set(1),
        data: Set(json!(r#"{"expr": "index_scan"}"#)),
        ..Default::default()
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
        variant_tag: Set(1),
        data: Set(json!(r#"{"expr": "index_scan"}"#)),
        ..Default::default()
    };
    physical_expression::Entity::insert(physical_expression)
        .exec(&db)
        .await
        .expect("Unable to insert physical expression");

    // Inserting into physical_property
    let physical_property = physical_property::ActiveModel {
        id: Set(1),
        physical_expression_id: Set(1),
        variant_tag: Set(1),
        data: Set(json!(r#"{"property": "indexed"}"#)),
        ..Default::default()
    };
    physical_property::Entity::insert(physical_property)
        .exec(&db)
        .await
        .expect("Unable to insert physical property");

    // Inserting into logical_property
    let logical_property = logical_property::ActiveModel {
        id: Set(1),
        group_id: Set(1),
        variant_tag: Set(1),
        data: Set(json!(r#"{"property": "indexed"}"#)),
        ..Default::default()
    };
    logical_property::Entity::insert(logical_property)
        .exec(&db)
        .await
        .expect("Unable to insert logical property");

    let logical_children = logical_children::ActiveModel {
        logical_expression_id: Set(1),
        group_id: Set(1),
        ..Default::default()
    };
    logical_children::Entity::insert(logical_children)
        .exec(&db)
        .await
        .expect("Unable to insert logical children");

    let physical_children = physical_children::ActiveModel {
        physical_expression_id: Set(1),
        group_id: Set(1),
        ..Default::default()
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
        ..Default::default()
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
            ..Default::default()
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
        ..Default::default()
    };
    group_winner::Entity::insert(group_winner)
        .exec(&db)
        .await
        .expect("Unable to insert group winner");

    Ok(())
}

#[tokio::main]
async fn main() {
    let db_file = "init.db";
    if let Err(e) = init_all_tables(db_file).await {
        eprintln!("Error initializing database: {}", e);
        std::process::exit(1);
    }

    println!("Database initialized successfully");
}

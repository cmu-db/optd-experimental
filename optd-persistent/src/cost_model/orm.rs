#![allow(dead_code, unused_imports, unused_variables)]

use crate::cost_model::interface::Cost;
use crate::entities::{prelude::*, *};
use crate::{BackendError, BackendManager, CostModelStorageLayer, StorageResult};
use sea_orm::prelude::{Expr, Json};
use sea_orm::sea_query::Query;
use sea_orm::{sqlx::types::chrono::Utc, EntityTrait};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbBackend, DbErr, DeleteResult, EntityOrSelect,
    ModelTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait, RuntimeErr, TransactionTrait,
};
use serde_json::json;

use super::catalog::mock_catalog::{self, MockCatalog};
use super::interface::{
    Attr, AttrId, AttrIndex, AttrType, CatalogSource, EpochId, EpochOption, ExprId, Stat, StatId,
    StatType, TableId,
};

impl BackendManager {
    /// The description is to concat `attr_ids` using commas
    /// Note that `attr_ids` should be sorted before concatenation
    /// e.g. [1, 2, 3] -> "1,2,3"
    fn get_description_from_attr_ids(&self, mut attr_ids: Vec<AttrId>) -> String {
        attr_ids.sort();
        attr_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl CostModelStorageLayer for BackendManager {
    /// TODO: documentation
    async fn create_new_epoch(&self, source: String, data: String) -> StorageResult<EpochId> {
        let new_event = event::ActiveModel {
            source_variant: sea_orm::ActiveValue::Set(source),
            timestamp: sea_orm::ActiveValue::Set(Utc::now()),
            data: sea_orm::ActiveValue::Set(sea_orm::JsonValue::String(data)),
            ..Default::default()
        };
        let insert_res = Event::insert(new_event).exec(&self.db).await?;
        Ok(insert_res.last_insert_id)
    }

    /// TODO: documentation
    async fn update_stats_from_catalog(&self, c: CatalogSource) -> StorageResult<EpochId> {
        let transaction = self.db.begin().await?;
        let source = match c {
            CatalogSource::Mock => "Mock",
            CatalogSource::Iceberg() => "Iceberg",
        };
        let new_event = event::ActiveModel {
            source_variant: sea_orm::ActiveValue::Set(source.to_string()),
            timestamp: sea_orm::ActiveValue::Set(Utc::now()),
            data: sea_orm::ActiveValue::Set(sea_orm::JsonValue::String(
                "Update stats from catalog".to_string(),
            )),
            ..Default::default()
        };
        let epoch_id = Event::insert(new_event)
            .exec(&transaction)
            .await?
            .last_insert_id;
        match c {
            CatalogSource::Mock => {
                let mock_catalog = MockCatalog::new();
                DatabaseMetadata::insert_many(mock_catalog.databases.iter().map(|database| {
                    database_metadata::ActiveModel {
                        name: sea_orm::ActiveValue::Set(database.name.clone()),
                        creation_time: sea_orm::ActiveValue::Set(Utc::now()),
                        ..Default::default()
                    }
                }))
                .exec(&transaction)
                .await?;
                NamespaceMetadata::insert_many(mock_catalog.namespaces.iter().map(|namespace| {
                    namespace_metadata::ActiveModel {
                        name: sea_orm::ActiveValue::Set(namespace.name.clone()),
                        database_id: sea_orm::ActiveValue::Set(namespace.database_id),
                        creation_time: sea_orm::ActiveValue::Set(Utc::now()),
                        ..Default::default()
                    }
                }))
                .exec(&transaction)
                .await?;
                TableMetadata::insert_many(mock_catalog.tables.iter().map(|table| {
                    table_metadata::ActiveModel {
                        name: sea_orm::ActiveValue::Set(table.name.clone()),
                        namespace_id: sea_orm::ActiveValue::Set(table.namespace_id),
                        creation_time: sea_orm::ActiveValue::Set(Utc::now()),
                        ..Default::default()
                    }
                }))
                .exec(&transaction)
                .await?;
                Attribute::insert_many(mock_catalog.attributes.iter().map(|attr| {
                    attribute::ActiveModel {
                        table_id: sea_orm::ActiveValue::Set(attr.table_id),
                        name: sea_orm::ActiveValue::Set(attr.name.clone()),
                        compression_method: sea_orm::ActiveValue::Set(
                            attr.compression_method.to_string(),
                        ),
                        variant_tag: sea_orm::ActiveValue::Set(attr.attr_type),
                        base_attribute_number: sea_orm::ActiveValue::Set(attr.attr_index),
                        is_not_null: sea_orm::ActiveValue::Set(attr.is_not_null),
                        ..Default::default()
                    }
                }))
                .exec(&transaction)
                .await?;
                Statistic::insert_many(mock_catalog.statistics.iter().map(|stat| {
                    statistic::ActiveModel {
                        name: sea_orm::ActiveValue::Set(stat.name.clone()),
                        table_id: sea_orm::ActiveValue::Set(stat.table_id),
                        number_of_attributes: sea_orm::ActiveValue::Set(stat.attr_ids.len() as i32),
                        creation_time: sea_orm::ActiveValue::Set(Utc::now()),
                        variant_tag: sea_orm::ActiveValue::Set(stat.stat_type),
                        description: sea_orm::ActiveValue::Set(
                            self.get_description_from_attr_ids(stat.attr_ids.clone()),
                        ),
                        ..Default::default()
                    }
                }))
                .exec(&transaction)
                .await?;
                VersionedStatistic::insert_many(mock_catalog.statistics.iter().map(|stat| {
                    versioned_statistic::ActiveModel {
                        epoch_id: sea_orm::ActiveValue::Set(epoch_id),
                        statistic_id: sea_orm::ActiveValue::Set(stat.id),
                        statistic_value: sea_orm::ActiveValue::Set(stat.stat_value.clone()),
                        ..Default::default()
                    }
                }))
                .exec(&transaction)
                .await?;
                StatisticToAttributeJunction::insert_many(mock_catalog.statistics.iter().flat_map(
                    |stat| {
                        stat.attr_ids.iter().map(move |attr_id| {
                            statistic_to_attribute_junction::ActiveModel {
                                statistic_id: sea_orm::ActiveValue::Set(stat.id),
                                attribute_id: sea_orm::ActiveValue::Set(*attr_id),
                            }
                        })
                    },
                ))
                .exec(&transaction)
                .await?;
                IndexMetadata::insert_many(
                    mock_catalog
                        .indexes
                        .iter()
                        .map(|index| index_metadata::ActiveModel {
                            name: sea_orm::ActiveValue::Set(index.name.clone()),
                            table_id: sea_orm::ActiveValue::Set(index.table_id),
                            number_of_attributes: sea_orm::ActiveValue::Set(
                                index.attr_ids.len() as i32
                            ),
                            variant_tag: sea_orm::ActiveValue::Set(index.index_type),
                            is_unique: sea_orm::ActiveValue::Set(index.is_unique),
                            nulls_not_distinct: sea_orm::ActiveValue::Set(index.nulls_not_distinct),
                            is_primary: sea_orm::ActiveValue::Set(index.is_primary),
                            is_clustered: sea_orm::ActiveValue::Set(index.is_clustered),
                            is_exclusion: sea_orm::ActiveValue::Set(index.is_exclusion),
                            description: sea_orm::ActiveValue::Set(
                                self.get_description_from_attr_ids(index.attr_ids.clone()),
                            ),
                            ..Default::default()
                        }),
                )
                .exec(&transaction)
                .await?;
                // TODO: initialize constraints
            }
            CatalogSource::Iceberg() => todo!(),
        }
        transaction.commit().await?;
        Ok(epoch_id)
    }

    /// TODO: improve the documentation
    /* Update the statistics in the database.
     * The statistic can be newly inserted or updated. If the statistic value
     * is the same as the latest existing one, the update will be ignored, and
     * the return value will be None.
     * If `epoch_option` is `EpochOption::Existed(epoch_id)`, the new statistic
     * will be associated with the given epoch_id. If `epoch_option` is
     * `EpochOption::New(source, data)`, a new epoch will be created with the
     * given source and data, and the new statistic will be associated with the
     * new epoch. And return the new epoch_id.
     * If the statistic value is the same as the latest existing one, this function
     * won't create a new epoch.
     *
     * For batch updates, the caller can directly call this function with
     * New epoch option at the first time, and if the epoch_id is returned, the
     * caller can use the returned epoch_id for the rest of the updates.
     * But if the epoch_id is not returned, the caller should continue using
     * the New epoch option for the next statistic update.
     */
    async fn update_stats(
        &self,
        stat: Stat,
        epoch_option: EpochOption,
    ) -> StorageResult<Option<EpochId>> {
        let transaction = self.db.begin().await?;
        // 0. Check if the stat already exists. If exists, get stat_id, else insert into statistic table.
        let stat_id = match stat.table_id {
            Some(table_id) => {
                // TODO(lanlou): only select needed fields
                let res = Statistic::find()
                    .filter(statistic::Column::TableId.eq(table_id))
                    .inner_join(versioned_statistic::Entity)
                    .select_also(versioned_statistic::Entity)
                    .order_by_desc(versioned_statistic::Column::EpochId)
                    .one(&transaction)
                    .await?;
                match res {
                    Some(stat_data) => {
                        if stat_data.1.unwrap().statistic_value == stat.stat_value {
                            return Ok(None);
                        }
                        stat_data.0.id
                    }
                    None => {
                        let new_stat = statistic::ActiveModel {
                            name: sea_orm::ActiveValue::Set(stat.name.clone()),
                            table_id: sea_orm::ActiveValue::Set(Some(table_id)),
                            number_of_attributes: sea_orm::ActiveValue::Set(
                                stat.attr_ids.len() as i32
                            ),
                            creation_time: sea_orm::ActiveValue::Set(Utc::now()),
                            variant_tag: sea_orm::ActiveValue::Set(stat.stat_type as i32),
                            description: sea_orm::ActiveValue::Set("".to_string()),
                            ..Default::default()
                        };
                        let res = Statistic::insert(new_stat).exec(&transaction).await;
                        match res {
                            Ok(insert_res) => insert_res.last_insert_id,
                            Err(_) => {
                                return Err(BackendError::BackendError(format!(
                                    "failed to insert statistic {:?} into statistic table",
                                    stat
                                )))
                            }
                        }
                    }
                }
            }
            None => {
                let description = self.get_description_from_attr_ids(stat.attr_ids.clone());
                let res = Statistic::find()
                    .filter(statistic::Column::NumberOfAttributes.eq(stat.attr_ids.len() as i32))
                    .filter(statistic::Column::Description.eq(description.clone()))
                    .filter(statistic::Column::VariantTag.eq(stat.stat_type as i32))
                    .inner_join(versioned_statistic::Entity)
                    .select_also(versioned_statistic::Entity)
                    .order_by_desc(versioned_statistic::Column::EpochId)
                    .one(&transaction)
                    .await?;
                match res {
                    Some(stat_data) => {
                        if stat_data.1.unwrap().statistic_value == stat.stat_value {
                            return Ok(None);
                        }
                        stat_data.0.id
                    }
                    None => {
                        let new_stat = statistic::ActiveModel {
                            name: sea_orm::ActiveValue::Set(stat.name.clone()),
                            number_of_attributes: sea_orm::ActiveValue::Set(
                                stat.attr_ids.len() as i32
                            ),
                            creation_time: sea_orm::ActiveValue::Set(Utc::now()),
                            variant_tag: sea_orm::ActiveValue::Set(stat.stat_type as i32),
                            description: sea_orm::ActiveValue::Set(description),
                            ..Default::default()
                        };
                        // TODO(lanlou): we should not clone here maybe...
                        let insert_res = Statistic::insert(new_stat.clone())
                            .exec(&transaction)
                            .await?;
                        for attr_id in stat.attr_ids {
                            let new_junction = statistic_to_attribute_junction::ActiveModel {
                                statistic_id: sea_orm::ActiveValue::Set(insert_res.last_insert_id),
                                attribute_id: sea_orm::ActiveValue::Set(attr_id),
                            };
                            let res = StatisticToAttributeJunction::insert(new_junction)
                                .exec(&transaction)
                                .await?;
                        }
                        insert_res.last_insert_id
                    }
                }
            }
        };
        // 1. Insert into attr_stats and related junction tables.
        let epoch_id = match epoch_option {
            EpochOption::Existed(e) => e,
            EpochOption::New(source, data) => {
                let new_event = event::ActiveModel {
                    source_variant: sea_orm::ActiveValue::Set(source),
                    timestamp: sea_orm::ActiveValue::Set(Utc::now()),
                    data: sea_orm::ActiveValue::Set(sea_orm::JsonValue::String(data)),
                    ..Default::default()
                };
                let insert_res = Event::insert(new_event).exec(&transaction).await?;
                insert_res.last_insert_id
            }
        };
        let new_stats = versioned_statistic::ActiveModel {
            epoch_id: sea_orm::ActiveValue::Set(epoch_id),
            statistic_id: sea_orm::ActiveValue::Set(stat_id),
            statistic_value: sea_orm::ActiveValue::Set(stat.stat_value),
            ..Default::default()
        };
        let _ = VersionedStatistic::insert(new_stats)
            .exec(&transaction)
            .await?;

        // 2. Invalidate all the related cost.
        let _ = plan_cost::Entity::update_many()
            .col_expr(plan_cost::Column::IsValid, Expr::value(false))
            .filter(plan_cost::Column::IsValid.eq(true))
            .filter(plan_cost::Column::EpochId.lt(epoch_id))
            .filter(
                plan_cost::Column::PhysicalExpressionId.in_subquery(
                    Query::select()
                        .column(
                            physical_expression_to_statistic_junction::Column::PhysicalExpressionId,
                        )
                        .from(physical_expression_to_statistic_junction::Entity)
                        .cond_where(
                            physical_expression_to_statistic_junction::Column::StatisticId
                                .eq(stat_id),
                        )
                        .to_owned(),
                ),
            )
            .exec(&transaction)
            .await?;

        transaction.commit().await?;
        Ok(Some(epoch_id))
    }

    /// TODO: documentation
    async fn store_expr_stats_mappings(
        &self,
        expr_id: ExprId,
        stat_ids: Vec<StatId>,
    ) -> StorageResult<()> {
        let to_insert_mappings = stat_ids
            .iter()
            .map(
                |stat_id| physical_expression_to_statistic_junction::ActiveModel {
                    physical_expression_id: sea_orm::ActiveValue::Set(expr_id),
                    statistic_id: sea_orm::ActiveValue::Set(*stat_id),
                },
            )
            .collect::<Vec<_>>();
        let _ = PhysicalExpressionToStatisticJunction::insert_many(to_insert_mappings)
            .exec(&self.db)
            .await?;
        Ok(())
    }

    /// TODO: documentation
    async fn get_stats_for_table(
        &self,
        table_id: TableId,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> StorageResult<Option<Json>> {
        match epoch_id {
            Some(epoch_id) => Ok(VersionedStatistic::find()
                .filter(versioned_statistic::Column::EpochId.eq(epoch_id))
                .inner_join(statistic::Entity)
                .filter(statistic::Column::TableId.eq(table_id))
                .filter(statistic::Column::VariantTag.eq(stat_type as i32))
                .one(&self.db)
                .await?
                .map(|stat| stat.statistic_value)),

            None => Ok(VersionedStatistic::find()
                .inner_join(statistic::Entity)
                .filter(statistic::Column::TableId.eq(table_id))
                .filter(statistic::Column::VariantTag.eq(stat_type as i32))
                .order_by_desc(versioned_statistic::Column::EpochId)
                .one(&self.db)
                .await?
                .map(|stat| stat.statistic_value)),
        }
    }

    /// TODO: documentation
    async fn get_stats_for_attr(
        &self,
        mut attr_ids: Vec<AttrId>,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> StorageResult<Option<Json>> {
        let attr_num = attr_ids.len() as i32;
        attr_ids.sort();
        let description = self.get_description_from_attr_ids(attr_ids);

        // We don't join with junction table here for faster lookup.
        match epoch_id {
            Some(epoch_id) => Ok(VersionedStatistic::find()
                .filter(versioned_statistic::Column::EpochId.eq(epoch_id))
                .inner_join(statistic::Entity)
                .filter(statistic::Column::NumberOfAttributes.eq(attr_num))
                .filter(statistic::Column::Description.eq(description))
                .filter(statistic::Column::VariantTag.eq(stat_type as i32))
                .one(&self.db)
                .await?
                .map(|stat| stat.statistic_value)),

            None => Ok(VersionedStatistic::find()
                .inner_join(statistic::Entity)
                .filter(statistic::Column::NumberOfAttributes.eq(attr_num))
                .filter(statistic::Column::Description.eq(description))
                .filter(statistic::Column::VariantTag.eq(stat_type as i32))
                .order_by_desc(versioned_statistic::Column::EpochId)
                .one(&self.db)
                .await?
                .map(|stat| stat.statistic_value)),
        }
    }

    async fn get_stats_for_attr_indices_based(
        &self,
        table_id: TableId,
        attr_base_indices: Vec<AttrIndex>,
        stat_type: StatType,
        epoch_id: Option<EpochId>,
    ) -> StorageResult<Option<Json>> {
        // Get the attribute ids based on table id and attribute base indices
        let mut condition = Condition::any();
        for attr_base_index in &attr_base_indices {
            condition = condition.add(attribute::Column::BaseAttributeNumber.eq(*attr_base_index));
        }
        let attr_ids = Attribute::find()
            .filter(attribute::Column::TableId.eq(table_id))
            .filter(condition)
            .all(&self.db)
            .await?
            .iter()
            .map(|attr| attr.id)
            .collect::<Vec<_>>();

        if attr_ids.len() != attr_base_indices.len() {
            return Err(BackendError::BackendError(format!(
                "Not all attributes found for table_id {} and base indices {:?}",
                table_id, attr_base_indices
            )));
        }

        self.get_stats_for_attr(attr_ids, stat_type, epoch_id).await
    }

    /// TODO: documentation
    async fn get_cost_analysis(
        &self,
        expr_id: ExprId,
        epoch_id: EpochId,
    ) -> StorageResult<Option<Cost>> {
        let cost = PlanCost::find()
            .filter(plan_cost::Column::PhysicalExpressionId.eq(expr_id))
            .filter(plan_cost::Column::EpochId.eq(epoch_id))
            .one(&self.db)
            .await?;
        assert!(cost.is_some(), "Cost not found in Cost table");
        assert!(cost.clone().unwrap().is_valid, "Cost is not valid");
        Ok(cost.map(|c| Cost {
            compute_cost: c.cost.get("compute_cost").unwrap().as_i64().unwrap() as i32,
            io_cost: c.cost.get("io_cost").unwrap().as_i64().unwrap() as i32,
            estimated_statistic: c.estimated_statistic,
        }))
    }

    async fn get_cost(&self, expr_id: ExprId) -> StorageResult<Option<Cost>> {
        let cost = PlanCost::find()
            .filter(plan_cost::Column::PhysicalExpressionId.eq(expr_id))
            .order_by_desc(plan_cost::Column::EpochId)
            .one(&self.db)
            .await?;
        assert!(cost.is_some(), "Cost not found in Cost table");
        assert!(cost.clone().unwrap().is_valid, "Cost is not valid");
        Ok(cost.map(|c| Cost {
            compute_cost: c.cost.get("compute_cost").unwrap().as_i64().unwrap() as i32,
            io_cost: c.cost.get("io_cost").unwrap().as_i64().unwrap() as i32,
            estimated_statistic: c.estimated_statistic,
        }))
    }

    /// TODO: documentation
    async fn store_cost(
        &self,
        physical_expression_id: ExprId,
        cost: Cost,
        epoch_id: EpochId,
    ) -> StorageResult<()> {
        let expr_exists = PhysicalExpression::find_by_id(physical_expression_id)
            .one(&self.db)
            .await?;
        if expr_exists.is_none() {
            return Err(BackendError::BackendError(format!(
                "physical expression id {} not found when storing cost",
                physical_expression_id
            )));
        }

        // Check if epoch_id exists in Event table
        let epoch_exists = Event::find()
            .filter(event::Column::EpochId.eq(epoch_id))
            .one(&self.db)
            .await
            .unwrap();
        if epoch_exists.is_none() {
            return Err(BackendError::BackendError(format!(
                "epoch id {} not found when storing cost",
                epoch_id
            )));
        }

        let new_cost = plan_cost::ActiveModel {
            physical_expression_id: sea_orm::ActiveValue::Set(physical_expression_id),
            epoch_id: sea_orm::ActiveValue::Set(epoch_id),
            cost: sea_orm::ActiveValue::Set(
                json!({"compute_cost": cost.compute_cost, "io_cost": cost.io_cost}),
            ),
            estimated_statistic: sea_orm::ActiveValue::Set(cost.estimated_statistic),
            is_valid: sea_orm::ActiveValue::Set(true),
            ..Default::default()
        };
        let _ = PlanCost::insert(new_cost).exec(&self.db).await?;
        Ok(())
    }

    async fn get_attribute(
        &self,
        table_id: TableId,
        attribute_base_index: AttrIndex,
    ) -> StorageResult<Option<Attr>> {
        let attr_res = Attribute::find()
            .filter(attribute::Column::TableId.eq(table_id))
            .filter(attribute::Column::BaseAttributeNumber.eq(attribute_base_index))
            .one(&self.db)
            .await?;
        match attr_res {
            Some(attr) => match AttrType::try_from(attr.variant_tag) {
                Ok(attr_type) => Ok(Some(Attr {
                    table_id: attr.table_id,
                    name: attr.name,
                    compression_method: attr.compression_method,
                    attr_type,
                    base_index: attr.base_attribute_number,
                    nullable: attr.is_not_null,
                })),
                Err(_) => Err(BackendError::BackendError(format!(
                    "Failed to convert variant tag {} to AttrType",
                    attr.variant_tag
                ))),
            },
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cost_model::interface::{Cost, EpochOption, StatType};
    use crate::{cost_model::interface::Stat, migrate, CostModelStorageLayer};
    use crate::{get_sqlite_url, TEST_DATABASE_FILE};
    use sea_orm::sqlx::database;
    use sea_orm::sqlx::types::chrono::Utc;
    use sea_orm::Statement;
    use sea_orm::{
        ColumnTrait, ConnectionTrait, Database, DbBackend, EntityTrait, ModelTrait, QueryFilter,
        QuerySelect, QueryTrait,
    };
    use sea_orm_migration::schema::json;
    use serde_json::{de, json};

    use crate::entities::{prelude::*, *};

    async fn run_migration(db_file: &str) -> String {
        let database_url = get_sqlite_url(db_file);
        remove_db_file(db_file);

        let db = Database::connect(database_url.clone())
            .await
            .expect("Unable to connect to the database");

        migrate(&db)
            .await
            .expect("Something went wrong during migration");

        db.execute(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            "PRAGMA foreign_keys = ON;".to_owned(),
        ))
        .await
        .expect("Unable to enable foreign keys");
        database_url.clone()
    }

    fn remove_db_file(db_file: &str) {
        let database_file = format!("./{}", db_file);
        let _ = std::fs::remove_file(database_file);
    }

    async fn copy_init_db(db_file: &str) -> String {
        let _ = std::fs::copy(TEST_DATABASE_FILE.clone(), db_file);
        get_sqlite_url(db_file)
    }

    #[tokio::test]
    async fn test_create_new_epoch() {
        const DATABASE_FILE: &str = "test_create_new_epoch.db";
        let database_url = run_migration(DATABASE_FILE).await;
        let mut binding = super::BackendManager::new(Some(&database_url)).await;
        let backend_manager = binding.as_mut().unwrap();
        let res = backend_manager
            .create_new_epoch("source".to_string(), "data".to_string())
            .await;
        println!("{:?}", res);
        assert!(res.is_ok());
        let inserted_id = res.unwrap();
        let lookup_res = Event::find_by_id(inserted_id)
            .all(&backend_manager.db)
            .await
            .unwrap();
        println!("{:?}", lookup_res);
        assert_eq!(lookup_res.len(), 1);
        assert_eq!(lookup_res[0].source_variant, "source");
        assert_eq!(
            lookup_res[0].data,
            serde_json::Value::String("data".to_string())
        );
        assert_eq!(lookup_res[0].epoch_id, inserted_id);

        remove_db_file(DATABASE_FILE);
    }

    #[tokio::test]
    async fn test_update_stats_from_catalog() {
        const DATABASE_FILE: &str = "test_update_stats_from_catalog.db";
        let database_url = run_migration(DATABASE_FILE).await;
        let mut binding = super::BackendManager::new(Some(&database_url)).await;
        let backend_manager = binding.as_mut().unwrap();
        let res = backend_manager
            .update_stats_from_catalog(super::CatalogSource::Mock)
            .await;
        println!("{:?}", res);
        assert!(res.is_ok());
        let epoch_id = res.unwrap();
        assert_eq!(epoch_id, 1);

        let lookup_res = Statistic::find().all(&backend_manager.db).await.unwrap();
        println!("{:?}", lookup_res);
        assert_eq!(lookup_res.len(), 3);

        let stat_res = backend_manager
            .get_stats_for_table(1, StatType::TableRowCount, Some(epoch_id))
            .await;
        assert!(stat_res.is_ok());
        assert_eq!(stat_res.unwrap().unwrap(), json!(300));
        let stat_res = backend_manager
            .get_stats_for_attr([2].to_vec(), StatType::NonNullCount, None)
            .await;
        assert!(stat_res.is_ok());
        assert_eq!(stat_res.unwrap().unwrap(), json!(200));

        remove_db_file(DATABASE_FILE);
    }

    #[tokio::test]
    async fn test_update_attr_stats() {
        const DATABASE_FILE: &str = "test_update_attr_stats.db";
        let database_url = copy_init_db(DATABASE_FILE).await;
        let mut binding = super::BackendManager::new(Some(&database_url)).await;
        let backend_manager = binding.as_mut().unwrap();
        // 1. Update non-existed stat
        let epoch_id1 = backend_manager
            .create_new_epoch("test".to_string(), "test_update_attr_stats".to_string())
            .await
            .unwrap();
        let stat = Stat {
            stat_type: StatType::NonNullCount,
            stat_value: json!(100),
            attr_ids: vec![1],
            table_id: None,
            name: "countattr1".to_string(),
        };
        let res = backend_manager
            .update_stats(stat, EpochOption::Existed(epoch_id1))
            .await;
        assert!(res.is_ok());
        let stat_res = Statistic::find()
            .filter(statistic::Column::Name.eq("countattr1"))
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(stat_res.len(), 1);
        println!("{:?}", stat_res);
        assert_eq!(stat_res[0].number_of_attributes, 1);
        assert_eq!(stat_res[0].description, "1".to_string());
        assert_eq!(stat_res[0].variant_tag, StatType::NonNullCount as i32);
        let stat_attr_res = StatisticToAttributeJunction::find()
            .filter(statistic_to_attribute_junction::Column::StatisticId.eq(stat_res[0].id))
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(stat_attr_res.len(), 1);
        assert_eq!(stat_attr_res[0].attribute_id, 1);
        let versioned_stat_res = VersionedStatistic::find()
            .filter(versioned_statistic::Column::StatisticId.eq(stat_res[0].id))
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(versioned_stat_res.len(), 1);
        assert_eq!(versioned_stat_res[0].statistic_value, json!(100));
        assert_eq!(versioned_stat_res[0].epoch_id, epoch_id1);

        // 2. Normal update
        // 2.1 Insert some costs
        let res = PhysicalExpression::insert(physical_expression::ActiveModel {
            group_id: sea_orm::ActiveValue::Set(1),
            fingerprint: sea_orm::ActiveValue::Set(12346),
            variant_tag: sea_orm::ActiveValue::Set(1),
            data: sea_orm::ActiveValue::Set(serde_json::Value::String("data".to_string())),
            ..Default::default()
        });
        let expr_id = res.exec(&backend_manager.db).await.unwrap().last_insert_id;
        let res = PhysicalExpressionToStatisticJunction::insert(
            physical_expression_to_statistic_junction::ActiveModel {
                physical_expression_id: sea_orm::ActiveValue::Set(expr_id),
                statistic_id: sea_orm::ActiveValue::Set(stat_res[0].id),
            },
        )
        .exec(&backend_manager.db)
        .await
        .unwrap();
        backend_manager
            .store_cost(
                expr_id,
                {
                    Cost {
                        compute_cost: 42,
                        io_cost: 42,
                        estimated_statistic: 42,
                    }
                },
                versioned_stat_res[0].epoch_id,
            )
            .await
            .unwrap();
        let cost_res = PlanCost::find()
            .filter(plan_cost::Column::PhysicalExpressionId.eq(expr_id))
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(cost_res.len(), 1);
        assert!(cost_res[0].is_valid);
        // 2.2 Normal update
        let epoch_id2 = backend_manager
            .create_new_epoch("test".to_string(), "test_update_attr_stats".to_string())
            .await
            .unwrap();
        let stat2 = Stat {
            stat_type: StatType::NonNullCount,
            stat_value: json!(200),
            attr_ids: vec![1],
            table_id: None,
            name: "countattr1".to_string(),
        };
        let res = backend_manager
            .update_stats(stat2, EpochOption::Existed(epoch_id2))
            .await;
        assert!(res.is_ok());
        // 2.3 Check statistic table
        let stat_res = Statistic::find()
            .filter(statistic::Column::Name.eq("countattr1"))
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(stat_res.len(), 1);
        assert_eq!(stat_res[0].description, "1".to_string());
        // 2.4 Check statistic_to_attribute_junction table
        let stat_attr_res = StatisticToAttributeJunction::find()
            .filter(statistic_to_attribute_junction::Column::StatisticId.eq(stat_res[0].id))
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(stat_attr_res.len(), 1);
        assert_eq!(stat_attr_res[0].attribute_id, 1);
        // 2.5 Check versioned_statistic table
        let versioned_stat_res = VersionedStatistic::find()
            .filter(versioned_statistic::Column::StatisticId.eq(stat_res[0].id))
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(versioned_stat_res.len(), 2);
        assert_eq!(versioned_stat_res[0].statistic_value, json!(100));
        assert_eq!(versioned_stat_res[0].epoch_id, epoch_id1);
        assert_eq!(versioned_stat_res[0].statistic_id, stat_res[0].id);
        assert_eq!(versioned_stat_res[1].statistic_value, json!(200));
        assert_eq!(versioned_stat_res[1].epoch_id, epoch_id2);
        assert_eq!(versioned_stat_res[1].statistic_id, stat_res[0].id);
        assert!(epoch_id1 < epoch_id2);
        // 2.6 Check plan_cost table (cost invalidation)
        let cost_res = PlanCost::find()
            .filter(plan_cost::Column::PhysicalExpressionId.eq(expr_id))
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(cost_res.len(), 1);
        assert_eq!(cost_res[0].cost, json!({"compute_cost": 42, "io_cost": 42}));
        assert_eq!(cost_res[0].epoch_id, epoch_id1);
        assert!(!cost_res[0].is_valid);

        // 3. Update existed stat with the same value
        let epoch_num = Event::find().all(&backend_manager.db).await.unwrap().len();
        let stat3 = Stat {
            stat_type: StatType::NonNullCount,
            stat_value: json!(200),
            attr_ids: vec![1],
            table_id: None,
            name: "CountAttr1".to_string(),
        };
        let res = backend_manager
            .update_stats(
                stat3,
                EpochOption::New("source".to_string(), "data".to_string()),
            )
            .await;
        assert!(res.is_ok());
        assert!(res.unwrap().is_none());
        let epoch_num2 = Event::find().all(&backend_manager.db).await.unwrap().len();
        assert_eq!(epoch_num, epoch_num2);

        remove_db_file(DATABASE_FILE);
    }

    #[tokio::test]
    async fn test_update_table_stats() {
        // Simulate batch updates, first insert an existed same stat with none epoch_id,
        // then insert some non-existed or different stats with New epoch_option.
        const DATABASE_FILE: &str = "test_update_table_stats.db";
        let database_url = copy_init_db(DATABASE_FILE).await;
        let mut binding = super::BackendManager::new(Some(&database_url)).await;
        let backend_manager = binding.as_mut().unwrap();

        let table_inserted_res = TableMetadata::insert(table_metadata::ActiveModel {
            name: sea_orm::ActiveValue::Set("Table2".to_string()),
            namespace_id: sea_orm::ActiveValue::Set(1),
            creation_time: sea_orm::ActiveValue::Set(Utc::now()),
            ..Default::default()
        })
        .exec(&backend_manager.db)
        .await
        .unwrap();

        let statistics: Vec<Stat> = vec![
            Stat {
                stat_type: StatType::TableRowCount,
                stat_value: json!(0),
                attr_ids: vec![],
                table_id: Some(1),
                name: "row_count".to_string(),
            },
            Stat {
                stat_type: StatType::TableRowCount,
                stat_value: json!(20),
                attr_ids: vec![],
                table_id: Some(1),
                name: "row_count".to_string(),
            },
            Stat {
                stat_type: StatType::TableRowCount,
                stat_value: json!(100),
                attr_ids: vec![],
                table_id: Some(table_inserted_res.last_insert_id),
                name: "Table2Count1".to_string(),
            },
        ];

        let mut epoch_id: Option<i32> = None;
        for stat in statistics {
            match epoch_id {
                Some(e) => {
                    let res = backend_manager
                        .update_stats(stat.clone(), EpochOption::Existed(e))
                        .await;
                    assert!(res.is_ok());
                    assert!(stat.name == "Table2Count1");
                    let res = res.unwrap();
                    assert!(res.is_some());
                    assert!(res.unwrap() == e);
                    let stat_res = Statistic::find()
                        .filter(statistic::Column::Name.eq(stat.name.clone()))
                        .all(&backend_manager.db)
                        .await
                        .unwrap();
                    assert_eq!(stat_res.len(), 1);
                    let versioned_stat_res = VersionedStatistic::find()
                        .filter(versioned_statistic::Column::StatisticId.eq(stat_res[0].id))
                        .all(&backend_manager.db)
                        .await
                        .unwrap();
                    assert_eq!(versioned_stat_res.len(), 1);
                    assert_eq!(versioned_stat_res[0].statistic_value, stat.stat_value);
                    assert_eq!(versioned_stat_res[0].epoch_id, e);
                }
                None => {
                    let res = backend_manager
                        .update_stats(
                            stat.clone(),
                            EpochOption::New("source".to_string(), "data".to_string()),
                        )
                        .await;
                    assert!(res.is_ok());
                    if stat.stat_value == json!(0) {
                        assert!(res.unwrap().is_none());
                    } else {
                        assert!(stat.stat_value == json!(20));
                        let res = res.unwrap();
                        assert!(res.is_some());
                        epoch_id = Some(res.unwrap());
                    }
                }
            }
        }

        remove_db_file(DATABASE_FILE);
    }

    #[tokio::test]
    async fn test_store_cost() {
        const DATABASE_FILE: &str = "test_store_cost.db";
        let database_url = copy_init_db(DATABASE_FILE).await;
        let mut binding = super::BackendManager::new(Some(&database_url)).await;
        let backend_manager = binding.as_mut().unwrap();
        let epoch_id = backend_manager
            .create_new_epoch("source".to_string(), "data".to_string())
            .await
            .unwrap();
        let physical_expression_id = 1;
        let cost = Cost {
            compute_cost: 42,
            io_cost: 42,
            estimated_statistic: 42,
        };
        backend_manager
            .store_cost(physical_expression_id, cost.clone(), epoch_id)
            .await
            .unwrap();
        let costs = super::PlanCost::find()
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(costs.len(), 2); // The first row one is the initialized data
        assert_eq!(costs[1].epoch_id, epoch_id);
        assert_eq!(costs[1].physical_expression_id, physical_expression_id);
        assert_eq!(
            costs[1].cost,
            json!({"compute_cost": cost.compute_cost, "io_cost": cost.io_cost})
        );
        assert_eq!(
            costs[1].estimated_statistic as i32,
            cost.estimated_statistic
        );

        remove_db_file(DATABASE_FILE);
    }

    #[tokio::test]
    async fn test_get_cost() {
        const DATABASE_FILE: &str = "test_get_cost.db";
        let database_url = copy_init_db(DATABASE_FILE).await;
        let mut binding = super::BackendManager::new(Some(&database_url)).await;
        let backend_manager = binding.as_mut().unwrap();
        let epoch_id = backend_manager
            .create_new_epoch("source".to_string(), "data".to_string())
            .await
            .unwrap();
        let physical_expression_id = 1;
        let cost = Cost {
            compute_cost: 42,
            io_cost: 42,
            estimated_statistic: 42,
        };
        let _ = backend_manager
            .store_cost(physical_expression_id, cost.clone(), epoch_id)
            .await;
        let costs = super::PlanCost::find()
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(costs.len(), 2); // The first row one is the initialized data
        assert_eq!(costs[1].epoch_id, epoch_id);
        assert_eq!(costs[1].physical_expression_id, physical_expression_id);
        assert_eq!(
            costs[1].cost,
            json!({"compute_cost": cost.compute_cost, "io_cost": cost.io_cost})
        );
        assert_eq!(
            costs[1].estimated_statistic as i32,
            cost.estimated_statistic
        );

        let res = backend_manager
            .get_cost(physical_expression_id)
            .await
            .unwrap();
        assert_eq!(res.unwrap(), cost);

        remove_db_file(DATABASE_FILE);
    }

    #[tokio::test]
    async fn test_get_cost_analysis() {
        const DATABASE_FILE: &str = "test_get_cost_analysis.db";
        let database_url = copy_init_db(DATABASE_FILE).await;
        let mut binding = super::BackendManager::new(Some(&database_url)).await;
        let backend_manager = binding.as_mut().unwrap();
        let epoch_id = backend_manager
            .create_new_epoch("source".to_string(), "data".to_string())
            .await
            .unwrap();
        let physical_expression_id = 1;
        let cost = Cost {
            compute_cost: 1420,
            io_cost: 42,
            estimated_statistic: 42,
        };
        let _ = backend_manager
            .store_cost(physical_expression_id, cost.clone(), epoch_id)
            .await;
        let costs = super::PlanCost::find()
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(costs.len(), 2); // The first row one is the initialized data
        assert_eq!(costs[1].epoch_id, epoch_id);
        assert_eq!(costs[1].physical_expression_id, physical_expression_id);
        assert_eq!(
            costs[1].cost,
            json!({"compute_cost": cost.compute_cost, "io_cost": cost.io_cost})
        );
        assert_eq!(
            costs[1].estimated_statistic as i32,
            cost.estimated_statistic
        );
        println!("{:?}", costs);

        // Retrieve physical_expression_id 1 and epoch_id 1
        let res = backend_manager
            .get_cost_analysis(physical_expression_id, 1)
            .await
            .unwrap();

        // The cost in the dummy data is 10
        assert_eq!(
            res.unwrap(),
            Cost {
                compute_cost: 10,
                io_cost: 10,
                estimated_statistic: 10,
            }
        );

        remove_db_file(DATABASE_FILE);
    }

    #[tokio::test]
    async fn test_get_stats_for_table() {
        const DATABASE_FILE: &str = "test_get_stats_for_table.db";
        let database_url = copy_init_db(DATABASE_FILE).await;
        let mut binding = super::BackendManager::new(Some(&database_url)).await;
        let backend_manager = binding.as_mut().unwrap();
        let epoch_id = 1;
        let table_id = 1;
        let stat_type = StatType::TableRowCount;

        // Get initial stats
        let res = backend_manager
            .get_stats_for_table(table_id, stat_type, None)
            .await
            .unwrap()
            .unwrap();
        let row_count = res.as_i64().unwrap();
        assert_eq!(row_count, 0);

        // Update stats
        let epoch_id2 = backend_manager
            .create_new_epoch("test".to_string(), "test_get_stats_for_table".to_string())
            .await
            .unwrap();
        let stat = Stat {
            stat_type: StatType::TableRowCount,
            stat_value: json!(100),
            attr_ids: vec![],
            table_id: Some(table_id),
            name: "row_count".to_string(),
        };
        backend_manager
            .update_stats(stat, EpochOption::Existed(epoch_id2))
            .await
            .unwrap();

        // Get updated stats
        let res = backend_manager
            .get_stats_for_table(table_id, stat_type, None)
            .await
            .unwrap()
            .unwrap();
        let row_count = res.as_i64().unwrap();
        assert_eq!(row_count, 100);

        // Get stats for a specific epoch
        let res = backend_manager
            .get_stats_for_table(table_id, stat_type, Some(epoch_id))
            .await
            .unwrap()
            .unwrap();
        let row_count = res.as_i64().unwrap();
        assert_eq!(row_count, 0);

        remove_db_file(DATABASE_FILE);
    }

    #[tokio::test]
    async fn test_get_stats_for_single_attr() {
        const DATABASE_FILE: &str = "test_get_stats_for_single_attr.db";
        let database_url = copy_init_db(DATABASE_FILE).await;
        let mut binding = super::BackendManager::new(Some(&database_url)).await;
        let backend_manager = binding.as_mut().unwrap();
        let epoch_id = 1;
        let attr_ids = vec![1];
        let stat_type = StatType::Cardinality;

        // Get initial stats
        let res = backend_manager
            .get_stats_for_attr(attr_ids.clone(), stat_type, None)
            .await
            .unwrap()
            .unwrap();
        let cardinality = res.as_i64().unwrap();
        assert_eq!(cardinality, 0);

        // Update stats
        let epoch_id2 = backend_manager
            .create_new_epoch(
                "test".to_string(),
                "test_get_stats_for_single_attr".to_string(),
            )
            .await
            .unwrap();
        let stat = Stat {
            stat_type: StatType::Cardinality,
            stat_value: json!(100),
            attr_ids: attr_ids.clone(),
            table_id: None,
            name: "cardinality".to_string(),
        };
        backend_manager
            .update_stats(stat, EpochOption::Existed(epoch_id2))
            .await
            .unwrap();

        // Get updated stats
        let res = backend_manager
            .get_stats_for_attr(attr_ids.clone(), stat_type, None)
            .await
            .unwrap()
            .unwrap();
        let cardinality = res.as_i64().unwrap();
        assert_eq!(cardinality, 100);

        // Get stats for a specific epoch
        let res = backend_manager
            .get_stats_for_attr(attr_ids.clone(), stat_type, Some(epoch_id))
            .await
            .unwrap()
            .unwrap();
        let cardinality = res.as_i64().unwrap();
        assert_eq!(cardinality, 0);

        remove_db_file(DATABASE_FILE);
    }

    #[tokio::test]
    async fn test_get_stats_for_multiple_attrs() {
        const DATABASE_FILE: &str = "test_get_stats_for_multiple_attrs.db";
        let database_url = copy_init_db(DATABASE_FILE).await;
        let mut binding = super::BackendManager::new(Some(&database_url)).await;
        let backend_manager = binding.as_mut().unwrap();
        let epoch_id = 1;
        let attr_ids = vec![2, 1];
        let stat_type = StatType::Cardinality;

        // Get initial stats
        let res = backend_manager
            .get_stats_for_attr(attr_ids.clone(), stat_type, None)
            .await
            .unwrap()
            .unwrap();
        let cardinality = res.as_i64().unwrap();
        assert_eq!(cardinality, 0);

        // Update stats
        let epoch_id2 = backend_manager
            .create_new_epoch(
                "test".to_string(),
                "test_get_stats_for_multiple_attrs".to_string(),
            )
            .await
            .unwrap();
        let stat = Stat {
            stat_type: StatType::Cardinality,
            stat_value: json!(111),
            attr_ids: attr_ids.clone(),
            table_id: None,
            name: "cardinality".to_string(),
        };
        backend_manager
            .update_stats(stat, EpochOption::Existed(epoch_id2))
            .await
            .unwrap();

        // Get updated stats
        let res = backend_manager
            .get_stats_for_attr(attr_ids.clone(), stat_type, None)
            .await
            .unwrap()
            .unwrap();
        let cardinality = res.as_i64().unwrap();
        assert_eq!(cardinality, 111);

        // Get stats for a specific epoch
        let res = backend_manager
            .get_stats_for_attr(attr_ids.clone(), stat_type, Some(epoch_id))
            .await
            .unwrap()
            .unwrap();
        let cardinality = res.as_i64().unwrap();
        assert_eq!(cardinality, 0);

        remove_db_file(DATABASE_FILE);
    }

    #[tokio::test]
    async fn test_get_stats_for_attr_indices_based() {
        const DATABASE_FILE: &str = "test_get_stats_for_attr_indices_based.db";
        let database_url = copy_init_db(DATABASE_FILE).await;
        let mut binding = super::BackendManager::new(Some(&database_url)).await;
        let backend_manager = binding.as_mut().unwrap();
        let epoch_id = 1;
        let table_id = 1;
        let attr_base_indices = vec![0, 1];
        let stat_type = StatType::Cardinality;

        // Statistics exist in the database
        let res = backend_manager
            .get_stats_for_attr_indices_based(table_id, attr_base_indices.clone(), stat_type, None)
            .await
            .unwrap()
            .unwrap();
        let cardinality = res.as_i64().unwrap();
        assert_eq!(cardinality, 0);

        // Statistics do not exist in the database
        let attr_base_indices = vec![1];
        let res = backend_manager
            .get_stats_for_attr_indices_based(table_id, attr_base_indices.clone(), stat_type, None)
            .await
            .unwrap();
        assert!(res.is_none());

        // Attribute base indices not valid.
        let attr_base_indices = vec![1, 2];
        let res = backend_manager
            .get_stats_for_attr_indices_based(table_id, attr_base_indices.clone(), stat_type, None)
            .await;
        assert!(res.is_err());

        remove_db_file(DATABASE_FILE);
    }
}

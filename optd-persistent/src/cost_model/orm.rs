#![allow(dead_code, unused_imports, unused_variables)]

use std::ptr::null;

use crate::entities::{prelude::*, *};
use crate::{BackendError, BackendManager, CostModelError, CostModelStorageLayer, StorageResult};
use sea_orm::prelude::{Expr, Json};
use sea_orm::sea_query::Query;
use sea_orm::{sqlx::types::chrono::Utc, EntityTrait};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbBackend, DbErr, DeleteResult, EntityOrSelect, ModelTrait,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait, RuntimeErr, TransactionTrait,
};

use super::catalog::mock_catalog::{self, MockCatalog};
use super::interface::{CatalogSource, Stat};

impl BackendManager {
    fn get_description_from_attr_ids(
        &self,
        attr_ids: Vec<<BackendManager as CostModelStorageLayer>::AttrId>,
    ) -> String {
        let mut attr_ids = attr_ids;
        attr_ids.sort();
        attr_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl CostModelStorageLayer for BackendManager {
    type GroupId = i32;
    type TableId = i32;
    type AttrId = i32;
    type ExprId = i32;
    type EpochId = i32;
    type StatId = i32;

    async fn create_new_epoch(
        &mut self,
        source: String,
        data: String,
    ) -> StorageResult<Self::EpochId> {
        let new_event = event::ActiveModel {
            source_variant: sea_orm::ActiveValue::Set(source),
            timestamp: sea_orm::ActiveValue::Set(Utc::now()),
            data: sea_orm::ActiveValue::Set(sea_orm::JsonValue::String(data)),
            ..Default::default()
        };
        let insert_res = Event::insert(new_event).exec(&self.db).await?;
        self.latest_epoch_id.store(
            insert_res.last_insert_id as usize,
            std::sync::atomic::Ordering::Relaxed,
        );
        Ok(insert_res.last_insert_id)
    }

    async fn update_stats_from_catalog(
        &self,
        c: CatalogSource,
        epoch_id: Self::EpochId,
    ) -> StorageResult<()> {
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
                .exec(&self.db)
                .await?;
                NamespaceMetadata::insert_many(mock_catalog.namespaces.iter().map(|namespace| {
                    namespace_metadata::ActiveModel {
                        name: sea_orm::ActiveValue::Set(namespace.name.clone()),
                        database_id: sea_orm::ActiveValue::Set(namespace.database_id),
                        creation_time: sea_orm::ActiveValue::Set(Utc::now()),
                        ..Default::default()
                    }
                }))
                .exec(&self.db)
                .await?;
                TableMetadata::insert_many(mock_catalog.tables.iter().map(|table| {
                    table_metadata::ActiveModel {
                        name: sea_orm::ActiveValue::Set(table.name.clone()),
                        namespace_id: sea_orm::ActiveValue::Set(table.namespace_id),
                        creation_time: sea_orm::ActiveValue::Set(Utc::now()),
                        ..Default::default()
                    }
                }))
                .exec(&self.db)
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
                .exec(&self.db)
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
                .exec(&self.db)
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
                .exec(&self.db)
                .await?;
                Ok(())
            }
            CatalogSource::Iceberg() => todo!(),
        }
    }

    // **IMPORTANT**: It is the caller's responsibility to ensure that the updated stat is not the same with the last stored stat if
    // if it is already exists.
    async fn update_stats(&self, stat: Stat, epoch_id: Self::EpochId) -> StorageResult<()> {
        // let transaction = self.db.begin().await?;
        // 0. Check if the stat already exists. If exists, get stat_id, else insert into statistic table.
        let stat_id = match stat.table_id {
            Some(table_id) => {
                // TODO(lanlou): only select needed fields
                let res = Statistic::find()
                    .filter(statistic::Column::TableId.eq(table_id))
                    .filter(statistic::Column::VariantTag.eq(stat.stat_type))
                    /*
                    TODO(FIX_ME, lanlou): Do we need the following filter?
                    I am really not sure although I add the top comment...
                    Since we already increase the epoch, so we should update the stat anyway.
                    (In theory, we can increase the epoch without updating the stat, but it is not
                    a straightforward design, and the epoch table will be very large.)
                    But it will increase the overhead, since the caller will need to make another
                    query to check if the stat is the same with the last one. We cannot put everything
                    in one query.
                    Let us assume we should update the stat anyway for now.
                    */
                    // .inner_join(versioned_statistic::Entity)
                    // .select_also(versioned_statistic::Entity)
                    // .order_by_desc(versioned_statistic::Column::EpochId)
                    .one(&self.db)
                    .await?;
                match res {
                    Some(stat_data) => {
                        // if stat_data.1.unwrap().statistic_value == stat.stat_value {
                        //     return Ok(());
                        // }
                        // stat_data.0.id
                        stat_data.id
                    }
                    None => {
                        let new_stat = statistic::ActiveModel {
                            name: sea_orm::ActiveValue::Set(stat.name.clone()),
                            table_id: sea_orm::ActiveValue::Set(Some(table_id)),
                            number_of_attributes: sea_orm::ActiveValue::Set(
                                stat.attr_ids.len() as i32
                            ),
                            creation_time: sea_orm::ActiveValue::Set(Utc::now()),
                            variant_tag: sea_orm::ActiveValue::Set(stat.stat_type),
                            description: sea_orm::ActiveValue::Set("".to_string()),
                            ..Default::default()
                        };
                        let res = Statistic::insert(new_stat).exec(&self.db).await;
                        match res {
                            Ok(insert_res) => insert_res.last_insert_id,
                            Err(_) => {
                                return Err(BackendError::Database(DbErr::Exec(
                                    RuntimeErr::Internal(
                                        "Failed to insert into statistic table".to_string(),
                                    ),
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
                    .filter(statistic::Column::VariantTag.eq(stat.stat_type))
                    // .inner_join(versioned_statistic::Entity)
                    // .select_also(versioned_statistic::Entity)
                    // .order_by_desc(versioned_statistic::Column::EpochId)
                    .one(&self.db)
                    .await?;
                match res {
                    Some(stat_data) => {
                        // if stat_data.1.unwrap().statistic_value == stat.stat_value {
                        //     return Ok(());
                        // }
                        // stat_data.0.id
                        stat_data.id
                    }
                    None => {
                        let new_stat = statistic::ActiveModel {
                            name: sea_orm::ActiveValue::Set(stat.name.clone()),
                            number_of_attributes: sea_orm::ActiveValue::Set(
                                stat.attr_ids.len() as i32
                            ),
                            creation_time: sea_orm::ActiveValue::Set(Utc::now()),
                            variant_tag: sea_orm::ActiveValue::Set(stat.stat_type),
                            description: sea_orm::ActiveValue::Set(description),
                            ..Default::default()
                        };
                        // TODO(lanlou): we should not clone here maybe...
                        let insert_res = Statistic::insert(new_stat.clone()).exec(&self.db).await?;
                        for attr_id in stat.attr_ids {
                            let new_junction = statistic_to_attribute_junction::ActiveModel {
                                statistic_id: sea_orm::ActiveValue::Set(insert_res.last_insert_id),
                                attribute_id: sea_orm::ActiveValue::Set(attr_id),
                            };
                            let res = StatisticToAttributeJunction::insert(new_junction)
                                .exec(&self.db)
                                .await;
                            if res.is_err() {
                                let _ = new_stat.delete(&self.db).await;
                                return Err(BackendError::Database(DbErr::Exec(
                                    RuntimeErr::Internal(
                                        "Failed to insert into statistic_to_attribute_junction table".to_string(),
                                    ),
                                )));
                            }
                        }
                        insert_res.last_insert_id
                    }
                }
            }
        };
        // 1. Insert into attr_stats and related junction tables.
        let new_stats = versioned_statistic::ActiveModel {
            epoch_id: sea_orm::ActiveValue::Set(epoch_id),
            statistic_id: sea_orm::ActiveValue::Set(stat_id),
            statistic_value: sea_orm::ActiveValue::Set(sea_orm::JsonValue::String(stat.stat_value)),
            ..Default::default()
        };
        let _ = VersionedStatistic::insert(new_stats).exec(&self.db).await;

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
            .exec(&self.db)
            .await;

        // transaction.commit().await?;
        Ok(())
    }

    async fn store_expr_stats_mappings(
        &self,
        expr_id: Self::ExprId,
        stat_ids: Vec<Self::StatId>,
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

    async fn get_stats_for_table(
        &self,
        table_id: i32,
        stat_type: i32,
        epoch_id: Option<Self::EpochId>,
    ) -> StorageResult<Option<Json>> {
        match epoch_id {
            Some(epoch_id) => Ok(VersionedStatistic::find()
                .filter(versioned_statistic::Column::EpochId.eq(epoch_id))
                .inner_join(statistic::Entity)
                .filter(statistic::Column::TableId.eq(table_id))
                .filter(statistic::Column::VariantTag.eq(stat_type))
                .one(&self.db)
                .await?
                .map(|stat| stat.statistic_value)),

            None => Ok(VersionedStatistic::find()
                .inner_join(statistic::Entity)
                .filter(statistic::Column::TableId.eq(table_id))
                .filter(statistic::Column::VariantTag.eq(stat_type))
                .order_by_desc(versioned_statistic::Column::EpochId)
                .one(&self.db)
                .await?
                .map(|stat| stat.statistic_value)),
        }
    }

    async fn get_stats_for_attr(
        &self,
        mut attr_ids: Vec<Self::AttrId>,
        stat_type: i32,
        epoch_id: Option<Self::EpochId>,
    ) -> StorageResult<Option<Json>> {
        let attr_num = attr_ids.len() as i32;
        // The description is to concat `attr_ids` using commas
        // Note that `attr_ids` should be sorted before concatenation
        // e.g. [1, 2, 3] -> "1,2,3"
        attr_ids.sort();
        let description = self.get_description_from_attr_ids(attr_ids);

        // We don't join with junction table here for faster lookup.
        match epoch_id {
            Some(epoch_id) => Ok(VersionedStatistic::find()
                .filter(versioned_statistic::Column::EpochId.eq(epoch_id))
                .inner_join(statistic::Entity)
                .filter(statistic::Column::NumberOfAttributes.eq(attr_num))
                .filter(statistic::Column::Description.eq(description))
                .filter(statistic::Column::VariantTag.eq(stat_type))
                .one(&self.db)
                .await?
                .map(|stat| stat.statistic_value)),

            None => Ok(VersionedStatistic::find()
                .inner_join(statistic::Entity)
                .filter(statistic::Column::NumberOfAttributes.eq(attr_num))
                .filter(statistic::Column::Description.eq(description))
                .filter(statistic::Column::VariantTag.eq(stat_type))
                .order_by_desc(versioned_statistic::Column::EpochId)
                .one(&self.db)
                .await?
                .map(|stat| stat.statistic_value)),
        }
    }

    async fn get_cost_analysis(
        &self,
        expr_id: Self::ExprId,
        epoch_id: Self::EpochId,
    ) -> StorageResult<Option<i32>> {
        let cost = PlanCost::find()
            .filter(plan_cost::Column::PhysicalExpressionId.eq(expr_id))
            .filter(plan_cost::Column::EpochId.eq(epoch_id))
            .one(&self.db)
            .await?;
        assert!(cost.is_some(), "Cost not found in Cost table");
        assert!(cost.clone().unwrap().is_valid, "Cost is not valid");
        Ok(cost.map(|c| c.cost))
    }

    async fn get_cost(&self, expr_id: Self::ExprId) -> StorageResult<Option<i32>> {
        let cost = PlanCost::find()
            .filter(plan_cost::Column::PhysicalExpressionId.eq(expr_id))
            .order_by_desc(plan_cost::Column::EpochId)
            .one(&self.db)
            .await?;
        assert!(cost.is_some(), "Cost not found in Cost table");
        assert!(cost.clone().unwrap().is_valid, "Cost is not valid");
        Ok(cost.map(|c| c.cost))
    }

    async fn store_cost(
        &self,
        physical_expression_id: Self::ExprId,
        cost: i32,
        epoch_id: Self::EpochId,
    ) -> StorageResult<()> {
        let expr_exists = PhysicalExpression::find_by_id(physical_expression_id)
            .one(&self.db)
            .await?;
        if expr_exists.is_none() {
            return Err(BackendError::Database(DbErr::RecordNotFound(
                "ExprId not found in PhysicalExpression table".to_string(),
            )));
        }

        // Check if epoch_id exists in Event table
        let epoch_exists = Event::find()
            .filter(event::Column::EpochId.eq(epoch_id))
            .one(&self.db)
            .await
            .unwrap();
        if epoch_exists.is_none() {
            return Err(BackendError::Database(DbErr::RecordNotFound(
                "EpochId not found in Event table".to_string(),
            )));
        }

        let new_cost = plan_cost::ActiveModel {
            physical_expression_id: sea_orm::ActiveValue::Set(physical_expression_id),
            epoch_id: sea_orm::ActiveValue::Set(epoch_id),
            cost: sea_orm::ActiveValue::Set(cost),
            is_valid: sea_orm::ActiveValue::Set(true),
            ..Default::default()
        };
        let _ = PlanCost::insert(new_cost).exec(&self.db).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cost_model::interface::StatisticType;
    use crate::{cost_model::interface::Stat, migrate, CostModelStorageLayer};
    use crate::{get_sqlite_url, TEST_DATABASE_FILE};
    use sea_orm::sqlx::database;
    use sea_orm::Statement;
    use sea_orm::{
        ColumnTrait, ConnectionTrait, Database, DbBackend, EntityTrait, ModelTrait, QueryFilter,
        QuerySelect, QueryTrait,
    };
    use serde_json::de;

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
        let database_url = get_sqlite_url(db_file);
        database_url
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
            .update_stats_from_catalog(super::CatalogSource::Mock, 1)
            .await;
        println!("{:?}", res);
        assert!(res.is_ok());

        let lookup_res = Statistic::find().all(&backend_manager.db).await.unwrap();
        println!("{:?}", lookup_res);
        assert_eq!(lookup_res.len(), 3);

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
            .create_new_epoch("test".to_string(), "InsertTest".to_string())
            .await
            .unwrap();
        let stat = Stat {
            stat_type: StatisticType::Count as i32,
            stat_value: "100".to_string(),
            attr_ids: vec![1],
            table_id: None,
            name: "CountAttr1".to_string(),
        };
        let res = backend_manager.update_stats(stat, epoch_id1).await;
        assert!(res.is_ok());
        let stat_res = Statistic::find()
            .filter(statistic::Column::Name.eq("CountAttr1"))
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(stat_res.len(), 1);
        println!("{:?}", stat_res);
        assert_eq!(stat_res[0].number_of_attributes, 1);
        assert_eq!(stat_res[0].description, "1".to_string());
        assert_eq!(stat_res[0].variant_tag, StatisticType::Count as i32);
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
        assert_eq!(
            versioned_stat_res[0].statistic_value,
            serde_json::Value::String("100".to_string())
        );
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
            .store_cost(expr_id, 42, versioned_stat_res[0].epoch_id)
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
            .create_new_epoch("test".to_string(), "InsertTest".to_string())
            .await
            .unwrap();
        let stat2 = Stat {
            stat_type: StatisticType::Count as i32,
            stat_value: "200".to_string(),
            attr_ids: vec![1],
            table_id: None,
            name: "CountAttr1".to_string(),
        };
        let res = backend_manager.update_stats(stat2, epoch_id2).await;
        assert!(res.is_ok());
        // 2.3 Check statistic table
        let stat_res = Statistic::find()
            .filter(statistic::Column::Name.eq("CountAttr1"))
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(stat_res.len(), 1);
        assert_eq!(stat_res[0].number_of_attributes, 1);
        assert_eq!(stat_res[0].description, "1".to_string());
        assert_eq!(stat_res[0].variant_tag, StatisticType::Count as i32);
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
        assert_eq!(
            versioned_stat_res[0].statistic_value,
            serde_json::Value::String("100".to_string())
        );
        assert_eq!(versioned_stat_res[0].epoch_id, epoch_id1);
        assert_eq!(versioned_stat_res[0].statistic_id, stat_res[0].id);
        assert_eq!(
            versioned_stat_res[1].statistic_value,
            serde_json::Value::String("200".to_string())
        );
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
        assert_eq!(cost_res[0].cost, 42);
        assert_eq!(cost_res[0].epoch_id, epoch_id1);
        assert!(!cost_res[0].is_valid);

        remove_db_file(DATABASE_FILE);
    }

    #[tokio::test]
    async fn test_update_table_stats() {}

    #[tokio::test]
    #[ignore] // Need to update all tables
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
        let cost = 42;
        let res = backend_manager
            .store_cost(physical_expression_id, cost, epoch_id)
            .await;
        match res {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
        let costs = super::PlanCost::find()
            .all(&backend_manager.db)
            .await
            .unwrap();
        assert_eq!(costs.len(), 2); // The first row one is the initialized data
        assert_eq!(costs[1].epoch_id, epoch_id);
        assert_eq!(costs[1].physical_expression_id, physical_expression_id);
        assert_eq!(costs[1].cost, cost);
    }
}

#![allow(dead_code, unused_imports, unused_variables)]

use std::ptr::null;

use crate::entities::{prelude::*, *};
use crate::{BackendError, BackendManager, CostModelError, CostModelStorageLayer, StorageResult};
use sea_orm::prelude::{Expr, Json};
use sea_orm::sea_query::Query;
use sea_orm::{sqlx::types::chrono::Utc, EntityTrait};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, DeleteResult, EntityOrSelect, ModelTrait, QueryFilter,
    QueryOrder, QuerySelect, RuntimeErr, TransactionTrait,
};

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
        todo!()
    }

    async fn update_stats(&self, stat: Stat, epoch_id: Self::EpochId) -> StorageResult<()> {
        let transaction = self.db.begin().await?;
        // 0. Check if the stat already exists. If exists, get stat_id, else insert into statistic table.
        let stat_id = match stat.table_id {
            Some(table_id) => {
                // TODO(lanlou): only select needed fields
                let res = Statistic::find()
                    .filter(statistic::Column::TableId.eq(table_id))
                    .filter(statistic::Column::VariantTag.eq(stat.stat_type))
                    // FIX_ME: Do we need the following filter?
                    .inner_join(versioned_statistic::Entity)
                    .select_also(versioned_statistic::Entity)
                    .order_by_desc(versioned_statistic::Column::EpochId)
                    .one(&self.db)
                    .await?;
                match res {
                    Some(stat_data) => {
                        if stat_data.1.unwrap().statistic_value == stat.stat_value {
                            return Ok(());
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
                            created_time: sea_orm::ActiveValue::Set(Utc::now()),
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
                    // FIX_ME: Do we need the following filter?
                    .inner_join(versioned_statistic::Entity)
                    .select_also(versioned_statistic::Entity)
                    .order_by_desc(versioned_statistic::Column::EpochId)
                    .one(&self.db)
                    .await?;
                match res {
                    Some(stat_data) => {
                        if stat_data.1.unwrap().statistic_value == stat.stat_value {
                            return Ok(());
                        }
                        stat_data.0.id
                    }
                    None => {
                        let new_stat = statistic::ActiveModel {
                            name: sea_orm::ActiveValue::Set(stat.name.clone()),
                            number_of_attributes: sea_orm::ActiveValue::Set(
                                stat.attr_ids.len() as i32
                            ),
                            created_time: sea_orm::ActiveValue::Set(Utc::now()),
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
            .filter(
                plan_cost::Column::PhysicalExpressionId.in_subquery(
                    (*Query::select()
                        .column(
                            physical_expression_to_statistic_junction::Column::PhysicalExpressionId,
                        )
                        .from(physical_expression_to_statistic_junction::Entity)
                        .and_where(
                            physical_expression_to_statistic_junction::Column::StatisticId
                                .eq(stat_id),
                        ))
                    .to_owned(),
                ),
            )
            .exec(&self.db)
            .await;

        transaction.commit().await?;
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
        expr_id: Self::ExprId,
        cost: i32,
        epoch_id: Self::EpochId,
    ) -> StorageResult<()> {
        let expr_exists = PhysicalExpression::find_by_id(expr_id)
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
            physical_expression_id: sea_orm::ActiveValue::Set(expr_id),
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
    use crate::{migrate, CostModelStorageLayer};
    use sea_orm::{
        ColumnTrait, ConnectionTrait, Database, DbBackend, EntityTrait, ModelTrait, QueryFilter,
        QuerySelect, QueryTrait,
    };
    use serde_json::de;

    use crate::entities::{prelude::*, *};

    async fn run_migration(db_file: &str) -> String {
        let database_url = format!("sqlite:./{}?mode=rwc", db_file);
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
}

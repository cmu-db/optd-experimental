#![allow(dead_code, unused_imports, unused_variables)]

use sea_orm::{Database, DatabaseConnection};

use crate::{EpochId, DATABASE_URL};

pub struct ORMManager {
    db_conn: DatabaseConnection,
    // TODO: Change EpochId to event::Model::epoch_id
    latest_epoch_id: EpochId,
}

impl ORMManager {
    pub async fn new(database_url: Option<&str>) -> Self {
        let latest_epoch_id = -1;
        let db_conn = Database::connect(database_url.unwrap_or(DATABASE_URL))
            .await
            .unwrap();
        Self {
            db_conn,
            latest_epoch_id,
        }
    }
}

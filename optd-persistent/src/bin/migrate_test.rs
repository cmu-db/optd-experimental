use optd_persistent::{migrate, TEST_DATABASE_URL};
use sea_orm::*;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    let _ = std::fs::remove_file(TEST_DATABASE_URL);

    let db = Database::connect(TEST_DATABASE_URL)
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
}
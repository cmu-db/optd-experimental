use optd_persistent::{migrate, TEST_DATABASE_URL};
use sea_orm::*;
use sea_orm_migration::prelude::*;

async fn run_migration() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::remove_file(TEST_DATABASE_URL);

    let db = Database::connect(TEST_DATABASE_URL)
        .await
        .expect("Unable to connect to the database");

    migrate(&db)
        .await
        .expect("Something went wrong during migration");

    Ok(())
}

#[tokio::main]
async fn main() {
    run_migration().await.expect("Migration failed");
}

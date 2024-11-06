use optd_persistent::{migrate, DATABASE_FILE, DATABASE_URL};
use sea_orm::*;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    let _ = std::fs::remove_file(DATABASE_FILE);

    let db = Database::connect(DATABASE_URL)
        .await
        .expect("Unable to connect to the database");

    migrate(&db)
        .await
        .expect("Something went wrong during migration");
}

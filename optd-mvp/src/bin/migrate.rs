//! A simple script that generates the database file needed for `sea-orm-cli` to extract the schemas
//! from and generate the `entities` module.

use optd_mvp::{migrate, DATABASE_FILENAME, DATABASE_URL};
use sea_orm::*;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    let _ = std::fs::remove_file(DATABASE_FILENAME);

    let db = Database::connect(DATABASE_URL)
        .await
        .expect("Unable to connect to the database");

    migrate(&db)
        .await
        .expect("Something went wrong during migration");
}

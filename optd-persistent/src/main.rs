// main.rs

use sea_orm::{Database, DbErr};
use tokio;

// Change this to the path where you want your SQLite database file to be created
const DATABASE_URL: &str = "sqlite:./memory.db";

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let _db = Database::connect(DATABASE_URL).await?;

    // Use db here if needed

    Ok(())
}

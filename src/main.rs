use sea_orm::{Database};

mod entity;

use entity::*;

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    let db = &Database::connect("postgres://postgres:changeit@localhost/postgres").await?;

    db.get_schema_registry("transactions-check::*")
        .sync(db)
        .await?;

    println!("Hello, world!");
    Ok(())
}


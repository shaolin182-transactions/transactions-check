use sea_orm::{Database, entity::*};
use tr_entity::bank_account;
use service::csv_parser;

mod tr_entity;
mod service;
mod model;

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    let db = &Database::connect("postgres://postgres:changeit@localhost/postgres").await?;

    db.get_schema_registry("transactions-check::*")
        .sync(db)
        .await?;

    let bk = bank_account::ActiveModel {
        id: Set(2.to_owned()),
        category: Set("PEE".to_owned()),
        label: Set("CAP".to_owned())

    };

    //let bk: bank_account::Model = bk.insert(db).await?;

    csv_parser::parse_file_to_transaction("resources/data.csv".to_string());

    println!("Hello, world!");
    Ok(())
}


use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "transaction_details")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "transaction_id")]
    pub transaction_id: Uuid,
    pub income: f32,
    pub outcome: f32,
    pub cost: i32,
    pub cost_abs: i32,
    pub description: String,
    pub bank_account_id: Option<i32>,
    #[sea_orm(belongs_to, from = "bank_account_id", to = "id")]
    pub bank_account: HasOne<super::bank_account::Entity>,
    pub category_id : i32,
    #[sea_orm(belongs_to, from = "category_id", to = "id")]
    pub category: HasOne<super::category::Entity>,
    
}

impl ActiveModelBehavior for ActiveModel {}
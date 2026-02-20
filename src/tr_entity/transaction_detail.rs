use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "transaction_details")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub bank_account_id: i32,
    pub category_id: Option<i32>,
    pub income: Option<f32>,
    pub outcome: Option<f32>,
    pub cost: Option<i64>,
    pub cost_abs: Option<i64>,
    pub transaction_id: Option<Uuid>,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bank_account::Entity",
        from = "Column::BankAccountId",
        to = "super::bank_account::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    BankAccount,
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::transaction::Entity",
        from = "Column::TransactionId",
        to = "super::transaction::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Transaction,
}

impl Related<super::bank_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BankAccount.def()
    }
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transaction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

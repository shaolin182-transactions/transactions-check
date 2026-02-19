use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub cost: Option<i64>,
    pub cost_abs: Option<i64>,
    pub date: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::transaction_detail::Entity")]
    TransactionDetails,
}

impl Related<super::transaction_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransactionDetails.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

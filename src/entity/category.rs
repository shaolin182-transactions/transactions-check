use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub category: Option<String>,
    pub label: Option<String>,
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
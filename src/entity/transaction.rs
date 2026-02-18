use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub cost: i32,
    pub cost_abs: i32,
    pub date_transaction: DateTimeWithTimeZone,
    pub description: String,
    #[sea_orm(column_name = "type")]
    pub type_transaction: String,
    #[sea_orm(has_many)]
    pub details: HasMany<super::transaction_detail::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
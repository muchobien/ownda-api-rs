use async_graphql::{Enum, SimpleObject};
use sea_orm::{entity::prelude::*, IntoActiveValue, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "Transaction")]
#[graphql(name = "Transaction", complex)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount: Decimal,
    #[graphql(skip)]
    pub account_id: Uuid,
    #[graphql(skip)]
    pub category_id: Uuid,
    pub r#type: TransactionType,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Enum, Copy, Eq)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "transaction_type_enum"
)]
pub enum TransactionType {
    #[sea_orm(string_value = "INCOME")]
    Income,
    #[sea_orm(string_value = "EXPENSE")]
    Expense,
}

impl IntoActiveValue<Self> for TransactionType {
    fn into_active_value(self) -> sea_orm::ActiveValue<Self> {
        Set(self)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Account,
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Category,
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

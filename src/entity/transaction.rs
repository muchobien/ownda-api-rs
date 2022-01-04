//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0
use super::sea_orm_active_enums::TransactionTypeEnum;
use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

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
    pub to_account_id: Option<Uuid>,
    pub category_id: Uuid,
    pub r#type: TransactionTypeEnum,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
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
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::ToAccountId",
        to = "super::account::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    ToAccount,
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

impl Linked for Entity {
    type FromEntity = super::account::Entity;

    type ToEntity = super::transaction::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![Relation::Account.def(), Relation::ToAccount.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

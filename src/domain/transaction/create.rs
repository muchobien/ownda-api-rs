use crate::entity::{sea_orm_active_enums::TransactionTypeEnum, transaction::ActiveModel};
use async_graphql::InputObject;
use sea_orm::{
    prelude::{DateTimeWithTimeZone, Decimal},
    DeriveIntoActiveModel,
};
use uuid::Uuid;

#[derive(InputObject, DeriveIntoActiveModel)]
pub struct TransactionInput {
    pub name: String,
    pub amount: Decimal,
    pub account_id: Uuid,
    pub category_id: Uuid,
    pub r#type: TransactionTypeEnum,
}

#[derive(InputObject)]
pub struct TransactionFilter {
    pub account_id: Uuid,
    pub date: Option<DateFilter>,
}

#[derive(InputObject)]
pub struct DateFilter {
    pub begin: DateTimeWithTimeZone,
    pub end: DateTimeWithTimeZone,
}

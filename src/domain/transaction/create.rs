use crate::entity::transaction::{ActiveModel, TransactionType};
use async_graphql::InputObject;
use sea_orm::{prelude::Decimal, DeriveIntoActiveModel};
use uuid::Uuid;

#[derive(InputObject, DeriveIntoActiveModel)]
pub struct TransactionInput {
    pub name: String,
    pub amount: Decimal,
    pub account_id: Uuid,
    pub category_id: Uuid,
    pub r#type: TransactionType,
}

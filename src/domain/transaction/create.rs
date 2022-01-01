use crate::entity::transaction::{ActiveModel, TransactionType};
use async_graphql::InputObject;
use sea_orm::{DeriveIntoActiveModel, prelude::Decimal};
use uuid::Uuid;

#[derive(InputObject, DeriveIntoActiveModel)]
pub struct TransactionInput {
    pub name: String,
    pub amount: Decimal,
    pub account_id: Uuid,
    pub category_id: Uuid,
    pub r#type: TransactionType,
}

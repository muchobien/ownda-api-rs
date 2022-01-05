use crate::entity::account::ActiveModel;
use async_graphql::InputObject;
use sea_orm::DeriveIntoActiveModel;

#[derive(InputObject, DeriveIntoActiveModel)]
pub struct AccountInput {
    pub name: String,
    #[graphql(skip)]
    pub user_id: uuid::Uuid,
    pub color: String,
    pub order: i32,
}

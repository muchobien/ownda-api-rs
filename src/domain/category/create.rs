use crate::entity::category::ActiveModel;
use async_graphql::InputObject;
use sea_orm::DeriveIntoActiveModel;

#[derive(InputObject, DeriveIntoActiveModel)]
pub struct CategoryInput {
    pub name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub color: String,
    pub order: i32,
}

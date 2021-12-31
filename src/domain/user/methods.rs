use async_graphql::{ErrorExtensions, Result};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::domain::error::OwdaError;
use crate::entity::user;

pub async fn find_by_id(conn: &DatabaseConnection, id: uuid::Uuid) -> Result<user::Model> {
    user::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(OwdaError::NotFound.extend())
}

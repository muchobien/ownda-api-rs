use async_graphql::{ErrorExtensions, Result};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};

use crate::domain::error::OwdaError;
use crate::entity::{account, user};

pub async fn find_by_id(conn: &DatabaseConnection, id: uuid::Uuid) -> Result<user::Model> {
    user::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| OwdaError::NotFound.extend())
}

impl user::Model {
    pub async fn get_accounts(&self, conn: &DatabaseConnection) -> Result<Vec<account::Model>> {
        Ok(self.find_related(account::Entity).all(conn).await?)
    }
}

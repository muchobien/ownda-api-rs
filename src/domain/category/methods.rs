use async_graphql::{ErrorExtensions, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::domain::error::OwdaError;
use crate::entity::category;

pub async fn find_by_id(conn: &DatabaseConnection, id: uuid::Uuid) -> Result<category::Model> {
    category::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(OwdaError::NotFound.extend())
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<category::Model>> {
    Ok(category::Entity::find().all(conn).await?)
}

impl category::Model {
    pub async fn get_parent(&self, conn: &DatabaseConnection) -> Result<Option<category::Model>> {
        match self.parent_id {
            Some(parent_id) => Ok(Some(
                category::Entity::find_by_id(parent_id)
                    .one(conn)
                    .await?
                    .ok_or(OwdaError::NotFound.extend())?,
            )),
            None => Ok(None),
        }
    }

    pub async fn get_children(&self, conn: &DatabaseConnection) -> Result<Vec<category::Model>> {
        match self.parent_id {
            Some(parent_id) => Ok(category::Entity::find()
                .filter(category::Column::ParentId.eq(parent_id))
                .all(conn)
                .await?),
            None => Ok(vec![]),
        }
    }
}

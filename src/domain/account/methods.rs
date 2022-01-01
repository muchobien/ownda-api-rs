use async_graphql::{ErrorExtensions, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

use crate::domain::error::OwdaError;
use crate::entity::{account, transaction};

pub async fn find_by_id(conn: &DatabaseConnection, id: uuid::Uuid) -> Result<account::Model> {
    account::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(OwdaError::NotFound.extend())
}

pub async fn find_user_accounts(
    conn: &DatabaseConnection,
    user_id: uuid::Uuid,
) -> Result<Vec<account::Model>> {
    Ok(account::Entity::find()
        .filter(account::Column::UserId.eq(user_id))
        .all(conn)
        .await?)
}

impl account::Model {
    pub async fn get_transactions(
        &self,
        conn: &DatabaseConnection,
    ) -> Result<Vec<transaction::Model>> {
        Ok(self.find_related(transaction::Entity).all(conn).await?)
    }
}

use async_graphql::{ErrorExtensions, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::domain::error::OwdaError;
use crate::entity::transaction;

pub async fn find_by_id(conn: &DatabaseConnection, id: uuid::Uuid) -> Result<transaction::Model> {
    transaction::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(OwdaError::NotFound.extend())
}

pub async fn find_by_account_id(
    conn: &DatabaseConnection,
    account_id: uuid::Uuid,
) -> Result<Vec<transaction::Model>> {
    Ok(transaction::Entity::find()
        .filter(transaction::Column::AccountId.eq(account_id))
        .all(conn)
        .await?)
}

use async_graphql::{ErrorExtensions, Result};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

use crate::domain::error::OwdaError;
use crate::entity::{category, transaction};

use super::create::TransactionFilter;

pub async fn find_by_id(conn: &DatabaseConnection, id: uuid::Uuid) -> Result<transaction::Model> {
    transaction::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| OwdaError::NotFound.extend())
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

pub async fn filter_between(
    conn: &DatabaseConnection,
    begin: DateTimeWithTimeZone,
    end: DateTimeWithTimeZone,
) -> Result<Vec<transaction::Model>> {
    Ok(transaction::Entity::find()
        .filter(transaction::Column::CreatedAt.between(begin, end))
        .all(conn)
        .await?)
}

impl transaction::Model {
    pub async fn get_category(&self, conn: &DatabaseConnection) -> Result<category::Model> {
        Ok(self
            .find_related(category::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
}

impl TransactionFilter {
    pub async fn apply(&self, conn: &DatabaseConnection) -> Result<Vec<transaction::Model>> {
        Ok(transaction::Entity::find()
            .filter(
                Condition::all()
                    .add(transaction::Column::AccountId.eq(self.account_id))
                    .add_option(
                        self.date.as_ref().map(|date| {
                            transaction::Column::CreatedAt.between(date.begin, date.end)
                        }),
                    ),
            )
            .all(conn)
            .await?)
    }
}

use crate::domain::transaction::create::TransactionInput;
use crate::gql::guard::AuthGuard;
use crate::{domain::transaction::methods, entity::transaction, gql::context::GqlContext};
use async_graphql::{Context, Object, Result};
use sea_orm::{ActiveModelTrait, IntoActiveModel};

#[derive(Default)]
pub struct TransactionQuery;

#[Object]
impl TransactionQuery {
    #[graphql(guard = "AuthGuard")]
    async fn transaction(
        &self,
        raw_ctx: &Context<'_>,
        id: uuid::Uuid,
    ) -> Result<transaction::Model> {
        let ctx = GqlContext::new(raw_ctx);

        methods::find_by_id(ctx.conn, id).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn transactions(
        &self,
        raw_ctx: &Context<'_>,
        account_id: uuid::Uuid,
    ) -> Result<Vec<transaction::Model>> {
        let ctx = GqlContext::new(raw_ctx);

        methods::find_by_account_id(ctx.conn, account_id).await
    }
}

#[derive(Default)]
pub struct TransactionMutation;

#[Object]
impl TransactionMutation {
    #[graphql(guard = "AuthGuard")]
    async fn create_transaction(
        &self,
        raw_ctx: &Context<'_>,
        input: TransactionInput,
    ) -> Result<bool> {
        let ctx = GqlContext::new(raw_ctx);
        let am = input.into_active_model();
        am.save(ctx.conn).await?;

        Ok(true)
    }
}

use crate::domain::account::create::AccountInput;
use crate::gql::guard::AuthGuard;
use crate::{
    domain::account::methods,
    entity::{account, transaction},
    gql::context::GqlContext,
};
use async_graphql::{ComplexObject, Context, Object, Result};
use sea_orm::{ActiveModelTrait, IntoActiveModel};

#[ComplexObject]
impl account::Model {
    async fn transactions(&self, raw_ctx: &Context<'_>) -> Result<Vec<transaction::Model>> {
        let ctx = GqlContext::new(raw_ctx);

        self.get_transactions(ctx.conn).await
    }
}

#[derive(Default)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {
    #[graphql(guard = "AuthGuard")]
    async fn account(&self, raw_ctx: &Context<'_>, id: uuid::Uuid) -> Result<account::Model> {
        let ctx = GqlContext::new(raw_ctx);

        methods::find_by_id(ctx.conn, id).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn accounts(&self, raw_ctx: &Context<'_>) -> Result<Vec<account::Model>> {
        let ctx = GqlContext::new(raw_ctx);

        methods::find_user_accounts(ctx.conn, ctx.get_user_id()).await
    }
}

#[derive(Default)]
pub struct AccountMutation;

#[Object]
impl AccountMutation {
    #[graphql(guard = "AuthGuard")]
    async fn create_account(&self, raw_ctx: &Context<'_>, mut input: AccountInput) -> Result<bool> {
        let ctx = GqlContext::new(raw_ctx);
        input.user_id = ctx.get_user_id();
        let am = input.into_active_model();

        am.save(ctx.conn).await?;

        Ok(true)
    }
}

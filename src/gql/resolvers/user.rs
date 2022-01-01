use crate::{
    domain::user::{
        auth::{AuthInput, Authenticated},
        methods,
    },
    entity::{account, user},
    gql::{context::GqlContext, guard::AuthGuard},
};
use async_graphql::{ComplexObject, Context, Object, Result};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard = "AuthGuard")]
    async fn me(&self, raw_ctx: &Context<'_>) -> Result<user::Model> {
        let ctx = GqlContext::new(raw_ctx);

        methods::find_by_id(ctx.conn, ctx.get_user_id()).await
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register(&self, raw_ctx: &Context<'_>, input: AuthInput) -> Result<Authenticated> {
        let ctx = GqlContext::new(raw_ctx);

        input.register(ctx.conn, ctx.argon2).await
    }

    async fn login(&self, raw_ctx: &Context<'_>, input: AuthInput) -> Result<Authenticated> {
        let ctx = GqlContext::new(raw_ctx);

        input.login(ctx.conn, ctx.argon2).await
    }
}

#[ComplexObject]
impl user::Model {
    async fn accounts(&self, raw_ctx: &Context<'_>) -> Result<Vec<account::Model>> {
        let ctx = GqlContext::new(raw_ctx);

        self.get_accounts(ctx.conn).await
    }
}

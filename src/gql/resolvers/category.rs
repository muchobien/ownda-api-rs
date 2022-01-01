use crate::domain::category::create::CategoryInput;
use crate::gql::guard::AuthGuard;
use crate::{domain::category::methods, entity::category, gql::context::GqlContext};
use async_graphql::{ComplexObject, Context, Object, Result};
use sea_orm::{ActiveModelTrait, IntoActiveModel};

#[ComplexObject]
impl category::Model {
    async fn parent(&self, raw_ctx: &Context<'_>) -> Result<Option<category::Model>> {
        let ctx = GqlContext::new(raw_ctx);

        self.get_parent(ctx.conn).await
    }

    async fn children(&self, raw_ctx: &Context<'_>) -> Result<Vec<category::Model>> {
        let ctx = GqlContext::new(raw_ctx);

        self.get_children(ctx.conn).await
    }
}

#[derive(Default)]
pub struct CategoryQuery;

#[Object]
impl CategoryQuery {
    #[graphql(guard = "AuthGuard")]
    async fn category(&self, raw_ctx: &Context<'_>, id: uuid::Uuid) -> Result<category::Model> {
        let ctx = GqlContext::new(raw_ctx);

        methods::find_by_id(ctx.conn, id).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn categories(&self, raw_ctx: &Context<'_>) -> Result<Vec<category::Model>> {
        let ctx = GqlContext::new(raw_ctx);

        methods::find_all(ctx.conn).await
    }
}

#[derive(Default)]
pub struct CategoryMutation;

#[Object]
impl CategoryMutation {
    #[graphql(guard = "AuthGuard")]
    async fn create_category(&self, raw_ctx: &Context<'_>, input: CategoryInput) -> Result<bool> {
        let ctx = GqlContext::new(raw_ctx);
        let am = input.into_active_model();

        am.save(ctx.conn).await?;

        Ok(true)
    }
}

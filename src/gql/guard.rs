use async_graphql::{async_trait, Context, ErrorExtensions, Guard, Result};

use crate::domain::{error::OwdaError, jwt::Claims};

#[derive(Default)]
pub struct AuthGuard;

#[async_trait::async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        match ctx.data::<Option<Claims>>()? {
            Some(_) => Ok(()),
            None => Err(OwdaError::Unauthorized.extend()),
        }
    }
}

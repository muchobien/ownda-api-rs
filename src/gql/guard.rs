use async_graphql::{async_trait, Guard, Context, Result, ErrorExtensions};

use crate::domain::{jwt::Claims, error::OwdaError};

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
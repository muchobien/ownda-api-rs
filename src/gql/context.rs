use argon2::Argon2;
use async_graphql::Context;
use sea_orm::DatabaseConnection;

use crate::domain::jwt::Claims;

pub struct GqlContext<'a> {
    pub conn: &'a DatabaseConnection,
    pub argon2: &'a Argon2<'a>,
    pub claims: &'a Option<Claims>,
}

impl<'a> GqlContext<'a> {
    pub fn new(ctx: &Context<'a>) -> Self {
        let conn = ctx
            .data::<DatabaseConnection>()
            .expect("DB Pool not found in Context");
        let argon2 = ctx.data::<Argon2>().expect("Argon2 not found in Context");
        let claims = ctx
            .data::<Option<Claims>>()
            .expect("Claims not found in Context");

        Self {
            conn,
            argon2,
            claims,
        }
    }

    pub fn is_logged_in(&self) -> bool {
        self.claims.is_some()
    }

    pub fn get_user_id(&self) -> uuid::Uuid {
        self.claims
            .as_ref()
            .map(|c| uuid::Uuid::parse_str(&c.sub).unwrap())
            .unwrap()
    }
}

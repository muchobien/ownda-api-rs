use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use async_graphql::{ErrorExtensions, InputObject, Result, SimpleObject};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::domain::{error::OwdaError, jwt::generate_token};
use crate::entity::{
    identity::{self, Provider},
    user,
};

use super::methods::find_by_id;

#[derive(SimpleObject)]
pub struct Credential {
    pub access_token: String,
    pub token_type: String,
}

impl Credential {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(SimpleObject)]
pub struct Authenticated {
    pub user: user::Model,
    pub credentials: Credential,
}

#[derive(InputObject)]
pub struct AuthInput {
    #[graphql(validator(email))]
    email: String,
    provider: Provider,
    hash: String,
}

pub fn hash_password(argon2: &Argon2<'_>, password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| OwdaError::InternalServerError.extend())?
        .to_string())
}

pub fn verify_password(argon2: &Argon2<'_>, password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow::anyhow!(e))?;

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

impl AuthInput {
    pub async fn register(
        &self,
        conn: &DatabaseConnection,
        argon2: &Argon2<'_>,
    ) -> Result<Authenticated> {
        let res = user::Entity::insert(user::ActiveModel {
            email: Set(self.email.clone()),
            ..Default::default()
        })
        .exec(conn)
        .await?;

        let m = find_by_id(conn, res.last_insert_id).await?;

        let hash = match self.provider {
            Provider::Local => Set(hash_password(argon2, &self.hash)?),
            _ => Set(self.hash.clone()),
        };

        identity::Entity::insert(identity::ActiveModel {
            user_id: Set(m.id),
            provider: Set(self.provider),
            hash,
            ..Default::default()
        })
        .exec(conn)
        .await?;

        let authenticated = generate_token(&m.id)
            .map(|token| Credential::new(token))
            .map(|credentials| Authenticated {
                user: m,
                credentials,
            })
            .map_err(|_| OwdaError::InternalServerError.extend())?;

        Ok(authenticated)
    }

    pub async fn login(
        &self,
        conn: &DatabaseConnection,
        argon2: &Argon2<'_>,
    ) -> Result<Authenticated> {
        let u = user::Entity::find()
            .filter(user::Column::Email.eq(self.email.clone()))
            .one(conn)
            .await?
            .ok_or(OwdaError::NotFound.extend())?;

        let credentials = identity::Entity::find()
            .filter(
                Condition::all()
                    .add(identity::Column::UserId.eq(u.id))
                    .add(identity::Column::Provider.eq(self.provider)),
            )
            .one(conn)
            .await?
            .ok_or(OwdaError::NotFound.extend())?;

        match verify_password(argon2, &self.hash, &credentials.hash)? {
            true => Ok(generate_token(&u.id)
                .map(|token| Credential::new(token))
                .map(|credentials| Authenticated {
                    user: u,
                    credentials,
                })
                .map_err(|_| OwdaError::InternalServerError.extend())?),
            false => Err(OwdaError::Unauthorized.extend()),
        }
    }
}

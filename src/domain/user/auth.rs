use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use async_graphql::{ErrorExtensions, InputObject, Result, SimpleObject};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::entity::{identity, user};
use crate::{
    domain::{error::OwdaError, jwt::generate_token},
    entity::sea_orm_active_enums::ProviderEnum,
};

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
    provider: ProviderEnum,
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
        let m = user::ActiveModel {
            email: Set(self.email.clone()),
            ..Default::default()
        }
        .insert(conn)
        .await?;

        let hash = match self.provider {
            ProviderEnum::Local => Set(hash_password(argon2, &self.hash)?),
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
            .map(Credential::new)
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
            .ok_or_else(|| OwdaError::NotFound.extend())?;

        let credentials = identity::Entity::find()
            .filter(
                Condition::all()
                    .add(identity::Column::UserId.eq(u.id))
                    .add(identity::Column::Provider.eq(self.provider)),
            )
            .one(conn)
            .await?
            .ok_or_else(|| OwdaError::NotFound.extend())?;

        match verify_password(argon2, &self.hash, &credentials.hash)? {
            true => Ok(generate_token(&u.id)
                .map(Credential::new)
                .map(|credentials| Authenticated {
                    user: u,
                    credentials,
                })
                .map_err(|_| OwdaError::InternalServerError.extend())?),
            false => Err(OwdaError::Unauthorized.extend()),
        }
    }
}

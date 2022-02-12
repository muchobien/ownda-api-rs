use async_graphql::Result;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use std::time::SystemTime;

use crate::settings::SETTINGS;

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

struct Secret {
    pub access: Keys,
    pub refresh: Keys,
}

impl Secret {
    fn new(access: &[u8], refresh: &[u8]) -> Self {
        Self {
            access: Keys::new(access),
            refresh: Keys::new(refresh),
        }
    }
}

static KEYS: Lazy<Secret> = Lazy::new(|| {
    Secret::new(
        SETTINGS.secret.jwt.access.as_bytes(),
        SETTINGS.secret.jwt.refresh.as_bytes(),
    )
});

pub fn generate_token(user_id: &uuid::Uuid, days: u64, key: &EncodingKey) -> Result<String> {
    let claims = Claims {
        sub: user_id.to_string(),
        company: "OWNDA".to_owned(),
        exp: (SystemTime::now() + Duration::from_secs(60 * 60 * 24 * days))
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .try_into()
            .unwrap(),
    };

    Ok(jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        key,
    )?)
}

pub fn generate_tokens(user_id: &uuid::Uuid) -> Result<(String, String)> {
    let access_token = generate_token(user_id, 7, &KEYS.access.encoding)?;
    let refresh_token = generate_token(user_id, 30, &KEYS.refresh.encoding)?;

    Ok((access_token, refresh_token))
}

pub fn user_id_from_refresh_token(refresh_token: &str) -> Result<uuid::Uuid> {
    let claims = decode::<Claims>(
        refresh_token,
        &KEYS.refresh.decoding,
        &Validation::default(),
    )?;

    Ok(uuid::Uuid::parse_str(&claims.claims.sub)?)
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &KEYS.access.decoding,
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

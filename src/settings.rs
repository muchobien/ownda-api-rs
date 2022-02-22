use once_cell::sync::Lazy;
use sea_orm::ConnectOptions;
use serde::Deserialize;
use std::net::SocketAddr;

pub static SETTINGS: Lazy<Settings> =
    Lazy::new(|| Settings::new().expect("Failed to load settings"));

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: PostgresSettings,
    pub secret: SecretSettings,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::Environment::with_prefix("OWNDA").separator("_"))
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: String,
    pub host: String,
    pub graphql: GraphQlSettings,
}

impl ApplicationSettings {
    pub fn get_address(&self) -> anyhow::Result<SocketAddr> {
        Ok(format!("{}:{}", self.host, self.port).parse()?)
    }
}

#[derive(Deserialize)]
pub struct GraphQlSettings {
    pub path: String,
    pub playground: bool,
}

#[derive(Deserialize)]
pub struct PostgresSettings {
    pub username: String,
    pub password: String,
    pub port: String,
    pub host: String,
    pub name: String,
    pub logging: Option<bool>,
}

impl PostgresSettings {
    pub fn get_connect_options(&self) -> ConnectOptions {
        let mut opt = ConnectOptions::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            &self.username, &self.password, &self.host, &self.port, &self.name
        ));

        opt.sqlx_logging(self.logging.unwrap_or(false));

        opt
    }
}

#[derive(Deserialize)]
pub struct SecretSettings {
    pub jwt: JWTSettings,
}

#[derive(Deserialize)]
pub struct JWTSettings {
    pub access: String,
    pub refresh: String,
}

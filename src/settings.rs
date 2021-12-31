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
        let mut settings = config::Config::default();

        let base_path = std::env::current_dir()?;
        let config_dir = base_path.join("config");

        settings.merge(config::File::from(config_dir.join("default")).required(true))?;
        let environment: Environment = std::env::var("OWNDA_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to get OWNDA_ENVIRONMENT");

        settings.merge(config::File::from(config_dir.join(environment.as_str())).required(true))?;

        settings.merge(config::Environment::with_prefix("OWNDA_").separator("__"))?;
        Ok(settings.try_into()?)
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
    pub playground_enabled: bool,
}

#[derive(Deserialize)]
pub struct PostgresSettings {
    pub username: String,
    pub password: String,
    pub port: String,
    pub host: String,
    pub database_name: String,
    pub logging: bool,
}

impl PostgresSettings {
    pub fn get_connect_options(&self) -> ConnectOptions {
        let mut opt = ConnectOptions::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            &self.username, &self.password, &self.host, &self.port, &self.database_name
        ));

        opt.sqlx_logging(self.logging);

        opt
    }
}

pub enum Environment {
    Local,
    Production,
    Docker,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
            Environment::Docker => "docker",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            "docker" => Ok(Self::Docker),
            other => Err(format!(
                "{} is not a supported environment. Use either 'local' or 'production'",
                other
            )),
        }
    }
}

#[derive(Deserialize)]
pub struct SecretSettings {
    pub jwt_key: String,
}

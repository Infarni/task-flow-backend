pub mod auth;
pub mod postgres;
pub mod server;

use auth::AuthConfig;
use postgres::PostgresConfig;
use serde::Deserialize;
use server::ServerConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub auth: AuthConfig,
    pub postgres: PostgresConfig,
    pub server: ServerConfig,
}

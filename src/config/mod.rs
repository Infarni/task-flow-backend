pub mod postgres;
pub mod server;

use postgres::PostgresConfig;
use serde::Deserialize;
use server::ServerConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub postgres: PostgresConfig,
    pub server: ServerConfig,
}

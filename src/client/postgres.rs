use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::{
    config::Config,
    error::client::{ClientError, ClientResult},
};

use super::ClientBuilder;

pub type PostgresClient = DatabaseConnection;

#[async_trait::async_trait]
pub trait PostgresClientExt {
    // async fn create(tx: &DatabaseTransaction, record: impl Into<Mode)
}

#[async_trait::async_trait]
impl ClientBuilder for PostgresClient {
    async fn from_config(config: &Config) -> ClientResult<Self> {
        let mut options: ConnectOptions = ConnectOptions::new(config.postgres.get_url());
        options
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        match Database::connect(options).await {
            Ok(value) => Ok(value),
            Err(_) => Err(ClientError::Postgres),
        }
    }
}

impl PostgresClientExt for PostgresClient {}

pub mod postgres;

use crate::{config::Config, error::client::ClientResult};

#[async_trait::async_trait]
pub trait ClientBuilder: Sized {
    async fn from_config(config: &Config) -> ClientResult<Self>;
}

use thiserror::Error;

use super::client::ClientError;

pub type ServerResult<T = ()> = Result<T, ServerError>;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Create client error: {0}")]
    ClientCreate(#[from] ClientError),

    #[error("Can't run server: {0}")]
    Run(String),
}

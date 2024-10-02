use thiserror::Error;

pub type ClientResult<T = ()> = Result<T, ClientError>;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Error creating postgres client")]
    Postgres,
}

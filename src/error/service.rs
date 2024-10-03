use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use garde::Report;
use sea_orm::DbErr;
use thiserror::Error;
use uuid::Uuid;

use crate::dto::error::{ErrorDto, FromReport, ValidateErrorDto};

pub type ServiceResult<T = ()> = Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Record with {field}={value} already exists error")]
    Conflict { field: String, value: String },

    #[error("Record with id={0} not found error")]
    NotFound(Uuid),

    #[error(transparent)]
    Validation(#[from] Report),

    #[error("Error with hashing value")]
    Hash,

    #[error("Unknow db error")]
    Unknow(#[from] DbErr),
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::Conflict { field: _, value: _ } => StatusCode::CONFLICT,
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            ServiceError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let status_code: StatusCode = self.status_code();

        match self {
            Self::Validation(err) => {
                HttpResponse::build(status_code).json(ValidateErrorDto::from_report(err))
            }
            _ => HttpResponse::build(status_code).json(ErrorDto {
                detail: self.to_string(),
            }),
        }
    }
}

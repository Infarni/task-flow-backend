use garde::Report;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorDto {
    #[schema(example = "Record with email=archdroider@proton.me already exists")]
    pub detail: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ValidateItemErrorDto {
    #[schema(example = "email")]
    pub field: String,

    #[schema(example = "not a valid email: value is missing `@`")]
    pub message: String,
}

pub type ValidateErrorDto = Vec<ValidateItemErrorDto>;

pub trait FromReport {
    fn from_report(value: &Report) -> Self;
}

impl FromReport for ValidateErrorDto {
    fn from_report(value: &Report) -> Self {
        value
            .iter()
            .map(|(p, e)| ValidateItemErrorDto {
                field: p.to_string(),
                message: e.to_string(),
            })
            .collect::<Self>()
    }
}

use garde::rules::pattern::regex::Regex;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::constants;

#[derive(Debug, Serialize)]
pub struct ClaimsDto {
    pub sub: Uuid,
    pub exp: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SignInDto {
    #[garde(pattern(Regex::new(constants::NAME_PATTERN).unwrap()), length(min = constants::NAME_MIN_LENGTH, max = constants::NAME_MAX_LENGTH))]
    #[schema(example = "archdrdr")]
    pub login: String,

    #[garde(length(min = constants::PASSWORD_MIN_LENGTH, max = constants::PASSWORD_MAX_LENGTH))]
    #[schema(example = "some_password12345")]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenDto {
    pub token: String,
}

use std::future::{ready, Ready};

use actix_web::{web, FromRequest};
use garde::rules::pattern::regex::Regex;
use garde::Validate;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    constants,
    error::service::{ServiceError, ServiceResult},
    server::State,
};

#[derive(Debug, Serialize, Deserialize)]
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

impl FromRequest for ClaimsDto {
    type Error = ServiceError;
    type Future = Ready<ServiceResult<Self>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let header = req.headers().get("Authorization");
        let state: Option<&web::Data<State>> = req.app_data::<web::Data<State>>();

        match state {
            Some(state) => match header {
                Some(value) => match value.to_str() {
                    Ok(value) => {
                        let decoding_key =
                            DecodingKey::from_secret(state.config.auth.secret.clone().as_ref());
                        let validation = Validation::default();

                        let token = value.split("Bearer ").last().unwrap();

                        match decode::<ClaimsDto>(token, &decoding_key, &validation) {
                            Ok(token_data) => ready(Ok(token_data.claims)),
                            Err(_) => ready(Err(ServiceError::InvalidCredentials(
                                "Invalid token".to_string(),
                            ))),
                        }
                    }
                    Err(_) => ready(Err(ServiceError::InvalidCredentials(
                        "Token is missing".to_string(),
                    ))),
                },
                None => ready(Err(ServiceError::InvalidCredentials(
                    "Missing 'Authorization' header".to_string(),
                ))),
            },
            None => ready(Err(ServiceError::Unknow(
                "Internal server error".to_string(),
            ))),
        }
    }
}

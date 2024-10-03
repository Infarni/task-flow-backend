use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    dto::auth::{ClaimsDto, SignInDto, TokenDto},
    error::service::{ServiceError, ServiceResult},
};

use super::{common::verify_hash, user::UserService};

pub struct AuthService;

impl AuthService {
    fn gen_token(id: Uuid, expire: u64, secret: String) -> ServiceResult<String> {
        let expiration = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(value) => value.as_secs() + expire,
            Err(_) => return Err(ServiceError::Token),
        };

        let claims: ClaimsDto = ClaimsDto {
            sub: id,
            exp: expiration,
        };

        match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        ) {
            Ok(value) => Ok(value),
            Err(_) => Err(ServiceError::Token),
        }
    }

    pub async fn sign_in(
        db: &DatabaseConnection,
        credentials: SignInDto,
        expire: u64,
        secret: String,
    ) -> ServiceResult<TokenDto> {
        let (id, hashed_password) = UserService::get_by_login(&db, credentials.login).await?;

        if verify_hash(credentials.password, hashed_password)? {
            Ok(TokenDto {
                token: Self::gen_token(id, expire, secret)?,
            })
        } else {
            Err(ServiceError::InvalidCredentials)
        }
    }
}

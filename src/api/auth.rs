use actix_web::{
    post,
    web::{self, Json},
    Scope,
};
use garde::Validate;

use crate::{
    dto::auth::{SignInDto, TokenDto},
    error::service::ServiceResult,
    server::State,
    service::auth::AuthService,
};

#[utoipa::path(
    path = "/auth/sign_in",
    request_body = SignInDto,
    responses(
        (status = 201, body = TokenDto),
        (status = 401, body = ErrorDto),
        (status = 422, body = ErrorDto)
    )
)]
#[post("/sign_in")]
pub async fn sign_in_handler(
    state: web::Data<State>,
    body: web::Json<SignInDto>,
) -> ServiceResult<Json<TokenDto>> {
    body.validate()?;

    Ok(Json(
        AuthService::sign_in(
            &state.postgres,
            body.into_inner(),
            state.config.auth.expire,
            state.config.auth.secret.clone(),
        )
        .await?,
    ))
}

pub fn get_scope() -> Scope {
    web::scope("/auth").service(sign_in_handler)
}

use actix_web::{
    post,
    web::{self, Json},
    Scope,
};
use garde::Validate;

use crate::{
    dto::user::{UserCreateDto, UserReadDto},
    error::service::ServiceResult,
    server::State,
    service::user::UserService,
};

#[utoipa::path(
    path = "/user",
    request_body = UserCreateDto,
    responses(
        (status = 201, body = UserReadDto),
        (status = 409, body = ErrorDto),
        (status = 422, body = [ValidateItemErrorDto])
    )
)]
#[post("")]
pub async fn create_user_handler(
    state: web::Data<State>,
    body: web::Json<UserCreateDto>,
) -> ServiceResult<Json<UserReadDto>> {
    body.validate()?;

    Ok(Json(
        UserService::create(&state.postgres, body.into_inner()).await?,
    ))
}

pub fn get_scope() -> Scope {
    web::scope("/user").service(create_user_handler)
}

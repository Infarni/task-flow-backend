use actix_web::{
    get, post,
    web::{self, Json},
    Scope,
};
use garde::Validate;
use uuid::Uuid;

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

#[utoipa::path(
    path = "/user/{id}",
    responses(
        (status = 200, body = UserReadDto),
        (status = 404, body = ErrorDto),
    )
)]
#[get("/{id}")]
pub async fn get_user_handler(
    state: web::Data<State>,
    params: web::Path<Uuid>,
) -> ServiceResult<Json<UserReadDto>> {
    let id: Uuid = params.into_inner();

    Ok(Json(UserService::get_by_id(&state.postgres, id).await?))
}

pub fn get_scope() -> Scope {
    web::scope("/user")
        .service(create_user_handler)
        .service(get_user_handler)
}

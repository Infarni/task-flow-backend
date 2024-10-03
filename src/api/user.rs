use actix_web::{
    get, post,
    web::{self, Json},
    Scope,
};
use garde::Validate;
use uuid::Uuid;

use crate::{
    dto::user::{UserCreateDto, UserReadDto, UserSearchQuery},
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
    path: web::Path<Uuid>,
) -> ServiceResult<Json<UserReadDto>> {
    let id: Uuid = path.into_inner();

    Ok(Json(UserService::get_by_id(&state.postgres, id).await?))
}

#[utoipa::path(
    path = "/user",
    responses(
        (status = 200, body = [UserReadDto])
    ),
    params(
        ("name" = String, Query, description = "User name"),
        ("limit" = u64, Query, description = "Limit of users"),
        ("offset" = u64, Query, description = "Offset of users"),
    ),
)]
#[get("")]
pub async fn search_user_handler(
    state: web::Data<State>,
    query: web::Query<UserSearchQuery>,
) -> ServiceResult<Json<Vec<UserReadDto>>> {
    let name: String = query.name.clone();
    let limit: u64 = query.limit;
    let offset: u64 = query.offset;

    Ok(Json(
        UserService::search_by_name(&state.postgres, name, limit, offset).await?,
    ))
}

pub fn get_scope() -> Scope {
    web::scope("/user")
        .service(create_user_handler)
        .service(get_user_handler)
        .service(search_user_handler)
}

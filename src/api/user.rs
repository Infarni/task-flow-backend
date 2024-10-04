use actix_web::{delete, get, patch, post, web, HttpResponse, Scope};
use garde::Validate;
use uuid::Uuid;

use crate::{
    dto::{
        auth::ClaimsDto,
        user::{UserCreateDto, UserSearchQuery, UserUpdateDto},
    },
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
    ),
    security()
)]
#[post("")]
pub async fn create_user_handler(
    state: web::Data<State>,
    body: web::Json<UserCreateDto>,
) -> ServiceResult<HttpResponse> {
    body.validate()?;

    Ok(
        HttpResponse::Created()
            .json(UserService::create(&state.postgres, body.into_inner()).await?),
    )
}

#[utoipa::path(
    path = "/user/me",
    responses(
        (status = 200, body = UserReadDto),
        (status = 404, body = ErrorDto),
    ),
)]
#[get("/me")]
pub async fn get_user_handler(
    state: web::Data<State>,
    claims: ClaimsDto,
) -> ServiceResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(UserService::get_by_id(&state.postgres, claims.sub).await?))
}

#[utoipa::path(
    path = "/user/{id}",
    responses(
        (status = 200, body = UserReadDto),
        (status = 404, body = ErrorDto),
    ),
)]
#[get("/{id}")]
pub async fn get_user_by_id_handler(
    state: web::Data<State>,
    path: web::Path<Uuid>,
    _: ClaimsDto,
) -> ServiceResult<HttpResponse> {
    let id = path.into_inner();

    Ok(HttpResponse::Ok().json(UserService::get_by_id(&state.postgres, id).await?))
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
) -> ServiceResult<HttpResponse> {
    let name: String = query.name.clone();
    let limit: u64 = query.limit;
    let offset: u64 = query.offset;

    Ok(HttpResponse::Ok()
        .json(UserService::search_by_name(&state.postgres, name, limit, offset).await?))
}

#[utoipa::path(
    path = "/user/me",
    request_body = UserUpdateDto,
    responses(
        (status = 200, body = UserReadDto),
        (status = 404, body = ErrorDto),
        (status = 409, body = ErrorDto),
        (status = 422, body = [ValidateItemErrorDto])
    )
)]
#[patch("/me")]
pub async fn update_user_handler(
    state: web::Data<State>,
    body: web::Json<UserUpdateDto>,
    claims: ClaimsDto,
) -> ServiceResult<HttpResponse> {
    body.validate()?;

    Ok(HttpResponse::Ok()
        .json(UserService::update(&state.postgres, claims.sub, body.into_inner()).await?))
}

#[utoipa::path(
    path = "/user/me",
    responses(
        (status = 204),
        (status = 404, body = ErrorDto)
    )
)]
#[delete("/me")]
pub async fn delete_user_handler(
    state: web::Data<State>,
    claims: ClaimsDto,
) -> ServiceResult<HttpResponse> {
    UserService::delete(&state.postgres, claims.sub).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn get_scope() -> Scope {
    web::scope("/user")
        .service(create_user_handler)
        .service(get_user_handler)
        .service(get_user_by_id_handler)
        .service(search_user_handler)
        .service(update_user_handler)
        .service(delete_user_handler)
}

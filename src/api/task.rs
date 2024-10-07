use actix_web::{get, patch, post, web, HttpResponse, Scope};
use garde::Validate;
use uuid::Uuid;

use crate::{
    dto::{
        auth::ClaimsDto,
        task::{TaskCreateDto, TaskGetQuery, TaskUpdateDto},
    },
    error::service::ServiceResult,
    server::State,
    service::task::TaskService,
};

#[utoipa::path(
    path = "/task",
    request_body = TaskCreateDto,
    responses(
        (status = 201, body = TaskReadDto),
        (status = 422, body = [ValidateItemErrorDto])
    )
)]
#[post("")]
pub async fn create_task_handler(
    state: web::Data<State>,
    claims: ClaimsDto,
    body: web::Json<TaskCreateDto>,
) -> ServiceResult<HttpResponse> {
    body.validate()?;

    Ok(HttpResponse::Created()
        .json(TaskService::create(&state.postgres, claims.sub, body.into_inner()).await?))
}

#[utoipa::path(
    path = "/task/me",
    params(
        ("limit" = u64, Query, description = "Limit of users"),
        ("offset" = u64, Query, description = "Offset of users"),
        ("status" = Option<TaskStatus>, Query, description = "Task status"),
    ),
    responses(
        (status = 200, body = [TaskReadDto])
    )
)]
#[get("/me")]
pub async fn get_task_handler(
    state: web::Data<State>,
    claims: ClaimsDto,
    query: web::Query<TaskGetQuery>,
) -> ServiceResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(
        TaskService::get(
            &state.postgres,
            claims.sub,
            query.limit,
            query.offset,
            query.status.clone(),
        )
        .await?,
    ))
}

#[utoipa::path(
    path = "/task/{id}",
    request_body = TaskUpdateDto,
    responses(
        (status = 200, body = TaskReadDto),
        (status = 404, body = ErrorDto),
        (status = 422, body = [ValidateItemErrorDto]),
    )
)]
#[patch("/{id}")]
pub async fn update_task_handler(
    state: web::Data<State>,
    claims: ClaimsDto,
    path: web::Path<Uuid>,
    body: web::Json<TaskUpdateDto>,
) -> ServiceResult<HttpResponse> {
    body.validate()?;

    let id: Uuid = path.into_inner();

    Ok(HttpResponse::Ok()
        .json(TaskService::update(&state.postgres, claims.sub, id, body.into_inner()).await?))
}

pub fn get_scope() -> Scope {
    web::scope("/task")
        .service(create_task_handler)
        .service(get_task_handler)
        .service(update_task_handler)
}

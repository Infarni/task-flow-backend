use actix_web::{delete, get, patch, post, web, HttpResponse, Scope};
use garde::Validate;
use uuid::Uuid;

use crate::{
    dto::{
        auth::ClaimsDto,
        task::{
            TaskCommentCreateDto, TaskCommentGetQuery, TaskCommentUpdateDto, TaskCreateDto,
            TaskGetQuery, TaskUpdateDto,
        },
    },
    error::service::ServiceResult,
    server::State,
    service::{task::TaskService, task_comment::TaskCommentService},
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
    path = "/task/{id}/comment",
    request_body = TaskCommentCreateDto,
    responses(
        (status = 201, body = TaskCommentReadDto),
        (status = 403, body = ErrorDto),
        (status = 404, body = ErrorDto),
        (status = 422, body = [ValidateItemErrorDto])
    )
)]
#[post("/{id}/comment")]
pub async fn create_task_comment_handler(
    state: web::Data<State>,
    claims: ClaimsDto,
    path: web::Path<Uuid>,
    body: web::Json<TaskCommentCreateDto>,
) -> ServiceResult<HttpResponse> {
    body.validate()?;

    let task_id: Uuid = path.into_inner();

    Ok(HttpResponse::Created().json(
        TaskCommentService::create(&state.postgres, claims.sub, task_id, body.into_inner()).await?,
    ))
}

#[utoipa::path(
    path = "/task/me",
    params(
        ("limit" = u64, Query, description = "Limit of tasks"),
        ("offset" = u64, Query, description = "Offset of tasks"),
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
        TaskService::list(
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
    path = "/task/{id}/comment",
    params(
        ("limit" = u64, Query, description = "Limit of comments"),
        ("offset" = u64, Query, description = "Offset of comments"),
    ),
    responses(
        (status = 200, body = [TaskCommentReadDto]),
        (status = 403, body = ErrorDto),
        (status = 404, body = ErrorDto),
    )
)]
#[get("/{id}/comment")]
pub async fn get_task_comment_handler(
    state: web::Data<State>,
    claims: ClaimsDto,
    path: web::Path<Uuid>,
    query: web::Query<TaskCommentGetQuery>,
) -> ServiceResult<HttpResponse> {
    let task_id: Uuid = path.into_inner();

    Ok(HttpResponse::Ok().json(
        TaskCommentService::list(
            &state.postgres,
            claims.sub,
            task_id,
            query.limit,
            query.offset,
        )
        .await?,
    ))
}

#[utoipa::path(
    path = "/task/{id}",
    request_body = TaskUpdateDto,
    responses(
        (status = 200, body = TaskReadDto),
        (status = 403, body = ErrorDto),
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

#[utoipa::path(
    path = "/task/{task_id}/comment/{id}",
    request_body = TaskCommentUpdateDto,
    responses(
        (status = 200, body = TaskCommentReadDto),
        (status = 403, body = ErrorDto),
        (status = 404, body = ErrorDto),
        (status = 422, body = [ValidateItemErrorDto])
    )
)]
#[patch("/{task_id}/comment/{id}")]
pub async fn update_task_comment_handler(
    state: web::Data<State>,
    claims: ClaimsDto,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<TaskCommentUpdateDto>,
) -> ServiceResult<HttpResponse> {
    body.validate()?;

    let (task_id, id) = path.into_inner();

    Ok(HttpResponse::Ok().json(
        TaskCommentService::update(&state.postgres, claims.sub, task_id, id, body.into_inner())
            .await?,
    ))
}

#[utoipa::path(
    path = "/task/{id}",
    responses(
        (status = 204),
        (status = 403, body = ErrorDto),
        (status = 404, body = ErrorDto)
    )
)]
#[delete("/{id}")]
pub async fn delete_task_handler(
    state: web::Data<State>,
    claims: ClaimsDto,
    path: web::Path<Uuid>,
) -> ServiceResult<HttpResponse> {
    let id: Uuid = path.into_inner();

    TaskService::delete(&state.postgres, claims.sub, id).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    path = "/task/{task_id}/comment/{id}",
    responses(
        (status = 204),
        (status = 403, body = ErrorDto),
        (status = 404, body = ErrorDto)
    )
)]
#[delete("/{task_id}/comment/{id}")]
pub async fn delete_task_comment_handler(
    state: web::Data<State>,
    claims: ClaimsDto,
    path: web::Path<(Uuid, Uuid)>,
) -> ServiceResult<HttpResponse> {
    let (task_id, id) = path.into_inner();

    TaskCommentService::delete(&state.postgres, claims.sub, task_id, id).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn get_scope() -> Scope {
    web::scope("/task")
        .service(create_task_handler)
        .service(get_task_handler)
        .service(update_task_handler)
        .service(delete_task_handler)
        .service(create_task_comment_handler)
        .service(get_task_comment_handler)
        .service(update_task_comment_handler)
        .service(delete_task_comment_handler)
}

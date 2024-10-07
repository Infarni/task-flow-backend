use actix_web::{post, web, HttpResponse, Scope};

use crate::{
    dto::{auth::ClaimsDto, task::TaskCreateDto},
    error::service::ServiceResult,
    server::State,
    service::task::TaskService,
};

#[utoipa::path(
    path = "/task",
    request_body = TaskCreateDto,
    responses(
        (status = 201, body = TaskReadDto)
    )
)]
#[post("")]
pub async fn create_task_handler(
    state: web::Data<State>,
    claims: ClaimsDto,
    body: web::Json<TaskCreateDto>,
) -> ServiceResult<HttpResponse> {
    Ok(HttpResponse::Created()
        .json(TaskService::create(&state.postgres, claims.sub, body.into_inner()).await?))
}

pub fn get_scope() -> Scope {
    web::scope("/task").service(create_task_handler)
}

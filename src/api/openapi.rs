use actix_web::{web, Scope};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::dto::{
    error::{ErrorDto, ValidateItemErrorDto},
    user::{UserCreateDto, UserReadDto},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::user::create_user_handler,
        crate::api::user::get_user_handler
    ),
    components(schemas(UserCreateDto, UserReadDto, ErrorDto, ValidateItemErrorDto))
)]
pub struct ApiDoc;

pub fn get_scope() -> Scope {
    web::scope("")
        .service(SwaggerUi::new("/docs/{_:.*}").url("/docs/openapi.json", ApiDoc::openapi()))
}

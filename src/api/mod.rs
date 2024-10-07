use actix_web::web::ServiceConfig;
use openapi::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod openapi;
pub mod task;
pub mod user;

pub fn service_configure(config: &mut ServiceConfig) {
    config
        .service(user::get_scope())
        .service(auth::get_scope())
        .service(task::get_scope())
        .service(SwaggerUi::new("/docs/{_:.*}").url("/docs/openapi.json", ApiDoc::openapi()));
}

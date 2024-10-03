use actix_web::{web, Scope};

pub mod openapi;
pub mod user;

pub fn service_configure() -> Scope {
    web::scope("")
        .service(user::get_scope())
        .service(openapi::get_scope())
}

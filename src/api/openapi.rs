use utoipa::{
    openapi::{
        self,
        security::{HttpBuilder, SecurityScheme},
    },
    Modify, OpenApi,
};

use crate::dto::{
    auth::{SignInDto, TokenDto},
    error::{ErrorDto, ValidateItemErrorDto},
    user::{UserCreateDto, UserReadDto, UserUpdateDto},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::user::create_user_handler,
        crate::api::user::get_user_handler,
        crate::api::user::search_user_handler,
        crate::api::user::update_user_handler,
        crate::api::user::delete_user_handler,
        crate::api::auth::sign_in_handler
    ),
    components(schemas(
        UserCreateDto,
        UserReadDto,
        UserUpdateDto,
        ErrorDto,
        ValidateItemErrorDto,
        SignInDto,
        TokenDto
    )),
    modifiers(&BearerAuth)
)]
pub struct ApiDoc;

pub struct BearerAuth;
impl Modify for BearerAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(openapi::security::HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

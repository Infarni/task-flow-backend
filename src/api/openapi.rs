use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::{
    dto::{
        auth::{SignInDto, TokenDto},
        error::{ErrorDto, ValidateItemErrorDto},
        task::{TaskCreateDto, TaskGetQuery, TaskReadDto, TaskUpdateDto},
        user::{UserAvatarUploadDto, UserCreateDto, UserReadDto, UserUpdateDto},
    },
    entity::sea_orm_active_enums::TaskStatus,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::user::create_user_handler,
        crate::api::user::get_user_handler,
        crate::api::user::get_user_by_id_handler,
        crate::api::user::search_user_handler,
        crate::api::user::update_user_handler,
        crate::api::user::delete_user_handler,
        crate::api::auth::sign_in_handler,
        crate::api::user::create_avatar_handler,
        crate::api::user::get_avatar_handler,
        crate::api::user::delete_avatar_handler,
        crate::api::user::get_avatar_by_user_id_handler,
        crate::api::task::create_task_handler,
        crate::api::task::get_task_handler,
        crate::api::task::update_task_handler,
    ),
    components(schemas(
        UserCreateDto,
        UserReadDto,
        UserUpdateDto,
        ErrorDto,
        ValidateItemErrorDto,
        SignInDto,
        TokenDto,
        UserAvatarUploadDto,
        TaskReadDto,
        TaskCreateDto,
        TaskStatus,
        TaskGetQuery,
        TaskUpdateDto,
    )),
    security(("JWT token" = [])),
    modifiers(&BearerAuth)
)]
pub struct ApiDoc;

pub struct BearerAuth;
impl Modify for BearerAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();

        components.add_security_scheme(
            "JWT token",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                "Authorization",
                "Access token",
            ))),
        );
    }
}

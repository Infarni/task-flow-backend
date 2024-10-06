use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use chrono::Local;
use garde::rules::pattern::regex::Regex;
use garde::Validate;
use sea_orm::{IntoActiveModel, NotSet, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::constants;
use crate::entity::prelude::{UserActiveModel, UserModel};

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UserCreateDto {
    #[garde(pattern(Regex::new(constants::NAME_PATTERN).unwrap()), length(min = constants::NAME_MIN_LENGTH, max = constants::NAME_MAX_LENGTH))]
    #[schema(example = "archdrdr")]
    pub name: String,

    #[garde(email)]
    #[schema(example = "archdroider@proton.me")]
    pub email: String,

    #[garde(length(min = constants::PASSWORD_MIN_LENGTH, max = constants::PASSWORD_MAX_LENGTH))]
    #[schema(example = "some_password12345")]
    pub password: String,
}

#[derive(Debug, MultipartForm, ToSchema)]
pub struct UserAvatarUploadDto {
    #[schema(value_type = String, format = Binary)]
    pub image: TempFile,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserReadDto {
    #[schema(example = "00000000-0000-0000-0000-000000000000")]
    pub id: Uuid,

    #[schema(example = "archdrdr")]
    pub name: String,

    #[schema(example = "archdroider@proton.me")]
    pub email: String,

    #[schema(example = "2024-05-15T15:36:21.434500+03:00")]
    pub created_at: String,

    #[schema(example = "2024-05-15T15:36:21.434500+03:00")]
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserSearchQuery {
    pub name: String,
    pub limit: u64,
    pub offset: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UserUpdateDto {
    #[garde(pattern(Regex::new(constants::NAME_PATTERN).unwrap()), length(min = constants::NAME_MIN_LENGTH, max = constants::NAME_MAX_LENGTH))]
    #[schema(example = "archdrdr")]
    pub name: Option<String>,

    #[garde(email)]
    #[schema(example = "archdroider@proton.me")]
    pub email: Option<String>,

    #[garde(length(min = constants::PASSWORD_MIN_LENGTH, max = constants::PASSWORD_MAX_LENGTH))]
    #[schema(example = "some_password12345")]
    pub password: Option<String>,
}

impl IntoActiveModel<UserActiveModel> for UserCreateDto {
    fn into_active_model(self) -> UserActiveModel {
        UserActiveModel {
            name: Set(self.name),
            email: Set(self.email),
            password: Set(self.password),
            ..Default::default()
        }
    }
}

impl From<UserModel> for UserReadDto {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
        }
    }
}

impl IntoActiveModel<UserActiveModel> for UserUpdateDto {
    fn into_active_model(self) -> UserActiveModel {
        UserActiveModel {
            name: match self.name {
                Some(name) => Set(name),
                None => NotSet,
            },
            email: match self.email {
                Some(email) => Set(email),
                None => NotSet,
            },
            password: match self.password {
                Some(password) => Set(password),
                None => NotSet,
            },
            updated_at: Set(Local::now().fixed_offset()),
            ..Default::default()
        }
    }
}

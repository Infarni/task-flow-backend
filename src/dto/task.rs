use chrono::{DateTime, Local};
use garde::Validate;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{IntoActiveModel, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::constants;
use crate::entity::prelude::{
    TaskActiveModel, TaskCommentActiveModel, TaskCommentModel, TaskModel,
};
use crate::entity::sea_orm_active_enums::TaskStatus;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TaskCreateDto {
    #[garde(length(min = constants::TASK_NAME_MIN_LENGTH, max = constants::TASK_NAME_MAX_LENGTH))]
    #[schema(example = "Implement auth")]
    pub name: String,

    #[garde(length(min = constants::TASK_DESCRIPTION_MIN_LENGTH, max = constants::TASK_DESCRIPTION_MAX_LENGTH))]
    #[schema(example = "Need implement auth into api with JWT tokens")]
    pub description: String,

    #[garde(skip)]
    #[schema(example = "to_do")]
    pub status: TaskStatus,

    #[garde(skip)]
    #[schema(example = "2024-10-15T13:34:20.282397+03:00")]
    pub deadline: Option<DateTime<Local>>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TaskCommentCreateDto {
    #[garde(length(min = constants::TASK_COMMENT_TEXT_MIN_LENGTH, max = constants::TASK_COMMENT_TEXT_MAX_LENGTH))]
    #[schema(example = "This is easy task)")]
    pub text: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TaskReadDto {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub status: TaskStatus,
    pub deadline: Option<String>,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TaskCommentReadDto {
    pub id: Uuid,
    pub text: String,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TaskGetQuery {
    pub limit: u64,
    pub offset: u64,
    pub status: Option<TaskStatus>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TaskCommentGetQuery {
    pub limit: u64,
    pub offset: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TaskUpdateDto {
    #[garde(length(min = constants::TASK_NAME_MIN_LENGTH, max = constants::TASK_NAME_MAX_LENGTH))]
    #[schema(example = "Implement auth")]
    pub name: Option<String>,

    #[garde(length(min = constants::TASK_DESCRIPTION_MIN_LENGTH, max = constants::TASK_DESCRIPTION_MAX_LENGTH))]
    #[schema(example = "Need implement auth into api with JWT tokens")]
    pub description: Option<String>,

    #[garde(skip)]
    #[schema(example = "to_do")]
    pub status: Option<TaskStatus>,

    #[garde(skip)]
    #[schema(example = "2024-10-15T13:34:20.282397+03:00")]
    pub deadline: Option<DateTime<Local>>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TaskCommentUpdateDto {
    #[garde(length(min = constants::TASK_COMMENT_TEXT_MIN_LENGTH, max = constants::TASK_COMMENT_TEXT_MAX_LENGTH))]
    #[schema(example = "This is easy task)")]
    pub text: Option<String>,
}

impl IntoActiveModel<TaskActiveModel> for TaskCreateDto {
    fn into_active_model(self) -> TaskActiveModel {
        TaskActiveModel {
            name: Set(self.name),
            description: Set(self.description),
            status: Set(self.status),
            deadline: match self.deadline {
                Some(value) => Set(Some(value.fixed_offset())),
                None => NotSet,
            },
            ..Default::default()
        }
    }
}

impl IntoActiveModel<TaskCommentActiveModel> for TaskCommentCreateDto {
    fn into_active_model(self) -> TaskCommentActiveModel {
        TaskCommentActiveModel {
            text: Set(self.text),
            ..Default::default()
        }
    }
}

impl From<TaskModel> for TaskReadDto {
    fn from(value: TaskModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            status: value.status,
            deadline: match value.deadline {
                Some(value) => Some(value.to_rfc3339()),
                None => None,
            },
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
        }
    }
}

impl From<TaskCommentModel> for TaskCommentReadDto {
    fn from(value: TaskCommentModel) -> Self {
        Self {
            id: value.id,
            text: value.text,
            updated_at: value.updated_at.to_rfc3339(),
            created_at: value.created_at.to_rfc3339(),
        }
    }
}

impl IntoActiveModel<TaskActiveModel> for TaskUpdateDto {
    fn into_active_model(self) -> TaskActiveModel {
        TaskActiveModel {
            name: match self.name {
                Some(value) => Set(value),
                None => NotSet,
            },
            description: match self.description {
                Some(value) => Set(value),
                None => NotSet,
            },
            status: match self.status {
                Some(value) => Set(value),
                None => NotSet,
            },
            deadline: match self.deadline {
                Some(value) => Set(Some(value.fixed_offset())),
                None => NotSet,
            },
            updated_at: Set(Local::now().fixed_offset()),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<TaskCommentActiveModel> for TaskCommentUpdateDto {
    fn into_active_model(self) -> TaskCommentActiveModel {
        TaskCommentActiveModel {
            text: match self.text {
                Some(value) => Set(value),
                None => NotSet,
            },
            updated_at: Set(Local::now().fixed_offset()),
            ..Default::default()
        }
    }
}

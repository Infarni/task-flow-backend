use garde::Validate;
use sea_orm::{IntoActiveModel, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::constants;
use crate::entity::prelude::{TaskActiveModel, TaskModel};
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
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TaskReadDto {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub status: TaskStatus,
    pub updated_at: String,
    pub created_at: String,
}

impl IntoActiveModel<TaskActiveModel> for TaskCreateDto {
    fn into_active_model(self) -> TaskActiveModel {
        TaskActiveModel {
            name: Set(self.name),
            description: Set(self.description),
            status: Set(self.status),
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
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
        }
    }
}

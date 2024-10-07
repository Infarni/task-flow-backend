use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    IntoActiveModel, ModelTrait, QueryFilter, QuerySelect, Set, TransactionTrait, TryIntoModel,
};
use uuid::Uuid;

use crate::{
    dto::task::{TaskCommentCreateDto, TaskCommentReadDto, TaskCommentUpdateDto},
    entity::prelude::{
        TaskCommentActiveModel, TaskCommentColumn, TaskCommentEntity, TaskCommentModel, TaskEntity,
    },
    error::service::{ServiceError, ServiceResult},
};

pub struct TaskCommentService;

impl TaskCommentService {
    pub async fn create(
        db: &DatabaseConnection,
        user_id: Uuid,
        task_id: Uuid,
        body: TaskCommentCreateDto,
    ) -> ServiceResult<TaskCommentReadDto> {
        let tx: DatabaseTransaction = db.begin().await?;

        match TaskEntity::find_by_id(task_id).one(&tx).await? {
            Some(value) => {
                if value.user_id != user_id {
                    return Err(ServiceError::Forbidden);
                }
            }
            None => return Err(ServiceError::NotFound(task_id)),
        }

        let mut active_model: TaskCommentActiveModel = body.into_active_model();
        active_model.user_id = Set(user_id);
        active_model.task_id = Set(task_id);

        let model: TaskCommentModel = active_model.save(&tx).await?.try_into_model()?;

        tx.commit().await?;

        let schema: TaskCommentReadDto = TaskCommentReadDto::from(model);

        Ok(schema)
    }

    pub async fn list(
        db: &DatabaseConnection,
        user_id: Uuid,
        task_id: Uuid,
        limit: u64,
        offset: u64,
    ) -> ServiceResult<Vec<TaskCommentReadDto>> {
        let tx: DatabaseTransaction = db.begin().await?;

        match TaskEntity::find_by_id(task_id).one(&tx).await? {
            Some(value) => {
                if value.user_id != user_id {
                    return Err(ServiceError::Forbidden);
                }
            }
            None => return Err(ServiceError::NotFound(task_id)),
        }

        let models: Vec<TaskCommentModel> = TaskCommentEntity::find()
            .filter(TaskCommentColumn::TaskId.eq(task_id))
            .limit(limit)
            .offset(offset)
            .all(&tx)
            .await?;

        let schemas: Vec<TaskCommentReadDto> = models
            .into_iter()
            .map(|model| TaskCommentReadDto::from(model))
            .collect::<Vec<TaskCommentReadDto>>();

        Ok(schemas)
    }

    pub async fn update(
        db: &DatabaseConnection,
        user_id: Uuid,
        task_id: Uuid,
        id: Uuid,
        body: TaskCommentUpdateDto,
    ) -> ServiceResult<TaskCommentReadDto> {
        let tx: DatabaseTransaction = db.begin().await?;

        match TaskEntity::find_by_id(task_id).one(&tx).await? {
            Some(value) => {
                if value.user_id != user_id {
                    return Err(ServiceError::Forbidden);
                }
                if let None = TaskCommentEntity::find_by_id(id).one(&tx).await? {
                    return Err(ServiceError::NotFound(id));
                }
            }
            None => return Err(ServiceError::NotFound(task_id)),
        }

        let mut active_model: TaskCommentActiveModel = body.into_active_model();
        active_model.user_id = Set(user_id);
        active_model.task_id = Set(task_id);
        active_model.id = Set(id);

        let model: TaskCommentModel = active_model.save(&tx).await?.try_into_model()?;

        tx.commit().await?;

        let schema: TaskCommentReadDto = TaskCommentReadDto::from(model);

        Ok(schema)
    }

    pub async fn delete(
        db: &DatabaseConnection,
        user_id: Uuid,
        task_id: Uuid,
        id: Uuid,
    ) -> ServiceResult {
        let tx: DatabaseTransaction = db.begin().await?;

        let model: TaskCommentModel = match TaskCommentEntity::find_by_id(id).one(&tx).await? {
            Some(value) => {
                if (value.user_id != user_id) | (value.task_id != task_id) {
                    return Err(ServiceError::Forbidden);
                } else {
                    value
                }
            }
            None => return Err(ServiceError::NotFound(id)),
        };

        model.delete(&tx).await?;

        tx.commit().await?;

        Ok(())
    }
}

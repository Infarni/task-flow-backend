use sea_orm::{
    sea_query::SimpleExpr, ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction,
    EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, QuerySelect, Set, TransactionTrait,
    TryIntoModel,
};
use uuid::Uuid;

use crate::{
    dto::task::{TaskCreateDto, TaskReadDto, TaskUpdateDto},
    entity::{
        prelude::{TaskActiveModel, TaskColumn, TaskEntity, TaskModel},
        sea_orm_active_enums::TaskStatus,
    },
    error::service::{ServiceError, ServiceResult},
};

pub struct TaskService;

impl TaskService {
    pub async fn create(
        db: &DatabaseConnection,
        user_id: Uuid,
        body: TaskCreateDto,
    ) -> ServiceResult<TaskReadDto> {
        let tx: DatabaseTransaction = db.begin().await?;

        let mut active_model: TaskActiveModel = body.into_active_model();
        active_model.user_id = Set(user_id);

        let model: TaskModel = active_model.save(&tx).await?.try_into_model()?;

        tx.commit().await?;

        let schema: TaskReadDto = TaskReadDto::from(model);

        Ok(schema)
    }

    pub async fn list(
        db: &DatabaseConnection,
        user_id: Uuid,
        limit: u64,
        offset: u64,
        status: Option<TaskStatus>,
    ) -> ServiceResult<Vec<TaskReadDto>> {
        let tx: DatabaseTransaction = db.begin().await?;

        let mut query: SimpleExpr = TaskColumn::UserId.eq(user_id);

        if let Some(value) = status {
            query = query.and(TaskColumn::Status.eq(value));
        } else {
            query = query.and(TaskColumn::Status.ne(TaskStatus::Done))
        }

        let models: Vec<TaskModel> = TaskEntity::find()
            .filter(query)
            .limit(limit)
            .offset(offset)
            .all(&tx)
            .await?;

        let schemas: Vec<TaskReadDto> = models
            .into_iter()
            .map(|model| TaskReadDto::from(model))
            .collect::<Vec<TaskReadDto>>();

        Ok(schemas)
    }

    pub async fn update(
        db: &DatabaseConnection,
        user_id: Uuid,
        id: Uuid,
        body: TaskUpdateDto,
    ) -> ServiceResult<TaskReadDto> {
        let tx: DatabaseTransaction = db.begin().await?;

        match TaskEntity::find_by_id(id).one(&tx).await? {
            Some(value) => {
                if value.user_id != user_id {
                    return Err(ServiceError::Forbidden);
                }
            }
            None => return Err(ServiceError::NotFound(id)),
        };

        let mut active_model: TaskActiveModel = body.into_active_model();
        active_model.id = Set(id);

        let model: TaskModel = active_model.save(&tx).await?.try_into_model()?;

        tx.commit().await?;

        let schema: TaskReadDto = TaskReadDto::from(model);

        Ok(schema)
    }

    pub async fn delete(db: &DatabaseConnection, user_id: Uuid, id: Uuid) -> ServiceResult {
        let tx: DatabaseTransaction = db.begin().await?;

        let model: TaskModel = match TaskEntity::find_by_id(id).one(&tx).await? {
            Some(value) => {
                if value.user_id != user_id {
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

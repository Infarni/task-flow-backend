use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DatabaseTransaction, IntoActiveModel, Set,
    TransactionTrait, TryIntoModel,
};
use uuid::Uuid;

use crate::{
    dto::task::{TaskCreateDto, TaskReadDto},
    entity::prelude::{TaskActiveModel, TaskModel},
    error::service::ServiceResult,
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
}

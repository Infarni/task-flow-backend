use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    IntoActiveModel, QueryFilter, TransactionTrait, TryIntoModel,
};

use crate::dto::user::{UserCreateDto, UserReadDto};
use crate::entity::prelude::{UserActiveModel, UserColumn, UserEntity, UserModel};
use crate::error::service::{ServiceError, ServiceResult};

use super::common;

pub struct UserService;

impl UserService {
    pub async fn create(
        db: &DatabaseConnection,
        mut body: UserCreateDto,
    ) -> ServiceResult<UserReadDto> {
        let tx: DatabaseTransaction = db.begin().await?;

        if Self::check_name_exists(&db, body.name.clone()).await? {
            return Err(ServiceError::Conflict {
                field: "name".to_string(),
                value: body.name.clone(),
            });
        }

        if Self::check_email_exists(&db, body.email.clone()).await? {
            return Err(ServiceError::Conflict {
                field: "email".to_string(),
                value: body.email.clone(),
            });
        }

        body.password = common::hash(body.password)?;

        let active_model: UserActiveModel = body.into_active_model();
        let model: UserModel = active_model.save(&tx).await?.try_into_model()?;

        tx.commit().await?;

        let schema: UserReadDto = UserReadDto::from(model);

        Ok(schema)
    }

    pub async fn check_name_exists(db: &DatabaseConnection, name: String) -> ServiceResult<bool> {
        let tx: DatabaseTransaction = db.begin().await?;

        Ok(UserEntity::find()
            .filter(UserColumn::Name.eq(name))
            .one(&tx)
            .await?
            .is_some())
    }

    pub async fn check_email_exists(db: &DatabaseConnection, email: String) -> ServiceResult<bool> {
        let tx: DatabaseTransaction = db.begin().await?;

        Ok(UserEntity::find()
            .filter(UserColumn::Email.eq(email))
            .one(&tx)
            .await?
            .is_some())
    }
}

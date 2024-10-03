use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    IntoActiveModel, ModelTrait, QueryFilter, QuerySelect, Set, TransactionTrait, TryIntoModel,
};
use uuid::Uuid;

use crate::dto::user::{UserCreateDto, UserReadDto, UserUpdateDto};
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

    pub async fn get_by_id(db: &DatabaseConnection, id: Uuid) -> ServiceResult<UserReadDto> {
        let tx: DatabaseTransaction = db.begin().await?;

        match UserEntity::find_by_id(id).one(&tx).await? {
            Some(value) => Ok(UserReadDto::from(value)),
            None => Err(ServiceError::NotFound(id)),
        }
    }

    pub async fn search_by_name(
        db: &DatabaseConnection,
        name: String,
        limit: u64,
        offset: u64,
    ) -> ServiceResult<Vec<UserReadDto>> {
        let tx: DatabaseTransaction = db.begin().await?;

        let models = UserEntity::find()
            .filter(UserColumn::Name.contains(name))
            .limit(limit)
            .offset(offset)
            .all(&tx)
            .await?;

        let schemas = models
            .into_iter()
            .map(|m| UserReadDto::from(m))
            .collect::<Vec<UserReadDto>>();

        Ok(schemas)
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

    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        mut body: UserUpdateDto,
    ) -> ServiceResult<UserReadDto> {
        let tx: DatabaseTransaction = db.begin().await?;

        Self::get_by_id(db, id).await?;

        if let Some(name) = body.name.clone() {
            if Self::check_name_exists(&db, name.clone()).await? {
                return Err(ServiceError::Conflict {
                    field: "name".to_string(),
                    value: name.clone(),
                });
            }
        }

        if let Some(email) = body.email.clone() {
            if Self::check_email_exists(&db, email.clone()).await? {
                return Err(ServiceError::Conflict {
                    field: "email".to_string(),
                    value: email.clone(),
                });
            }
        }

        if let Some(password) = body.password {
            body.password = Some(common::hash(password)?);
        }

        let mut active_model: UserActiveModel = body.into_active_model();
        active_model.id = Set(id);

        let model: UserModel = active_model.save(&tx).await?.try_into_model()?;

        tx.commit().await?;

        let schema: UserReadDto = UserReadDto::from(model);

        Ok(schema)
    }

    pub async fn delete(db: &DatabaseConnection, id: Uuid) -> ServiceResult {
        let tx: DatabaseTransaction = db.begin().await?;

        let model: UserModel = match UserEntity::find_by_id(id).one(&tx).await? {
            Some(value) => value,
            None => return Err(ServiceError::NotFound(id)),
        };

        model.delete(&tx).await?;

        tx.commit().await?;

        Ok(())
    }
}

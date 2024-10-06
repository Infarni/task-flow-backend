use std::io::Cursor;

use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use image::{imageops, RgbaImage};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    IntoActiveModel, ModelTrait, QueryFilter, Set, TransactionTrait,
};
use uuid::Uuid;

use crate::entity::prelude::{
    UserAvatarActiveModel, UserAvatarColumn, UserAvatarEntity, UserAvatarModel,
};

use crate::{
    constants,
    error::service::{ServiceError, ServiceResult},
};

pub struct UserAvatarService;

impl UserAvatarService {
    pub async fn set(
        db: &DatabaseConnection,
        user_id: Uuid,
        mut body: Multipart,
    ) -> ServiceResult<Vec<u8>> {
        let mut file: Vec<u8> = Vec::new();

        while let Some(mut field) = body.try_next().await? {
            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(value) => value,
                    Err(err) => return Err(ServiceError::Multipart(err)),
                };
                file.extend_from_slice(&data);
            }
        }

        let image: RgbaImage = image::load_from_memory(&file)?.to_rgba8();
        let resized_image: RgbaImage = imageops::resize(
            &image,
            constants::AVATAR_SIDE_SIZE as u32,
            constants::AVATAR_SIDE_SIZE as u32,
            imageops::FilterType::Lanczos3,
        );

        let mut saved_file: Vec<u8> = Vec::new();
        let mut cursor: Cursor<&mut Vec<u8>> = Cursor::new(&mut saved_file);
        resized_image.write_to(&mut cursor, image::ImageFormat::Png)?;

        let tx: DatabaseTransaction = db.begin().await?;

        let mut active_model: UserAvatarActiveModel = match UserAvatarEntity::find()
            .filter(UserAvatarColumn::UserId.eq(user_id))
            .one(&tx)
            .await?
        {
            Some(value) => value.into_active_model(),
            None => UserAvatarActiveModel {
                user_id: Set(user_id),
                ..Default::default()
            },
        };

        active_model.file = Set(Some(saved_file.clone()));
        active_model.save(&tx).await?;

        tx.commit().await?;

        Ok(saved_file)
    }

    pub async fn get_by_user_id(db: &DatabaseConnection, user_id: Uuid) -> ServiceResult<Vec<u8>> {
        let tx: DatabaseTransaction = db.begin().await?;

        match UserAvatarEntity::find()
            .filter(UserAvatarColumn::UserId.eq(user_id))
            .one(&tx)
            .await?
        {
            Some(value) => match value.file {
                Some(value) => Ok(value),
                None => Err(ServiceError::NotFound(user_id)),
            },
            None => Err(ServiceError::NotFound(user_id)),
        }
    }

    pub async fn delete(db: &DatabaseConnection, user_id: Uuid) -> ServiceResult {
        let tx: DatabaseTransaction = db.begin().await?;

        let model: UserAvatarModel = match UserAvatarEntity::find()
            .filter(UserAvatarColumn::UserId.eq(user_id))
            .one(&tx)
            .await?
        {
            Some(value) => value,
            None => return Err(ServiceError::NotFound(user_id)),
        };

        model.delete(&tx).await?;

        tx.commit().await?;

        Ok(())
    }
}

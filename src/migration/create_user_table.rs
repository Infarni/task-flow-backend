use sea_orm_migration::prelude::*;

use crate::constants;

use super::create_table_extension::GenerateUuidFunc;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Func::cust(GenerateUuidFunc)),
                    )
                    .col(
                        ColumnDef::new(User::Name)
                            .string_len(constants::NAME_MAX_LENGTH as u32)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string_len(constants::EMAIL_MAX_LENGTH as u32)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(
                        ColumnDef::new(User::Avatar)
                            .binary_len(constants::AVATAR_MAX_SIZE as u32)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Name,
    Email,
    Password,
    Avatar,
    CreatedAt,
    UpdatedAt,
}

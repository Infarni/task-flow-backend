use extension::postgres::Type;
use sea_orm::{ActiveEnum, DeriveActiveEnum, EnumIter};
use sea_orm_migration::prelude::*;

use crate::constants;

use super::{create_table_extension::GenerateUuidFunc, create_user_table::User};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TaskStatus::name())
                    .values(TaskStatus::iden_values())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Task::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Func::cust(GenerateUuidFunc)),
                    )
                    .col(
                        ColumnDef::new(Task::Name)
                            .string_len(constants::TASK_NAME_MAX_LENGTH as u32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Task::Description)
                            .string_len(constants::TASK_DESCRIPTION_MAX_LENGTH as u32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Task::Status)
                            .enumeration(TaskStatus::name(), TaskStatus::iden_values())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Task::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Task::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(Task::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-user-id")
                            .from(Task::Table, Task::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TaskComment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TaskComment::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Func::cust(GenerateUuidFunc)),
                    )
                    .col(
                        ColumnDef::new(TaskComment::Text)
                            .string_len(constants::TASK_COMMENT_TEXT_MAX_LENGTH as u32)
                            .not_null(),
                    )
                    .col(ColumnDef::new(TaskComment::TaskId).uuid().not_null())
                    .col(ColumnDef::new(TaskComment::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(TaskComment::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(TaskComment::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-comment-task-id")
                            .from(TaskComment::Table, TaskComment::TaskId)
                            .to(Task::Table, Task::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-comment-user-id")
                            .from(TaskComment::Table, TaskComment::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TaskComment::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Task {
    Table,
    Id,
    Name,
    Description,
    Status,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "task_status")]
pub enum TaskStatus {
    #[sea_orm(string_value = "to_do")]
    ToDo,

    #[sea_orm(string_value = "in_progress")]
    InProgress,

    #[sea_orm(string_value = "done")]
    Done,
}

#[derive(DeriveIden)]
pub enum TaskComment {
    Table,
    Id,
    Text,
    TaskId,
    UserId,
    UpdatedAt,
    CreatedAt,
}

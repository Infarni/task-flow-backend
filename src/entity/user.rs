use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::task::Entity")]
    Task,
    #[sea_orm(has_many = "super::task_comment::Entity")]
    TaskComment,
    #[sea_orm(has_one = "super::user_avatar::Entity")]
    UserAvatar,
}

impl Related<super::task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl Related<super::task_comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskComment.def()
    }
}

impl Related<super::user_avatar::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserAvatar.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

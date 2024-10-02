use sea_orm_migration::prelude::*;

pub struct GenerateUuidFunc;

impl Iden for GenerateUuidFunc {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "uuid_generate_v4").unwrap()
    }
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DOWN EXTENSION IF EXISTS \"uuid-ossp\"")
            .await?;

        Ok(())
    }
}

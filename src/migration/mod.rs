mod create_table_extension;
mod create_user_table;

use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_table_extension::Migration),
            Box::new(create_user_table::Migration),
        ]
    }
}

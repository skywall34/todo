pub use sea_orm_migration::prelude::*;

mod m20230226_212703_create_tasks_table;
mod m20230305_164028_create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // users MUST be before tasks to run fresh migration
            Box::new(m20230305_164028_create_users_table::Migration),
            Box::new(m20230226_212703_create_tasks_table::Migration),
        ]
    }
}

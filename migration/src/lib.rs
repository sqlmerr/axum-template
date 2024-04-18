pub use sea_orm_migration::prelude::*;

mod m20240418_160547_task;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240418_160547_task::Migration),
        ]
    }
}

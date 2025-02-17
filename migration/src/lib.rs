pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250217_153113_create_posts_table;
mod m20250217_153204_create_comments_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250217_153113_create_posts_table::Migration),
            Box::new(m20250217_153204_create_comments_table::Migration),
        ]
    }
}

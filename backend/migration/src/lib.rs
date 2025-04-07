pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250407_143033_create_table_clothes;
mod m20250407_143055_create_table_outfits;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250407_143033_create_table_clothes::Migration),
            Box::new(m20250407_143055_create_table_outfits::Migration),
        ]
    }
}

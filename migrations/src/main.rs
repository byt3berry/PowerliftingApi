use async_std::main;
use sea_orm_migration::prelude::cli;

#[main]
async fn main() {
    cli::run_cli(migrations::Migrator).await;
}

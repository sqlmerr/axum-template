use dotenvy::dotenv;
use sea_orm::{Database, DbConn, DbErr};
use std::env;
use crate::Config;

pub async fn db_connection(settings: &Config) -> Result<DbConn, DbErr> {
    let db = Database::connect(&settings.database_url)
        .await
        .unwrap();

    Ok(db)
}

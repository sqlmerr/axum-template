use migration::{Migrator, MigratorTrait};
use std::net::SocketAddr;

pub mod db;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod schemas;
pub mod services;
pub mod state;
pub mod utils;
pub mod config;

pub use config::Config;

#[tokio::main]
async fn main() {
    let settings = Config::from_env();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let db = db::db_connection(&settings).await.unwrap();
    Migrator::up(&db, None).await.unwrap(); // Run migrations

    let app = routes::init_routers(&settings).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("listening on http://{}", addr);

    let server = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(server, app).await.unwrap();
}

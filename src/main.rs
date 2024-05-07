use axum::Router;
use std::net::SocketAddr;
use log::info;

pub mod db;
pub mod models;
mod routes;
pub mod schemas;
pub mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let app = Router::new().merge(routes::init_routers());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    info!("listening on http://{}", addr);

    let server = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(server, app).await.unwrap();
}

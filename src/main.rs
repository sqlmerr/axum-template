use axum::Router;
use std::net::SocketAddr;

pub mod db;
pub mod models;
mod routes;
pub mod schemas;
pub mod utils;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = Router::new().merge(routes::init_routers());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("listening on http://{}", addr);

    let server = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(server, app).await.unwrap();
}

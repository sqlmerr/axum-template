use axum::{
    Router
};
use std::net::SocketAddr;

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(routes::init_routers());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", addr);

    let server = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(server, app).await.unwrap();
}

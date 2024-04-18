mod example;

use axum::{
    Router,
    routing::{
        get, post, delete, put
    },
    response::{IntoResponse, Json}
};
use serde_json::json;


pub fn init_routers() -> Router {
    let root_router = Router::new()
        .route("/", get(|| async { Json(json!({"message": "Hello world"})) }))
        .route("/example", post(example::example_post_route));


    return root_router
}
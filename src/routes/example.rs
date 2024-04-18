use std::collections::HashMap;
use axum::{
    response::{IntoResponse, Json},
    extract::Query
};
use serde_json::json;


pub async fn example_post_route(
    Query(params): Query<HashMap<String, String>>
) -> impl IntoResponse {
    Json(json!({"something": params}))
}
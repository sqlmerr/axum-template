use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

use axum_template::schemas::task::UpdateTaskSchema;
use axum_template::{routes, schemas::task::CreateTaskSchema, Config};

async fn app() -> Router {
    let settings = Config::from_env();
    routes::init_routers(&settings).await
}

#[tokio::test]
async fn test_task1_create() {
    let app = app().await;
    let data = CreateTaskSchema {
        title: "test".to_string(),
        description: "test_description".to_string(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .uri("/tasks")
                .method("POST")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(serde_json::to_string(&data).unwrap())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["title"], "test".to_string());
    assert_eq!(body["description"], "test_description".to_string());
}

#[tokio::test]
async fn test_task2_get_from_many() {
    let app = app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/tasks")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body[0]["title"], "test".to_string());
    assert_eq!(body[0]["description"], "test_description".to_string());
}

#[tokio::test]
async fn test_task3_get() {
    let app = app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/tasks/1")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["title"], "test".to_string());
    assert_eq!(body["description"], "test_description".to_string());
}

#[tokio::test]
async fn test_task4_update() {
    let data = UpdateTaskSchema {
        title: Some("test2".to_string()),
        description: Some("test_description2".to_string()),
    };

    let app = app().await;
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/tasks/1")
                .method("PATCH")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(serde_json::to_string(&data).unwrap())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["message"], "Task updated!".to_string());

    let response2 = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/tasks/1")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response2.status(), StatusCode::OK);

    let body = response2.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["title"], "test2".to_string());
    assert_eq!(body["description"], "test_description2".to_string());
}

#[tokio::test]
async fn test_task5_delete() {
    let app = app().await;
    let response_delete = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/tasks/1")
                .method("DELETE")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_delete.status(), StatusCode::OK);

    let response_get = app
        .oneshot(
            Request::builder()
                .uri("/tasks/1")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_get.status(), StatusCode::NOT_FOUND);
}

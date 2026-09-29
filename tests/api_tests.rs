use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use rustwebapp::{create_app, store::PaperStore};

#[tokio::test]
async fn test_health_check() {
    let store = PaperStore::new();
    let app = create_app(store);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
    assert_eq!(json["service"], "arxiv-paper-storage");
    assert!(json["paper_count"].as_u64().unwrap() >= 6);
}

#[tokio::test]
async fn test_list_and_search_papers() {
    let store = PaperStore::new();
    let app = create_app(store);

    // List all
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/papers")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["total"], 7);
    assert_eq!(json["items"].as_array().unwrap().len(), 7);

    // Filter by query "attention"
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/papers?q=attention")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert!(json["total"].as_u64().unwrap() >= 2); // Attention Is All You Need + FlashAttention
}

#[tokio::test]
async fn test_get_paper_by_id_and_not_found() {
    let store = PaperStore::new();
    let app = create_app(store);

    // Success
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/papers/1706.03762")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["title"], "Attention Is All You Need");

    // Not found
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/papers/non-existent-9999")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_create_and_conflict_paper() {
    let store = PaperStore::new();
    let app = create_app(store);

    let new_paper = json!({
        "id": "2401.00001",
        "title": "Quantum Foundations of Neural Information",
        "authors": ["Alice Smith", "Bob Jones"],
        "summary": "We explore quantum foundations for neural representations.",
        "categories": ["quant-ph", "cs.AI"],
        "primary_category": "quant-ph"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/papers")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&new_paper).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(response.headers().contains_key("location"));

    // Attempt to create duplicate should result in 409 CONFLICT
    let response_conflict = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/papers")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&new_paper).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_conflict.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_update_and_delete_paper() {
    let store = PaperStore::new();
    let app = create_app(store);

    // Update title
    let update_body = json!({
        "title": "Attention Is All You Need (Updated Edition)"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/papers/1706.03762")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&update_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["title"], "Attention Is All You Need (Updated Edition)");
    assert!(json["updated"].is_string());

    // Delete paper
    let response_del = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/papers/1706.03762")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_del.status(), StatusCode::OK);

    // Fetch deleted paper -> 404
    let response_get = app
        .oneshot(
            Request::builder()
                .uri("/api/papers/1706.03762")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_get.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_stats_and_reset() {
    let store = PaperStore::new();
    let app = create_app(store);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/stats")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert!(json["total_papers"].as_u64().unwrap() >= 6);
    assert!(json["total_authors"].as_u64().unwrap() > 10);

    // Reset
    let response_reset = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/papers/reset")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_reset.status(), StatusCode::OK);
}

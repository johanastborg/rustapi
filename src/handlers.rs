use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    Json,
};
use chrono::Utc;
use serde_json::json;

use crate::{
    error::AppError,
    models::{
        CreatePaperPayload, HealthStatus, PaginatedResponse, Paper, PaperQuery, StoreStats,
        UpdatePaperPayload,
    },
    store::PaperStore,
};

pub async fn health_check(State(store): State<PaperStore>) -> Json<HealthStatus> {
    let count = store.count().await;
    Json(HealthStatus {
        status: "ok",
        service: "arxiv-paper-storage",
        version: env!("CARGO_PKG_VERSION"),
        timestamp: Utc::now().to_rfc3339(),
        paper_count: count,
    })
}

pub async fn get_stats(State(store): State<PaperStore>) -> Json<StoreStats> {
    let stats = store.stats().await;
    Json(stats)
}

pub async fn list_papers(
    State(store): State<PaperStore>,
    Query(query): Query<PaperQuery>,
) -> Json<PaginatedResponse<Paper>> {
    let result = store.list(&query).await;
    Json(result)
}

pub async fn get_paper(
    State(store): State<PaperStore>,
    Path(id): Path<String>,
) -> Result<Json<Paper>, AppError> {
    match store.get(&id).await {
        Some(paper) => Ok(Json(paper)),
        None => Err(AppError::NotFound(format!("Paper with ID '{}' not found", id))),
    }
}

pub async fn create_paper(
    State(store): State<PaperStore>,
    Json(payload): Json<CreatePaperPayload>,
) -> Result<(StatusCode, HeaderMap, Json<Paper>), AppError> {
    let paper = store.create(payload).await?;

    let mut headers = HeaderMap::new();
    if let Ok(loc) = HeaderValue::from_str(&format!("/api/papers/{}", paper.id)) {
        headers.insert(header::LOCATION, loc);
    }

    Ok((StatusCode::CREATED, headers, Json(paper)))
}

pub async fn update_paper(
    State(store): State<PaperStore>,
    Path(id): Path<String>,
    Json(payload): Json<UpdatePaperPayload>,
) -> Result<Json<Paper>, AppError> {
    let updated = store.update(&id, payload).await?;
    Ok(Json(updated))
}

pub async fn delete_paper(
    State(store): State<PaperStore>,
    Path(id): Path<String>,
) -> Result<Json<Paper>, AppError> {
    let deleted = store.delete(&id).await?;
    Ok(Json(deleted))
}

pub async fn reset_papers(State(store): State<PaperStore>) -> Json<serde_json::Value> {
    store.reset().await;
    let count = store.count().await;
    Json(json!({
        "message": "Storage reset to default mock seed papers",
        "total_papers": count
    }))
}

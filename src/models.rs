use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub summary: String,
    pub categories: Vec<String>,
    pub primary_category: String,
    pub published: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    pub pdf_url: String,
    pub arxiv_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePaperPayload {
    pub id: Option<String>,
    pub title: String,
    pub authors: Vec<String>,
    pub summary: String,
    pub categories: Vec<String>,
    pub primary_category: Option<String>,
    pub published: Option<String>,
    pub doi: Option<String>,
    pub pdf_url: Option<String>,
    pub arxiv_url: Option<String>,
    pub comment: Option<String>,
    pub journal_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UpdatePaperPayload {
    pub title: Option<String>,
    pub authors: Option<Vec<String>>,
    pub summary: Option<String>,
    pub categories: Option<Vec<String>>,
    pub primary_category: Option<String>,
    pub doi: Option<String>,
    pub pdf_url: Option<String>,
    pub arxiv_url: Option<String>,
    pub comment: Option<String>,
    pub journal_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PaperQuery {
    pub q: Option<String>,
    pub category: Option<String>,
    pub author: Option<String>,
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub sort: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub limit: usize,
    pub total_pages: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct StoreStats {
    pub total_papers: usize,
    pub categories_count: HashMap<String, usize>,
    pub total_authors: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthStatus {
    pub status: &'static str,
    pub service: &'static str,
    pub version: &'static str,
    pub timestamp: String,
    pub paper_count: usize,
}

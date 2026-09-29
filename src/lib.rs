pub mod error;
pub mod handlers;
pub mod models;
pub mod store;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};

use store::PaperStore;

pub fn create_app(store: PaperStore) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/api/health", get(handlers::health_check))
        .route("/api/stats", get(handlers::get_stats))
        .route(
            "/api/papers",
            get(handlers::list_papers).post(handlers::create_paper),
        )
        .route(
            "/api/papers/{id}",
            get(handlers::get_paper)
                .put(handlers::update_paper)
                .delete(handlers::delete_paper),
        )
        .route("/api/papers/reset", post(handlers::reset_papers))
        .fallback_service(ServeDir::new("static"))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(store)
}

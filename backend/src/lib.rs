use actix_web::{App, middleware::Logger, web};
use std::sync::Arc;

pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod services;
pub mod ws;

pub use config::Config;
pub use services::FraudEngine;

pub struct AppState {
    pub db: sqlx::PgPool,
    pub fraud_engine: Arc<FraudEngine>,
    pub broadcast_tx: tokio::sync::broadcast::Sender<models::FraudAlert>,
}

// Configuration function - applies routes to any App
// This is the standard Actix-web pattern for reusable config
pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(handlers::health_check))
        .route(
            "/transactions",
            web::post().to(handlers::analyze_transaction),
        )
        .route("/transactions", web::get().to(handlers::get_transactions))
        .route("/stats", web::get().to(handlers::get_stats))
        .route("/ws", web::get().to(ws::ws_handler));
}

// Helper to create CORS middleware
pub fn create_cors() -> actix_cors::Cors {
    actix_cors::Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::ACCEPT,
            actix_web::http::header::CONTENT_TYPE,
        ])
        .max_age(3600)
}

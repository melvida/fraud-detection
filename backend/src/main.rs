use actix_web::{App, HttpServer, middleware::Logger, web};
use fraud_detection::{AppState, Config, FraudEngine, db};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::from_env();

    let db_pool = db::init_db(&config.database_url)
        .await
        .expect("Failed to initialize database");

    log::info!("✓ Database initialized");

    let (tx, _rx) = tokio::sync::broadcast::channel(1000);
    let fraud_engine = Arc::new(FraudEngine::new());

    let state = web::Data::new(AppState {
        db: db_pool,
        fraud_engine,
        broadcast_tx: tx,
    });

    let addr = format!("{}:{}", config.server_host, config.server_port);
    log::info!("🚀 Starting fraud detection server on {}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(fraud_detection::create_cors())
            .wrap(Logger::default())
            .configure(fraud_detection::configure_app)
    })
    .bind(&addr)?
    .run()
    .await
}

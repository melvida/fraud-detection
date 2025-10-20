use crate::AppState;
use crate::models::StatsResponse;
use crate::services::TransactionService;
use actix_web::{HttpResponse, web};

pub async fn get_stats(data: web::Data<AppState>) -> HttpResponse {
    let service = TransactionService::new(data.db.clone());

    match service.get_stats().await {
        Ok((total, fraud_count, avg_score, high_risk)) => {
            let response = StatsResponse {
                total_transactions: total,
                fraud_count,
                avg_fraud_score: avg_score,
                high_risk_count: high_risk,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("Failed to fetch stats: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch stats"
            }))
        }
    }
}

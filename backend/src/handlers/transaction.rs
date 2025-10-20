use actix_web::{HttpResponse, web};
use chrono::Utc;
use uuid::Uuid;

use crate::AppState;
use crate::models::{CreateTransactionRequest, TransactionAnalysisResponse};
use crate::services::TransactionService;

pub async fn analyze_transaction(
    data: web::Data<AppState>,
    req: web::Json<CreateTransactionRequest>,
) -> HttpResponse {
    let transaction_id = Uuid::new_v4();

    // Create temporary transaction for fraud analysis
    let temp_tx = crate::models::Transaction {
        id: transaction_id,
        user_id: req.user_id.clone(),
        amount: req.amount,
        merchant: req.merchant.clone(),
        country: req.country.clone(),
        card_last_4: req.card_last_4.clone(),
        fraud_score: 0.0,
        is_fraud: false,
        created_at: Utc::now(),
    };

    // Analyze fraud (fast, in-memory)
    let (fraud_score, reasons) = data.fraud_engine.analyze(&temp_tx);
    let reasons_clone = reasons.clone();
    let is_fraud = fraud_score > 0.6;

    // Store in database asynchronously
    let req_clone = req.into_inner();
    let db = data.db.clone();
    let broadcast_tx = data.broadcast_tx.clone();

    tokio::spawn(async move {
        let service = TransactionService::new(db);

        if let Ok(_tx) = service
            .create_transaction(&req_clone, fraud_score, is_fraud)
            .await
        {
            if is_fraud {
                let alert = crate::models::FraudAlert {
                    id: Uuid::new_v4(),
                    user_id: req_clone.user_id.clone(),
                    transaction_id,
                    fraud_score,
                    reasons: reasons.clone(),
                    created_at: Utc::now().to_rfc3339(),
                };
                let _ = broadcast_tx.send(alert);
            }
        }
    });

    let response = TransactionAnalysisResponse {
        id: transaction_id,
        user_id: temp_tx.user_id.clone(),
        amount: temp_tx.amount,
        merchant: temp_tx.merchant.clone(),
        country: temp_tx.country.clone(),
        fraud_score,
        is_fraud,
        fraud_reasons: reasons_clone,
        card_last_4: temp_tx.card_last_4.clone(),
    };

    HttpResponse::Ok().json(response)
}

pub async fn get_transactions(
    data: web::Data<AppState>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let limit = query
        .get("limit")
        .and_then(|l| l.parse::<i64>().ok())
        .unwrap_or(20);

    let fraud_only = query
        .get("fraud_only")
        .map(|f| f == "true")
        .unwrap_or(false);

    let service = TransactionService::new(data.db.clone());

    match service.get_recent_transactions(limit, fraud_only).await {
        Ok(transactions) => {
            let response: Vec<TransactionAnalysisResponse> = transactions
                .into_iter()
                .map(|t| TransactionAnalysisResponse {
                    id: t.id,
                    user_id: t.user_id,
                    amount: t.amount,
                    merchant: t.merchant,
                    country: t.country,
                    card_last_4: t.card_last_4,
                    fraud_score: t.fraud_score,
                    is_fraud: t.is_fraud,
                    fraud_reasons: vec![],
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("Failed to fetch transactions: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch transactions"
            }))
        }
    }
}

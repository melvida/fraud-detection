use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: String,
    pub amount: f64,
    pub merchant: String,
    pub country: String,
    pub card_last_4: String,
    pub fraud_score: f64,
    pub is_fraud: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub user_id: String,
    pub amount: f64,
    pub merchant: String,
    pub country: String,
    pub card_last_4: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionAnalysisResponse {
    pub user_id: String,
    pub amount: f64,
    pub merchant: String,
    pub country: String,
    pub card_last_4: String,
    pub id: Uuid,
    pub fraud_score: f64,
    pub is_fraud: bool,
    pub fraud_reasons: Vec<String>,
}

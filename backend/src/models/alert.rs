use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudAlert {
    pub id: Uuid,
    pub user_id: String,
    pub transaction_id: Uuid,
    pub fraud_score: f64,
    pub reasons: Vec<String>,
    pub created_at: String,
}

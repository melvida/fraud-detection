use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total_transactions: i64,
    pub fraud_count: i64,
    pub avg_fraud_score: f64,
    pub high_risk_count: i64,
}

use crate::error::{AppError, Result};
use crate::models::{CreateTransactionRequest, Transaction};
use chrono::Utc;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;

pub struct TransactionService {
    db: PgPool,
}

impl TransactionService {
    pub fn new(db: PgPool) -> Self {
        TransactionService { db }
    }

    pub async fn create_transaction(
        &self,
        req: &CreateTransactionRequest,
        fraud_score: f64,
        is_fraud: bool,
    ) -> Result<Transaction> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO transactions (id, user_id, amount, merchant, country, card_last_4, fraud_score, is_fraud, created_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(id)
        .bind(&req.user_id)
        .bind(req.amount)
        .bind(&req.merchant)
        .bind(&req.country)
        .bind(&req.card_last_4)
        .bind(fraud_score)
        .bind(is_fraud)
        .bind(now)
        .execute(&self.db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(Transaction {
            id,
            user_id: req.user_id.clone(),
            amount: req.amount,
            merchant: req.merchant.clone(),
            country: req.country.clone(),
            card_last_4: req.card_last_4.clone(),
            fraud_score,
            is_fraud,
            created_at: now,
        })
    }

    pub async fn get_recent_transactions(
        &self,
        limit: i64,
        fraud_only: bool,
    ) -> Result<Vec<Transaction>> {
        let query_str = if fraud_only {
            "SELECT * FROM transactions WHERE is_fraud = true ORDER BY created_at DESC LIMIT $1"
        } else {
            "SELECT * FROM transactions ORDER BY created_at DESC LIMIT $1"
        };

        sqlx::query_as::<_, Transaction>(query_str)
            .bind(limit)
            .fetch_all(&self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_stats(&self) -> Result<(i64, i64, f64, i64)> {
        let row = sqlx::query(
            "SELECT COUNT(*) as total, COALESCE(SUM(CASE WHEN is_fraud THEN 1 ELSE 0 END), 0) as fraud_count, 
             COALESCE(AVG(fraud_score), 0.0) as avg_score FROM transactions"
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let total: i64 = row.get("total");
        let fraud_count: i64 = row.get("fraud_count");
        let avg_score: f64 = row.get("avg_score");

        let high_risk: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM transactions WHERE fraud_score > 0.6")
                .fetch_one(&self.db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

        Ok((total, fraud_count, avg_score, high_risk))
    }
}

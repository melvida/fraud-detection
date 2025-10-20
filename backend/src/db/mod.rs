use crate::error::Result;
use sqlx::PgPool;

pub async fn init_db(database_url: &str) -> Result<PgPool> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|e| crate::error::AppError::Database(e.to_string()))?;

    run_migrations(&pool).await?;

    Ok(pool)
}

async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            id UUID PRIMARY KEY,
            user_id VARCHAR(50) NOT NULL,
            amount FLOAT NOT NULL,
            merchant VARCHAR(100) NOT NULL,
            country VARCHAR(2) NOT NULL,
            card_last_4 VARCHAR(4) NOT NULL,
            fraud_score FLOAT NOT NULL,
            is_fraud BOOLEAN NOT NULL DEFAULT false,
            created_at TIMESTAMPTZ NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::error::AppError::Database(e.to_string()))?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS fraud_alerts (
            id UUID PRIMARY KEY,
            user_id VARCHAR(50) NOT NULL,
            transaction_id UUID NOT NULL,
            reasons JSONB NOT NULL,
            fraud_score FLOAT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::error::AppError::Database(e.to_string()))?;

    Ok(())
}

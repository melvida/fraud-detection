use crate::models::Transaction;
use chrono::Utc;
use dashmap::DashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub avg_transaction_amount: f64,
    pub max_transaction_amount: f64,
    pub common_countries: Vec<(String, i32)>,
    pub recent_transactions: VecDeque<Transaction>,
}

pub struct FraudEngine {
    user_profiles: DashMap<String, UserProfile>,
}

impl FraudEngine {
    pub fn new() -> Self {
        FraudEngine {
            user_profiles: DashMap::new(),
        }
    }

    pub fn analyze(&self, transaction: &Transaction) -> (f64, Vec<String>) {
        let mut fraud_score: f64 = 0.0;
        let mut reasons = Vec::new();

        // Get or create user profile
        let mut profile = self
            .user_profiles
            .entry(transaction.user_id.clone())
            .or_insert_with(|| UserProfile {
                avg_transaction_amount: 100.0,
                max_transaction_amount: 500.0,
                common_countries: vec![(transaction.country.clone(), 1)],
                recent_transactions: VecDeque::new(),
            })
            .clone();

        // Rule 1: Velocity Check
        let recent_count = profile
            .recent_transactions
            .iter()
            .filter(|t| (Utc::now() - t.created_at).num_minutes() < 5)
            .count();

        if recent_count > 5 {
            fraud_score += 0.35;
            reasons.push(format!(
                "Velocity fraud: {} transactions in 5 minutes",
                recent_count
            ));
        }

        // Rule 2: Amount Anomaly
        if transaction.amount > profile.max_transaction_amount * 2.0 {
            fraud_score += 0.25;
            reasons.push(format!(
                "Amount anomaly: ${} vs max ${}",
                transaction.amount, profile.max_transaction_amount
            ));
        }

        // Rule 3: Geographic Impossibility
        let now = Utc::now();
        for prev_tx in profile.recent_transactions.iter().rev().take(1) {
            let time_diff = (now - prev_tx.created_at).num_seconds();
            if time_diff < 3600 && prev_tx.country != transaction.country {
                fraud_score += 0.30;
                reasons.push(format!(
                    "Geographic impossibility: {} to {} in {} seconds",
                    prev_tx.country, transaction.country, time_diff
                ));
                break;
            }
        }
        // Rule 4: Unusual Country
        let country_count = profile
            .common_countries
            .iter()
            .find(|(c, _)| c == &transaction.country)
            .map(|(_, count)| *count)
            .unwrap_or(0);

        if country_count == 0 && !profile.common_countries.is_empty() {
            fraud_score += 0.20;
            reasons.push("New country detected".to_string());
        }

        // Rule 5: Duplicate Transaction
        if let Some(last_tx) = profile.recent_transactions.back() {
            if (now - last_tx.created_at).num_seconds() < 10
                && (last_tx.amount - transaction.amount).abs() < 0.01
                && last_tx.merchant == transaction.merchant
            {
                fraud_score += 0.40;
                reasons.push("Duplicate transaction detected".to_string());
            }
        }
        // Rule 6: Card Velocity
        let card_velocity = profile
            .recent_transactions
            .iter()
            .filter(|t| {
                t.card_last_4 == transaction.card_last_4
                    && (Utc::now() - t.created_at).num_minutes() < 10
            })
            .count();

        if card_velocity > 8 {
            fraud_score += 0.25;
            reasons.push(format!(
                "Card velocity: {} uses in 10 minutes",
                card_velocity
            ));
        }

        fraud_score = fraud_score.min(1.0);

        // Update profile
        profile.recent_transactions.push_back(transaction.clone());
        if profile.recent_transactions.len() > 50 {
            profile.recent_transactions.pop_front();
        }

        let total = profile.avg_transaction_amount * profile.recent_transactions.len() as f64;
        profile.avg_transaction_amount =
            (total + transaction.amount) / (profile.recent_transactions.len() as f64);
        profile.max_transaction_amount = profile.max_transaction_amount.max(transaction.amount);

        self.user_profiles
            .insert(transaction.user_id.clone(), profile);

        (fraud_score, reasons)
    }
}

impl Default for FraudEngine {
    fn default() -> Self {
        Self::new()
    }
}

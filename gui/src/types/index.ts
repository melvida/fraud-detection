export interface Transaction {
    id: string;
    user_id: string;
    amount: number;
    merchant: string;
    country: string;
    card_last_4: string;
    fraud_score: number;
    is_fraud: boolean;
    fraud_reasons: string[];
}

export interface FraudAlert {
    id: string;
    user_id: string;
    transaction_id: string;
    fraud_score: number;
    reasons: string[];
    created_at: string;
}

export interface Stats {
    total_transactions: number;
    fraud_count: number;
    avg_fraud_score: number;
    high_risk_count: number;
}

export interface FormData {
    user_id: string;
    amount: number;
    merchant: string;
    country: string;
    card_last_4: string;
}
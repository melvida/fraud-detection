import { useState, useCallback } from 'react';
import { Transaction, FraudAlert, Stats, FormData } from '../types';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

export const useFraudApi = () => {
    const [transactions, setTransactions] = useState<Transaction[]>([]);

    const [stats, setStats] = useState<Stats>({
        total_transactions: 0,
        fraud_count: 0,
        avg_fraud_score: 0,
        high_risk_count: 0,
    });

    const [loading, setLoading] = useState(false);

    const fetchTransactions = useCallback(async () => {
        setLoading(true);
        try {
            const response = await fetch(`${API_URL}/transactions?limit=50`);
            const data = await response.json();
            setTransactions(data || []);
        } catch (error) {
            console.error('Error fetching transactions:', error);
        }
        setLoading(false);
    }, []);

    const fetchStats = useCallback(async () => {
        try {
            const response = await fetch(`${API_URL}/stats`);
            const data = await response.json();
            setStats(data);
        } catch (error) {
            console.error('Error fetching stats:', error);
        }
    }, []);

    const submitTransaction = useCallback(async (formData: any) => {
        try {
            const response = await fetch(`${API_URL}/transactions`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    user_id: formData.user_id,
                    amount: parseFloat(formData.amount),
                    merchant: formData.merchant,
                    country: formData.country,
                    card_last_4: formData.card_last_4,
                }),
            });
            return await response.json();
        } catch (error) {
            console.error('Error submitting transaction:', error);
            throw error;
        }
    }, []);

    return { transactions, stats, loading, fetchTransactions, fetchStats, submitTransaction };
}
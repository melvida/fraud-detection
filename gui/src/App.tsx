import { useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Header } from '@/components/Header';
import { StatsGrid } from '@/components/StatsGrid';
import { TransactionForm } from '@/components/TransactionForm';
import { AlertsList } from '@/components/AlertsList';
import { TransactionsList } from '@/components/TransactionsList';
import { useFraudApi } from '@/hooks/useFraudApi';
import { useWebSocket } from '@/hooks/useWebSocket';
import { useTransactionForm } from '@/hooks/useTransactionForm';

export default function App() {
  const { transactions, stats, fetchTransactions, fetchStats, submitTransaction } = useFraudApi();
  const { alerts } = useWebSocket();
  const { formData, setFormData, showForm, setShowForm, handleSubmit, isLoading } = useTransactionForm(
    async (data) => {
      await submitTransaction(data);
      fetchTransactions();
      fetchStats();
    }
  );

  useEffect(() => {
    fetchTransactions();
    fetchStats();
    const interval = setInterval(() => {
      fetchStats();
      fetchTransactions();
    }, 2000);
    return () => clearInterval(interval);
  }, [fetchTransactions, fetchStats]);

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
      <Header />

      <div className="max-w-7xl mx-auto p-4 py-8">
        <StatsGrid stats={stats} />

        <div className="mb-8">
          <Button
            onClick={() => setShowForm(!showForm)}
            className="bg-blue-600 hover:bg-blue-700"
          >
            {showForm ? '✕ Cancel' : '➕ Simulate Transaction'}
          </Button>
        </div>

        {showForm && (
          <TransactionForm
            formData={formData}
            setFormData={setFormData}
            onSubmit={handleSubmit}
            isLoading={isLoading}
          />
        )}

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <AlertsList alerts={alerts} />
          <TransactionsList transactions={transactions} />
        </div>
      </div>
    </div>
  );
}
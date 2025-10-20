import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Transaction } from '@/types';
import { getRiskColor, getRiskLabel } from '@/utils/fraud';

interface TransactionsListProps {
    transactions: Transaction[];
}

export function TransactionsList({ transactions }: TransactionsListProps) {
    return (
        <Card className="bg-slate-800 border-slate-700 h-fit lg:max-h-96 overflow-y-auto">
            <CardHeader>
                <CardTitle>📊 Recent Transactions</CardTitle>
                <CardDescription>Latest transaction analysis</CardDescription>
            </CardHeader>
            <CardContent>
                <div className="space-y-2">
                    {transactions.slice(0, 20).map((tx) => (
                        <div key={tx.id} className="bg-slate-700/50 border-l-4 border-blue-500 p-3 rounded">
                            <div className="flex justify-between items-start mb-2">
                                <div>
                                    <div className="font-semibold text-white">{tx.merchant}</div>
                                    <div className="text-sm text-slate-400">{tx.user_id}</div>
                                </div>
                                <Badge variant={getRiskColor(tx.fraud_score)}>
                                    {getRiskLabel(tx.fraud_score)}
                                </Badge>
                            </div>
                            <div className="flex justify-between text-sm">
                                <span className="text-green-400 font-semibold">${tx.amount.toFixed(2)}</span>
                                <span className="text-slate-400">{tx.country} •••• {tx.card_last_4}</span>
                            </div>
                        </div>
                    ))}
                </div>
            </CardContent>
        </Card>
    );
}
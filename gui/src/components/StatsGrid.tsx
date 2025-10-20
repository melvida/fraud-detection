import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Stats } from '@/types';

interface StatsGridProps {
    stats: Stats;
}

export function StatsGrid({ stats }: StatsGridProps) {
    const statCards = [
        {
            title: 'Total Transactions',
            value: stats.total_transactions,
            bgColor: 'bg-slate-800',
            borderColor: 'border-slate-700',
            textColor: 'text-slate-300',
        },
        {
            title: 'Fraud Cases',
            value: stats.fraud_count,
            bgColor: 'bg-red-900',
            borderColor: 'border-red-700',
            textColor: 'text-red-200',
        },
        {
            title: 'High Risk',
            value: stats.high_risk_count,
            bgColor: 'bg-orange-900',
            borderColor: 'border-orange-700',
            textColor: 'text-orange-200',
        },
        {
            title: 'Avg Fraud Score',
            value: stats.avg_fraud_score.toFixed(2),
            bgColor: 'bg-green-900',
            borderColor: 'border-green-700',
            textColor: 'text-green-200',
        },
    ];

    return (
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
            {statCards.map((card) => (
                <Card key={card.title} className={`${card.bgColor} ${card.borderColor}`}>
                    <CardHeader>
                        <CardTitle className={`text-sm font-medium ${card.textColor}`}>
                            {card.title}
                        </CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div className="text-3xl font-bold text-white">{card.value}</div>
                    </CardContent>
                </Card>
            ))}
        </div>
    );
}
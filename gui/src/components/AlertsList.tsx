import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { FraudAlert } from '@/types';
import { getRiskColor, getRiskLabel } from '@/utils/fraud';

interface AlertsListProps {
    alerts: FraudAlert[];
}

export function AlertsList({ alerts }: AlertsListProps) {
    return (
        <Card className="bg-slate-800 border-slate-700 h-fit lg:max-h-96 overflow-y-auto">
            <CardHeader>
                <CardTitle>⚡ Real-time Alerts</CardTitle>
                <CardDescription>Fraud alerts as they happen</CardDescription>
            </CardHeader>
            <CardContent>
                {alerts.length === 0 ? (
                    <p className="text-slate-400 text-center py-8">No fraud alerts yet</p>
                ) : (
                    <div className="space-y-3">
                        {alerts.map((alert) => (
                            <div key={alert.id} className="bg-red-900/20 border border-red-700 rounded-lg p-3">
                                <div className="flex justify-between items-start mb-2">
                                    <span className="font-semibold text-white">{alert.user_id}</span>
                                    <Badge variant={getRiskColor(alert.fraud_score)}>
                                        {getRiskLabel(alert.fraud_score)} ({alert.fraud_score.toFixed(2)})
                                    </Badge>
                                </div>
                                <div className="text-sm text-slate-300 space-y-1">
                                    {alert.reasons.map((reason, idx) => (
                                        <div key={idx}>• {reason}</div>
                                    ))}
                                </div>
                                <div className="text-xs text-slate-500 mt-2">
                                    {new Date(alert.created_at).toLocaleTimeString()}
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </CardContent>
        </Card>
    );
}
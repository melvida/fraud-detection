export function getRiskColor(score: number): 'destructive' | 'default' | 'secondary' | 'outline' {
    if (score > 0.7) return 'destructive';
    if (score > 0.5) return 'default';
    if (score > 0.3) return 'secondary';
    return 'outline';
}

export function getRiskLabel(score: number): string {
    if (score > 0.7) return 'CRITICAL';
    if (score > 0.5) return 'HIGH';
    if (score > 0.3) return 'MEDIUM';
    return 'LOW';
}
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { FormData } from '@/types';

interface TransactionFormProps {
    formData: FormData;
    setFormData: (data: FormData) => void;
    onSubmit: (e: React.FormEvent<HTMLFormElement>) => void;
    isLoading: boolean;
}

export function TransactionForm({ formData, setFormData, onSubmit, isLoading }: TransactionFormProps) {
    return (
        <Card className="bg-slate-800 border-slate-700 mb-8">
            <CardHeader>
                <CardTitle>Create Test Transaction</CardTitle>
            </CardHeader>
            <CardContent>
                <form onSubmit={onSubmit} className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <Input
                        placeholder="User ID"
                        value={formData.user_id}
                        onChange={(e) => setFormData({ ...formData, user_id: e.target.value })}
                        required
                        className="bg-slate-700 border-slate-600 text-white"
                    />
                    <Input
                        type="number"
                        placeholder="Amount"
                        step="0.01"
                        value={formData.amount}
                        onChange={(e) => setFormData({ ...formData, amount: Number(e.target.value) })}
                        required
                        className="bg-slate-700 border-slate-600 text-white"
                    />
                    <Input
                        placeholder="Merchant"
                        value={formData.merchant}
                        onChange={(e) => setFormData({ ...formData, merchant: e.target.value })}
                        required
                        className="bg-slate-700 border-slate-600 text-white"
                    />
                    <Select value={formData.country} onValueChange={(val) => setFormData({ ...formData, country: val })}>
                        <SelectTrigger className="bg-slate-700 border-slate-600 text-white">
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="US">US</SelectItem>
                            <SelectItem value="UK">UK</SelectItem>
                            <SelectItem value="JP">JP</SelectItem>
                            <SelectItem value="CN">CN</SelectItem>
                            <SelectItem value="IN">IN</SelectItem>
                        </SelectContent>
                    </Select>
                    <Input
                        placeholder="Card Last 4"
                        value={formData.card_last_4}
                        onChange={(e) => setFormData({ ...formData, card_last_4: e.target.value })}
                        required
                        maxLength={4}
                        className="bg-slate-700 border-slate-600 text-white"
                    />
                    <Button type="submit" disabled={isLoading} className="bg-green-600 hover:bg-green-700 md:col-span-2">
                        {isLoading ? 'Submitting...' : 'Submit Transaction'}
                    </Button>
                </form>
            </CardContent>
        </Card>
    );
}
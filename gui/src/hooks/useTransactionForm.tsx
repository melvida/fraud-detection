import { useState } from 'react';
import { FormData } from '@/types';

const initialFormData: FormData = {
    user_id: '',
    amount: '',
    merchant: '',
    country: 'US',
    card_last_4: '',
};

export function useTransactionForm(onSubmit: (data: FormData) => Promise<void>) {
    const [formData, setFormData] = useState<FormData>(initialFormData);
    const [showForm, setShowForm] = useState(false);
    const [isLoading, setIsLoading] = useState(false);

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setIsLoading(true);
        try {
            await onSubmit(formData);
            setFormData(initialFormData);
            setShowForm(false);
        } catch (error) {
            console.error('Form submission error:', error);
        }
        setIsLoading(false);
    };

    return { formData, setFormData, showForm, setShowForm, handleSubmit, isLoading };
}
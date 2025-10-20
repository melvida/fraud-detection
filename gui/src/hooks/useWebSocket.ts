import { useState, useEffect, useCallback } from 'react';
import { FraudAlert } from '@/types';

const WS_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:8080/ws';

export function useWebSocket() {
    const [alerts, setAlerts] = useState<FraudAlert[]>([]);
    const [ws, setWs] = useState<WebSocket | null>(null);

    const connectWebSocket = useCallback(() => {
        const socket = new WebSocket(WS_URL);

        socket.onopen = () => {
            console.log('Connected to fraud alerts');
            setWs(socket);
        };

        socket.onmessage = (event) => {
            try {
                const alert = JSON.parse(event.data);
                setAlerts((prev) => [alert, ...prev].slice(0, 10));
            } catch (e) {
                console.error('Error parsing alert:', e);
            }
        };

        socket.onclose = () => {
            console.log('Disconnected, reconnecting...');
            setTimeout(connectWebSocket, 3000);
        };

        socket.onerror = (error) => {
            console.error('WebSocket error:', error);
        };
    }, []);

    useEffect(() => {
        connectWebSocket();
        return () => {
            ws?.close();
        };
    }, [connectWebSocket, ws]);

    return { alerts, setAlerts, ws };
}
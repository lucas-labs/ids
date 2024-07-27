import { useEffect, useState } from 'preact/hooks';
import Socket from 'sockette';

export interface WebSocketMessage {
    payload: string;
    timestamp: number;
}

/**
 * a hook that connects to a websocket and returns the messages received from the server
 */
export const useWs = (url: string) => {
    const [message, setMessage] = useState<WebSocketMessage | null>(null);

    useEffect(() => {
        const ws = new Socket(url, {
            onmessage: (e) => {
                setMessage({
                    payload: e.data,
                    timestamp: Date.now(),
                });
            },
        });

        return () => {
            ws.close();
        };
    }, [url]);

    return message;
};

import { IconCard } from '@cmp/icon-card.cmp';
import { useStore } from '@nanostores/preact';
import { useEffect } from 'preact/hooks';
import { useWs } from '../hooks/use-ws.hook';
import { $contentsStore } from '../stores/contents.store';

const BASE_URL = import.meta.env.IDS_SERVER_URL;

export function Home() {
    const { data: contents } = useStore($contentsStore);
    const message = useWs(`${BASE_URL}_ids_runtime/ws`.replace('http', 'ws'));

    if (!contents) {
        return <></>;
    }

    useEffect(() => {
        if (message.payload === 'R') {
            $contentsStore.revalidate();
        }
    }, [message]);

    return (
        <div class='flex flex-col gap-12'>
            {contents.length === 0 && <span>No changes detected</span>}
            {contents.map((content) => (
                <section key={content.dir} class='p-2 rounded-lg bg-zinc-950/40'>
                    <h2 class='text-xl font-semibold mb-8 font-mono'>/ {content.dir}</h2>

                    <div class='flex gap-8 flex-wrap'>
                        {content.files.map((file) => (
                            <IconCard key={file.name} file={file} />
                        ))}
                    </div>
                </section>
            ))}
        </div>
    );
}

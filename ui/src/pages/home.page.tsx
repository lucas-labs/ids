import { IconCard } from '@cmp/icon-card.cmp';
import { useStore } from '@nanostores/preact';
import { $contentsStore } from '../stores/contents.store';

export function Home() {
    const { data: contents } = useStore($contentsStore);

    if (!contents) {
        return <></>;
    }

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

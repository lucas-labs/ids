import { useStore } from '@nanostores/preact';
import { useEffect, useRef } from 'preact/hooks';
import { $iconsSizeStore } from '../stores/contents.store';
import type { File, FileGroup } from '../stores/types/store';

const BASE_URL = import.meta.env.IDS_SERVER_URL.replace(/\/$/, '');

const fileName = (path: string) => {
    const parts = path.replace(/\\/g, '/').split('/');
    const name = parts[parts.length - 1];
    return name.split('.')[0];
};

export const IconCard = ({ file }: { file: FileGroup }) => {
    const footerClasses =
        file.status === 'new'
            ? 'bg-green-500/10'
            : file.status === 'mod'
              ? 'bg-yellow-500/10'
              : file.status === 'add'
                ? 'bg-blue-500/10'
                : file.status === 'del'
                  ? 'bg-red-500/10'
                  : '';

    return (
        <article class={'flex flex-col items-center  bg-zinc-800/30 rounded-lg overflow-hidden'}>
            <div class='flex justify-center'>
                <Icon file={file.dark} variant='dark' />
                <Icon file={file.light} variant='light' />
            </div>

            <footer className={`${footerClasses} text-zinc-100/50 w-full text-center`}>
                <h3 class='text-sm py-1'>{fileName(file.name)}</h3>
            </footer>
        </article>
    );
};

/**
 * shows an icon with dark or light background
 */
const Icon = ({ file, variant }: { file: File; variant: 'dark' | 'light' }) => {
    const iconsSize = useStore($iconsSizeStore);
    const imgRef = useRef<HTMLImageElement>();

    // set the size in the style when the size changes
    useEffect(() => {
        imgRef.current.style.width = `${iconsSize}px`;
        imgRef.current.style.height = `${iconsSize}px`;
    }, [iconsSize]);

    return (
        <span className={`${variant === 'dark' ? 'bg-zinc-900' : 'bg-zinc-200'} p-6`}>
            <img
                ref={imgRef}
                src={BASE_URL + file.link}
                alt={fileName(file.path)}
                class='transition-[width] duration-100'
            />
        </span>
    );
};

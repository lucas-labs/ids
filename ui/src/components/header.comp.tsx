import { useStore } from '@nanostores/preact';
import { $iconsSizeStore } from '../stores/contents.store';
import { IdsLogo } from './ids-logo.comp';

const sizes = [16, 32, 64, 128];

export function Header() {
    const activeSize = useStore($iconsSizeStore);

    function sizeBtnClick(e: Event) {
        // get the size from the button
        const size = (e.target as HTMLButtonElement).textContent;
        // set the size in the store
        $iconsSizeStore.set(Number(size));
    }

    return (
        <header className={'py-4 container sticky mx-auto top-0 flex flex-col items-center'}>
            <div
                class={
                    'flex items-center gap-8 bg-slate-400/10 backdrop-blur-md px-6 py-1 rounded-full'
                }
            >
                {/* first two sizes */}
                {sizes.slice(0, 2).map((size) => (
                    <SizeButton key={`size-btn-${size}`} size={size} onClick={sizeBtnClick} />
                ))}

                <IdsLogo />

                {/* last two sizes */}
                {sizes.slice(2).map((size) => (
                    <SizeButton key={`size-btn-${size}`} size={size} onClick={sizeBtnClick} />
                ))}
            </div>
        </header>
    );
}

const SizeButton = ({ size, onClick }: { size: number; onClick: (e: Event) => void }) => {
    const activeSize = useStore($iconsSizeStore);

    return (
        <button
            key={`icon-size-${size}`}
            type='button'
            onClick={onClick}
            className={`${activeSize === size ? 'text-zinc-100' : 'text-zinc-400'} text-sm hover:text-zinc-100`}
        >
            {size}
        </button>
    );
};

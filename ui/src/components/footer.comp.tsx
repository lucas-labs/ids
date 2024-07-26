import { LucodearLogo } from './lucodear-logo.comp';

const sizes = [16, 32, 64, 128];

export function Footer() {
    return (
        <footer class='container mx-auto pt-4 pb-4 flex-1 flex flex-col justify-end items-center'>
            <a
                href='https://github.com/lucas-labs'
                target='_blank'
                rel='noopener noreferrer'
                class='text-zinc-50/10 hover:text-blue-600 transition-colors'
            >
                <LucodearLogo size={32} />
            </a>
        </footer>
    );
}

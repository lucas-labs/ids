import { persistentAtom } from '@nanostores/persistent';
import { createFetcherStore } from './contents.fetcher';
import type { Contents } from './types/store';

const contentsUrl = `${import.meta.env.IDS_SERVER_URL}_ids_runtime/contents.json`;

/** Stores the size of the icons using localStorage */
export const $iconsSizeStore = persistentAtom('icons-size', 16, {
    encode: JSON.stringify,
    decode: JSON.parse,
});

export const $contentsStore = createFetcherStore<Contents>([contentsUrl]);

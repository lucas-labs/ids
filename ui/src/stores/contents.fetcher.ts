import { nanoquery } from '@nanostores/query';
import type { ApiContents } from './types/api';
import type { Content, FileGroup } from './types/store';

/** Returns the name of the icon without the extension and theme */
const iconName = (path: string) => {
    const parts = path.replace(/\\/g, '/').split('/');
    const name = parts[parts.length - 1];
    return name.split('.')[0].replace('_light', '');
};

export const [createFetcherStore, createMutatorStore] = nanoquery({
    fetcher: (...keys: (string | number)[]) =>
        fetch(keys.join('')).then((r) =>
            r.json().then((data: ApiContents) =>
                data.map((content) => {
                    const files: FileGroup[] = content.files.reduce((acc, file) => {
                        // the api returns each file separately, whithout taking _light files as
                        // related to its dark counterpart. We group them here.

                        const name = iconName(file.path);

                        if (file.path.includes('_light')) {
                            // check if the dark version also exists, and if not, we use the light
                            // version as the dark version
                            const dark = content.files.find(
                                (f) => f.path === file.path.replace('_light', ''),
                            );
                            if (!dark) {
                                console.log('dark not found', file);
                                acc.push({
                                    status: file.status,
                                    dark: file,
                                    light: file,
                                    name,
                                });
                            }
                        } else {
                            // if the file is not a light version, we check if a light version
                            // exists and use it, or use the dark version as the light version
                            // otherwise

                            const light = content.files.find(
                                (f) => f.path === file.path.replace('.svg', '_light.svg'),
                            );

                            acc.push({
                                status: file.status,
                                dark: file,
                                light: light || file,
                                name,
                            });
                        }

                        return acc;
                    }, [] as FileGroup[]);

                    return { dir: content.dir, files } satisfies Content;
                }),
            ),
        ),
});

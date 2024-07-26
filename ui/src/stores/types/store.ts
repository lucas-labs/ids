import type { ApiFileResponse } from './api';

/** A group of a modified file, with a dark and light version */
export interface FileGroup {
    status: 'new' | 'mod' | 'del' | 'add' | '???';
    name: string;
    dark: File;
    light: File;
}

/** Represents a themed file */
export type File = Omit<ApiFileResponse, 'status'>;

export interface Content {
    dir: string;
    files: FileGroup[];
}

export type Contents = Content[];

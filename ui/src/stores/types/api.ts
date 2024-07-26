/** Content as returned by the server */
export interface ApiContentResponse {
    dir: string;
    files: ApiFileResponse[];
}

/* File with status, as returned by the server */
export interface ApiFileResponse {
    link: string;
    path: string;
    status: 'new' | 'mod' | 'del' | 'add' | '???';
}

/** List of contents as returned by the server */
export type ApiContents = ApiContentResponse[];

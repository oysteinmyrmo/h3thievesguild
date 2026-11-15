// Reading save files into a data set. The parsing runs in a worker (worker.ts), so the page
// stays responsive; `buildDataset` is also used directly by the tests in Node.js.

import { DatasetBuilder, type Dataset } from "./wasm/pkg/h3wasm";

/** A save file's name and contents. */
export interface SaveInput {
    name: string;
    bytes: Uint8Array;
}

/**
 * Why loading failed. `message` comes from the parser and names every file involved; `kind`
 * is one of the parser's error kinds, or "unexpected".
 */
export interface LoadError {
    kind: string;
    message: string;
}

export type LoadResult = { ok: true; dataset: Dataset } | { ok: false; error: LoadError };

/** Turn whatever loading threw into a `LoadError`. */
export function toLoadError(thrown: unknown): LoadError {
    if (thrown instanceof Error) {
        const kind = (thrown as Error & { kind?: unknown }).kind;
        return {
            kind: thrown.name === "DatasetError" && typeof kind === "string" ? kind : "unexpected",
            message: thrown.message,
        };
    }
    return { kind: "unexpected", message: String(thrown) };
}

/** Build a data set from save files. The WebAssembly module must be initialised. */
export function buildDataset(files: SaveInput[]): LoadResult {
    const builder = new DatasetBuilder();
    for (const file of files) {
        builder.add(file.name, file.bytes);
    }
    try {
        return { ok: true, dataset: builder.finish() };
    } catch (thrown) {
        return { ok: false, error: toLoadError(thrown) };
    }
}

export async function readFiles(files: Iterable<File>): Promise<SaveInput[]> {
    return Promise.all(
        [...files].map(async (file) => ({ name: file.name, bytes: new Uint8Array(await file.arrayBuffer()) })),
    );
}

/** Parse save files in a worker. */
export function loadSaves(inputs: SaveInput[]): Promise<LoadResult> {
    return new Promise((resolve) => {
        const worker = new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });
        const done = (result: LoadResult) => {
            worker.terminate();
            resolve(result);
        };
        worker.onmessage = (event: MessageEvent<LoadResult>) => done(event.data);
        worker.onerror = (event) => done({ ok: false, error: toLoadError(new Error(event.message || "The save parser failed.")) });
        worker.postMessage(inputs, inputs.map((input) => input.bytes.buffer as ArrayBuffer));
    });
}

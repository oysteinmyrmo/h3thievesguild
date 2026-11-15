// Reads the chosen files and parses them in a worker.

import type { LoadResult, SaveInput } from "./types";

export async function readFiles(files: Iterable<File>): Promise<SaveInput[]> {
    return Promise.all(
        [...files].map(async (file) => ({ name: file.name, bytes: new Uint8Array(await file.arrayBuffer()) })),
    );
}

export function loadSaves(inputs: SaveInput[]): Promise<LoadResult> {
    return new Promise((resolve) => {
        const worker = new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });
        worker.onmessage = (event: MessageEvent<LoadResult>) => {
            worker.terminate();
            resolve(event.data);
        };
        worker.onerror = (event) => {
            worker.terminate();
            resolve({ ok: false, error: { kind: "unexpected", message: event.message || "The save parser failed." } });
        };
        worker.postMessage(inputs, inputs.map((input) => input.bytes.buffer as ArrayBuffer));
    });
}

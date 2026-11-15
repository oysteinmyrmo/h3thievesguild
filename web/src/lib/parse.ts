// Building a data set with the WebAssembly module. Used by the worker, and by the tests in
// Node.js, so it does not touch the DOM.

import { DatasetBuilder } from "./wasm/pkg/h3wasm";
import type { LoadError, LoadResult, SaveInput } from "./types";

const KINDS = new Set(["noSaves", "unreadable", "mixedGames", "duplicateDays", "tooManyDays"]);

/** Turn whatever `finish` threw into a `LoadError`. */
export function toLoadError(thrown: unknown): LoadError {
    if (thrown instanceof Error && thrown.name === "DatasetError") {
        const fields = thrown as Error & { kind?: string; details?: unknown };
        if (fields.kind && KINDS.has(fields.kind)) {
            return { kind: fields.kind as LoadError["kind"], message: thrown.message, details: fields.details };
        }
    }
    const message = thrown instanceof Error ? thrown.message : String(thrown);
    return { kind: "unexpected", message };
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

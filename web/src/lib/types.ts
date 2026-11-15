// Types of the data set and its errors, from the TypeScript types the WebAssembly module ships.
export type { Dataset, DatasetError, Day, DayRange, HeroSeries, MapInfo, PlayerSeries, StatInfo } from "./wasm/pkg/h3wasm";

import type { Dataset } from "./wasm/pkg/h3wasm";

/** A save file's name and contents. */
export interface SaveInput {
    name: string;
    bytes: Uint8Array;
}

/** A `DatasetError` as plain data, so it can cross from the worker. */
export interface LoadError {
    kind: "noSaves" | "unreadable" | "mixedGames" | "duplicateDays" | "tooManyDays" | "unexpected";
    message: string;
    details?: unknown;
}

export type LoadResult = { ok: true; dataset: Dataset } | { ok: false; error: LoadError };

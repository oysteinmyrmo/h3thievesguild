// Remembers the last loaded game in the browser, so a reload shows it again. Storage can be
// unavailable or full; the page then simply does not remember.

import type { Dataset } from "./types";

export const STORAGE_KEY = "h3thievesguild.lastGame.v1";

export interface StoredGame {
    dataset: Dataset;
    savedAt: string;
}

/** Minimal storage interface, so tests can pass their own. */
export interface KeyValueStore {
    getItem(key: string): string | null;
    setItem(key: string, value: string): void;
    removeItem(key: string): void;
}

function store(): KeyValueStore | undefined {
    try {
        return globalThis.localStorage;
    } catch {
        return undefined;
    }
}

function looksLikeDataset(value: unknown): value is Dataset {
    const d = value as Partial<Dataset> | null;
    return (
        typeof d === "object" &&
        d !== null &&
        Array.isArray(d.days) &&
        Array.isArray(d.players) &&
        Array.isArray(d.playerStats) &&
        typeof d.map === "object"
    );
}

export function rememberGame(dataset: Dataset, storage: KeyValueStore | undefined = store()): boolean {
    try {
        const game: StoredGame = { dataset, savedAt: new Date().toISOString() };
        storage?.setItem(STORAGE_KEY, JSON.stringify(game));
        return storage !== undefined;
    } catch {
        return false;
    }
}

export function recallGame(storage: KeyValueStore | undefined = store()): StoredGame | undefined {
    try {
        const text = storage?.getItem(STORAGE_KEY);
        if (!text) {
            return undefined;
        }
        const game = JSON.parse(text) as Partial<StoredGame>;
        return looksLikeDataset(game.dataset) && typeof game.savedAt === "string"
            ? { dataset: game.dataset, savedAt: game.savedAt }
            : undefined;
    } catch {
        return undefined;
    }
}

export function forgetGame(storage: KeyValueStore | undefined = store()): void {
    try {
        storage?.removeItem(STORAGE_KEY);
    } catch {
        // Nothing to do: storage is unavailable.
    }
}

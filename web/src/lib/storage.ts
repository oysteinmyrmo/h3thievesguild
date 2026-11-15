// Remembers the last loaded game in the browser, so a reload shows it again. Storage can be
// unavailable or full; the page then simply does not remember.

import { cleanNames, type PlayerNames } from "./names";
import type { Dataset } from "./types";

export const STORAGE_KEY = "h3thievesguild.lastGame.v1";

export interface StoredGame {
    dataset: Dataset;
    /** Names the user gave the players. */
    names: PlayerNames;
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

export function rememberGame(
    dataset: Dataset,
    names: PlayerNames = {},
    storage: KeyValueStore | undefined = store(),
): boolean {
    try {
        const game: StoredGame = { dataset, names, savedAt: new Date().toISOString() };
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
        if (!looksLikeDataset(game.dataset) || typeof game.savedAt !== "string") {
            return undefined;
        }
        // Names are optional: games remembered before players could be named have none.
        const names = typeof game.names === "object" && game.names !== null ? cleanNames(game.names, game.dataset) : {};
        return { dataset: game.dataset, names, savedAt: game.savedAt };
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

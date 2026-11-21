// Remembers the last loaded game in the browser, so a reload shows it again. Storage can be
// unavailable or full; the page then simply does not remember.

import { cleanNames, type PlayerNames } from "./names";
import { cleanOptions, cleanThemeMode, type ChartOptions, type ThemeMode } from "./options";
import type { Dataset } from "./wasm/pkg/h3wasm";

export const STORAGE_KEY = "h3thievesguild.lastGame.v1";
export const OPTIONS_KEY = "h3thievesguild.chartOptions.v1";
export const THEME_KEY = "h3thievesguild.theme.v1";

export interface StoredGame {
    dataset: Dataset;
    /** Names the user gave the players. */
    names: PlayerNames;
    savedAt: string;
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

/** Store the game. Returns whether it could. */
export function rememberGame(dataset: Dataset, names: PlayerNames = {}): boolean {
    try {
        const game: StoredGame = { dataset, names, savedAt: new Date().toISOString() };
        localStorage.setItem(STORAGE_KEY, JSON.stringify(game));
        return true;
    } catch {
        return false;
    }
}

export function recallGame(): StoredGame | undefined {
    try {
        const text = localStorage.getItem(STORAGE_KEY);
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

/** Store the chart options. Returns whether it could. */
export function rememberOptions(options: ChartOptions): boolean {
    try {
        localStorage.setItem(OPTIONS_KEY, JSON.stringify(options));
        return true;
    } catch {
        return false;
    }
}

/** The stored chart options, with defaults for any missing or not valid. */
export function recallOptions(): ChartOptions {
    try {
        const text = localStorage.getItem(OPTIONS_KEY);
        return cleanOptions(text ? JSON.parse(text) : undefined);
    } catch {
        return cleanOptions(undefined);
    }
}

/** Store the light or dark choice. Returns whether it could. */
export function rememberThemeMode(mode: ThemeMode): boolean {
    try {
        localStorage.setItem(THEME_KEY, mode);
        return true;
    } catch {
        return false;
    }
}

/** The stored light or dark choice, or the system's if there is none. */
export function recallThemeMode(): ThemeMode {
    try {
        return cleanThemeMode(localStorage.getItem(THEME_KEY));
    } catch {
        return "system";
    }
}

export function forgetGame(): void {
    try {
        localStorage.removeItem(STORAGE_KEY);
    } catch {
        // Nothing to do: storage is unavailable.
    }
}

// The loaded game and the user's choices about it, shared by the components.

import { cleanNames, type PlayerNames } from "./names";
import type { ChartOptions } from "./options";
import { forgetGame, recallGame, recallOptions, rememberGame, rememberOptions } from "./storage";
import type { LoadError } from "./load";
import type { Dataset } from "./wasm/pkg/h3wasm";

class Game {
    dataset = $state<Dataset | undefined>(undefined);
    /** IDs of players hidden in every chart. */
    hidden = $state<ReadonlySet<string>>(new Set());
    /** Names the user gave the players of this game. */
    names = $state<PlayerNames>({});
    /** Set when the game was restored from the last visit rather than loaded now. */
    restoredAt = $state<string | undefined>(undefined);
    error = $state<LoadError | undefined>(undefined);
    loading = $state(false);
    /** How the charts are drawn, for every game. */
    options = $state<ChartOptions>(recallOptions());

    restore(): void {
        const stored = recallGame();
        if (stored) {
            this.dataset = stored.dataset;
            this.names = stored.names;
            this.restoredAt = stored.savedAt;
        }
    }

    show(dataset: Dataset): void {
        this.dataset = dataset;
        this.hidden = new Set();
        this.names = {};
        this.restoredAt = undefined;
        this.error = undefined;
        rememberGame(dataset);
    }

    /** Name the players; an empty name gives a player back its colour's name. */
    rename(names: Record<string, string>): void {
        if (!this.dataset) {
            return;
        }
        this.names = cleanNames(names, this.dataset);
        rememberGame(this.dataset, this.names);
    }

    clear(): void {
        this.dataset = undefined;
        this.hidden = new Set();
        this.names = {};
        this.restoredAt = undefined;
        this.error = undefined;
        forgetGame();
    }

    setOptions(changes: Partial<ChartOptions>): void {
        this.options = { ...this.options, ...changes };
        rememberOptions(this.options);
    }

    togglePlayer(id: string): void {
        const hidden = new Set(this.hidden);
        if (hidden.has(id)) {
            hidden.delete(id);
        } else {
            hidden.add(id);
        }
        this.hidden = hidden;
    }
}

export const game = new Game();

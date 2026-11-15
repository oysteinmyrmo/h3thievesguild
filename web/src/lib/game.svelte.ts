// The loaded game and the user's choices about it, shared by the components.

import { forgetGame, recallGame, rememberGame } from "./storage";
import type { Dataset, LoadError } from "./types";

class Game {
    dataset = $state<Dataset | undefined>(undefined);
    /** IDs of players hidden in every chart. */
    hidden = $state<ReadonlySet<string>>(new Set());
    /** Set when the game was restored from the last visit rather than loaded now. */
    restoredAt = $state<string | undefined>(undefined);
    error = $state<LoadError | undefined>(undefined);
    loading = $state(false);

    restore(): void {
        const stored = recallGame();
        if (stored) {
            this.dataset = stored.dataset;
            this.restoredAt = stored.savedAt;
        }
    }

    show(dataset: Dataset): void {
        this.dataset = dataset;
        this.hidden = new Set();
        this.restoredAt = undefined;
        this.error = undefined;
        rememberGame(dataset);
    }

    clear(): void {
        this.dataset = undefined;
        this.hidden = new Set();
        this.restoredAt = undefined;
        this.error = undefined;
        forgetGame();
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

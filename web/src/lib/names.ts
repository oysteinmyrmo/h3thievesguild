// Names the user gives the players. A player without a name keeps its colour's name.

import type { Dataset, PlayerSeries } from "./wasm/pkg/h3wasm";

/** Names by player colour ID, such as `{ red: "Alice" }`. */
export type PlayerNames = Readonly<Record<string, string>>;

/** The name to show for a player: the user's name for it, or its colour's name. */
export function displayName(player: Pick<PlayerSeries, "id" | "name">, names: PlayerNames): string {
    const given = names[player.id];
    return given !== undefined && given.trim() !== "" ? given : player.name;
}

/**
 * Names worth keeping: those of players in the data set that are not empty. Any characters are
 * allowed; a name of only spaces counts as empty.
 */
export function cleanNames(names: Record<string, unknown>, dataset: Dataset): PlayerNames {
    const ids = new Set(dataset.players.map((p) => p.id));
    const clean: Record<string, string> = {};
    for (const [id, name] of Object.entries(names)) {
        if (ids.has(id) && typeof name === "string" && name.trim() !== "") {
            clean[id] = name;
        }
    }
    return clean;
}

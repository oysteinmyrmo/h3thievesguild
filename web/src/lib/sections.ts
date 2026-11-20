// The categories of the player statistics, such as Economy or Kingdom, and the order to step
// through them.

import type { StatInfo } from "./wasm/pkg/h3wasm";

export interface Category {
    group: string;
    stats: StatInfo[];
}

/** The statistics by category, the categories in the order they first appear. */
export function categories(stats: readonly StatInfo[]): Category[] {
    const byGroup = new Map<string, StatInfo[]>();
    for (const stat of stats) {
        byGroup.set(stat.group, [...(byGroup.get(stat.group) ?? []), stat]);
    }
    return [...byGroup].map(([group, inGroup]) => ({ group, stats: inGroup }));
}

/** Where a chart is, and the charts before and after it, wrapping around at the ends. */
export interface Position {
    current: StatInfo;
    previous: StatInfo;
    next: StatInfo;
    category: Category;
}

/**
 * The chart with this ID, or the first chart if there is none, among the statistics in category
 * order. Undefined only if there are no statistics.
 */
export function position(cats: readonly Category[], id: string | undefined): Position | undefined {
    const ordered = cats.flatMap((c) => c.stats);
    const found = ordered.findIndex((s) => s.id === id);
    const index = Math.max(found, 0);
    const current = ordered[index];
    const category = cats.find((c) => c.stats.includes(current!));
    if (!current || !category) {
        return undefined;
    }
    return {
        current,
        previous: ordered[(index - 1 + ordered.length) % ordered.length]!,
        next: ordered[(index + 1) % ordered.length]!,
        category,
    };
}

import { describe, expect, it } from "vitest";
import { categories, position } from "../../src/lib/sections";
import { RESOURCES, dataset } from "./saves";

const stat = (id: string, group: string) => ({ id, name: id.toUpperCase(), group, unit: "count" as const });

describe("categories", () => {
    it("groups the player statistics as Economy, Army, Kingdom and Map, in order", () => {
        const stats = dataset(RESOURCES).playerStats;
        const result = categories(stats);
        expect(result.map((c) => c.group)).toEqual(["Economy", "Army", "Kingdom", "Map"]);
        expect(result.map((c) => c.stats.length)).toEqual([7, 2, 6, 1]);
        expect(result[1]!.stats.map((s) => s.id)).toEqual(["armyStrength", "bestCreature"]);
        expect(result[3]!.stats.map((s) => s.id)).toEqual(["mapExplored"]);
        expect(result[0]!.stats.map((s) => s.id)).toEqual(["gold", "wood", "ore", "mercury", "sulfur", "crystal", "gems"]);
        expect(result[2]!.stats.map((s) => s.id)).toEqual([
            "heroes", "heroesOnMap", "totalHeroLevels", "highestHeroLevel", "totalExperience", "towns",
        ]);
        expect(result.flatMap((c) => c.stats)).toHaveLength(stats.length);
    });

    it("keeps a category together and in first-seen order, even when its statistics are apart", () => {
        const result = categories([stat("a", "X"), stat("b", "Y"), stat("c", "X"), stat("d", "Z")]);
        expect(result.map((c) => [c.group, c.stats.map((s) => s.id)])).toEqual([
            ["X", ["a", "c"]],
            ["Y", ["b"]],
            ["Z", ["d"]],
        ]);
        expect(categories([])).toEqual([]);
    });
});

describe("position", () => {
    const cats = categories([stat("a", "X"), stat("b", "X"), stat("c", "Y"), stat("d", "Z")]);
    const ids = (id: string | undefined) => {
        const at = position(cats, id)!;
        return [at.previous.id, at.current.id, at.next.id, at.category.group];
    };

    it("steps through the charts in category order, across categories and around the ends", () => {
        expect(ids("a")).toEqual(["d", "a", "b", "X"]);
        expect(ids("b")).toEqual(["a", "b", "c", "X"]);
        expect(ids("c")).toEqual(["b", "c", "d", "Y"]);
        expect(ids("d")).toEqual(["c", "d", "a", "Z"]);
    });

    it("shows the first chart for no chart or an unknown one, and nothing without charts", () => {
        expect(ids(undefined)).toEqual(["d", "a", "b", "X"]);
        expect(ids("nonsense")).toEqual(["d", "a", "b", "X"]);
        expect(position([], "a")).toBeUndefined();
        const one = categories([stat("a", "X")]);
        expect(position(one, "a")).toMatchObject({ previous: { id: "a" }, current: { id: "a" }, next: { id: "a" } });
    });
});

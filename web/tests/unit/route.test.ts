import { describe, expect, it } from "vitest";
import { parseRoute, routeHash } from "../../src/lib/route";

describe("routes", () => {
    it.each([
        ["", { page: "saves" }],
        ["#", { page: "saves" }],
        ["#/saves", { page: "saves" }],
        ["#/saves/", { page: "saves" }],
        ["#/saves/gold", { page: "saves", stat: "gold" }],
        ["#/calculators", { page: "calculators" }],
        ["#/nowhere", { page: "saves" }],
    ])("parses %j", (hash, route) => {
        expect(parseRoute(hash)).toEqual(route);
    });

    it("round-trips every route", () => {
        for (const route of [{ page: "saves" }, { page: "saves", stat: "armyStrength" }, { page: "calculators" }] as const) {
            expect(parseRoute(routeHash(route))).toEqual(route);
        }
    });

    it("encodes statistic IDs", () => {
        expect(routeHash({ page: "saves", stat: "a b" })).toBe("#/saves/a%20b");
        expect(parseRoute("#/saves/a%20b")).toEqual({ page: "saves", stat: "a b" });
    });
});

// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { STORAGE_KEY, forgetGame, recallGame, rememberGame } from "../../src/lib/storage";
import { RESOURCES, dataset } from "./saves";

beforeEach(() => localStorage.clear());
afterEach(() => {
    vi.restoreAllMocks();
    vi.unstubAllGlobals();
});

describe("remembering the last game", () => {
    const game = dataset(RESOURCES);

    it("stores and restores the data set", () => {
        expect(rememberGame(game, {})).toBe(true);
        const restored = recallGame();
        expect(restored?.dataset).toEqual(game);
        expect(restored?.names).toEqual({});
        expect(Date.parse(restored!.savedAt)).not.toBeNaN();
    });

    it("stores the players' names with it, whatever characters they have", () => {
        const names = { red: "Ålesund {a} <b>&\"'", blue: "  spaced  ", teal: "👑 Ἀθηνᾶ" };
        rememberGame(game, names);
        expect(recallGame()?.names).toEqual(names);
    });

    it("keeps only names of players in the game, and only text", () => {
        const stored = { dataset: game, savedAt: new Date().toISOString(), names: { red: "Alice", nobody: "x", blue: 7, tan: "   " } };
        localStorage.setItem(STORAGE_KEY, JSON.stringify(stored));
        expect(recallGame()?.names).toEqual({ red: "Alice" });
    });

    it("restores games remembered before players had names", () => {
        localStorage.setItem(STORAGE_KEY, JSON.stringify({ dataset: game, savedAt: new Date().toISOString() }));
        expect(recallGame()?.names).toEqual({});
        localStorage.setItem(STORAGE_KEY, JSON.stringify({ dataset: game, savedAt: new Date().toISOString(), names: null }));
        expect(recallGame()?.names).toEqual({});
    });

    it("forgets it", () => {
        rememberGame(game, {});
        forgetGame();
        expect(recallGame()).toBeUndefined();
    });

    it("ignores missing, damaged or foreign data", () => {
        expect(recallGame()).toBeUndefined();
        for (const text of ["not json", "null", "{}", '{"dataset":{},"savedAt":"x"}', '{"dataset":{"days":[]}}']) {
            localStorage.setItem(STORAGE_KEY, text);
            expect(recallGame()).toBeUndefined();
        }
    });

    it("copes without storage, or with storage that fails", () => {
        vi.stubGlobal("localStorage", undefined);
        expect(rememberGame(game, {})).toBe(false);
        expect(recallGame()).toBeUndefined();
        expect(() => forgetGame()).not.toThrow();
        vi.unstubAllGlobals();
        const fail = () => {
            throw new Error("quota or denied");
        };
        vi.spyOn(localStorage, "getItem").mockImplementation(fail);
        vi.spyOn(localStorage, "setItem").mockImplementation(fail);
        vi.spyOn(localStorage, "removeItem").mockImplementation(fail);
        expect(rememberGame(game, {})).toBe(false);
        expect(recallGame()).toBeUndefined();
        expect(() => forgetGame()).not.toThrow();
    });
});

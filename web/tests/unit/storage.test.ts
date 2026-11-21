// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { DEFAULT_OPTIONS, cleanOptions } from "../../src/lib/options";
import {
    OPTIONS_KEY,
    STORAGE_KEY,
    THEME_KEY,
    forgetGame,
    recallGame,
    recallOptions,
    recallThemeMode,
    rememberGame,
    rememberOptions,
    rememberThemeMode,
} from "../../src/lib/storage";
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
        const spies = [
            vi.spyOn(localStorage, "getItem").mockImplementation(fail),
            vi.spyOn(localStorage, "setItem").mockImplementation(fail),
            vi.spyOn(localStorage, "removeItem").mockImplementation(fail),
        ];
        expect(rememberGame(game, {})).toBe(false);
        expect(recallGame()).toBeUndefined();
        expect(() => forgetGame()).not.toThrow();
        // Restored here: restoreAllMocks in afterEach does not undo spies on happy-dom's storage.
        spies.forEach((spy) => spy.mockRestore());
    });
});

describe("remembering the chart options", () => {
    it("numbers the days on the x-axis until told otherwise", () => {
        expect(DEFAULT_OPTIONS).toEqual({ xAxis: "day", chartType: "lines" });
        expect(recallOptions()).toEqual(DEFAULT_OPTIONS);
    });

    it("stores and restores them, apart from the game", () => {
        expect(rememberOptions({ xAxis: "date", chartType: "share" })).toBe(true);
        expect(recallOptions()).toEqual({ xAxis: "date", chartType: "share" });
        forgetGame();
        expect(recallOptions()).toEqual({ xAxis: "date", chartType: "share" });
    });

    it("keeps options stored before there was a chart type, with lines", () => {
        localStorage.setItem(OPTIONS_KEY, '{"xAxis":"date"}');
        expect(recallOptions()).toEqual({ xAxis: "date", chartType: "lines" });
    });

    it("falls back to the default for anything not valid", () => {
        for (const text of ["not json", "null", "7", "{}", '{"xAxis":"weeks"}', '{"xAxis":1}', '{"chartType":"pie"}']) {
            localStorage.setItem(OPTIONS_KEY, text);
            expect(recallOptions()).toEqual(DEFAULT_OPTIONS);
        }
        expect(cleanOptions({ xAxis: "date", other: 1 })).toEqual({ xAxis: "date", chartType: "lines" });
        expect(cleanOptions({ xAxis: "nope", chartType: "share" })).toEqual({ xAxis: "day", chartType: "share" });
    });

    it("copes without storage, or with storage that fails", () => {
        vi.stubGlobal("localStorage", undefined);
        expect(rememberOptions({ xAxis: "date", chartType: "share" })).toBe(false);
        expect(recallOptions()).toEqual(DEFAULT_OPTIONS);
        vi.unstubAllGlobals();
        const spy = vi.spyOn(localStorage, "getItem").mockImplementation(() => {
            throw new Error("denied");
        });
        expect(recallOptions()).toEqual(DEFAULT_OPTIONS);
        spy.mockRestore();
    });
});

describe("remembering the theme", () => {
    it("follows the system until told otherwise", () => {
        expect(recallThemeMode()).toBe("system");
    });

    it("stores and restores light or dark, apart from the game", () => {
        for (const mode of ["dark", "light", "system"] as const) {
            expect(rememberThemeMode(mode)).toBe(true);
            expect(recallThemeMode()).toBe(mode);
        }
        rememberThemeMode("dark");
        forgetGame();
        expect(recallThemeMode()).toBe("dark");
    });

    it("falls back to the system for anything not valid", () => {
        for (const text of ["", "Dark", "blue", '"dark"', "null"]) {
            localStorage.setItem(THEME_KEY, text);
            expect(recallThemeMode()).toBe("system");
        }
    });

    it("copes without storage", () => {
        vi.stubGlobal("localStorage", undefined);
        expect(rememberThemeMode("dark")).toBe(false);
        expect(recallThemeMode()).toBe("system");
    });
});

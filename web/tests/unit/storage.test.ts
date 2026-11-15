import { describe, expect, it } from "vitest";
import { STORAGE_KEY, forgetGame, recallGame, rememberGame, type KeyValueStore } from "../../src/lib/storage";
import { RESOURCES, dataset } from "./saves";

class MemoryStore implements KeyValueStore {
    items = new Map<string, string>();
    getItem(key: string) {
        return this.items.get(key) ?? null;
    }
    setItem(key: string, value: string) {
        this.items.set(key, value);
    }
    removeItem(key: string) {
        this.items.delete(key);
    }
}

describe("remembering the last game", () => {
    const game = dataset(RESOURCES);

    it("stores and restores the data set", () => {
        const store = new MemoryStore();
        expect(rememberGame(game, {}, store)).toBe(true);
        const restored = recallGame(store);
        expect(restored?.dataset).toEqual(game);
        expect(restored?.names).toEqual({});
        expect(Date.parse(restored!.savedAt)).not.toBeNaN();
    });

    it("stores the players' names with it, whatever characters they have", () => {
        const store = new MemoryStore();
        const names = { red: "Ålesund {a} <b>&\"'", blue: "  spaced  ", teal: "👑 Ἀθηνᾶ" };
        rememberGame(game, names, store);
        expect(recallGame(store)?.names).toEqual(names);
    });

    it("keeps only names of players in the game, and only text", () => {
        const store = new MemoryStore();
        const stored = { dataset: game, savedAt: new Date().toISOString(), names: { red: "Alice", nobody: "x", blue: 7, tan: "   " } };
        store.setItem(STORAGE_KEY, JSON.stringify(stored));
        expect(recallGame(store)?.names).toEqual({ red: "Alice" });
    });

    it("restores games remembered before players had names", () => {
        const store = new MemoryStore();
        store.setItem(STORAGE_KEY, JSON.stringify({ dataset: game, savedAt: new Date().toISOString() }));
        expect(recallGame(store)?.names).toEqual({});
        store.setItem(STORAGE_KEY, JSON.stringify({ dataset: game, savedAt: new Date().toISOString(), names: null }));
        expect(recallGame(store)?.names).toEqual({});
    });

    it("forgets it", () => {
        const store = new MemoryStore();
        rememberGame(game, {}, store);
        forgetGame(store);
        expect(recallGame(store)).toBeUndefined();
    });

    it("ignores missing, damaged or foreign data", () => {
        const store = new MemoryStore();
        expect(recallGame(store)).toBeUndefined();
        for (const text of ["not json", "null", "{}", '{"dataset":{},"savedAt":"x"}', '{"dataset":{"days":[]}}']) {
            store.setItem(STORAGE_KEY, text);
            expect(recallGame(store)).toBeUndefined();
        }
    });

    it("copes without storage, or with storage that fails", () => {
        expect(rememberGame(game, {}, undefined)).toBe(false);
        expect(recallGame(undefined)).toBeUndefined();
        const full: KeyValueStore = {
            getItem: () => {
                throw new Error("denied");
            },
            setItem: () => {
                throw new Error("quota");
            },
            removeItem: () => {
                throw new Error("denied");
            },
        };
        expect(rememberGame(game, {}, full)).toBe(false);
        expect(recallGame(full)).toBeUndefined();
        expect(() => forgetGame(full)).not.toThrow();
    });
});

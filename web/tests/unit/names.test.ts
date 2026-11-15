import { describe, expect, it } from "vitest";
import { cleanNames, displayName } from "../../src/lib/names";
import { RESOURCES, dataset } from "./saves";

describe("player names", () => {
    const red = { id: "red", name: "Red" };

    it("shows the given name, or the colour's name without one", () => {
        expect(displayName(red, { red: "Alice" })).toBe("Alice");
        expect(displayName(red, {})).toBe("Red");
        expect(displayName(red, { blue: "Bob" })).toBe("Red");
        expect(displayName(red, { red: "" })).toBe("Red");
        expect(displayName(red, { red: " \t " })).toBe("Red");
    });

    it("allows any character, kept as typed", () => {
        for (const name of ["{a}", "<script>", "a & b", '"quoted"', "Ålesund", "👑", "  padded  ", "tab\there", "x".repeat(200)]) {
            expect(displayName(red, { red: name })).toBe(name);
        }
    });

    it("keeps only non-empty names of the game's players", () => {
        const game = dataset(RESOURCES);
        expect(cleanNames({ red: "Alice", blue: "", tan: "  ", nobody: "Zed", green: 42, orange: "Ola" }, game)).toEqual({
            red: "Alice",
            orange: "Ola",
        });
    });
});

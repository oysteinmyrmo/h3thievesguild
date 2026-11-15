import { describe, expect, it } from "vitest";
import { buildDataset, toLoadError } from "../../src/lib/load";
import { RESOURCES, dataset, initWasm, save } from "./saves";

describe("buildDataset with the WebAssembly parser", () => {
    it("builds a data set from a game's saves, in date order", () => {
        const d = dataset([RESOURCES[2]!, RESOURCES[0]!, RESOURCES[1]!]);
        expect(d.days.map((day) => day.label)).toEqual(["1-1-1", "1-1-2", "1-1-3"]);
        expect(d.players).toHaveLength(8);
        const blue = d.players.find((p) => p.id === "blue")!;
        // From the resources test folder's info.txt; blue is vanquished on day 3.
        expect(blue.values.gold).toEqual([3400, 6700, null]);
        expect(blue.eliminatedDay).toBe(3);
        expect(d.missingDaysWarning).toBeNull();
    });

    it("keeps a day without a save, with a warning", () => {
        const d = dataset([RESOURCES[0]!, RESOURCES[2]!]);
        expect(d.days.map((day) => day.file)).toEqual(["[hotseat] 111.GM8", null, "[hotseat] 113.GM5"]);
        expect(d.missingDays).toEqual([{ first: 2, last: 2, label: "Month 1, Week 1, Day 2" }]);
        expect(d.missingDaysWarning).toContain("Continue?");
    });

    it.each([
        ["duplicate days", [save("resources", "[hotseat] 111.GM8", "a"), save("resources", "[hotseat] 111.GM8", "b")], "duplicateDays"],
        ["mixed games", [save("resources", "[hotseat] 111.GM8"), save("towns", "[hotseat] 111.GM2")], "mixedGames"],
        ["unreadable files", [{ name: "notes.txt", bytes: new TextEncoder().encode("hello") }], "unreadable"],
        ["no files", [], "noSaves"],
    ] as const)("reports %s as an error with its kind and a message naming the files", (_, inputs, kind) => {
        initWasm();
        const result = buildDataset([...inputs]);
        expect(result.ok).toBe(false);
        if (!result.ok) {
            expect(result.error.kind).toBe(kind);
            for (const input of inputs) {
                expect(result.error.message).toContain(input.name);
            }
            expect(result.error.message.length).toBeGreaterThan(0);
        }
    });
});

describe("toLoadError", () => {
    it("keeps the kind and message of a DatasetError", () => {
        const error = Object.assign(new Error("two saves"), { name: "DatasetError", kind: "duplicateDays", details: [1] });
        expect(toLoadError(error)).toEqual({ kind: "duplicateDays", message: "two saves" });
    });

    it("treats anything else as unexpected", () => {
        expect(toLoadError(new TypeError("boom"))).toEqual({ kind: "unexpected", message: "boom" });
        expect(toLoadError("text")).toEqual({ kind: "unexpected", message: "text" });
        const kindless = Object.assign(new Error("odd"), { name: "DatasetError" });
        expect(toLoadError(kindless).kind).toBe("unexpected");
        const notDatasetError = Object.assign(new Error("odd"), { kind: "duplicateDays" });
        expect(toLoadError(notDatasetError).kind).toBe("unexpected");
    });
});

// The TypeScript types the WebAssembly module ships are generated from the Rust types. These
// tests check them against real data: every field of a data set and of each error is declared.

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { RESOURCES, dataset } from "./saves";

const declarations = readFileSync(
    join(dirname(fileURLToPath(import.meta.url)), "..", "..", "src", "lib", "wasm", "pkg", "h3wasm.d.ts"),
    "utf8",
);

/** Every key of every object in `value`, except the keys of `values`, `labels` and `icons` maps: statistic IDs. */
function keys(value: unknown, out = new Set<string>()): Set<string> {
    if (Array.isArray(value)) {
        value.forEach((item) => keys(item, out));
    } else if (typeof value === "object" && value !== null) {
        for (const [key, item] of Object.entries(value)) {
            out.add(key);
            if (key !== "values" && key !== "labels" && key !== "icons") {
                keys(item, out);
            }
        }
    }
    return out;
}

function declared(field: string): boolean {
    return new RegExp(`\\b${field}\\??:`).test(declarations);
}

describe("the generated TypeScript types", () => {
    it("name every field of the data set and errors", () => {
        // A data set with a missing day, so the missing-day fields have values.
        const fields = keys(dataset([RESOURCES[0]!, RESOURCES[2]!]));
        const errors = [
            { kind: "unreadable", details: [{ file: "f", reason: "r" }] },
            { kind: "mixedGames", details: [{ mapName: "m", mapSize: "S", underground: false, files: [] }] },
            { kind: "duplicateDays", details: [{ date: "d", files: [] }] },
            { kind: "tooManyDays", details: { days: 1, first: { date: "d", files: [] }, last: { date: "d", files: [] } } },
        ];
        keys(errors, fields);
        for (const field of fields) {
            expect(declared(field), `TypeScript types lack \`${field}\``).toBe(true);
        }
        for (const kind of ["noSaves", "unreadable", "mixedGames", "duplicateDays", "tooManyDays"]) {
            expect(declarations).toContain(`"${kind}"`);
        }
    });

    it("type the data set's missing values as null", () => {
        expect(declarations).toMatch(/file: string \| null;/);
        expect(declarations).toMatch(/eliminatedDay: number \| null;/);
        expect(declarations).toMatch(/missingDaysWarning: string \| null;/);
        expect(declarations).toMatch(/values: Record<string, \(number \| null\)\[\]>;/);
        expect(declarations).toMatch(/labels: Record<string, \(string \| null\)\[\]>;/);
        expect(declarations).toMatch(/icons: Record<string, \(string \| null\)\[\]>;/);
        expect(declarations).toContain("finish(): Dataset;");
    });
});

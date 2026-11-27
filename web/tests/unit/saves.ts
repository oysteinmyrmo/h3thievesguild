// Test saves from the repository's test_data folder, and the WebAssembly module set up for
// Node.js.

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { initSync } from "../../src/lib/wasm/pkg/h3wasm";
import { buildDataset, type SaveInput } from "../../src/lib/load";
import type { Dataset } from "../../src/lib/wasm/pkg/h3wasm";

const here = dirname(fileURLToPath(import.meta.url));
const testData = join(here, "..", "..", "..", "test_data");

let initialised = false;

export function initWasm(): void {
    if (!initialised) {
        const wasm = readFileSync(join(here, "..", "..", "src", "lib", "wasm", "pkg", "h3wasm_bg.wasm"));
        initSync({ module: wasm });
        initialised = true;
    }
}

/** A save from test_data, under its own name or the one given. */
export function save(folder: string, file: string, name = file): SaveInput {
    return { name, bytes: new Uint8Array(readFileSync(join(testData, folder, file))) };
}

/** A data set from test saves, which must load. */
export function dataset(inputs: SaveInput[]): Dataset {
    initWasm();
    const result = buildDataset(inputs);
    if (!result.ok) {
        throw new Error(result.error.message);
    }
    return result.dataset;
}

export const MAP_VISIBILITY = ["[hotseat] 111.GM5", "[hotseat] 112.GM5", "[hotseat] 113.GM5"].map((f) =>
    save("map_visibility", f),
);

export const CREATURES = ["[hotseat] 111.GM8", "[hotseat] 112.GM8"].map((f) => save("creatures", f));

export const RESOURCES = ["[hotseat] 111.GM8", "[hotseat] 112.GM8", "[hotseat] 113.GM5"].map((f) => save("resources", f));

// Parses save files off the main thread, so the page stays responsive.

import init from "./wasm/pkg/h3wasm";
import { buildDataset, toLoadError } from "./parse";
import type { LoadResult, SaveInput } from "./types";

const ready = init();

self.onmessage = async (event: MessageEvent<SaveInput[]>) => {
    let result: LoadResult;
    try {
        await ready;
        result = buildDataset(event.data);
    } catch (thrown) {
        result = { ok: false, error: toLoadError(thrown) };
    }
    self.postMessage(result);
};

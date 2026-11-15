// Parses save files off the main thread.

import init from "./wasm/pkg/h3wasm";
import { buildDataset, toLoadError, type LoadResult, type SaveInput } from "./load";

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

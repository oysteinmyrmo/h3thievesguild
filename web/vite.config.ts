import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vitest/config";

export default defineConfig({
    plugins: [svelte()],
    // Relative paths, so the built site works from any folder or sub-path.
    base: "./",
    worker: { format: "es" },
    // ECharts is most of the JavaScript; the line chart alone is about 200 kB gzipped.
    build: { chunkSizeWarningLimit: 800 },
    test: {
        include: ["tests/unit/**/*.test.ts"],
        environment: "node",
    },
});

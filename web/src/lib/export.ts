// Charts saved as PNG images: one at a time, or all of them in a ZIP file.
//
// Each image is drawn from scratch at a fixed size with the chart's title, so it looks the same
// whatever the size of the window.

import { statChartOption, type ChartSettings } from "./chart";
import { echarts } from "./echarts";
import { zipStore } from "./zip";
import type { Dataset, StatInfo } from "./wasm/pkg/h3wasm";

export const EXPORT_WIDTH = 1600;
export const EXPORT_HEIGHT = 900;
/** Pixels per chart pixel, so the image stays sharp on high-resolution screens and in print. */
export const EXPORT_PIXEL_RATIO = 2;

/** Text without control characters, which some maps put in their names. */
export function printable(text: string): string {
    return text.replace(/[\u0000-\u001f\u007f]+/g, " ").replace(/\s+/g, " ").trim();
}

/** A file name from any text: no characters that file systems reject, never empty. */
export function fileName(text: string, extension: string): string {
    const base = printable(text)
        .replace(/[\\/:*?"<>|]+/g, "_")
        .replace(/^[.\s]+|[.\s]+$/g, "")
        .slice(0, 120);
    return `${base || "chart"}.${extension}`;
}

/** The image's name: the map and the statistic, such as "My map - Gold.png". */
export function chartFileName(dataset: Dataset, stat: StatInfo): string {
    return fileName(`${printable(dataset.map.name)} - ${stat.name}`, "png");
}

/** Draw one chart off screen and return it as a PNG image. */
export async function chartPng(dataset: Dataset, stat: StatInfo, settings: Omit<ChartSettings, "compact" | "title">): Promise<Blob> {
    const element = document.createElement("div");
    const chart = echarts.init(element, null, { renderer: "canvas", width: EXPORT_WIDTH, height: EXPORT_HEIGHT });
    try {
        const name = settings.chartType === "share" ? `${stat.name}, share of the total` : stat.name;
        const title = { text: name, subtext: printable(dataset.map.name) };
        chart.setOption(statChartOption(dataset, stat.id, { ...settings, compact: false, title }));
        const url = chart.getDataURL({
            type: "png",
            pixelRatio: EXPORT_PIXEL_RATIO,
            backgroundColor: settings.theme.tooltipBackground,
        });
        return await (await fetch(url)).blob();
    } finally {
        chart.dispose();
    }
}

/** Every chart as a PNG image in one ZIP file. */
export async function chartsZip(dataset: Dataset, settings: Omit<ChartSettings, "compact" | "title">): Promise<Blob> {
    const entries = [];
    for (const stat of dataset.playerStats) {
        const png = await chartPng(dataset, stat, settings);
        entries.push({ name: chartFileName(dataset, stat), data: new Uint8Array(await png.arrayBuffer()) });
    }
    return new Blob([zipStore(entries)], { type: "application/zip" });
}

/** Have the browser save the file. */
export function download(blob: Blob, name: string): void {
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = name;
    document.body.append(link);
    link.click();
    link.remove();
    // Some browsers read the file after the click returns.
    setTimeout(() => URL.revokeObjectURL(url), 10_000);
}

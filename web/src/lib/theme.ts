// Chart colours from the CSS custom properties of the page.

import type { ChartTheme } from "./chart";

export function readTheme(element: Element = document.documentElement): ChartTheme {
    const style = getComputedStyle(element);
    const value = (name: string, fallback: string) => style.getPropertyValue(name).trim() || fallback;
    return {
        text: value("--text", "#1d1d1f"),
        muted: value("--text-muted", "#6e6e73"),
        gridLine: value("--border", "#d9d9de"),
        missing: value("--missing", "rgba(128, 128, 128, 0.12)"),
        tooltipBackground: value("--surface", "#ffffff"),
    };
}

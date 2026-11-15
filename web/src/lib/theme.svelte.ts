// Chart colours from the page's CSS custom properties, updated once for the whole page when
// the system switches between light and dark.

/** Colours from the page's theme, so charts follow light and dark mode. */
export interface ChartTheme {
    text: string;
    muted: string;
    gridLine: string;
    missing: string;
    tooltipBackground: string;
}

function readTheme(): ChartTheme {
    const style = getComputedStyle(document.documentElement);
    const value = (name: string, fallback: string) => style.getPropertyValue(name).trim() || fallback;
    return {
        text: value("--text", "#1d1d1f"),
        muted: value("--text-muted", "#6e6e73"),
        gridLine: value("--border", "#d9d9de"),
        missing: value("--missing", "rgba(128, 128, 128, 0.12)"),
        tooltipBackground: value("--surface", "#ffffff"),
    };
}

class Theme {
    current = $state<ChartTheme>(readTheme());

    constructor() {
        matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
            this.current = readTheme();
        });
    }
}

export const theme = new Theme();

// The page's light or dark theme, and the chart colours that go with it.
//
// The theme is the system's unless the user picks light or dark in the sidebar; the choice sets
// data-theme on the page, which selects the colours in app.css. The charts read the colours from
// the page's CSS custom properties, again whenever the theme changes.

import type { ThemeMode } from "./options";
import { recallThemeMode, rememberThemeMode } from "./storage";

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

/** Light or dark on the page: no data-theme follows the system. */
function apply(mode: ThemeMode): void {
    if (mode === "system") {
        delete document.documentElement.dataset.theme;
    } else {
        document.documentElement.dataset.theme = mode;
    }
}

class Theme {
    mode = $state<ThemeMode>(recallThemeMode());
    current = $state<ChartTheme>(readTheme());

    constructor() {
        // Before the first chart is drawn, so it starts in the right colours.
        apply(this.mode);
        this.current = readTheme();
        matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
            this.current = readTheme();
        });
    }

    setMode(mode: ThemeMode): void {
        this.mode = mode;
        rememberThemeMode(mode);
        apply(mode);
        this.current = readTheme();
    }
}

export const theme = new Theme();

// How the charts are drawn. The same for every game, and remembered between visits.

/** What the x-axis shows for each day: the day number, or the month-week-day date. */
export type XAxisMode = "day" | "date";

export const X_AXIS_MODES: readonly { value: XAxisMode; label: string }[] = [
    { value: "day", label: "Day number (1, 2, 3…)" },
    { value: "date", label: "Month-week-day (1-1-1…)" },
];

/** How the players' values are drawn: a line each, or stacked areas of their share of the total. */
export type ChartType = "lines" | "share";

export const CHART_TYPES: readonly { value: ChartType; label: string }[] = [
    { value: "lines", label: "Lines (values)" },
    { value: "share", label: "Stacked area (% of total)" },
];

export interface ChartOptions {
    xAxis: XAxisMode;
    chartType: ChartType;
}

export const DEFAULT_OPTIONS: ChartOptions = { xAxis: "day", chartType: "lines" };

/** Options from untrusted storage: each one that is not valid falls back to its default. */
export function cleanOptions(value: unknown): ChartOptions {
    const options = typeof value === "object" && value !== null ? (value as Record<string, unknown>) : {};
    const xAxis = X_AXIS_MODES.find((m) => m.value === options.xAxis)?.value ?? DEFAULT_OPTIONS.xAxis;
    const chartType = CHART_TYPES.find((t) => t.value === options.chartType)?.value ?? DEFAULT_OPTIONS.chartType;
    return { xAxis, chartType };
}

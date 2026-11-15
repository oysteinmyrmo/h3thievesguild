// How the charts are drawn. The same for every game, and remembered between visits.

/** What the x-axis shows for each day: the day number, or the month-week-day date. */
export type XAxisMode = "day" | "date";

export const X_AXIS_MODES: readonly { value: XAxisMode; label: string }[] = [
    { value: "day", label: "Day number (1, 2, 3…)" },
    { value: "date", label: "Month-week-day (1-1-1…)" },
];

export interface ChartOptions {
    xAxis: XAxisMode;
}

export const DEFAULT_OPTIONS: ChartOptions = { xAxis: "day" };

/** Options from untrusted storage: each one that is not valid falls back to its default. */
export function cleanOptions(value: unknown): ChartOptions {
    const options = typeof value === "object" && value !== null ? (value as Record<string, unknown>) : {};
    const xAxis = X_AXIS_MODES.find((m) => m.value === options.xAxis)?.value ?? DEFAULT_OPTIONS.xAxis;
    return { xAxis };
}

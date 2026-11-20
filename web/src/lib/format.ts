// Text shown for numbers and dates.

import type { XAxisMode } from "./options";
import type { Dataset, Day } from "./wasm/pkg/h3wasm";

const whole = new Intl.NumberFormat("en-US", { maximumFractionDigits: 0 });
const short = new Intl.NumberFormat("en-US", { notation: "compact", maximumFractionDigits: 1 });

/** A value for a tooltip or table: "12,345", or an em dash when there is none. */
export function formatValue(value: number | null | undefined): string {
    return value === null || value === undefined ? "—" : whole.format(value);
}

/** A share for a tooltip or table: "32.4%", or an em dash when there is none. */
export function formatShare(share: number | null | undefined): string {
    return share === null || share === undefined ? "—" : `${share.toFixed(1)}%`;
}

/** A value for an axis: "12K". */
export function formatAxis(value: number): string {
    return Math.abs(value) < 10_000 ? whole.format(value) : short.format(value);
}

export function longDate(day: Pick<Day, "month" | "week" | "dayOfWeek">): string {
    return `Month ${day.month}, Week ${day.week}, Day ${day.dayOfWeek}`;
}

/** A day on the x-axis: its number, such as "12", or its date, such as "1-2-5". */
export function axisLabel(day: Pick<Day, "day" | "label">, mode: XAxisMode): string {
    return mode === "day" ? String(day.day) : day.label;
}

/** The game's span, such as "1-1-1 to 1-2-3 (10 days)". */
export function daySpan(dataset: Dataset): string {
    const first = dataset.days[0];
    const last = dataset.days[dataset.days.length - 1];
    if (!first || !last) {
        return "";
    }
    const count = dataset.days.length;
    const days = count === 1 ? "1 day" : `${count} days`;
    return first === last ? `${first.label} (${days})` : `${first.label} to ${last.label} (${days})`;
}

/** How many days have no save, such as "2 days without a save", or "" for none. */
export function missingDaysText(dataset: Dataset): string {
    const count = dataset.missingDays.reduce((sum, run) => sum + run.last - run.first + 1, 0);
    if (count === 0) {
        return "";
    }
    return count === 1 ? "1 day without a save" : `${count} days without a save`;
}

const ESCAPES: Record<string, string> = { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" };

/** Escape text for HTML. File and map names come from users. */
export function escapeHtml(text: string): string {
    return text.replace(/[&<>"']/g, (c) => ESCAPES[c] ?? c);
}

// ECharts options for a player statistic: one line per player in the player's colour.

import { escapeHtml, formatAxis, formatValue, longDate } from "./format";
import { displayName, type PlayerNames } from "./names";
import type { ChartTheme } from "./theme.svelte";
import type { Dataset, PlayerSeries } from "./wasm/pkg/h3wasm";

export interface ChartSettings {
    /** Small chart in the overview grid: no legend, labels or zoom. */
    compact: boolean;
    /** IDs of players not to show. */
    hidden: ReadonlySet<string>;
    /** Names the user gave the players. */
    names: PlayerNames;
    theme: ChartTheme;
}

/** Name of the series that shades days without a save. Not a player. */
export const MISSING_SERIES = "No save";

/** Name of the series that marks the days players were eliminated. Not a player. */
export const ELIMINATED_SERIES = "Eliminated";

export function visiblePlayers(dataset: Dataset, hidden: ReadonlySet<string>): PlayerSeries[] {
    return dataset.players.filter((p) => !hidden.has(p.id));
}

/** The row of values of a statistic for a player, `null` on days without a value. */
export function playerValues(dataset: Dataset, player: PlayerSeries, statId: string): (number | null)[] {
    return player.values[statId] ?? dataset.days.map(() => null);
}

/** HTML for the tooltip of one day. */
export function tooltipHtml(
    dataset: Dataset,
    statId: string,
    dayIndex: number,
    players: PlayerSeries[],
    names: PlayerNames = {},
): string {
    const day = dataset.days[dayIndex];
    if (!day) {
        return "";
    }
    const source = day.file === null ? "No save for this day" : escapeHtml(day.file);
    const header = `<strong>${escapeHtml(longDate(day))}</strong><br><span class="tooltip-file">${source}</span>`;
    const rows = players.map((p) => {
        const value = formatValue(playerValues(dataset, p, statId)[dayIndex]);
        const marker = `<span class="tooltip-marker" style="background:${escapeHtml(p.color)}"></span>`;
        return `${marker}${escapeHtml(displayName(p, names))}: <strong>${value}</strong>`;
    });
    return [header, ...rows].join("<br>");
}

function labelOfDay(dataset: Dataset, dayNumber: number): string | undefined {
    return dataset.days.find((d) => d.day === dayNumber)?.label;
}

/** One dashed line per elimination day, labelled with every player out that day. */
function eliminationSeries(dataset: Dataset, players: PlayerSeries[], settings: ChartSettings) {
    const byDay = new Map<string, string[]>();
    for (const player of players) {
        const label = player.eliminatedDay === null ? undefined : labelOfDay(dataset, player.eliminatedDay);
        if (label !== undefined) {
            byDay.set(label, [...(byDay.get(label) ?? []), displayName(player, settings.names)]);
        }
    }
    if (byDay.size === 0) {
        return undefined;
    }
    return {
        name: ELIMINATED_SERIES,
        type: "line",
        data: [],
        silent: true,
        tooltip: { show: false },
        markLine: {
            silent: true,
            symbol: ["none", "none"],
            lineStyle: { color: settings.theme.muted, type: "dashed", width: 1.5 },
            label: { show: !settings.compact, color: settings.theme.text, position: "end" },
            // A function, not a string: ECharts reads {braces} in string formatters as templates,
            // and a player's name may contain any character.
            // The label runs towards the middle of the chart, so it is not cut off at an edge.
            data: [...byDay].map(([label, names]) => {
                const text = `${names.join(", ")} out`;
                const index = dataset.days.findIndex((d) => d.label === label);
                const align = index > (dataset.days.length - 1) / 2 ? "right" : "left";
                return { xAxis: label, label: { formatter: () => text, align } };
            }),
        },
    };
}

/**
 * Shades each run of days without a save, from the last saved day before it to the first
 * after it: the stretch where the lines are broken.
 */
function missingDaysArea(dataset: Dataset, settings: ChartSettings) {
    const areas = dataset.missingDays.flatMap((run) => {
        const before = labelOfDay(dataset, run.first - 1);
        const after = labelOfDay(dataset, run.last + 1);
        return before && after ? [[{ xAxis: before }, { xAxis: after }]] : [];
    });
    return {
        name: MISSING_SERIES,
        type: "line",
        data: [],
        silent: true,
        tooltip: { show: false },
        markArea: {
            silent: true,
            itemStyle: { color: settings.theme.missing },
            label: { show: !settings.compact, formatter: "No save", color: settings.theme.muted },
            data: areas,
        },
    };
}

/** The full ECharts option for one player statistic. */
export function statChartOption(dataset: Dataset, statId: string, settings: ChartSettings) {
    const { compact, theme } = settings;
    const players = visiblePlayers(dataset, settings.hidden);
    // Series are named by colour ID, which is unique; the legend shows the player's name. Two
    // players may be given the same name.
    const series: object[] = players.map((player) => ({
        id: player.id,
        name: player.id,
        type: "line",
        data: playerValues(dataset, player, statId),
        connectNulls: false,
        color: player.color,
        // Shown in the small charts too, so a value between two missing days is visible.
        showSymbol: true,
        symbolSize: compact ? 3 : 6,
        lineStyle: { width: compact ? 1.5 : 2.5 },
        emphasis: { focus: "series" },
    }));
    const eliminated = eliminationSeries(dataset, players, settings);
    if (eliminated) {
        series.push(eliminated);
    }
    if (dataset.missingDays.length > 0) {
        series.push(missingDaysArea(dataset, settings));
    }
    const zoom = !compact && dataset.days.length > 28;
    return {
        animation: !compact,
        textStyle: { color: theme.text },
        grid: compact
            ? { left: 8, right: 12, top: 12, bottom: 8, containLabel: true }
            : { left: 16, right: 24, top: 56, bottom: zoom ? 64 : 24, containLabel: true },
        legend: {
            show: !compact,
            top: 0,
            data: players.map((p) => p.id),
            formatter: (id: string) => {
                const player = players.find((p) => p.id === id);
                return player ? displayName(player, settings.names) : id;
            },
            textStyle: { color: theme.text },
        },
        tooltip: {
            trigger: "axis",
            backgroundColor: theme.tooltipBackground,
            borderColor: theme.gridLine,
            textStyle: { color: theme.text },
            formatter: (params: { dataIndex: number }[] | { dataIndex: number }) => {
                const first = Array.isArray(params) ? params[0] : params;
                return first ? tooltipHtml(dataset, statId, first.dataIndex, players, settings.names) : "";
            },
        },
        xAxis: {
            type: "category",
            data: dataset.days.map((d) => d.label),
            boundaryGap: false,
            axisLabel: { color: theme.muted, hideOverlap: true },
            axisLine: { lineStyle: { color: theme.gridLine } },
        },
        yAxis: {
            type: "value",
            // Every statistic is a whole number.
            minInterval: 1,
            axisLabel: { color: theme.muted, formatter: formatAxis },
            splitLine: { lineStyle: { color: theme.gridLine } },
        },
        dataZoom: zoom ? [{ type: "inside" }, { type: "slider", bottom: 16 }] : [],
        series,
    };
}

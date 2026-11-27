// ECharts options for a player statistic: one line per player in the player's colour.

import { axisLabel, escapeHtml, formatAxis, formatShare, formatValue, longDate } from "./format";
import { displayName, type PlayerNames } from "./names";
import type { ChartType, XAxisMode } from "./options";
import type { ChartTheme } from "./theme.svelte";
import type { Dataset, PlayerSeries, Unit } from "./wasm/pkg/h3wasm";

export interface ChartSettings {
    /** Small chart in the overview grid: no legend, labels or zoom. */
    compact: boolean;
    /** IDs of players not to show. */
    hidden: ReadonlySet<string>;
    /** Names the user gave the players. */
    names: PlayerNames;
    theme: ChartTheme;
    /** What the x-axis shows for each day. */
    xAxis: XAxisMode;
    /** A line per player, or stacked areas of each player's share of the total. */
    chartType: ChartType;
    /** For an image: a title above the chart, no zoom bar and no animation. */
    title?: { text: string; subtext: string };
}

/** Name of the series that shades days without a save. Not a player. */
export const MISSING_SERIES = "No save";

/** Name of the series that marks the days players were eliminated. Not a player. */
export const ELIMINATED_SERIES = "Eliminated";

export function visiblePlayers(dataset: Dataset, hidden: ReadonlySet<string>): PlayerSeries[] {
    return dataset.players.filter((p) => !hidden.has(p.id));
}

/** What a statistic's values count: whole things, or a percentage. */
export function unitOf(dataset: Dataset, statId: string): Unit {
    return dataset.playerStats.find((s) => s.id === statId)?.unit ?? "count";
}

/**
 * Text to show with a player's value on a day, such as the name of the best creature, or `null`.
 * Games remembered from before statistics had labels have none.
 */
export function playerLabel(player: PlayerSeries, statId: string, dayIndex: number): string | null {
    return player.labels?.[statId]?.[dayIndex] ?? null;
}

/**
 * The picture to show with a player's label on a day, as a path relative to the page, such as
 * "assets/creatures/Archangel.png", or `null`. Games remembered from before pictures have none.
 */
export function playerIcon(player: PlayerSeries, statId: string, dayIndex: number): string | null {
    const icon = player.icons?.[statId]?.[dayIndex];
    // Names like "creatures/BlackDragon" only: the data set may come from the browser's storage.
    return icon && /^[A-Za-z0-9_-]+(\/[A-Za-z0-9_-]+)*$/.test(icon) ? `assets/${icon}.png` : null;
}

/** A value with its label, if any: "8,776 (Archangel)". */
export function valueWithLabel(value: string, label: string | null): string {
    return label === null ? value : `${value} (${label})`;
}

/** The row of values of a statistic for a player, `null` on days without a value. */
export function playerValues(dataset: Dataset, player: PlayerSeries, statId: string): (number | null)[] {
    return player.values[statId] ?? dataset.days.map(() => null);
}

/**
 * Each player's share of the players' total on one day, in percent, in the order of `players`.
 *
 * A player without a value on a day with a save is out of the game, or not yet in it, and has
 * no share. A day without a save, or with a total of 0, has no shares at all: there is nothing
 * to divide.
 */
export function dayShares(dataset: Dataset, players: PlayerSeries[], statId: string, dayIndex: number): (number | null)[] {
    const day = dataset.days[dayIndex];
    const values = players.map((p) => playerValues(dataset, p, statId)[dayIndex] ?? 0);
    const total = values.reduce((sum, v) => sum + v, 0);
    if (!day || day.file === null || total === 0) {
        return players.map(() => null);
    }
    return values.map((v) => (v / total) * 100);
}

/** Each player's share of the total on every day, in percent, by player ID. */
export function playerShares(dataset: Dataset, players: PlayerSeries[], statId: string): Map<string, (number | null)[]> {
    const byDay = dataset.days.map((_, i) => dayShares(dataset, players, statId, i));
    return new Map(players.map((p, j) => [p.id, byDay.map((shares) => shares[j] ?? null)]));
}

/** HTML for the tooltip of one day. With `share`, each player's share of the total comes first. */
export function tooltipHtml(
    dataset: Dataset,
    statId: string,
    dayIndex: number,
    players: PlayerSeries[],
    names: PlayerNames = {},
    share = false,
): string {
    const day = dataset.days[dayIndex];
    if (!day) {
        return "";
    }
    const source = day.file === null ? "No save for this day" : escapeHtml(day.file);
    const date = `Day ${day.day}: ${longDate(day)}`;
    const header = `<strong>${escapeHtml(date)}</strong><br><span class="tooltip-file">${source}</span>`;
    const shares = share ? dayShares(dataset, players, statId, dayIndex) : [];
    // With labels, each row is a player, a picture and the name with the value, in a grid so
    // the pictures line up. Without, each row is a player and the value.
    const labelled = players.some((p) => playerLabel(p, statId, dayIndex) !== null);
    const rows = players.map((p, i) => {
        const raw = playerValues(dataset, p, statId)[dayIndex];
        const value = formatValue(raw, unitOf(dataset, statId));
        const label = playerLabel(p, statId, dayIndex);
        const icon = playerIcon(p, statId, dayIndex);
        const marker = `<span class="tooltip-marker" style="background:${escapeHtml(p.color)}"></span>`;
        const player = `${marker}${escapeHtml(displayName(p, names))}:`;
        let shown: string;
        if (share && (raw === null || raw === undefined)) {
            // A player out of the game has no share, rather than 0%.
            shown = `<strong>—</strong>`;
        } else if (label === null) {
            shown = share ? `<strong>${formatShare(shares[i])}</strong> (${value})` : `<strong>${value}</strong>`;
        } else {
            // The name, then the value in small text. In the stacked view the share comes first
            // and is the one in bold.
            const name = share ? escapeHtml(label) : `<strong>${escapeHtml(label)}</strong>`;
            const named = `${name} <span class="tooltip-small">${value}</span>`;
            shown = share ? `<strong>${formatShare(shares[i])}</strong> ${named}` : named;
        }
        if (!labelled) {
            return `${player} ${shown}`;
        }
        const picture = icon === null ? "" : `<img class="tooltip-icon" src="${escapeHtml(icon)}" alt="" width="32" height="32">`;
        return `<span>${player}</span><span>${picture}</span><span>${shown}</span>`;
    });
    return labelled
        ? `${header}<div class="tooltip-grid">${rows.join("")}</div>`
        : [header, ...rows].join("<br>");
}

function labelOfDay(dataset: Dataset, dayNumber: number, mode: XAxisMode): string | undefined {
    const day = dataset.days.find((d) => d.day === dayNumber);
    return day && axisLabel(day, mode);
}

/** Height of one elimination label, to stack the labels of players out on nearby days. */
const ELIMINATION_LABEL_HEIGHT = 16;

/** Labels of eliminations closer than this share of the days are stacked, as they would touch. */
const NEARBY_SHARE = 0.08;

/** One dashed line per eliminated player, in the player's colour, labelled with the name. */
function eliminationSeries(dataset: Dataset, players: PlayerSeries[], settings: ChartSettings) {
    const lines: { label: string; index: number; player: PlayerSeries }[] = [];
    for (const player of players) {
        const index = dataset.days.findIndex((d) => d.day === player.eliminatedDay);
        const day = dataset.days[index];
        if (day) {
            lines.push({ label: axisLabel(day, settings.xAxis), index, player });
        }
    }
    if (lines.length === 0) {
        return undefined;
    }
    const nearby = Math.round(NEARBY_SHARE * (dataset.days.length - 1));
    const nearbyBefore = (i: number) =>
        lines.slice(0, i).filter((l) => Math.abs(l.index - lines[i]!.index) <= nearby).length;
    return {
        name: ELIMINATED_SERIES,
        type: "line",
        data: [],
        silent: true,
        tooltip: { show: false },
        markLine: {
            silent: true,
            symbol: ["none", "none"],
            lineStyle: { type: "dashed", width: 1.5 },
            label: {
                show: !settings.compact,
                position: "end",
                // A halo in the background colour keeps the label readable over the lines.
                textBorderColor: settings.theme.tooltipBackground,
                textBorderWidth: 3,
            },
            // A function, not a string: ECharts reads {braces} in string formatters as templates,
            // and a player's name may contain any character.
            // The label runs towards the middle of the chart, so it is not cut off at an edge.
            // Labels of players out on the same or nearby days are stacked downwards.
            data: lines.map(({ label, index, player }, i) => {
                const text = `${displayName(player, settings.names)} out`;
                const align = index > (dataset.days.length - 1) / 2 ? "right" : "left";
                return {
                    xAxis: label,
                    lineStyle: { color: player.color },
                    label: {
                        formatter: () => text,
                        align,
                        color: player.color,
                        offset: [0, nearbyBefore(i) * ELIMINATION_LABEL_HEIGHT],
                    },
                };
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
        const before = labelOfDay(dataset, run.first - 1, settings.xAxis);
        const after = labelOfDay(dataset, run.last + 1, settings.xAxis);
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
    const { compact, theme, title } = settings;
    const players = visiblePlayers(dataset, settings.hidden);
    const share = settings.chartType === "share";
    const shares = share ? playerShares(dataset, players, statId) : undefined;
    // Series are named by colour ID, which is unique; the legend shows the player's name. Two
    // players may be given the same name.
    const series: object[] = players.map((player) => ({
        id: player.id,
        name: player.id,
        type: "line",
        data: shares?.get(player.id) ?? playerValues(dataset, player, statId),
        connectNulls: false,
        color: player.color,
        ...(share
            ? {
                  // The areas stack up to 100% on every day with a save.
                  stack: "share",
                  areaStyle: { opacity: 0.85 },
                  showSymbol: false,
                  lineStyle: { width: compact ? 0.5 : 1 },
              }
            : {
                  // Shown in the small charts too, so a value between two missing days is visible.
                  showSymbol: true,
                  symbolSize: compact ? 3 : 6,
                  lineStyle: { width: compact ? 1.5 : 2.5 },
              }),
        // Hovering a player does not fade the others.
        emphasis: { focus: "none" },
    }));
    const eliminated = eliminationSeries(dataset, players, settings);
    if (eliminated) {
        series.push(eliminated);
    }
    if (dataset.missingDays.length > 0) {
        series.push(missingDaysArea(dataset, settings));
    }
    const zoom = !compact && !title && dataset.days.length > 28;
    // Room for the title above the legend.
    const top = title ? 64 : 0;
    return {
        animation: !compact && !title,
        textStyle: { color: theme.text },
        title: title
            ? {
                  text: title.text,
                  subtext: title.subtext,
                  left: "center",
                  textStyle: { color: theme.text, fontSize: 20 },
                  subtextStyle: { color: theme.muted },
              }
            : { show: false },
        grid: compact
            ? { left: 8, right: 12, top: 12, bottom: 8, containLabel: true }
            : { left: 16, right: 24, top: top + 56, bottom: zoom ? 64 : 24, containLabel: true },
        legend: {
            show: !compact,
            top,
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
                return first ? tooltipHtml(dataset, statId, first.dataIndex, players, settings.names, share) : "";
            },
        },
        xAxis: {
            type: "category",
            data: dataset.days.map((d) => axisLabel(d, settings.xAxis)),
            boundaryGap: false,
            axisLabel: { color: theme.muted, hideOverlap: true },
            axisLine: { lineStyle: { color: theme.gridLine } },
        },
        yAxis: share
            ? {
                  type: "value",
                  min: 0,
                  max: 100,
                  interval: 25,
                  axisLabel: { color: theme.muted, formatter: (value: number) => `${value}%` },
                  splitLine: { lineStyle: { color: theme.gridLine } },
              }
            : unitOf(dataset, statId) === "percent"
              ? {
                    type: "value",
                    min: 0,
                    axisLabel: { color: theme.muted, formatter: (value: number) => `${value}%` },
                    splitLine: { lineStyle: { color: theme.gridLine } },
                }
              : {
                    type: "value",
                    // Every other statistic is a whole number.
                    minInterval: 1,
                    axisLabel: { color: theme.muted, formatter: formatAxis },
                    splitLine: { lineStyle: { color: theme.gridLine } },
                },
        dataZoom: zoom ? [{ type: "inside" }, { type: "slider", bottom: 16 }] : [],
        series,
    };
}

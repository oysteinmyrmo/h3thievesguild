import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
    ELIMINATED_SERIES,
    MISSING_SERIES,
    dayShares,
    statChartOption,
    unitOf,
    playerIcon,
    playerLabel,
    tooltipHtml,
    valueWithLabel,
    type ChartSettings,
} from "../../src/lib/chart";
import { CREATURES, MAP_VISIBILITY, RESOURCES, dataset } from "./saves";

const theme = { text: "#111", muted: "#666", gridLine: "#ddd", missing: "rgba(0,0,0,0.1)", tooltipBackground: "#fff" };
// Month-week-day labels, so the tests can tell days apart from counts.
const detail: ChartSettings = { compact: false, hidden: new Set(), names: {}, theme, xAxis: "date", chartType: "lines" };
const compact: ChartSettings = { compact: true, hidden: new Set(), names: {}, theme, xAxis: "date", chartType: "lines" };
const share: ChartSettings = { ...detail, chartType: "share" };

type Marker = {
    xAxis: string;
    lineStyle: { color: string };
    label: { formatter: () => string; align: string; color: string; offset: number[] };
};

type Series = {
    id?: string;
    name: string;
    data: (number | null)[];
    color?: string;
    showSymbol?: boolean;
    stack?: string;
    emphasis?: { focus?: string };
    areaStyle?: object;
    markLine?: { data: Marker[]; label: { show: boolean } };
    markArea?: { data: { xAxis: string }[][] };
};

function series(option: ReturnType<typeof statChartOption>): Series[] {
    return option.series as Series[];
}

describe("statChartOption", () => {
    const full = dataset(RESOURCES);

    it("has one line per player in the player's colour, on a 1-2-3 axis", () => {
        const option = statChartOption(full, "gold", detail);
        expect(option.xAxis.data).toEqual(["1-1-1", "1-1-2", "1-1-3"]);
        const lines = series(option).filter((s) => s.id !== undefined);
        expect(lines.map((s) => s.id)).toEqual(["red", "blue", "tan", "green", "orange", "purple", "teal", "pink"]);
        expect(lines[0]!.color).toBe("#f80000");
        // Gold from info.txt, with teal's estates correction on day 3.
        expect(lines.find((s) => s.id === "teal")!.data).toEqual([2500, 5950, 6200]);
        // The legend shows the players' names; the series are named by colour ID.
        expect(option.legend.data).toEqual(lines.map((s) => s.id));
        expect(lines.map((s) => s.name)).toEqual(lines.map((s) => s.id));
        expect(option.legend.data.map(option.legend.formatter)).toEqual([
            "Red", "Blue", "Tan", "Green", "Orange", "Purple", "Teal", "Pink",
        ]);
    });

    it("drops a player to 0 on the day they are out of the game and marks the elimination", () => {
        const lines = series(statChartOption(full, "gold", detail));
        expect(lines.find((s) => s.id === "blue")!.data).toEqual([3400, 6700, 0]);
        // Blue, tan and orange are out on day 3: a line each, in the player's colour.
        const eliminated = lines.find((s) => s.name === ELIMINATED_SERIES)!;
        const markers = eliminated.markLine!.data;
        expect(markers.map((d) => [d.xAxis, d.label.formatter()])).toEqual([
            ["1-1-3", "Blue out"],
            ["1-1-3", "Tan out"],
            ["1-1-3", "Orange out"],
        ]);
        const colours = ["blue", "tan", "orange"].map((id) => full.players.find((p) => p.id === id)!.color);
        expect(markers.map((d) => d.lineStyle.color)).toEqual(colours);
        expect(markers.map((d) => d.label.color)).toEqual(colours);
        // Out on the same day, so the labels are stacked rather than drawn over each other.
        expect(markers.map((d) => d.label.offset)).toEqual([[0, 0], [0, 16], [0, 32]]);
        expect(eliminated.markLine?.label.show).toBe(true);
        // Day 3 is the last day, so the labels run leftwards, into the chart.
        expect(markers.map((d) => d.label.align)).toEqual(["right", "right", "right"]);
        expect(lines).toHaveLength(9);
    });

    it("only names shown players as eliminated, and leaves the marker out when none is", () => {
        const some = series(statChartOption(full, "gold", { ...detail, hidden: new Set(["tan"]) }));
        const markers = some.find((s) => s.name === ELIMINATED_SERIES)!.markLine!.data;
        expect(markers.map((d) => d.label.formatter())).toEqual(["Blue out", "Orange out"]);
        expect(markers.map((d) => d.label.offset)).toEqual([[0, 0], [0, 16]]);
        const none = series(statChartOption(full, "gold", { ...detail, hidden: new Set(["blue", "tan", "orange"]) }));
        expect(none.find((s) => s.name === ELIMINATED_SERIES)).toBeUndefined();
    });

    it("hides players the user turned off", () => {
        const option = statChartOption(full, "gold", { ...detail, hidden: new Set(["red", "pink"]) });
        expect(series(option).map((s) => s.id)).not.toContain("red");
        expect(series(option).filter((s) => s.id !== undefined)).toHaveLength(6);
        expect(option.legend.data).not.toContain("pink");
    });

    it("is simpler in the overview grid", () => {
        const option = statChartOption(full, "gold", compact);
        expect(option.legend.show).toBe(false);
        expect(option.dataZoom).toEqual([]);
        expect(series(option).find((s) => s.name === ELIMINATED_SERIES)!.markLine?.label.show).toBe(false);
        // Points stay visible, so a value with missing days on both sides still shows.
        expect(series(option).find((s) => s.id === "red")!.showSymbol).toBe(true);
    });

    it("shades days without a save, between the saved days around them", () => {
        const gap = dataset([RESOURCES[0]!, RESOURCES[2]!]);
        const lines = series(statChartOption(gap, "gold", detail));
        const shade = lines.find((s) => s.name === MISSING_SERIES)!;
        expect(shade.markArea?.data).toEqual([[{ xAxis: "1-1-1" }, { xAxis: "1-1-3" }]]);
        expect(lines.find((s) => s.id === "red")!.data).toEqual([2500, null, 10700]);
        expect(series(statChartOption(full, "gold", detail)).find((s) => s.name === MISSING_SERIES)).toBeUndefined();
    });

    it("stacks the labels of eliminations on nearby days, but not of those far apart", () => {
        // A 63-day game, as if blue, tan and orange were out on days 10, 13 and 40.
        const days = Array.from({ length: 63 }, (_, i) => ({ ...full.days[0]!, day: i + 1, label: `d${i + 1}` }));
        const out: Record<string, number> = { blue: 10, tan: 13, orange: 40 };
        const players = full.players.map((p) => ({ ...p, eliminatedDay: out[p.id] ?? null }));
        const option = statChartOption({ ...full, days, players }, "gold", detail);
        const markers = series(option).find((s) => s.name === ELIMINATED_SERIES)!.markLine!.data;
        expect(markers.map((d) => [d.xAxis, d.label.offset])).toEqual([
            ["d10", [0, 0]],
            ["d13", [0, 16]],
            ["d40", [0, 0]],
        ]);
    });

    it("numbers the days on the x-axis by default, everywhere a day is placed", () => {
        const byNumber = { ...detail, xAxis: "day" as const };
        const option = statChartOption(full, "gold", byNumber);
        expect(option.xAxis.data).toEqual(["1", "2", "3"]);
        const eliminated = series(option).find((s) => s.name === ELIMINATED_SERIES)!;
        expect(eliminated.markLine?.data.map((d) => d.xAxis)).toEqual(["3", "3", "3"]);
        const gap = dataset([RESOURCES[0]!, RESOURCES[2]!]);
        const shade = series(statChartOption(gap, "gold", byNumber)).find((s) => s.name === MISSING_SERIES)!;
        expect(shade.markArea?.data).toEqual([[{ xAxis: "1" }, { xAxis: "3" }]]);
    });

    it("has a title, and no zoom bar or animation, when drawn as an image", () => {
        const long = dataset(RESOURCES);
        const days = Array.from({ length: 40 }, (_, i) => ({ ...long.days[0]!, day: i + 1, label: `d${i + 1}` }));
        const many = { ...long, days };
        expect(statChartOption(many, "gold", detail).dataZoom).toHaveLength(2);
        const option = statChartOption(many, "gold", { ...detail, title: { text: "Gold", subtext: "My map" } });
        expect(option.title).toMatchObject({ text: "Gold", subtext: "My map" });
        expect(option.dataZoom).toEqual([]);
        expect(option.animation).toBe(false);
        // The legend and the plot move down to make room for the title.
        expect(option.legend.top).toBeGreaterThan(0);
        expect((option.grid as { top: number }).top).toBeGreaterThan(statChartOption(many, "gold", detail).grid.top);
        expect(statChartOption(many, "gold", detail).title).toEqual({ show: false });
    });

    it("does not fade the other players when one is hovered, in either view", () => {
        for (const settings of [detail, share]) {
            const lines = series(statChartOption(full, "gold", settings)).filter((s) => s.id !== undefined);
            expect(lines.every((s) => s.emphasis?.focus === "none")).toBe(true);
        }
    });

    it("uses whole numbers on the value axis", () => {
        expect(statChartOption(full, "heroes", detail).yAxis.minInterval).toBe(1);
    });

    it("shows the players' names in the legend and elimination line", () => {
        const names = { blue: "Bob {b} <i>", orange: "Bob {b} <i>", red: "Ålesund" };
        const option = statChartOption(full, "gold", { ...detail, names });
        expect(option.legend.data.map(option.legend.formatter)).toEqual([
            "Ålesund", "Bob {b} <i>", "Tan", "Green", "Bob {b} <i>", "Purple", "Teal", "Pink",
        ]);
        // Two players with the same name are still two series.
        expect(new Set(series(option).map((s) => s.name)).size).toBe(series(option).length);
        const eliminated = series(option).find((s) => s.name === ELIMINATED_SERIES)!;
        expect(eliminated.markLine?.data.map((d) => d.label.formatter())).toEqual(["Bob {b} <i> out", "Tan out", "Bob {b} <i> out"]);
    });

    it("uses the colour's name for a player named only with spaces", () => {
        const option = statChartOption(full, "gold", { ...detail, names: { red: "   " } });
        expect(option.legend.formatter("red")).toBe("Red");
    });

    it("gives every player an empty row for an unknown statistic", () => {
        const lines = series(statChartOption(full, "nonsense", detail));
        expect(lines[0]!.data).toEqual([null, null, null]);
    });
});

describe("the stacked share view", () => {
    const full = dataset(RESOURCES);
    const lines = (settings: ChartSettings, data = full) => series(statChartOption(data, "gold", settings)).filter((s) => s.id !== undefined);
    const sum = (values: (number | null)[]) => values.reduce<number>((total, v) => total + (v ?? 0), 0);

    it("stacks each player's share of the total, adding up to 100% every day", () => {
        const stacked = lines(share);
        expect(stacked).toHaveLength(8);
        expect(stacked.every((s) => s.stack === "share" && s.areaStyle !== undefined)).toBe(true);
        for (let day = 0; day < 3; day++) {
            expect(sum(stacked.map((s) => s.data[day]!))).toBeCloseTo(100, 9);
        }
        // Each share is the player's gold over the total gold that day.
        const gold = full.players.map((p) => p.values.gold!);
        for (const [i, s] of stacked.entries()) {
            const total = sum(gold.map((row) => row[0]!));
            expect(s.data[0]).toBeCloseTo((gold[i]![0]! / total) * 100, 9);
        }
        // Blue is out on day 3: no gold, so no share.
        expect(stacked.find((s) => s.id === "blue")!.data[2]).toBe(0);
    });

    it("shows a 0-100% axis and keeps the lines view as it was", () => {
        const option = statChartOption(full, "gold", share);
        expect(option.yAxis).toMatchObject({ min: 0, max: 100 });
        expect((option.yAxis.axisLabel.formatter as (v: number) => string)(50)).toBe("50%");
        const plain = lines(detail);
        expect(plain.every((s) => s.stack === undefined && s.areaStyle === undefined)).toBe(true);
        expect(plain.find((s) => s.id === "teal")!.data).toEqual([2500, 5950, 6200]);
    });

    it("leaves out hidden players from the total", () => {
        const two = lines({ ...share, hidden: new Set(["tan", "green", "orange", "purple", "teal", "pink"]) });
        const [red, blue] = full.players.map((p) => p.values.gold!);
        expect(two.map((s) => s.id)).toEqual(["red", "blue"]);
        expect(two[0]!.data[0]).toBeCloseTo((red![0]! / (red![0]! + blue![0]!)) * 100, 9);
        expect(sum(two.map((s) => s.data[0]!))).toBeCloseTo(100, 9);
        // On day 3 blue is out, so red has it all.
        expect(two[0]!.data[2]).toBe(100);
    });

    it("has no shares on a day without a save or with a total of 0", () => {
        const gap = dataset([RESOURCES[0]!, RESOURCES[2]!]);
        expect(lines(share, gap).every((s) => s.data[1] === null)).toBe(true);
        expect(dayShares(gap, gap.players, "gold", 1).every((v) => v === null)).toBe(true);
        // A statistic no player has any of: nothing to divide.
        expect(dayShares(full, full.players, "nonsense", 0).every((v) => v === null)).toBe(true);
        expect(dayShares(full, full.players, "gold", 99).every((v) => v === null)).toBe(true);
    });

    it("shows each player's share and value in the tooltip", () => {
        const two = full.players.slice(0, 2);
        const html = tooltipHtml(full, "gold", 0, two, {}, true);
        // Red 2,500 and blue 3,400 gold on day 1.
        expect(html).toContain("Red: <strong>42.4%</strong> (2,500)");
        expect(html).toContain("Blue: <strong>57.6%</strong> (3,400)");
        const gap = dataset([RESOURCES[0]!, RESOURCES[2]!]);
        expect(tooltipHtml(gap, "gold", 1, gap.players, {}, true)).toContain("Red: <strong>—</strong><br>");
        // Out of the game on day 3, blue has no gold that day and no share.
        expect(tooltipHtml(full, "gold", 2, full.players, {}, true)).toContain("Blue: <strong>0.0%</strong> (0)");
        // After that day a player has no value at all: no share, and the others still make 100%.
        const after = {
            ...full,
            players: full.players.map((p) => (p.id === "tan" ? { ...p, values: { ...p.values, gold: [2500, 3100, null] } } : p)),
        };
        expect(tooltipHtml(after, "gold", 2, after.players, {}, true)).toContain("Tan: <strong>—</strong><br>");
        expect(sum(dayShares(after, after.players, "gold", 2))).toBeCloseTo(100, 9);
    });
});

describe("percentages", () => {
    const map = dataset(MAP_VISIBILITY);

    it("are drawn on a percent axis, not limited to whole numbers", () => {
        const option = statChartOption(map, "mapExplored", detail);
        expect(option.yAxis).toMatchObject({ min: 0 });
        expect((option.yAxis as { minInterval?: number }).minInterval).toBeUndefined();
        expect((option.yAxis.axisLabel.formatter as (v: number) => string)(12.5)).toBe("12.5%");
        const red = series(option).find((s) => s.id === "red")!;
        expect(red.data.map((v) => v!.toFixed(4))).toEqual(["3.7423", "5.2855", "6.8287"]);
        // Counts keep their whole-number axis.
        expect(statChartOption(map, "gold", detail).yAxis.minInterval).toBe(1);
        expect(unitOf(map, "mapExplored")).toBe("percent");
        expect(unitOf(map, "gold")).toBe("count");
        expect(unitOf(map, "nonsense")).toBe("count");
    });

    it("show with a percent sign in the tooltip", () => {
        const html = tooltipHtml(map, "mapExplored", 0, map.players);
        expect(html).toContain("Red: <strong>3.7%</strong>");
        expect(html).toContain("Orange: <strong>22.3%</strong>");
    });
});

describe("tooltipHtml", () => {
    const full = dataset(RESOURCES);

    it("shows the full date, the file and each player's value", () => {
        const html = tooltipHtml(full, "gold", 2, full.players.slice(0, 2));
        expect(html).toContain("Day 3: Month 1, Week 1, Day 3");
        expect(html).toContain("[hotseat] 113.GM5");
        expect(html).toContain("Red: <strong>10,700</strong>");
        // Out of the game that day.
        expect(html).toContain("Blue: <strong>0</strong>");
    });

    it("says when a day has no save", () => {
        const gap = dataset([RESOURCES[0]!, RESOURCES[2]!]);
        const html = tooltipHtml(gap, "gold", 1, gap.players);
        expect(html).toContain("No save for this day");
        expect(html).toContain("Red: <strong>—</strong>");
    });

    it("shows players' names, escaped", () => {
        const html = tooltipHtml(full, "gold", 0, full.players.slice(0, 2), { red: "<b>Ann</b> & {a}" });
        expect(html).toContain("&lt;b&gt;Ann&lt;/b&gt; &amp; {a}: <strong>2,500</strong>");
        expect(html).toContain("Blue: <strong>3,400</strong>");
    });

    it("escapes file names", () => {
        const named = dataset([{ ...RESOURCES[0]!, name: "<img src=x onerror=alert(1)>.GM8" }]);
        const html = tooltipHtml(named, "gold", 0, named.players);
        expect(html).not.toContain("<img");
        expect(html).toContain("&lt;img src=x onerror=alert(1)&gt;.GM8");
    });

    it("is empty for a day out of range", () => {
        expect(tooltipHtml(full, "gold", 99, full.players)).toBe("");
    });
});

describe("labels", () => {
    const creatures = dataset(CREATURES);
    const img = (name: string) => `<img class="tooltip-icon" src="assets/creatures/${name}.png" alt="" width="32" height="32">`;

    const small = (value: string) => `<span class="tooltip-small">${value}</span>`;
    /** A row of the grid: the player, the picture, and the name with the value. */
    const row = (player: string, picture: string, shown: string) => `${player}:</span><span>${picture}</span><span>${shown}</span>`;

    it("show the best creature's picture, then its name, then its AI value in small text", () => {
        const html = tooltipHtml(creatures, "bestCreature", 0, creatures.players);
        expect(html).toContain('<div class="tooltip-grid">');
        expect(html).toContain(row("Red", img("ArchAngel"), `<strong>Archangel</strong> ${small("8,776")}`));
        expect(html).toContain(row("Purple", img("BlackDragon"), `<strong>Black Dragon</strong> ${small("8,721")}`));
        expect(html).toContain(row("Pink", img("AzureDragon"), `<strong>Azure Dragon</strong> ${small("78,845")}`));
        // Other statistics have no labels or pictures.
        const army = tooltipHtml(creatures, "armyStrength", 0, creatures.players);
        expect(army).not.toContain("<img");
        expect(army).not.toContain("tooltip-grid");
        expect(army).toContain("Red: <strong>");
    });

    it("follow the share in the stacked shares tooltip, which is the one in bold", () => {
        const html = tooltipHtml(creatures, "bestCreature", 1, creatures.players, {}, true);
        expect(html).toMatch(
            /Red:<\/span><span><img [^>]*ArchAngel\.png[^>]*><\/span><span><strong>[\d.]+%<\/strong> Archangel <span class="tooltip-small">8,776<\/span><\/span>/,
        );
    });

    it("are left out on days without one, and in games remembered from before labels", () => {
        const red = creatures.players[0]!;
        expect(playerLabel(red, "bestCreature", 0)).toBe("Archangel");
        expect(playerIcon(red, "bestCreature", 0)).toBe("assets/creatures/ArchAngel.png");
        expect(playerLabel(red, "bestCreature", 5)).toBeNull();
        expect(playerIcon(red, "bestCreature", 5)).toBeNull();
        expect(playerLabel(red, "gold", 0)).toBeNull();
        expect(playerIcon(red, "gold", 0)).toBeNull();
        const old = { ...creatures, players: creatures.players.map(({ labels: _, icons: __, ...p }) => p as typeof red) };
        expect(playerLabel(old.players[0]!, "bestCreature", 0)).toBeNull();
        expect(tooltipHtml(old, "bestCreature", 0, old.players)).toContain("Red: <strong>8,776</strong><br>");
        // Remembered with names but before pictures: the name and value without a picture.
        const named = creatures.players.map(({ icons: _, ...p }) => p as typeof red);
        expect(tooltipHtml(creatures, "bestCreature", 0, named)).toContain(
            row("Red", "", `<strong>Archangel</strong> ${small("8,776")}`),
        );
        // A player without a name that day among players with one: the value, in the grid.
        const some = creatures.players.map((p, i) => (i === 0 ? p : { ...p, labels: {}, icons: {} }));
        const mixed = tooltipHtml(creatures, "bestCreature", 0, some);
        expect(mixed).toContain(row("Red", img("ArchAngel"), `<strong>Archangel</strong> ${small("8,776")}`));
        expect(mixed).toContain(row("Blue", "", "<strong>8,613</strong>"));
        // A day without a name, such as one without creatures.
        const none = creatures.players.map((p) => ({ ...p, labels: { bestCreature: [null, null] } }));
        expect(tooltipHtml(creatures, "bestCreature", 0, none)).toContain("Red: <strong>8,776</strong><br>");
    });

    it("are escaped as HTML, and pictures are only taken from the assets folder", () => {
        const odd = creatures.players.map((p) => ({ ...p, labels: { bestCreature: ["<b>&", null] } }));
        expect(tooltipHtml(creatures, "bestCreature", 0, odd)).toContain("<strong>&lt;b&gt;&amp;</strong>");
        const red = creatures.players[0]!;
        for (const icon of ['x" onerror="alert(1)', "../secret", "https://example.com/x", "/abs", "a//b", ""]) {
            expect(playerIcon({ ...red, icons: { bestCreature: [icon] } }, "bestCreature", 0), icon).toBeNull();
        }
    });

    it("go in parentheses after a value in the table", () => {
        expect(valueWithLabel("8,776", "Archangel")).toBe("8,776 (Archangel)");
        expect(valueWithLabel("—", null)).toBe("—");
    });

    it("name pictures that are in the site, for every creature in the test saves", () => {
        const assets = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "public");
        const icons = creatures.players.flatMap((p) => p.icons.bestCreature ?? []).filter((i) => i !== null);
        expect(icons.length).toBeGreaterThan(0);
        for (const icon of icons) {
            const path = playerIcon({ ...creatures.players[0]!, icons: { bestCreature: [icon] } }, "bestCreature", 0)!;
            expect(existsSync(join(assets, path)), path).toBe(true);
        }
    });
});

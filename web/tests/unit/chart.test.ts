import { describe, expect, it } from "vitest";
import { ELIMINATED_SERIES, MISSING_SERIES, statChartOption, tooltipHtml, type ChartSettings } from "../../src/lib/chart";
import { RESOURCES, dataset } from "./saves";

const theme = { text: "#111", muted: "#666", gridLine: "#ddd", missing: "rgba(0,0,0,0.1)", tooltipBackground: "#fff" };
// Month-week-day labels, so the tests can tell days apart from counts.
const detail: ChartSettings = { compact: false, hidden: new Set(), names: {}, theme, xAxis: "date" };
const compact: ChartSettings = { compact: true, hidden: new Set(), names: {}, theme, xAxis: "date" };

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

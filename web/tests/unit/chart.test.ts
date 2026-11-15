import { describe, expect, it } from "vitest";
import { ELIMINATED_SERIES, MISSING_SERIES, statChartOption, tooltipHtml, type ChartSettings } from "../../src/lib/chart";
import { RESOURCES, dataset } from "./saves";

const theme = { text: "#111", muted: "#666", gridLine: "#ddd", missing: "rgba(0,0,0,0.1)", tooltipBackground: "#fff" };
const detail: ChartSettings = { compact: false, hidden: new Set(), theme };
const compact: ChartSettings = { compact: true, hidden: new Set(), theme };

type Series = {
    id?: string;
    name: string;
    data: (number | null)[];
    color?: string;
    showSymbol?: boolean;
    markLine?: { data: { xAxis: string; label: { formatter: string } }[]; label: { show: boolean } };
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
        expect(option.legend.data).toEqual(lines.map((s) => s.name));
    });

    it("leaves gaps where a player is out of the game and marks the elimination", () => {
        const lines = series(statChartOption(full, "gold", detail));
        expect(lines.find((s) => s.id === "blue")!.data).toEqual([3400, 6700, null]);
        // Blue, tan and orange are out on day 3: one line, naming all three.
        const eliminated = lines.find((s) => s.name === ELIMINATED_SERIES)!;
        expect(eliminated.markLine?.data).toEqual([{ xAxis: "1-1-3", label: { formatter: "Blue, Tan, Orange out" } }]);
        expect(eliminated.markLine?.label.show).toBe(true);
        expect(lines).toHaveLength(9);
    });

    it("only names shown players as eliminated, and leaves the marker out when none is", () => {
        const some = series(statChartOption(full, "gold", { ...detail, hidden: new Set(["tan"]) }));
        expect(some.find((s) => s.name === ELIMINATED_SERIES)!.markLine?.data[0]?.label.formatter).toBe("Blue, Orange out");
        const none = series(statChartOption(full, "gold", { ...detail, hidden: new Set(["blue", "tan", "orange"]) }));
        expect(none.find((s) => s.name === ELIMINATED_SERIES)).toBeUndefined();
    });

    it("hides players the user turned off", () => {
        const option = statChartOption(full, "gold", { ...detail, hidden: new Set(["red", "pink"]) });
        expect(series(option).map((s) => s.id)).not.toContain("red");
        expect(series(option).filter((s) => s.id !== undefined)).toHaveLength(6);
        expect(option.legend.data).not.toContain("Pink");
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

    it("uses whole numbers on the value axis", () => {
        expect(statChartOption(full, "heroes", detail).yAxis.minInterval).toBe(1);
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
        expect(html).toContain("Month 1, Week 1, Day 3");
        expect(html).toContain("[hotseat] 113.GM5");
        expect(html).toContain("Red: <strong>10,700</strong>");
        expect(html).toContain("Blue: <strong>—</strong>");
    });

    it("says when a day has no save", () => {
        const gap = dataset([RESOURCES[0]!, RESOURCES[2]!]);
        expect(tooltipHtml(gap, "gold", 1, gap.players)).toContain("No save for this day");
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

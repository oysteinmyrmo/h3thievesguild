import { describe, expect, it } from "vitest";
import { errorView } from "../../src/lib/errors";
import { daySpan, escapeHtml, formatAxis, formatValue, longDate, missingDaysText } from "../../src/lib/format";
import { RESOURCES, dataset } from "./saves";

describe("formatting", () => {
    it("formats values and axis labels", () => {
        expect(formatValue(1234567)).toBe("1,234,567");
        expect(formatValue(0)).toBe("0");
        expect(formatValue(null)).toBe("—");
        expect(formatValue(undefined)).toBe("—");
        expect(formatAxis(9999)).toBe("9,999");
        expect(formatAxis(12_500)).toBe("12.5K");
    });

    it("writes dates and spans", () => {
        expect(longDate({ month: 2, week: 3, dayOfWeek: 4 })).toBe("Month 2, Week 3, Day 4");
        expect(daySpan(dataset(RESOURCES))).toBe("1-1-1 to 1-1-3 (3 days)");
        expect(daySpan(dataset([RESOURCES[0]!]))).toBe("1-1-1 (1 day)");
        expect(missingDaysText(dataset(RESOURCES))).toBe("");
        expect(missingDaysText(dataset([RESOURCES[0]!, RESOURCES[2]!]))).toBe("1 day without a save");
    });

    it("escapes HTML", () => {
        expect(escapeHtml(`<a href="x">Tom's & co</a>`)).toBe("&lt;a href=&quot;x&quot;&gt;Tom&#39;s &amp; co&lt;/a&gt;");
    });
});

describe("error views", () => {
    it("lists the files of each problem", () => {
        expect(
            errorView({ kind: "duplicateDays", message: "", details: [{ date: "Month 1, Week 1, Day 1", files: ["a", "b"] }] }).items,
        ).toEqual(["Month 1, Week 1, Day 1: a, b"]);
        expect(
            errorView({
                kind: "mixedGames",
                message: "",
                details: [{ mapName: "Arena", mapSize: "M", underground: true, files: ["x", "y"] }],
            }).items,
        ).toEqual(["Arena (M, with underground): x, y"]);
        expect(errorView({ kind: "unreadable", message: "", details: [{ file: "f", reason: "bad" }] }).items).toEqual([
            "f: bad",
        ]);
        expect(
            errorView({
                kind: "tooManyDays",
                message: "",
                details: { days: 4000, first: { date: "d1", files: ["a"] }, last: { date: "d2", files: ["b"] } },
            }).items,
        ).toEqual(["First: a (d1)", "Last: b (d2)"]);
        expect(errorView({ kind: "noSaves", message: "" }).items).toEqual([]);
        expect(errorView({ kind: "unexpected", message: "boom" }).items).toEqual(["boom"]);
    });
});

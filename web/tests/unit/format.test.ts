import { describe, expect, it } from "vitest";
import { axisLabel, daySpan, escapeHtml, formatShare, formatAxis, formatValue, longDate, missingDaysText } from "../../src/lib/format";
import { splitMessage } from "../../src/lib/message";
import { RESOURCES, dataset } from "./saves";

describe("formatting", () => {
    it("formats values and axis labels", () => {
        expect(formatValue(1234567)).toBe("1,234,567");
        expect(formatValue(0)).toBe("0");
        expect(formatValue(null)).toBe("—");
        expect(formatValue(undefined)).toBe("—");
        expect(formatAxis(9999)).toBe("9,999");
        expect(formatAxis(12_500)).toBe("12.5K");
        expect(formatShare(42.3728)).toBe("42.4%");
        expect(formatShare(100)).toBe("100.0%");
        expect(formatShare(null)).toBe("—");
    });

    it("writes dates and spans", () => {
        expect(longDate({ month: 2, week: 3, dayOfWeek: 4 })).toBe("Month 2, Week 3, Day 4");
        expect(daySpan(dataset(RESOURCES))).toBe("1-1-1 to 1-1-3 (3 days)");
        expect(daySpan(dataset([RESOURCES[0]!]))).toBe("1-1-1 (1 day)");
        expect(missingDaysText(dataset(RESOURCES))).toBe("");
        expect(missingDaysText(dataset([RESOURCES[0]!, RESOURCES[2]!]))).toBe("1 day without a save");
    });

    it("labels a day on the x-axis by its number or its date", () => {
        const day = { day: 40, label: "2-2-5" };
        expect(axisLabel(day, "day")).toBe("40");
        expect(axisLabel(day, "date")).toBe("2-2-5");
    });

    it("escapes HTML", () => {
        expect(escapeHtml(`<a href="x">Tom's & co</a>`)).toBe("&lt;a href=&quot;x&quot;&gt;Tom&#39;s &amp; co&lt;/a&gt;");
    });
});

describe("messages", () => {
    // The parser's messages, as h3dataset writes them.
    it("lists the files of each problem", () => {
        expect(splitMessage("more than one save for the same day; keep one save per day:\n- Month 1, Week 1, Day 1: a, b")).toEqual({
            title: "More than one save for the same day; keep one save per day:",
            items: ["Month 1, Week 1, Day 1: a, b"],
            after: [],
        });
        expect(
            splitMessage('the saves come from 2 different games; select the saves of one game:\n- map "Arena" (M with underground): x, y\n- map "B" (S): z').items,
        ).toEqual(['map "Arena" (M with underground): x, y', 'map "B" (S): z']);
        expect(splitMessage("these files could not be read as saves:\n- f: bad").items).toEqual(["f: bad"]);
        expect(
            splitMessage(
                "the saves span 4000 days, more than a data set can hold; check the dates of the first save, a (d1), and the last, b (d2)",
            ),
        ).toEqual({
            title: "The saves span 4000 days, more than a data set can hold; check the dates of the first save, a (d1), and the last, b (d2)",
            items: [],
            after: [],
        });
        expect(splitMessage("no save files were given")).toEqual({ title: "No save files were given", items: [], after: [] });
        expect(splitMessage("boom")).toEqual({ title: "Boom", items: [], after: [] });
    });

    it("keeps the lines after the list", () => {
        const warning = "There are no saves for 3 days between the first and the last save:\n- Month 1, Week 1, Day 2\n- Month 1, Week 1, Day 5 to Month 1, Week 1, Day 6\nThese days are left empty in the charts. Continue?";
        expect(splitMessage(warning)).toEqual({
            title: "There are no saves for 3 days between the first and the last save:",
            items: ["Month 1, Week 1, Day 2", "Month 1, Week 1, Day 5 to Month 1, Week 1, Day 6"],
            after: ["These days are left empty in the charts. Continue?"],
        });
        expect(splitMessage("")).toEqual({ title: "", items: [], after: [] });
    });
});

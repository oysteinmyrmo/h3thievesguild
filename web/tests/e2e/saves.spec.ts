import { expect, test, type Page } from "@playwright/test";
import { readFileSync } from "node:fs";
import type { Download } from "@playwright/test";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const testData = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "..", "test_data");

/** A test save as a file to choose, under its own name or the one given. */
function save(folder: string, file: string, name = file) {
    return { name, mimeType: "application/octet-stream", buffer: readFileSync(join(testData, folder, file)) };
}

const RESOURCES = [save("resources", "[hotseat] 111.GM8"), save("resources", "[hotseat] 112.GM8"), save("resources", "[hotseat] 113.GM5")];

async function choose(page: Page, files: ReturnType<typeof save>[]) {
    await page.getByTestId("file-input").setInputFiles(files);
}

test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await page.evaluate(() => localStorage.clear());
    await page.goto("/#/saves");
});

test("the page has the menu and asks for saves", async ({ page }) => {
    await expect(page).toHaveTitle("Thieves' Guild");
    const menu = page.getByRole("navigation", { name: "Main" });
    await expect(menu.getByRole("link", { name: "Save games" })).toHaveAttribute("aria-current", "page");
    await expect(menu.getByText("Calculators")).toHaveAttribute("aria-disabled", "true");
    await expect(menu.getByText("Coming soon")).toBeVisible();
    await expect(page.getByTestId("empty")).toBeVisible();
});

test("loading a game shows the map, the players and a chart per statistic", async ({ page }) => {
    await choose(page, RESOURCES);
    await expect(page.getByTestId("map-name")).toHaveText("SaveGameTester_Resources");
    await expect(page.getByTestId("day-span")).toHaveText("1-1-1 to 1-1-3 (3 days)");
    await expect(page.getByRole("group", { name: "Players shown in the charts" }).getByRole("button")).toHaveCount(8);
    await expect(page.getByTestId("chip-blue")).toContainText("out on day 3");
    // One card per player statistic, each with a drawn chart.
    await expect(page.locator("[data-testid^=card-]")).toHaveCount(14);
    await expect(page.getByTestId("card-gold").locator("canvas").first()).toBeVisible();
    // The values, as the table behind each chart: blue's gold from info.txt, 0 on day 3 when
    // blue is out of the game.
    const gold = page.getByTestId("table-gold").first();
    await expect(gold.locator("tbody tr").nth(0)).toContainText("3,400");
    await expect(gold.locator("tbody tr").nth(2).locator("td").nth(1)).toHaveText("0");
});

test("a chart opens in a full-page view and Back returns to the grid", async ({ page }) => {
    await choose(page, RESOURCES);
    await page.getByTestId("card-gold").click();
    await expect(page).toHaveURL(/#\/saves\/gold$/);
    await expect(page.getByRole("heading", { name: "Gold", level: 2 })).toBeVisible();
    await expect(page.locator("[data-testid^=card-]")).toHaveCount(0);
    await page.goBack();
    await expect(page.locator("[data-testid^=card-]")).toHaveCount(14);
    await page.getByTestId("card-armyStrength").click();
    await page.getByRole("link", { name: "← All charts" }).click();
    await expect(page).toHaveURL(/#\/saves$/);
});

test("a player chip hides and shows the player in every chart", async ({ page }) => {
    await choose(page, RESOURCES);
    const chip = page.getByTestId("chip-blue");
    await expect(chip).toHaveAttribute("aria-pressed", "true");
    await chip.click();
    await expect(chip).toHaveAttribute("aria-pressed", "false");
    await expect(page.getByTestId("table-gold").locator("thead th")).not.toContainText(["Blue"]);
    await expect(page.getByTestId("table-wood").locator("thead th")).toHaveCount(8);
    await chip.click();
    await expect(page.getByTestId("table-wood").locator("thead th")).toHaveCount(9);
});

test("two saves from the same day are an error naming both files", async ({ page }) => {
    await choose(page, [save("resources", "[hotseat] 111.GM8", "a.GM8"), save("resources", "[hotseat] 111.GM8", "b.GM8")]);
    const error = page.getByTestId("error");
    await expect(error).toHaveAttribute("data-kind", "duplicateDays");
    await expect(error).toContainText("Month 1, Week 1, Day 1: a.GM8, b.GM8");
    await expect(page.getByTestId("map-name")).toHaveCount(0);
});

test("saves from different games are an error naming each map", async ({ page }) => {
    await choose(page, [RESOURCES[0]!, save("towns", "[hotseat] 111.GM2")]);
    const error = page.getByTestId("error");
    await expect(error).toHaveAttribute("data-kind", "mixedGames");
    await expect(error).toContainText("SaveGameTester_Resources");
    await expect(error).toContainText("[hotseat] 111.GM2");
});

test("files that are not saves are an error naming them", async ({ page }) => {
    await choose(page, [RESOURCES[0]!, { name: "notes.txt", mimeType: "text/plain", buffer: Buffer.from("hello") }]);
    const error = page.getByTestId("error");
    await expect(error).toHaveAttribute("data-kind", "unreadable");
    await expect(error).toContainText("notes.txt");
});

test("a missing day asks before showing the game", async ({ page }) => {
    await choose(page, [RESOURCES[0]!, RESOURCES[2]!]);
    const dialog = page.getByTestId("missing-days-dialog");
    await expect(dialog).toBeVisible();
    await expect(dialog).toContainText("Month 1, Week 1, Day 2");
    await dialog.getByRole("button", { name: "Cancel" }).click();
    await expect(dialog).toHaveCount(0);
    await expect(page.getByTestId("map-name")).toHaveCount(0);

    await choose(page, [RESOURCES[0]!, RESOURCES[2]!]);
    await page.getByTestId("missing-days-dialog").getByRole("button", { name: "Continue" }).click();
    await expect(page.getByTestId("map-name")).toBeVisible();
    await expect(page.getByTestId("missing-days")).toHaveText("1 day without a save");
    await expect(page.getByTestId("table-gold").locator("tbody tr").nth(1)).toContainText("—");
});

test("the last game is remembered across a reload until cleared", async ({ page }) => {
    await choose(page, RESOURCES);
    await expect(page.getByTestId("map-name")).toBeVisible();
    await page.reload();
    await expect(page.getByTestId("map-name")).toHaveText("SaveGameTester_Resources");
    await expect(page.getByTestId("restored")).toBeVisible();
    await page.getByRole("button", { name: "Clear" }).click();
    await expect(page.getByTestId("empty")).toBeVisible();
    await page.reload();
    await expect(page.getByTestId("empty")).toBeVisible();
});

test("a new load replaces an error and the previous game", async ({ page }) => {
    await choose(page, [{ name: "notes.txt", mimeType: "text/plain", buffer: Buffer.from("hello") }]);
    await expect(page.getByTestId("error")).toBeVisible();
    await choose(page, [save("experience", "[hotseat] 111.GM2"), save("experience", "[hotseat] 112.GM2")]);
    await expect(page.getByTestId("error")).toHaveCount(0);
    await expect(page.getByTestId("map-name")).toHaveText("SaveGameTester_Experience");
    await expect(page.getByRole("group", { name: "Players shown in the charts" }).getByRole("button")).toHaveCount(2);
});

test("the calculators page says they are coming", async ({ page }) => {
    await page.goto("/#/calculators");
    await expect(page.getByRole("heading", { name: "Calculators" })).toBeVisible();
    await expect(page.getByText("Calculators are coming soon.")).toBeVisible();
});

test("players can be named, with any characters, and the charts use the names", async ({ page }) => {
    await choose(page, RESOURCES);
    await page.getByTestId("name-players").click();
    const dialog = page.getByTestId("name-players-dialog");
    await expect(dialog).toBeVisible();
    await expect(dialog.getByTestId("name-red")).toHaveAttribute("placeholder", "Red");
    const odd = `Ålesund {a} <b>&"'`;
    await dialog.getByTestId("name-red").fill(odd);
    await dialog.getByTestId("name-blue").fill("Bob");
    await dialog.getByRole("button", { name: "Save", exact: true }).click();
    await expect(dialog).toHaveCount(0);

    // Chips show only the name; the colour is in the dot and the tooltip.
    await expect(page.getByTestId("chip-name-red")).toHaveText(odd);
    await expect(page.getByTestId("chip-red")).not.toContainText("Red");
    await expect(page.getByTestId("chip-red")).toHaveAttribute("title", `Hide ${odd} (Red)`);
    await expect(page.getByTestId("chip-name-blue")).toHaveText("Bob");
    await expect(page.getByTestId("chip-name-tan")).toHaveText("Tan");
    // Every chart's data uses the names.
    const header = page.getByTestId("table-gold").locator("thead th");
    await expect(header).toHaveText(["Day", odd, "Bob", "Tan", "Green", "Orange", "Purple", "Teal", "Pink"]);
    await expect(page.getByTestId("table-towns").locator("thead th").nth(1)).toHaveText(odd);
    await page.getByTestId("card-gold").click();
    await expect(page.getByTestId("table-gold").locator("thead th").nth(2)).toHaveText("Bob");
});

test("player names are remembered, and an empty name gives back the colour", async ({ page }) => {
    await choose(page, RESOURCES);
    await page.getByTestId("name-players").click();
    await page.getByTestId("name-teal").fill("Tea Lady");
    await page.getByTestId("name-pink").fill("Pinky");
    await page.getByTestId("name-players-dialog").getByRole("button", { name: "Save", exact: true }).click();
    await page.reload();
    await expect(page.getByTestId("chip-name-teal")).toHaveText("Tea Lady");
    await expect(page.getByTestId("chip-name-pink")).toHaveText("Pinky");

    // The dialog starts from the current names; emptying one, or Cancel, behave as expected.
    await page.getByTestId("name-players").click();
    await expect(page.getByTestId("name-teal")).toHaveValue("Tea Lady");
    await page.getByTestId("name-teal").fill("");
    await page.getByTestId("name-pink").fill("   ");
    await page.getByTestId("name-players-dialog").getByRole("button", { name: "Save", exact: true }).click();
    await expect(page.getByTestId("chip-name-teal")).toHaveText("Teal");
    await expect(page.getByTestId("chip-name-pink")).toHaveText("Pink");

    await page.getByTestId("name-players").click();
    await page.getByTestId("name-red").fill("Not saved");
    await page.getByTestId("name-players-dialog").getByRole("button", { name: "Cancel", exact: true }).click();
    await expect(page.getByTestId("chip-name-red")).toHaveText("Red");
});

test("a new game starts without names", async ({ page }) => {
    await choose(page, RESOURCES);
    await page.getByTestId("name-players").click();
    await page.getByTestId("name-purple").fill("Violet");
    await page.getByTestId("name-players-dialog").getByRole("button", { name: "Save", exact: true }).click();
    await expect(page.getByTestId("chip-name-purple")).toHaveText("Violet");
    await choose(page, [save("experience", "[hotseat] 111.GM2"), save("experience", "[hotseat] 112.GM2")]);
    await expect(page.getByTestId("map-name")).toHaveText("SaveGameTester_Experience");
    await expect(page.getByTestId("chip-name-purple")).toHaveText("Purple");
});

test("the full-page view steps to the previous and next chart, wrapping around", async ({ page }) => {
    await choose(page, RESOURCES);
    await expect(page.locator("[data-testid^=card-]")).toHaveCount(14);
    const ids = await page.locator("[data-testid^=card-]").evaluateAll((cards) =>
        cards.map((c) => c.getAttribute("data-testid")!.replace("card-", "")),
    );
    expect(ids).toHaveLength(14);
    await page.getByTestId(`card-${ids[0]}`).click();
    await page.getByTestId("next-chart").click();
    await expect(page).toHaveURL(new RegExp(`#/saves/${ids[1]}$`));
    await page.getByTestId("previous-chart").click();
    await page.getByTestId("previous-chart").click();
    await expect(page).toHaveURL(new RegExp(`#/saves/${ids[13]}$`));
    await page.getByTestId("next-chart").click();
    await expect(page).toHaveURL(new RegExp(`#/saves/${ids[0]}$`));
    // The page has caught up with the URL once the chart's table is there.
    await expect(page.getByTestId(`table-${ids[0]}`)).toHaveCount(1);
    // The arrow keys do the same.
    await page.keyboard.press("ArrowRight");
    await expect(page.getByTestId(`table-${ids[1]}`)).toHaveCount(1);
    await page.keyboard.press("ArrowRight");
    await expect(page.getByTestId(`table-${ids[2]}`)).toHaveCount(1);
    await expect(page).toHaveURL(new RegExp(`#/saves/${ids[2]}$`));
    await page.keyboard.press("ArrowLeft");
    await expect(page).toHaveURL(new RegExp(`#/saves/${ids[1]}$`));
    await expect(page.getByTestId(`table-${ids[1]}`)).toHaveCount(1);
});

test("the x-axis shows day numbers or dates, and the choice is remembered", async ({ page }) => {
    await choose(page, RESOURCES);
    const days = page.getByTestId("table-gold").first().locator("tbody th");
    await expect(days).toHaveText(["1", "2", "3"]);
    await page.getByTestId("card-gold").click();
    const select = page.getByTestId("x-axis");
    await expect(select).toHaveValue("day");
    await select.selectOption("date");
    await expect(page.getByTestId("table-gold").locator("tbody th")).toHaveText(["1-1-1", "1-1-2", "1-1-3"]);
    // Arrow keys in the dropdown are for the dropdown, not for stepping through the charts.
    // (The last option is chosen, so this one leaves it as it is.)
    await select.press("ArrowRight");
    await expect(page).toHaveURL(/#\/saves\/gold$/);
    await page.reload();
    await expect(page.getByTestId("x-axis")).toHaveValue("date");
    await page.getByRole("link", { name: "← All charts" }).click();
    await expect(page.getByTestId("table-wood").locator("tbody th")).toHaveText(["1-1-1", "1-1-2", "1-1-3"]);
});

/** The bytes of a download. */
async function bytes(download: Download): Promise<Buffer> {
    return readFileSync(await download.path());
}

/** WebKit in Playwright suggests download names with spaces turned into underscores. */
function expectFileName(download: Download, name: string) {
    expect(download.suggestedFilename().replaceAll(" ", "_")).toBe(name.replaceAll(" ", "_"));
}

const PNG_SIGNATURE = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);

/** Width and height of a PNG image, from its header. */
function pngSize(png: Buffer): [number, number] {
    return [png.readUInt32BE(16), png.readUInt32BE(20)];
}

test("a chart downloads as a PNG image, from its page or its card", async ({ page }) => {
    await choose(page, RESOURCES);
    let wait = page.waitForEvent("download");
    await page.getByTestId("download-wood").click();
    let file = await wait;
    expectFileName(file, "SaveGameTester_Resources - Wood.png");
    let png = await bytes(file);
    expect(png.subarray(0, 8)).toEqual(PNG_SIGNATURE);
    // Drawn at a fixed size, twice over for sharpness, whatever the size of the card.
    expect(pngSize(png)).toEqual([3200, 1800]);

    await page.getByTestId("card-gold").click();
    wait = page.waitForEvent("download");
    await page.getByTestId("download-chart").click();
    file = await wait;
    expectFileName(file, "SaveGameTester_Resources - Gold.png");
    png = await bytes(file);
    expect(png.subarray(0, 8)).toEqual(PNG_SIGNATURE);
    expect(pngSize(png)).toEqual([3200, 1800]);
});

test("all charts download as PNG images in one ZIP file", async ({ page }) => {
    await choose(page, RESOURCES);
    const wait = page.waitForEvent("download");
    await page.getByTestId("download-all").click();
    const file = await wait;
    expectFileName(file, "SaveGameTester_Resources - charts.zip");
    const zip = await bytes(file);
    // Read the central directory: one PNG image per chart.
    const end = zip.length - 22;
    expect(zip.readUInt32LE(end)).toBe(0x06054b50);
    expect(zip.readUInt16LE(end + 10)).toBe(14);
    let pos = zip.readUInt32LE(end + 16);
    const names: string[] = [];
    for (let i = 0; i < 14; i++) {
        const nameLength = zip.readUInt16LE(pos + 28);
        const local = zip.readUInt32LE(pos + 42);
        names.push(zip.subarray(pos + 46, pos + 46 + nameLength).toString("utf8"));
        const dataStart = local + 30 + zip.readUInt16LE(local + 26) + zip.readUInt16LE(local + 28);
        expect(zip.subarray(dataStart, dataStart + 8)).toEqual(PNG_SIGNATURE);
        pos += 46 + nameLength;
    }
    expect(names).toContain("SaveGameTester_Resources - Gold.png");
    expect(names).toContain("SaveGameTester_Resources - Kingdom army strength.png");
    expect(new Set(names).size).toBe(14);
    await expect(page.getByTestId("download-all")).toBeEnabled();
});

test("every chart switches to stacked shares from the overview or a chart's page, and stays so", async ({ page }) => {
    await choose(page, RESOURCES);
    const type = page.getByTestId("chart-type");
    await expect(type).toHaveValue("lines");
    await type.selectOption("share");
    // Day 1: red has 2,500 of the 23,600 gold all players have (info.txt).
    const gold = page.getByTestId("table-gold").first();
    await expect(gold.locator("caption")).toContainText("share of the total");
    await expect(gold.locator("tbody tr").nth(0).locator("td").nth(0)).toHaveText("10.6%");
    // Every day's shares add up to 100%, in every chart.
    for (const id of ["gold", "wood", "heroes"]) {
        const cells = await page.getByTestId(`table-${id}`).first().locator("tbody tr").nth(1).locator("td").allTextContents();
        expect(cells).toHaveLength(8);
        const total = cells.reduce((sum, text) => sum + parseFloat(text), 0);
        expect(Math.abs(total - 100), `${id}: ${cells.join(" ")}`).toBeLessThan(0.5);
    }
    // No one has any wood on day 1, so there is no share to show.
    const wood = page.getByTestId("table-wood").first().locator("tbody tr").nth(0).locator("td");
    await expect(wood).toHaveText(Array(8).fill("—"));
    // The chart's page shows the same choice, and switching back there switches the overview too.
    await page.getByTestId("card-wood").click();
    await expect(page.getByTestId("chart-type")).toHaveValue("share");
    // Red has 15 of the 37 wood on day 2.
    await expect(page.getByTestId("table-wood").locator("tbody tr").nth(1).locator("td").nth(0)).toHaveText("40.5%");
    await page.reload();
    await expect(page.getByTestId("chart-type")).toHaveValue("share");
    await page.getByTestId("chart-type").selectOption("lines");
    await page.getByRole("link", { name: "← All charts" }).click();
    await expect(page.getByTestId("chart-type")).toHaveValue("lines");
    await expect(page.getByTestId("table-gold").first().locator("tbody tr").nth(0)).toContainText("2,500");
});

test("the theme follows the system, or is light or dark as chosen, and is remembered", async ({ page }) => {
    const background = () => page.evaluate(() => getComputedStyle(document.body).backgroundColor);
    const LIGHT = "rgb(245, 245, 247)";
    const DARK = "rgb(22, 23, 26)";
    const pressed = (mode: string) => expect(page.getByTestId(`theme-${mode}`)).toHaveAttribute("aria-pressed", "true");

    await page.emulateMedia({ colorScheme: "light" });
    await pressed("system");
    expect(await background()).toBe(LIGHT);
    await page.emulateMedia({ colorScheme: "dark" });
    await expect.poll(background).toBe(DARK);

    // Light or dark whatever the system says.
    await page.getByTestId("theme-light").click();
    await pressed("light");
    await expect(page.locator("html")).toHaveAttribute("data-theme", "light");
    await expect.poll(background).toBe(LIGHT);
    await page.getByTestId("theme-dark").click();
    await page.emulateMedia({ colorScheme: "light" });
    await expect.poll(background).toBe(DARK);

    // Remembered, and the charts are drawn in the chosen theme.
    await page.reload();
    await pressed("dark");
    await expect.poll(background).toBe(DARK);
    await choose(page, RESOURCES);
    await expect(page.getByTestId("card-gold").locator("canvas").first()).toBeVisible();

    // Back to the system's choice.
    await page.getByTestId("theme-system").click();
    await expect(page.locator("html")).not.toHaveAttribute("data-theme", /.*/);
    await expect.poll(background).toBe(LIGHT);
});

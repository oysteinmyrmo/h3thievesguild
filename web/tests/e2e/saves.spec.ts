import { expect, test, type Page } from "@playwright/test";
import { readFileSync } from "node:fs";
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
    // The values, as the table behind each chart: blue's gold from info.txt, gone on day 3.
    const gold = page.getByTestId("table-gold").first();
    await expect(gold.locator("tbody tr").nth(0)).toContainText("3,400");
    await expect(gold.locator("tbody tr").nth(2).locator("td").nth(1)).toHaveText("—");
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

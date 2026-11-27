# Feature requests

Features planned for Thieves' Guild, in the order they are to be built. Each has its own file
with a short spec: what it is for, how it behaves, where its data comes from, how it is tested,
and what is still open.

Every feature is built on its own branch, with tests that pass in every commit (Rust, web unit
tests, and the browser tests in Chromium, Firefox and WebKit). Once a feature is merged, delete
its file and add a line for it under Done. To add a feature, write its file and put it where it
fits in the order below.

## Planned

1. [Ask to reload a remembered game that lacks newer charts](reload-notice.md)
2. [Strongest hero army (Army)](strongest-hero-army.md)
3. [Choose resources with checkboxes (Economy)](resource-checkboxes.md)
4. [More Army and Kingdom charts](army-and-kingdom-charts.md)
5. [Hero charts](hero-charts.md)
6. [Other views of the numbers](views-of-the-numbers.md)
7. [Army composition](army-composition.md)
8. [Battles](battles.md)
9. [Load saves by drag and drop](drag-and-drop.md)
10. [Calculators](calculators.md)
11. [Phone layout](phone-layout.md)
12. [Charts that need more of the save read first](charts-needing-save-data.md)
13. [Mage guilds and spell research](mage-guild.md)
14. [Housekeeping](housekeeping.md)

## Done

- Hosting on GitHub Pages, deployed from main ([hosting.md](hosting.md) has the one-time setup).
- Best creature (Army category, with the creature's picture and name in the tooltip).
- Map explored (Map category, from the fog of war).
- One large chart at a time, chosen by category, with Previous and Next.
- Light and dark theme setting.
- Percentage stacked area view.
- PNG export of one chart or all charts.
- Day number or month-week-day on the x-axis.
- Player names.
- HotA 1.7 saves.

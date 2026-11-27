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
2. [Best creature (Army)](best-creature.md)
3. [Strongest hero army (Army)](strongest-hero-army.md)
4. [Choose resources with checkboxes (Economy)](resource-checkboxes.md)
5. [More Army and Kingdom charts](army-and-kingdom-charts.md)
6. [Hero charts](hero-charts.md)
7. [Other views of the numbers](views-of-the-numbers.md)
8. [Army composition](army-composition.md)
9. [Battles](battles.md)
10. [Load saves by drag and drop](drag-and-drop.md)
11. [Hosting](hosting.md)
12. [Calculators](calculators.md)
13. [Phone layout](phone-layout.md)
14. [Charts that need more of the save read first](charts-needing-save-data.md)
15. [Mage guilds and spell research](mage-guild.md)
16. [Housekeeping](housekeeping.md)

## Done

- Map explored (Map category, from the fog of war).
- One large chart at a time, chosen by category, with Previous and Next.
- Light and dark theme setting.
- Percentage stacked area view.
- PNG export of one chart or all charts.
- Day number or month-week-day on the x-axis.
- Player names.
- HotA 1.7 saves.

# Feature requests

Features planned for Thieves' Guild, in the order they are to be built. Each has a short spec:
what it is for, how it behaves, where its data comes from, how it is tested, and what is still
open. Move a feature to the bottom section once it is merged, and add new ones where they fit in
the order.

Every feature is built on its own branch, with tests that pass in every commit (Rust, web unit
tests, and the browser tests in Chromium, Firefox and WebKit).

## 1. Ask to reload a remembered game that lacks newer charts

**Why:** the page remembers the last game as processed data, not as save files. After a new
chart is added, a remembered game has no data for it, and the chart is silently missing.

**Behaviour:** when the remembered game lacks any of the statistics the current version has,
show a notice above the charts: "This game was loaded with an older version. Choose the save
files again to see all charts." The old charts still show until then.

**Data:** compare the statistic IDs of the remembered data set with those the parser has now.

**Tests:** unit test of the comparison; browser test that stores an older data set and checks
the notice, and that it goes away after loading the saves.

## 2. Best creature (Army)

**Why:** the strongest single creature a player has shows how far up the dwellings they are.

**Behaviour:** a "Best creature" chart in Army: per player and day, the highest AI value among
all creatures in their heroes' armies and town garrisons. The tooltip names the creature, such
as "Red: 4,872 (Black Dragon)".

**Data:** `Player::best_creature_ai_value` in the parser, plus the creature's name.

**Tests:** values against test saves with known armies (`creatures`, `bulwark`, `factory`);
a player with no creatures has 0.

## 3. Strongest hero army (Army)

**Why:** kingdom army strength is a total; this shows whether a player has one main army or
several small ones.

**Behaviour:** a "Strongest hero army" chart in Army: per player and day, the army strength of
their strongest hero, as in-game (with attack and defense).

**Data:** `Hero::army_strength` over the player's heroes on the map.

**Tests:** values against `kingdom_army_strength` test saves.

**Open:** whether heroes in town garrisons count; whether to also chart garrison strength
(towns only) so the two add up to kingdom army strength.

## 4. Choose resources with checkboxes (Economy)

**Why:** one chart per resource is many charts to step through; ticking the resources to see
is more flexible.

**Behaviour:**
- Gold keeps its own chart.
- The wood, mercury, ore, sulfur, crystal and gems charts become one "Resources" chart, with a
  checkbox per resource beside or above it, all ticked at first.
- The chart shows, per player, the sum of the ticked resources. With one box ticked it is that
  resource's chart as today.
- The ticked boxes are remembered with the display options.
- The tooltip lists the total and each ticked resource, such as
  "Red: 42 (wood 15, ore 20, gems 7)".
- With no box ticked, the chart says "Tick a resource to show it".
- Stacked share mode works on the sum.
- The PNG title names the ticked resources, such as "Resources: wood, ore".

**Data:** the existing per-resource series; the sum is computed in the frontend.

**Tests:** unit tests of the sum and the tooltip; browser test that ticks and unticks boxes,
checks the table behind the chart, and that the choice is remembered.

**Open:** whether a sum is right, or a line per resource per player (many lines), or one
player at a time.

## 5. More Army and Kingdom charts

Each from data the parser already reads:

- **Garrison strength** (Army): army strength of the creatures in the player's towns.
- **Artifact value** (Kingdom): total gold cost of the artifacts the player's heroes carry.
- **Relics** (Kingdom): number of relic artifacts.
- **Highest primary skills** (Kingdom): attack + defense + power + knowledge of the best hero.
- **Expert skills** (Kingdom): secondary skills at Expert across all heroes.
- **Spells known** (Kingdom): different spells across all heroes.

**Tests:** each against the test saves made for it (`artifacts`, `primary_skills`,
`secondary_skills`, `spells`).

## 6. Hero charts

**Why:** the data set already has each hero's level, experience and primary skills per day, but
nothing shows them.

**Behaviour:** a Heroes category: pick a statistic, and see a line per hero, coloured by owner,
with the hero's name. Hide heroes with a checkbox list, as the player chips do.

**Open:** how to show a hero who changes owner, and whether to limit the number of lines.

## 7. Other views of the numbers

A display option beside Chart and X-axis:

- **Change per day:** each day's value minus the day before.
- **Running total:** the sum of the values so far.
- **Day comparison:** two days side by side, as bars per player.

**Open:** how change and running total treat days without a save and eliminated players.

## 8. Load saves by drag and drop

Drop save files anywhere on the page to load them, as with "Choose save files…". Folders
dropped are searched for saves.

## 9. Hosting

Publish the built site, for example as a static site on GitHub Pages, built and deployed by CI
from main.

**Open:** where to host, and whether the repository is to stay private.

## 10. Calculators

The Calculators page, "coming soon" today.

**Open:** which calculators.

## 11. Phone layout

A layout for narrow screens: the sidebar as a menu, the display options under the chart.

## 12. Charts that need more of the save read first

- **Income and mines:** who owns which mines. Needs the map objects in the save.
- **Total resource value in gold:** needs exchange rates for the rare resources decided.
- **Creature tiers,** such as level-7 creatures owned: needs creature stats.
- **Factory and Bulwark artifacts.**

## 13. Housekeeping

- **Faster parsing:** about 40 ms per save today, most of it finding the hero array.

## Done

- Map explored (Map category, from the fog of war).
- One large chart at a time, chosen by category, with Previous and Next.
- Light and dark theme setting.
- Percentage stacked area view.
- PNG export of one chart or all charts.
- Day number or month-week-day on the x-axis.
- Player names.
- HotA 1.7 saves.

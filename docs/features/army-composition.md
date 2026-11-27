# Army composition

**Why:** army strength is one number. The creatures behind it show what a player actually has:
a few Archangels or a horde of Pikemen, and how far up the dwellings they are.

## Behaviour

- The data set gives each player's creatures on each day: every creature type the player has in
  their heroes' armies and town garrisons, with the total count of each.
- A view of one player's army on one day, as a list of creatures and counts. It is sorted by the
  creature's value, highest first, then by count, so the strongest creatures come first. It
  shows no army value, only the creatures.
- Step through the days, or pick a player, to see the army change. Where the view goes in the
  page is open, as for the [mage guild](mage-guild.md) view.

## Data

- Per player and day: creature type → count, over all heroes and garrisons. Missing on days
  without a save and after the player is out, as other statistics are.
- The creature's value to sort by is its AI value per creature, which the parser already has.
- The creature names come from the parser. Unknown creatures (`Creature(200)`, say) are kept
  and sorted last.
- [Battles](battles.md) uses this data to find what each player lost.

## Tests

- The creatures of each player against test saves with known armies (`creatures`, `bulwark`,
  `factory`, `kingdom_army_strength`): heroes and garrisons added up, empty stacks left out.
- Unit test of the sort: by value, then by count, unknown creatures last.
- Browser test of the view for a player and day.

## Open

- Whether to split the list by where the creatures are: per hero and per town, or only the total.
- Whether to show creature pictures, which would need images we are allowed to use.
- Whether the view also compares players, such as each player's army side by side on one day.

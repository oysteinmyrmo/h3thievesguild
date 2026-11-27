# Mage guilds and spell research

**Why:** which spells each town's mage guild offered, and when, explains much of how a game
went. In HotA a town can also research spells: replace a spell in the guild with another of the
same level. Seeing what each guild held at the start, how it grew as levels were built, and what
was researched, shows each player's magic over the game.

This is two pieces of work, built in order: reading the guilds from the save, then a view of
them. Where the view goes in the page is still open (see below).

## Game rules

From the wiki pages [Mage Guild](https://heroes.thelazy.net/index.php/Mage_Guild) and
[Spell research](https://heroes.thelazy.net/index.php/Spell_research):

- Guild levels 1 to 5 teach 5, 4, 3, 2 and 1 spells. Stronghold and Fortress stop at level 3;
  Castle, Cove and Bulwark at level 4; the other towns at level 5.
- Tower's Library adds one spell to each level. Conflux's Aurora Borealis teaches every spell
  not banned on the map.
- The spells are random, weighted by town type. A map can ban spells, so a slot can stay empty.
- Research replaces the spell in one slot. It costs 1000 gold and 2 × the spell level of
  mercury, sulfur, crystal and gems. It offers a choice of two new spells, or one at level 5.
  It can be done once per day per town.
- The same slot can be researched again until the research is finalized. Heroes cannot learn the
  spell until then, and finalizing makes the slot permanent. Only one slot can be under research
  at a time.
- A spell that has been in the guild or been offered is not offered again until every other
  possible spell has been. Town Portal and Dimension Door are guaranteed on the third research.
- Research can be turned off for a map in the editor.

## What the save holds

Found by comparing the 67 saves of `test_data/long_game` (HotA 1.7.3) day by day, and checked
against the 1.8 saves in `towns`, `factory` and `bulwark`. Offsets are from the end of the town's
name, in the part of the town record the parser does not read yet. They are to be confirmed with
the new test saves below, and then moved to `docs/save-format.md`.

| Offset | Type | Field |
|---|---|---|
| 84 | u8 × 5 | Spells shown per guild level: 0 until the level is built, then all of them, such as `5 4 3 2 0` for a Castle with guild level 4 |
| 89 | 3 × u64? | Probably the buildings built and allowed; the bits change on the days something is built |
| 113 | u32 × 5 × 6 | The guild's spells, 6 slots per level, in the order the guild shows them |
| 320 | 9 bytes | One bit per spell ID: the spells that have been in the guild or offered by research |
| 329 | u32 × 5 × 6 | State of each slot: 1 open to research, 2 research not finalized, 0 finalized or no spell |
| 457 | u32 | Changes on each research and only then; most likely the town's research random seed |
| 469 | u8 | 1 on a day the town researched |
| 470 | u32 | The original spell of the slot under research, `0xFFFFFFFF` when none is |

What this tells us:

- **Every level is filled at the start of the game**, built or not, up to the town's highest
  level. So the first save shows the whole guild the town would have without research.
- Slots past a level's count hold leftover bytes. Spell 0 is Summon Boat, a real spell, so an
  empty slot is told by the count, not by the value.
- The save holds only the guild as it is now. What was researched, and in what order, comes from
  comparing one day's save with the next.
- The spells research can still offer are those of the level that the town type allows and that
  are not yet in the bit set. Their order is not in the save: it depends on the seed and on how
  HotA draws, which is not known.

What `long_game` shows:

- Backwater (Fortress), level 3: on 2-2-3 slot 2 Earthquake → Fireball, finalized at once; on
  2-2-4 slot 1 Forgetfulness → Anti-Magic, finalized; on 2-2-5 slot 3 Teleport → Force Field,
  not finalized; slot 3 again on 2-3-1, Force Field → Mirth; and on 2-3-6, Mirth → Hypnotize,
  then finalized.
- Scar (Dungeon), level 3: on 2-1-1 slot 1 Land Mine → Hypnotize, never finalized.

## Part 1: read the guilds

**Behaviour:** `Town::mage_guild` gives, per level, the spells in slot order and how many are
shown, each slot's state, the original spell of a slot under research, and whether the town
researched that day.

**Tests:** the guilds of the towns in `long_game` against what was seen in game, and the
research in Backwater and Scar day by day; the new test saves below; the robustness tests.

## Part 2: a view of the guilds

**Behaviour:**

- Per town, the guild level by level: the spells shown, and those of levels not yet built,
  marked as such.
- Step through the days to see the guild change: levels built, and spells researched.
- Each slot's history, such as "Teleport → Force Field (2-2-5) → Mirth (2-3-1) → Hypnotize
  (2-3-6), finalized".
- The guild at the start, from the first save, beside the guild now.
- For a slot open to research, the spells it can still bring, without their order.

**Data:** the data set gets a series per town: owner, type, name, and the guild on each day. It
has only player and hero series today. Research events (day, town, level, slot, old and new
spell, finalized) are worked out from the series in the data set builder.

**Tests:** the series and research events for `long_game`; browser test of stepping through a
town's guild.

## Test saves needed

A small HotA 1.8 map, saved every day, with an `info.txt` saying what was done on which day:

- A Tower with and without the Library, to see the sixth slot.
- A Conflux with Aurora Borealis.
- A research left unfinalized, researched again, and finalized on a later day.
- Three researches in one slot, to find the research count and see Town Portal or Dimension Door.
- A map with spells banned, to see how an empty slot is stored.
- A research by the first player and one by a later player on the same day, to see whether a
  day's save holds that day's research or the day before's for players after the first.
- A town that changes owner.

## Open

- Where the view goes: beside a chart, in a Kingdom or Town overview, or a page of its own
  stepping through the towns. To be decided with the page layout.
- Whether to show the levels not yet built, which tell what the guild will teach, always or only
  when asked.
- Whether the order of the spells research would bring can be worked out from the seed. It needs
  HotA's generator, so it is left out unless that becomes known.
- What a save holds when research is turned off for the map.
- Charts from the guilds, such as the different spells in a player's guilds or the number of
  researches: a later feature.

# HotA save file format

What the parser in `crates/h3save` reads from a Heroes of Might and Magic III: Horn of the Abyss
save, and how it finds it. It covers HotA 1.7 and 1.8 saves; much of the rest of a save is not
known yet. Offsets are in bytes, numbers are little-endian, and "u8", "u16" and "u32" are
unsigned integers of 1, 2 and 4 bytes.

Each section names the code that reads it and the tests that check it, so this document and the
parser can be kept in step.

## Overview

A save (`.GM1`…`.GM8` for scenarios, `.GMC` for campaigns) is gzip-compressed. Its gzip trailer
never matches the data, so the integrity check at the very end fails and is ignored once all data
is out. Decompressed, the parts the parser reads come in this order:

| Part | Where | Size |
|---|---|---|
| [Header](#header) | Offset 0 | Map name and description vary |
| … | Not read | Varies |
| [Player records](#player-records) | Right before the town block | 8 × 149 |
| [Town block](#town-block) | Right before the hero array | Varies |
| [Hero array](#hero-array) | Found by its shape | Heroes × record size |
| … | Not read | Fixed |
| [Date](#date) | A fixed distance after the hero array | 6 |
| … | Not read | 113 |
| [Visibility](#visibility-fog-of-war) | 119 bytes after the start of the date | Tiles × 2 |

Nothing before the hero array is at a fixed position, so the parser finds the blocks by the shape
of their data (`Layout::find` in `savefile.rs`):

1. The hero array is the one position where every record, as many as the header says, looks
   like a hero record (`hero::is_plausible_record`).
2. The town block ends exactly where the hero array starts. It starts with a town count byte,
   followed by each town's index (0, 1, 2, …) and record.
3. The eight player records end right where the town block starts.
4. The date and the visibility are a fixed distance after the hero array.

## Versions

The HotA version is at offset 16 (major, u32) and 20 (minor, u32), with the patch at 24. The
few sizes that differ between versions are in `Format` (`savefile.rs`):

| | HotA 1.7 | HotA 1.8 |
|---|---|---|
| Heroes | 198 | 215 (17 Bulwark heroes) |
| Secondary skills | 29 | 30 (Runes) |
| Hero record | 1159 | 1160 (one more skill slot) |
| Hero array end to date | 407 | 441 (two bytes more per hero) |

A save of any other version is an `UnsupportedVersion` error.

## Header

| Offset | Type | Field |
|---|---|---|
| 0 | 5 bytes and a null | `H3SVG` for a scenario, `H3SVC` for a campaign |
| 16 | u32 × 3 | HotA version: major, minor, patch |
| 80 | u32 | Number of hero records |
| 90 | u8 | Map size in tiles per side: 36, 72, 108, 144, 180, 216 or 252 |
| 94 | u8 | 1 if the map has an underground level |
| 95 | u16 | Length of the map name |
| 97 | Bytes | Map name |
| 97 + name | u16 | Length of the description |
| 99 + name | Bytes | Description |

Lobby games of HotA 1.7 put the players' names in the map name, separated by control characters.

## Player records

Eight records of 149 bytes, red first, ending where the town block starts (`player.rs`).

| Offset | Type | Field |
|---|---|---|
| 0 | u8 | Player index, 0–7 |
| 1 | u8 | Heroes on the map |
| 2 | u8 | Selected hero |
| 3 | u8 × 8 | IDs of the heroes on the map, `0xFF` for none |
| 24 | u8 | Towns |
| 25 | u8 | Selected town |
| 26 | u8 × 48 | Indices of the towns in the town block, `0xFF` for none |
| 98 | u32 × 7 | Wood, mercury, ore, sulfur, crystal, gems, gold |

## Town block

A town count byte, then for each town its index byte and a record of 566 bytes plus the length
of its name (`town.rs`). In each record, after the index byte:

| Offset | Type | Field |
|---|---|---|
| 0 | u8 | Owner, 0–7, `0xFF` for none |
| 3 | u8 | Town type: 0 Castle … 8 Conflux, 9 Cove, 10 Factory, 11 Bulwark |
| 4 | u8 × 3 | x, y and level of the town's entrance |
| 9 | u32 × 7 | Garrison creature IDs, `0xFFFFFFFF` for an empty stack |
| 37 | u32 × 7 | Garrison creature counts |
| 69 | u16 | Length of the name |
| 71 | Bytes | Name |

## Hero array

One record per hero, hired or not, in hero ID order (`hero.rs`). In each record:

| Offset | Type | Field |
|---|---|---|
| 0 | u16 × 3 | x, y and level; `0xFFFF` when not on the map |
| 6 | u8 | 1 when on the map; 0 in a town garrison or a prison |
| 26 | u8 | Owner, 0–7, `0xFF` for none |
| 41 | u8 | Direction the hero faces, 0–7 |
| 45 | u32 × 3 | Destination x, y and level |
| 57 | u32 | Movement points at the start of the day |
| 61 | u32 | Movement points left |
| 65 | u32 | Experience |
| 69 | u32 | Number of secondary skills |
| 73 | u16 | Spell points |
| 75 | u8 | Level |
| 139 | u32 × 7 | Army creature IDs |
| 167 | u32 × 7 | Army creature counts |
| 195 | 13 bytes | Name, null-padded |
| 208 | u8 × 30 | Level of each secondary skill, 0–3; Runes last (always 0 before 1.8) |
| 264 | u8 × 4 | Attack, defense, power, knowledge |
| 268 | u8 × 70 | Spells written in the spell book |
| 338 | u8 × 70 | Spells available, from the spell book and artifacts |
| 408 | (u32, u32) × 83 | Artifacts, worn then the backpack: artifact ID and a scroll's spell |
| 1118 | u8 × 30 (29 in 1.7) | Slot 1–8 of each secondary skill, 0 if not learned |

The record ends with 12 more bytes, not read.

## Date

Three u16s, day, week and month, each from 1 (`date.rs`). The distance from the end of the hero
array is in the table under [Versions](#versions); it is the same in every test save.

## Visibility (fog of war)

The tiles each player has seen (`visibility.rs`). A tile stays seen once a hero or town of the
player has had it in view, so this is how much of the map a player has explored. It starts 119
bytes after the start of the date, in HotA 1.7 and 1.8, and holds 2 bytes per tile:

- Tiles go level by level (surface first), row by row, so the tile at `(x, y, z)` is number
  `(z × size + y) × size + x`, at byte `2 × number`.
- In the first byte, bit `p` is set when player `p` (0 red, 1 blue, … 7 pink) has seen the tile.
- The second byte is 0 or 1 in every test save. What it means is not known.

A hero sees every tile `(x, y)` on its own level with `dx² + dy² ≤ r(r + 1)`, that is within
`r + ½` tiles, where `r` is its scouting radius: 5, plus 1, 3 or 5 for Scouting, and more for
some artifacts and specialities.

How it was found, with `test_data/map_visibility`: on a small map with an underground level,
`36 × 36 × 2 = 2592` tiles, the players' heroes see 97, 137, 225, 349 and 577 tiles on day 1.
The one place in the save where a run of tiles holds exactly those counts in bits 0–4 is this
block, and the tiles each player has seen form discs centred on their heroes. In every test save,
every hero on the map has seen its own tile.

Tests: `crates/h3save/tests/visibility.rs`.

## Not known yet

- Most of the data before the player records: map objects, mines, dwellings and their owners.
- The 113 bytes between the date and the visibility, and everything after the visibility.
- The second byte of each tile in the visibility.

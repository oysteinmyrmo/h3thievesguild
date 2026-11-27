# Creature pictures

The small (32 × 32) creature portraits shown in the chart tooltips, one per creature, named after
the creature's constant in `crates/h3save/src/creatures.rs` (for example `BlackDragon.png`).

## Source

Downloaded on 2026-11-27 from the "Mini Portrait" images on the Heroes 3 wiki's
[List of creatures](https://heroes.thelazy.net/index.php/List_of_creatures), for example
`Mini_Portrait_Black_Dragon.png`. Where the wiki has a Horn of the Abyss version of a picture,
such as `Mini_Portrait_Pikeman_(HotA).png`, that one is used, as these are HotA games. The
Halfling is the HotA picture, as the creature is both the neutral one and the Factory's first.

The pictures are unchanged apart from removing metadata: 14 of them carried an EXIF block with
only an embedded thumbnail, and an upload timestamp. The image data, palette and transparency
are as downloaded.

## Rights

The pictures are not ours and come with no licence. The wiki does not state one: its image pages
have none, and its disclaimer page does not exist. The art belongs to its owners: the original
creatures to the owners of Heroes of Might and Magic III (Ubisoft), the Cove, Factory and
Bulwark creatures and the redrawn pictures to the Horn of the Abyss team.

They are used here in a non-commercial fan tool, pending permission from the Horn of the Abyss
team. If any owner asks, remove them, and the icon in `best_creature` in
`crates/h3dataset/src/stats.rs` with the tests that check the pictures; the tooltips then show
the name and value without a picture.

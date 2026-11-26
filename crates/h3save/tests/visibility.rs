// The tiles each player has seen: the fog of war.
//
// test_data/map_visibility is a small map (36 x 36) with an underground level, 2592 tiles, saved
// on days 1 to 3; its info.txt lists what each player sees. A hero sees every tile within its
// scouting radius r, that is every tile (x, y) with dx² + dy² <= r(r + 1), on its own level.
// The default radius is 5; Scouting adds 1, 3 or 5, artifacts more.
//
// Three things in info.txt are off: orange's 577 tiles on day 1 are 22.260_8% of the map, not
// 22.2698%; orange has 31 underground tiles unseen on day 3, not 30, so sees 1265 tiles rather
// than 1266; and red's 177 tiles on day 3 are a radius of 7, so the Speculum adds 1 to the
// radius, not 2.

mod common;

use common::{Decompressed, all_saves, data_dir, find_hero, open};
use h3save::SaveFile;
use h3save::player::Color;
use h3save::visibility::{self, Visibility};

const SIZE: usize = 36;
const TILES: usize = SIZE * SIZE * 2;

fn day(n: u32) -> SaveFile {
    open(&data_dir(&format!("map_visibility/[hotseat] 11{n}.GM5")))
}

/// Tiles seen by each of red, blue, tan, green and orange, on the surface and underground.
fn seen(save: &SaveFile) -> Vec<[usize; 2]> {
    PLAYERS
        .iter()
        .map(|&c| {
            [
                save.visibility.tiles_seen_on_level(c, 0),
                save.visibility.tiles_seen_on_level(c, 1),
            ]
        })
        .collect()
}

const PLAYERS: [Color; 5] = [Color::Red, Color::Blue, Color::Tan, Color::Green, Color::Orange];

#[test]
fn the_map_has_two_levels_of_36_by_36_tiles() {
    for n in 1..=3 {
        let v = day(n).visibility;
        assert_eq!((v.size(), v.levels(), v.tile_count()), (SIZE, 2, TILES));
    }
}

#[test]
fn each_player_has_seen_the_tiles_info_txt_gives() {
    // Red, blue, tan and green are on the surface; orange's Miriam is underground all three days.
    assert_eq!(seen(&day(1)), [[97, 0], [137, 0], [225, 0], [349, 0], [0, 577]]);
    // Red picks up the Spyglass; orange moves around underground.
    assert_eq!(seen(&day(2)), [[137, 0], [137, 0], [225, 0], [349, 0], [0, 990]]);
    // Red picks up the Speculum; orange has seen all but 31 underground tiles.
    assert_eq!(seen(&day(3)), [[177, 0], [137, 0], [225, 0], [349, 0], [0, 1265]]);
    for n in 1..=3 {
        let save = day(n);
        for color in PLAYERS {
            let v = &save.visibility;
            assert_eq!(
                v.tiles_seen(color),
                v.tiles_seen_on_level(color, 0) + v.tiles_seen_on_level(color, 1)
            );
        }
        // Purple, teal and pink are not in the game.
        for color in [Color::Purple, Color::Teal, Color::Pink] {
            assert_eq!(save.visibility.tiles_seen(color), 0, "{color:?}");
        }
    }
}

#[test]
fn the_share_of_the_map_seen_is_out_of_both_levels() {
    let close = |a: f64, b: f64| (a - b).abs() < 1e-5;
    let v = day(1).visibility;
    for (color, percent) in [
        (Color::Red, 3.742_28),
        (Color::Blue, 5.285_49),
        (Color::Tan, 8.680_56),
        (Color::Green, 13.464_506),
        (Color::Orange, 22.260_8),
    ] {
        assert!(
            close(v.share_seen(color) * 100.0, percent),
            "{color:?}: {}",
            v.share_seen(color) * 100.0
        );
    }
    assert!(close(day(2).visibility.share_seen(Color::Red) * 100.0, 5.285_49));
    assert!(close(day(3).visibility.share_seen(Color::Red) * 100.0, 6.828_70));
    // 1265 of 2592 tiles.
    assert!(close(day(3).visibility.share_seen(Color::Orange) * 100.0, 48.804_01));
    assert_eq!(v.share_seen(Color::Pink), 0.0);
}

/// Whether the player has seen exactly the tiles within radius `r` of (x, y) on level z.
fn sees_disc(v: &Visibility, color: Color, (x, y, z): (usize, usize, usize), radius: usize) -> bool {
    (0..2).all(|level| {
        (0..SIZE).all(|ty| {
            (0..SIZE).all(|tx| {
                let (dx, dy) = (tx.abs_diff(x), ty.abs_diff(y));
                let inside = level == z && dx * dx + dy * dy <= radius * (radius + 1);
                v.is_seen(color, tx, ty, level) == inside
            })
        })
    })
}

#[test]
fn a_hero_that_has_not_moved_has_seen_a_disc_of_its_scouting_radius() {
    let first = day(1);
    let at = |name: &str| {
        let hero = find_hero(&first, name);
        (usize::from(hero.x), usize::from(hero.y), usize::from(hero.z))
    };
    for (color, name, radius) in [
        (Color::Red, "Sylvia", 5),
        // Basic, Advanced and Expert Scouting.
        (Color::Blue, "Giselle", 6),
        (Color::Tan, "Astral", 8),
        (Color::Green, "Lacus", 10),
        // Expert Scouting, and her Scouting speciality at level 18.
        (Color::Orange, "Miriam", 13),
    ] {
        assert!(sees_disc(&first.visibility, color, at(name), radius), "{name} on day 1");
    }
    // Red's Sylvia stays put, and her radius grows with the Spyglass and then the Speculum.
    assert!(sees_disc(&day(2).visibility, Color::Red, at("Sylvia"), 6));
    assert!(sees_disc(&day(3).visibility, Color::Red, at("Sylvia"), 7));
}

#[test]
fn a_tile_once_seen_stays_seen() {
    let days: Vec<SaveFile> = (1..=3).map(day).collect();
    for pair in days.windows(2) {
        let [before, after] = pair else { unreachable!() };
        for color in PLAYERS {
            for z in 0..2 {
                for y in 0..SIZE {
                    for x in 0..SIZE {
                        if before.visibility.is_seen(color, x, y, z) {
                            assert!(after.visibility.is_seen(color, x, y, z), "{color:?} ({x}, {y}, {z})");
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn tiles_off_the_map_are_not_seen() {
    let v = day(3).visibility;
    assert!(!v.is_seen(Color::Orange, 35, 0, 1));
    assert!(v.is_seen(Color::Orange, 1, 0, 1));
    for (x, y, z) in [
        (SIZE, 0, 1),
        (0, SIZE, 1),
        (1, 0, 2),
        (usize::MAX, usize::MAX, usize::MAX),
    ] {
        assert!(!v.is_seen(Color::Orange, x, y, z), "({x}, {y}, {z})");
    }
    assert_eq!(v.tiles_seen_on_level(Color::Orange, 2), 0);
}

#[test]
fn every_hero_on_the_map_has_seen_its_own_tile_in_every_save() {
    // The strongest check that the tiles are read from the right place: it holds in every test
    // save, of every map size, with and without an underground level.
    let saves = all_saves();
    assert!(saves.len() > 70);
    for path in saves {
        let save = open(&path);
        let v = &save.visibility;
        let levels = if save.has_underground == 0 { 1 } else { 2 };
        assert_eq!(
            (v.size(), v.levels()),
            (usize::from(save.map_size.0), levels),
            "{}",
            path.display()
        );
        for hero in save.heroes.iter().filter(|h| h.on_map == 1 && h.player < 8) {
            let color = Color::ALL[usize::from(hero.player)];
            let (x, y, z) = (usize::from(hero.x), usize::from(hero.y), usize::from(hero.z));
            assert!(
                v.is_seen(color, x, y, z),
                "{}: {} at ({x}, {y}, {z})",
                path.display(),
                hero.name()
            );
        }
    }
}

#[test]
fn the_second_byte_of_every_tile_is_0_or_1() {
    // What the second byte holds is not known; this is what the documentation says of it.
    for path in all_saves() {
        let d = Decompressed::read(&path);
        let save = SaveFile::parse(&d.data).unwrap();
        let start = d.heroes_end() + d.format.date_offset + visibility::OFFSET_AFTER_DATE;
        let tiles = &d.data[start..start + save.visibility.tile_count() * visibility::TILE_SIZE];
        assert!(tiles.chunks(2).all(|t| t[1] <= 1), "{}", path.display());
    }
}

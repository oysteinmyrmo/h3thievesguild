// Tests for the player records: heroes on the map, towns and their place in the save.
// Expected values come from each test folder's info.txt and the existing tests.

#[macro_use]
mod common;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use common::{data_dir, open};
use h3save::player::{MAX_PLAYERS, NONE, PLAYER_RECORD_SIZE, players};
use h3save::savefile::{Error, Layout, decompress_file};
use h3save::{Color, SaveFile, TownType};

fn all_saves() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e.to_string_lossy().starts_with("GM")) {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(&data_dir(""), &mut out);
    out.sort();
    out
}

/// Names of the heroes a player has on the map, in the player's order.
fn heroes_on_map(save: &SaveFile, color: Color) -> Vec<String> {
    save.players[color.index()]
        .hero_ids_on_map()
        .iter()
        .map(|&id| save.heroes[usize::from(id)].name())
        .collect()
}

/// Names of a player's towns, in the player's order.
fn town_names(save: &SaveFile, color: Color) -> Vec<&str> {
    save.players[color.index()]
        .town_indices()
        .iter()
        .map(|&i| save.towns[usize::from(i)].name.as_str())
        .collect()
}

#[test]
fn player_records_agree_with_the_hero_and_town_records_in_every_save() {
    let saves = all_saves();
    assert_eq!(saves.len(), 70);
    for path in saves {
        let save = open(&path);
        let what = path.display();
        for color in Color::ALL {
            let c = u8::try_from(color.index()).unwrap();
            let data = &save.players[color.index()];
            // Every hero the player owns that is on the map is listed, once.
            let listed: BTreeSet<u8> = data.hero_ids_on_map().iter().copied().collect();
            assert_eq!(
                listed.len(),
                data.hero_ids_on_map().len(),
                "{what} {color:?}: duplicate hero"
            );
            let owned_on_map: BTreeSet<u8> = (0u8..)
                .zip(&save.heroes)
                .filter(|(_, h)| h.player == c && h.on_map != 0)
                .map(|(id, _)| id)
                .collect();
            assert_eq!(listed, owned_on_map, "{what} {color:?}: heroes on map");
            // Every town the player owns is listed, once.
            let listed: BTreeSet<u8> = data.town_indices().iter().copied().collect();
            assert_eq!(
                listed.len(),
                data.town_indices().len(),
                "{what} {color:?}: duplicate town"
            );
            let owned: BTreeSet<u8> = (0u8..)
                .zip(&save.towns)
                .filter(|(_, t)| t.owner == c)
                .map(|(i, _)| i)
                .collect();
            assert_eq!(listed, owned, "{what} {color:?}: towns");
            // Unused slots are empty; a selection is one of the player's own.
            assert!(
                data.hero_ids[data.hero_ids_on_map().len()..]
                    .iter()
                    .all(|&id| id == NONE)
            );
            assert!(data.town_ids[data.town_indices().len()..].iter().all(|&id| id == NONE));
            assert!(data.selected_hero == NONE || data.hero_ids_on_map().contains(&data.selected_hero));
            assert!(data.selected_town == NONE || data.town_indices().contains(&data.selected_town));
        }
    }
}

#[test]
fn player_records_end_where_the_town_block_starts() {
    for path in all_saves() {
        let data = decompress_file(&path).unwrap();
        let save = SaveFile::parse(&data).unwrap();
        let hero_count = usize::try_from(save.hero_count).unwrap();
        let layout = Layout::find(&data, hero_count).unwrap();
        assert_eq!(
            layout.players + MAX_PLAYERS * PLAYER_RECORD_SIZE,
            layout.town_count,
            "{}",
            path.display()
        );
        for i in 0..MAX_PLAYERS {
            assert_eq!(usize::from(data[layout.players + i * PLAYER_RECORD_SIZE]), i);
        }
        // Resources are at offset 98 of each record.
        for (i, player) in save.players.iter().enumerate() {
            let gold_at = layout.players + i * PLAYER_RECORD_SIZE + 98 + 6 * 4;
            assert_eq!(
                u32::from_le_bytes(data[gold_at..gold_at + 4].try_into().unwrap()),
                player.gold
            );
        }
    }
}

#[test]
fn one_hero_per_player() {
    let _guard = common::Guard::new("players-one-hero-per-player");
    let names = [
        "Orrin",
        "Valeska",
        "Edric",
        "Sylvia",
        "Lord Haart",
        "Sorsha",
        "Christian",
        "Tyris",
    ];
    // In the second save every hero moved one colour up, and pink got Orrin.
    for (file, shift) in [("[hotseat] 111.01.GM8", 0), ("[hotseat] 111.02.GM8", 1)] {
        let save = open(&data_dir("hero_player").join(file));
        for color in Color::ALL {
            let expected = names[(color.index() + shift) % MAX_PLAYERS];
            require_eq!(heroes_on_map(&save, color), vec![expected]);
            require_eq!(save.players[color.index()].town_count, 0);
        }
    }
}

#[test]
fn heroes_garrisoned_in_a_town_are_not_on_the_map() {
    let _guard = common::Guard::new("players-garrisoned-heroes");
    let expected: [(&str, [[&str; 3]; 3]); 2] = [
        (
            "[hotseat] 111.GM3",
            [
                ["Orrin", "Valeska", "Edric"],
                ["Mephala", "Ufretin", "Jenova"],
                ["Piquedram", "Thane", "Josephine"],
            ],
        ),
        (
            "[hotseat] 112.GM3",
            [
                ["Orrin", "Valeska", "Edric"],
                ["Mephala", "Ufretin", "Jenova"],
                ["Piquedram", "Thane", "Josephine"],
            ],
        ),
    ];
    for (file, heroes) in expected {
        let save = open(&data_dir("hero_town").join(file));
        for (color, names) in Color::ALL.into_iter().zip(heroes) {
            require_eq!(heroes_on_map(&save, color), names.to_vec());
        }
    }
    // Day 3: the second hero of each player is in the town garrison.
    let save = open(&data_dir("hero_town").join("[hotseat] 113.GM3"));
    require_eq!(heroes_on_map(&save, Color::Red), vec!["Orrin", "Edric"]);
    require_eq!(heroes_on_map(&save, Color::Blue), vec!["Mephala", "Jenova"]);
    require_eq!(heroes_on_map(&save, Color::Tan), vec!["Piquedram", "Josephine"]);
    // Day 4: the first and second hero swapped places. A hero leaving the garrison goes to the
    // end of the list.
    let save = open(&data_dir("hero_town").join("[hotseat] 114.GM3"));
    require_eq!(heroes_on_map(&save, Color::Red), vec!["Edric", "Valeska"]);
    require_eq!(heroes_on_map(&save, Color::Blue), vec!["Jenova", "Ufretin"]);
    require_eq!(heroes_on_map(&save, Color::Tan), vec!["Josephine", "Thane"]);
    for color in [Color::Red, Color::Blue, Color::Tan] {
        let player = &players(&save)[color.index()];
        require_eq!(player.heroes().len(), 3);
        require_eq!(save.players[color.index()].hero_count, 2);
    }
}

#[test]
fn heroes_and_towns_of_the_kingdom_army_strength_players() {
    let _guard = common::Guard::new("players-kingdom-army-strength");
    let players_and_towns = [
        (Color::Green, "Rion", "GreensTown"),
        (Color::Orange, "Adelaide", "OrangesTown"),
        (Color::Purple, "Ingham", "PurplesTown"),
    ];
    for day in 1..=4 {
        let save = open(&data_dir("kingdom_army_strength").join(format!("[hotseat] 11{day}.GM3")));
        for color in Color::ALL {
            if let Some(&(_, hero, town)) = players_and_towns.iter().find(|(c, _, _)| *c == color) {
                require_eq!(heroes_on_map(&save, color), vec![hero]);
                require_eq!(town_names(&save, color), vec![town]);
            } else {
                require_eq!(save.players[color.index()].hero_count, 0);
                require_eq!(save.players[color.index()].town_count, 0);
            }
        }
    }
    // Day 5: the heroes are in their town garrisons, so none is on the map.
    let save = open(&data_dir("kingdom_army_strength").join("[hotseat] 115.GM3"));
    for (color, _, town) in players_and_towns {
        require_eq!(save.players[color.index()].hero_count, 0);
        require_eq!(players(&save)[color.index()].heroes().len(), 1);
        require_eq!(town_names(&save, color), vec![town]);
    }
}

#[test]
fn vanquished_players_have_no_heroes_or_towns() {
    let _guard = common::Guard::new("players-vanquished");
    let save = open(&data_dir("resources").join("[hotseat] 112.GM8"));
    for player in players(&save) {
        require!(player.alive());
        require!(save.players[player.color().index()].hero_count >= 1);
    }
    let save = open(&data_dir("resources").join("[hotseat] 113.GM5"));
    for player in players(&save) {
        let vanquished = [Color::Blue, Color::Tan, Color::Orange].contains(&player.color());
        let data = &save.players[player.color().index()];
        require_eq!(player.alive(), !vanquished);
        require_eq!(data.hero_count == 0, vanquished);
        require_eq!(data.hero_ids_on_map().is_empty(), vanquished);
        require_eq!(data.town_count, 0);
        if vanquished {
            require_eq!(data.selected_hero, NONE);
        }
    }
}

#[test]
fn each_town_name_test_player_owns_sixteen_towns_of_one_type() {
    let _guard = common::Guard::new("players-town-names");
    let owners: [(&str, &[(Color, TownType)]); 4] = [
        (
            "01",
            &[
                (Color::Green, TownType::CASTLE),
                (Color::Orange, TownType::RAMPART),
                (Color::Purple, TownType::TOWER),
            ],
        ),
        (
            "02",
            &[
                (Color::Blue, TownType::INFERNO),
                (Color::Purple, TownType::NECROPOLIS),
                (Color::Pink, TownType::DUNGEON),
            ],
        ),
        (
            "03",
            &[(Color::Red, TownType::STRONGHOLD), (Color::Teal, TownType::FORTRESS)],
        ),
        (
            "04",
            &[(Color::Orange, TownType::CONFLUX), (Color::Green, TownType::COVE)],
        ),
    ];
    for (folder, owners) in owners {
        let dir = data_dir("town_names").join(folder);
        let file = std::fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| p.extension().is_some_and(|e| e.to_string_lossy().starts_with("GM")))
            .unwrap();
        let save = open(&file);
        for color in Color::ALL {
            let data = &save.players[color.index()];
            require_eq!(data.hero_count, 0);
            if let Some(&(_, town_type)) = owners.iter().find(|(c, _)| *c == color) {
                require_eq!(data.town_count, 16);
                for &i in data.town_indices() {
                    require_eq!(save.towns[usize::from(i)].town_type, town_type);
                }
            } else {
                require_eq!(data.town_count, 0);
            }
        }
    }
}

#[test]
fn prison_hero_joins_the_player() {
    let _guard = common::Guard::new("players-prison");
    let save = open(&data_dir("prison").join("[hotseat] 111.GM2"));
    require_eq!(heroes_on_map(&save, Color::Red), vec!["Spint"]);
    require_eq!(heroes_on_map(&save, Color::Blue), vec!["Solmyr"]);
    let save = open(&data_dir("prison").join("[hotseat] 112.GM2"));
    require_eq!(heroes_on_map(&save, Color::Red), vec!["Spint", "Sandro"]);
    require_eq!(heroes_on_map(&save, Color::Blue), vec!["Solmyr"]);
}

#[test]
fn players_without_towns() {
    let _guard = common::Guard::new("players-without-towns");
    for name in ["start.GM2", "[hotseat] 111.GM2", "[hotseat] 117.GM2"] {
        let save = open(&data_dir("movement").join(name));
        require_eq!(heroes_on_map(&save, Color::Red), vec!["Orrin"]);
        require_eq!(heroes_on_map(&save, Color::Blue), vec!["Valeska"]);
        for color in Color::ALL {
            require_eq!(save.players[color.index()].town_count, 0);
        }
    }
}

/// Decompressed sample data and the position of the first player record.
fn sample() -> (Vec<u8>, usize) {
    let data = decompress_file(data_dir("resources").join("[hotseat] 111.GM8")).unwrap();
    let hero_count = usize::try_from(u32::from_le_bytes(data[80..84].try_into().unwrap())).unwrap();
    let players = Layout::find(&data, hero_count).unwrap().players;
    (data, players)
}

#[test]
fn a_damaged_player_record_is_an_error() {
    let (data, players) = sample();
    let record = |i: usize| players + i * PLAYER_RECORD_SIZE;
    let changes = [
        (record(0), 1),        // Red's record carries index 1.
        (record(7), 0),        // Pink's record carries index 0.
        (record(3) + 1, 9),    // Nine heroes on the map.
        (record(3) + 1, 2),    // Two heroes, but the second slot is empty.
        (record(2) + 1, 0),    // No heroes, but the first slot is used.
        (record(4) + 3, 215),  // A hero ID past the last hero.
        (record(4) + 3, NONE), // An empty slot inside the used ones.
        (record(5) + 2, 3),    // The selected hero is not the player's.
        (record(6) + 24, 1),   // A town, but this map has none.
        (record(6) + 25, 0),   // A selected town the player does not own.
        (record(1) + 10, 0),   // A hero in an unused slot.
        (record(1) + 73, 0),   // A town in an unused slot.
    ];
    for (pos, value) in changes {
        let mut damaged = data.clone();
        damaged[pos] = value;
        let result = SaveFile::parse(&damaged);
        assert!(
            matches!(result, Err(Error::PlayersNotFound)),
            "byte {pos} set to {value}: {result:?}"
        );
    }
    // Bytes the parser does not interpret can change freely.
    let mut untouched = data.clone();
    for i in 0..MAX_PLAYERS {
        untouched[record(i) + 11] ^= 0xFF;
        untouched[record(i) + 148] ^= 0xFF;
    }
    assert!(SaveFile::parse(&untouched).is_ok());
}

#[test]
fn players_cannot_start_before_the_data() {
    // Cut the data so the town block starts less than eight player records in. The heroes and
    // towns are still found, but there is no room for the players.
    let (data, players) = sample();
    for cut in [
        players + 1,
        players + PLAYER_RECORD_SIZE,
        players + 7 * PLAYER_RECORD_SIZE,
    ] {
        let result = Layout::find(&data[cut..], 215);
        assert!(
            matches!(result, Err(Error::PlayersNotFound)),
            "cut at {cut}: {result:?}"
        );
    }
    assert!(Layout::find(&data[players..], 215).is_ok());
}

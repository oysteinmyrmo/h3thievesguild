// Tests for building data sets. Expected values come from the test folders' info.txt files.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use h3dataset::{Builder, Dataset, DatasetError, DayFiles, DayRange, GameFiles, MAX_DAYS, PlayerSeries, build};
use h3save::Date;
use serde_json::{Value, json};

fn test_data(folder: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_data")
        .join(folder)
}

/// Build from the named files of a test folder, added in the order given.
fn dataset_from(folder: &str, files: &[&str]) -> Result<Dataset, DatasetError> {
    let mut builder = Builder::new();
    for file in files {
        builder.add_file(*file, &std::fs::read(test_data(folder).join(file)).unwrap());
    }
    builder.finish()
}

fn player<'a>(dataset: &'a Dataset, id: &str) -> &'a PlayerSeries {
    dataset
        .players
        .iter()
        .find(|p| p.id == id)
        .unwrap_or_else(|| panic!("no player {id}"))
}

fn values(dataset: &Dataset, player_id: &str, stat: &str) -> Vec<Option<f64>> {
    player(dataset, player_id).values[stat].clone()
}

/// Wood, mercury, ore, sulfur, crystal, gems and gold.
type Resources = [u32; 7];

/// Whole numbers as the `Option<f64>` values of a series.
fn some(values: &[u32]) -> Vec<Option<f64>> {
    values.iter().map(|&v| Some(f64::from(v))).collect()
}

/// Checks that hold for every data set.
fn check_shape(dataset: &Dataset) {
    let days = dataset.days.len();
    // Every day from the first save to the last, both of which have a save.
    assert!(
        dataset.days.windows(2).all(|w| w[0].day + 1 == w[1].day),
        "consecutive days"
    );
    assert!(dataset.days.first().unwrap().file.is_some() && dataset.days.last().unwrap().file.is_some());
    let has_save: Vec<bool> = dataset.days.iter().map(|d| d.file.is_some()).collect();
    // The missing days are exactly the days without a file, as runs.
    let listed: Vec<u32> = dataset.missing_days.iter().flat_map(|r| r.first..=r.last).collect();
    let without_file: Vec<u32> = dataset
        .days
        .iter()
        .filter(|d| d.file.is_none())
        .map(|d| d.day)
        .collect();
    assert_eq!(listed, without_file);
    assert!(
        dataset.missing_days.windows(2).all(|w| w[0].last + 1 < w[1].first),
        "separate runs"
    );
    assert_eq!(dataset.missing_days_warning.is_some(), !dataset.missing_days.is_empty());
    for day in &dataset.days {
        assert_eq!(day.label, format!("{}-{}-{}", day.month, day.week, day.day_of_week));
        let from_parts = (u32::from(day.month) - 1) * 28 + (u32::from(day.week) - 1) * 7 + u32::from(day.day_of_week);
        assert_eq!(day.day, from_parts);
    }
    let player_stats: BTreeSet<&str> = dataset.player_stats.iter().map(|s| s.id).collect();
    assert_eq!(player_stats.len(), dataset.player_stats.len(), "unique player stat IDs");
    let hero_stats: BTreeSet<&str> = dataset.hero_stats.iter().map(|s| s.id).collect();
    assert_eq!(hero_stats.len(), dataset.hero_stats.len(), "unique hero stat IDs");
    let ids: Vec<&str> = dataset.players.iter().map(|p| p.id.as_str()).collect();
    let order = ["red", "blue", "tan", "green", "orange", "purple", "teal", "pink"];
    assert!(
        ids.windows(2)
            .all(|w| order.iter().position(|o| *o == w[0]) < order.iter().position(|o| *o == w[1]))
    );
    for p in &dataset.players {
        assert_eq!(
            p.values.keys().map(String::as_str).collect::<BTreeSet<_>>(),
            player_stats
        );
        assert!(p.values.values().all(|row| row.len() == days));
        // In the game on the same days for every statistic, and on at least one day. Days without
        // a save have no values.
        let alive: Vec<bool> = p.values["gold"].iter().map(Option::is_some).collect();
        assert!(alive.iter().zip(&has_save).all(|(&a, &s)| s || !a));
        assert!(alive.contains(&true));
        assert!(
            p.values
                .values()
                .all(|row| row.iter().map(Option::is_some).eq(alive.iter().copied()))
        );
        // Eliminated on the first day with a save where the player is out of the game, after
        // being in it.
        let first_in = alive.iter().position(|&a| a).unwrap();
        let first_out = (first_in..days)
            .find(|&i| has_save[i] && !alive[i])
            .map(|i| dataset.days[i].day);
        assert_eq!(p.eliminated_day, first_out);
        assert!(p.color.len() == 7 && p.color.starts_with('#'));
    }
    for hero in &dataset.heroes {
        assert_eq!(hero.owner.len(), days);
        assert_eq!(
            hero.values.keys().map(String::as_str).collect::<BTreeSet<_>>(),
            hero_stats
        );
        for row in hero.values.values() {
            assert_eq!(row.len(), days);
            assert!(row.iter().zip(&hero.owner).all(|(v, o)| v.is_some() == o.is_some()));
        }
        assert!(hero.owner.iter().flatten().all(|o| ids.contains(&o.as_str())));
    }
}

#[test]
fn resources_of_every_player_by_day() {
    // Added out of order: the data set sorts by date.
    let dataset = dataset_from(
        "resources",
        &["[hotseat] 113.GM5", "[hotseat] 111.GM8", "[hotseat] 112.GM8"],
    )
    .unwrap();
    check_shape(&dataset);
    let days: Vec<(u32, &str, Option<&str>)> = dataset
        .days
        .iter()
        .map(|d| (d.day, d.label.as_str(), d.file.as_deref()))
        .collect();
    assert_eq!(
        days,
        [
            (1, "1-1-1", Some("[hotseat] 111.GM8")),
            (2, "1-1-2", Some("[hotseat] 112.GM8")),
            (3, "1-1-3", Some("[hotseat] 113.GM5"))
        ]
    );
    // Wood, mercury, ore, sulfur, crystal, gems and gold from info.txt, with the correction the
    // resources test makes: teal's estates add 250 gold on day 3. Blue, tan and orange are
    // vanquished on day 3.
    // Player, then the resources on days 1, 2 and 3.
    let table: [(&str, Resources, Resources, Option<Resources>); 8] = [
        (
            "red",
            [0, 5, 5, 0, 0, 0, 2500],
            [15, 17, 16, 5, 5, 0, 6700],
            Some([19, 32, 21, 5, 21, 0, 10700]),
        ),
        ("blue", [0, 0, 5, 0, 0, 0, 3400], [0, 12, 23, 0, 8, 11, 6700], None),
        ("tan", [0, 5, 5, 0, 0, 0, 2500], [0, 5, 15, 18, 6, 0, 3100], None),
        (
            "green",
            [0, 0, 5, 0, 0, 0, 3400],
            [5, 10, 10, 14, 0, 0, 6800],
            Some([5, 10, 10, 14, 0, 0, 6800]),
        ),
        ("orange", [0, 3, 5, 0, 0, 0, 2500], [5, 3, 5, 0, 6, 12, 7250], None),
        (
            "purple",
            [0, 0, 5, 0, 0, 0, 3300],
            [0, 0, 20, 5, 6, 10, 6900],
            Some([0, 0, 20, 5, 6, 10, 6900]),
        ),
        (
            "teal",
            [0, 6, 5, 0, 0, 0, 2500],
            [6, 6, 11, 4, 6, 10, 5950],
            Some([6, 6, 11, 4, 6, 10, 6200]),
        ),
        (
            "pink",
            [0, 0, 5, 0, 0, 0, 3500],
            [6, 6, 11, 17, 6, 10, 5000],
            Some([6, 6, 11, 17, 6, 10, 5000]),
        ),
    ];
    assert_eq!(dataset.players.len(), 8);
    let stats = ["wood", "mercury", "ore", "sulfur", "crystal", "gems", "gold"];
    for (id, day1, day2, day3) in table {
        for (i, stat) in stats.iter().enumerate() {
            let expected = vec![
                Some(f64::from(day1[i])),
                Some(f64::from(day2[i])),
                day3.map(|d| f64::from(d[i])),
            ];
            assert_eq!(values(&dataset, id, stat), expected, "{id} {stat}");
        }
        assert_eq!(player(&dataset, id).eliminated_day, day3.is_none().then_some(3), "{id}");
    }
}

#[test]
fn hero_levels_and_experience_by_day() {
    let files: Vec<String> = ["111", "112", "113", "114", "115", "116", "117", "121"]
        .iter()
        .map(|d| format!("[hotseat] {d}.GM2"))
        .collect();
    let names: Vec<&str> = files.iter().map(String::as_str).collect();
    let dataset = dataset_from("experience", &names).unwrap();
    check_shape(&dataset);
    assert_eq!(
        dataset.days.iter().map(|d| d.day).collect::<Vec<_>>(),
        (1..=8).collect::<Vec<_>>()
    );
    // Only purple and teal play this map.
    assert_eq!(
        dataset.players.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(),
        ["purple", "teal"]
    );
    // From info.txt, after each day's battle. The day 1 save is from before the first battle.
    let giselle = dataset.heroes.iter().find(|h| h.name == "Giselle").unwrap();
    assert_eq!(giselle.values["level"], some(&[2, 7, 9, 10, 13, 14, 16, 18]));
    assert_eq!(
        giselle.values["experience"],
        some(&[1500, 9000, 13000, 17000, 26000, 29750, 41750, 62375])
    );
    assert_eq!(giselle.values["attack"], some(&[1, 4, 4, 4, 5, 5, 6, 7]));
    assert_eq!(giselle.values["knowledge"], some(&[1, 2, 2, 2, 2, 3, 3, 3]));
    assert!(giselle.owner.iter().all(|o| o.as_deref() == Some("purple")));
    let thant = dataset.heroes.iter().find(|h| h.name == "Thant").unwrap();
    assert_eq!(thant.values["level"], some(&[2, 12, 18, 20, 21, 21, 21, 22]));
    assert_eq!(
        thant.values["experience"],
        some(&[1800, 21800, 61800, 91800, 101_800, 107_800, 112_800, 120_720])
    );
    assert_eq!(thant.values["power"], some(&[2, 5, 6, 7, 7, 7, 7, 7]));
    assert!(thant.owner.iter().all(|o| o.as_deref() == Some("teal")));
    // The player statistics sum up the same heroes.
    assert_eq!(
        values(&dataset, "purple", "highestHeroLevel"),
        some(&[2, 7, 9, 10, 13, 14, 16, 18])
    );
    assert_eq!(
        values(&dataset, "teal", "totalExperience"),
        some(&[1800, 21800, 61800, 91800, 101_800, 107_800, 112_800, 120_720])
    );
}

#[test]
fn kingdom_army_strength_by_day() {
    let files = [
        "[hotseat] 111.GM3",
        "[hotseat] 112.GM3",
        "[hotseat] 113.GM3",
        "[hotseat] 114.GM3",
    ];
    let dataset = dataset_from("kingdom_army_strength", &files).unwrap();
    check_shape(&dataset);
    // From info.txt.
    assert_eq!(
        values(&dataset, "green", "armyStrength"),
        some(&[5019, 5142, 5142, 6752])
    );
    assert_eq!(
        values(&dataset, "orange", "armyStrength"),
        some(&[5019, 5142, 5142, 6752])
    );
    assert_eq!(
        values(&dataset, "purple", "armyStrength"),
        some(&[5019, 5019, 5269, 5269])
    );
    for id in ["green", "orange", "purple"] {
        assert_eq!(values(&dataset, id, "heroes"), some(&[1, 1, 1, 1]));
        assert_eq!(values(&dataset, id, "towns"), some(&[1, 1, 1, 1]));
    }
}

#[test]
fn heroes_in_a_garrison_count_but_are_not_on_the_map() {
    let files = [
        "[hotseat] 111.GM3",
        "[hotseat] 112.GM3",
        "[hotseat] 113.GM3",
        "[hotseat] 114.GM3",
    ];
    let dataset = dataset_from("hero_town", &files).unwrap();
    check_shape(&dataset);
    for id in ["red", "blue", "tan"] {
        assert_eq!(values(&dataset, id, "heroes"), some(&[3, 3, 3, 3]));
        assert_eq!(values(&dataset, id, "heroesOnMap"), some(&[3, 3, 2, 2]));
    }
}

#[test]
fn every_test_folder_of_one_game_gives_a_consistent_data_set() {
    let folders = [
        "artifacts",
        "bulwark",
        "creatures",
        "experience",
        "hero_town",
        "kingdom_army_strength",
        "movement_gigantic",
        "primary_skills",
        "prison",
        "resources",
        "secondary_skills",
        "spell_points",
        "spells",
        "towns",
    ];
    for folder in folders {
        let mut names: Vec<String> = std::fs::read_dir(test_data(folder))
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .filter(|n| n.contains(".GM") && !n.starts_with("start"))
            .collect();
        names.sort();
        let files: Vec<(String, Vec<u8>)> = names
            .iter()
            .map(|n| (n.clone(), std::fs::read(test_data(folder).join(n)).unwrap()))
            .collect();
        let dataset =
            build(files.iter().map(|(n, b)| (n.as_str(), b.as_slice()))).unwrap_or_else(|e| panic!("{folder}: {e}"));
        check_shape(&dataset);
        assert_eq!(dataset.days.len(), names.len(), "{folder}");
    }
}

#[test]
fn no_saves_is_an_error() {
    assert_eq!(Builder::new().finish(), Err(DatasetError::NoSaves));
    assert!(Builder::new().is_empty());
}

#[test]
fn unreadable_files_are_all_reported_by_name() {
    let save = std::fs::read(test_data("resources").join("[hotseat] 111.GM8")).unwrap();
    let mut builder = Builder::new();
    builder.add_file("notes.txt", b"not a save");
    builder.add_file("good.GM8", &save);
    builder.add_file("cut.GM8", &save[..save.len() / 2]);
    assert_eq!(builder.len(), 3);
    let Err(DatasetError::Unreadable(files)) = builder.finish() else {
        panic!("expected Unreadable")
    };
    let names: Vec<&str> = files.iter().map(|f| f.file.as_str()).collect();
    assert_eq!(names, ["cut.GM8", "notes.txt"]);
    assert!(files.iter().all(|f| !f.reason.is_empty()));
}

#[test]
fn saves_from_different_games_are_an_error_listing_each_game() {
    let result = dataset_from("towns", &["[hotseat] 112.GM2", "[hotseat] 111.GM2"])
        .and_then(|_| dataset_from("hero_player", &["[hotseat] 111.02.GM8", "[hotseat] 111.01.GM8"]));
    let Err(DatasetError::MixedGames(games)) = result else {
        panic!("expected MixedGames, got {result:?}")
    };
    assert_eq!(
        games,
        [
            GameFiles {
                map_name: "SaveGameTester_Hero_Players_01".into(),
                map_size: "M".into(),
                underground: true,
                files: vec!["[hotseat] 111.01.GM8".into()],
            },
            GameFiles {
                map_name: "SaveGameTester_Hero_Players_02".into(),
                map_size: "M".into(),
                underground: true,
                files: vec!["[hotseat] 111.02.GM8".into()],
            },
        ]
    );
}

#[test]
fn mixed_games_are_reported_before_duplicate_days() {
    // Two maps, and both saves from day 1 and two from day 2 of one of them.
    let read = |folder: &str, file: &str| std::fs::read(test_data(folder).join(file)).unwrap();
    let mut builder = Builder::new();
    builder.add_file("towns 1.GM2", &read("towns", "[hotseat] 111.GM2"));
    builder.add_file("towns 2.GM2", &read("towns", "[hotseat] 112.GM2"));
    builder.add_file("towns 2 again.GM2", &read("towns", "[hotseat] 112.GM2"));
    builder.add_file("experience 1.GM2", &read("experience", "[hotseat] 111.GM2"));
    let Err(DatasetError::MixedGames(games)) = builder.finish() else {
        panic!("expected MixedGames")
    };
    let files: Vec<Vec<String>> = games.into_iter().map(|g| g.files).collect();
    assert_eq!(
        files,
        [
            vec!["experience 1.GM2"],
            vec!["towns 1.GM2", "towns 2 again.GM2", "towns 2.GM2"]
        ]
    );
}

#[test]
fn two_saves_from_one_day_are_an_error_listing_the_files() {
    // The movement folder has an autosave and a manual save from day 1.
    let result = dataset_from("movement", &["[hotseat] 112.GM2", "start.GM2", "[hotseat] 111.GM2"]);
    assert_eq!(
        result,
        Err(DatasetError::DuplicateDays(vec![DayFiles {
            date: "Month 1, Week 1, Day 1".into(),
            files: vec!["[hotseat] 111.GM2".into(), "start.GM2".into()],
        }]))
    );
}

#[test]
fn every_duplicated_day_is_listed_in_date_order() {
    let read = |file: &str| std::fs::read(test_data("experience").join(file)).unwrap();
    let mut builder = Builder::new();
    for (name, file) in [
        ("d3 b", "[hotseat] 113.GM2"),
        ("d1", "[hotseat] 111.GM2"),
        ("d3 a", "[hotseat] 113.GM2"),
        ("d2", "[hotseat] 112.GM2"),
        ("w2 b", "[hotseat] 121.GM2"),
        ("w2 a", "[hotseat] 121.GM2"),
        ("w2 c", "[hotseat] 121.GM2"),
    ] {
        builder.add_file(name, &read(file));
    }
    let Err(error) = builder.finish() else {
        panic!("expected an error")
    };
    let DatasetError::DuplicateDays(days) = &error else {
        panic!("expected DuplicateDays, got {error:?}")
    };
    let found: Vec<(&str, Vec<&str>)> = days
        .iter()
        .map(|d| (d.date.as_str(), d.files.iter().map(String::as_str).collect()))
        .collect();
    assert_eq!(
        found,
        [
            ("Month 1, Week 1, Day 3", vec!["d3 a", "d3 b"]),
            ("Month 1, Week 2, Day 1", vec!["w2 a", "w2 b", "w2 c"])
        ]
    );
    // The message names every file, so the user knows what to remove.
    let message = error.to_string();
    for file in ["d3 a", "d3 b", "w2 a", "w2 b", "w2 c"] {
        assert!(message.contains(file), "{message}");
    }
    assert!(!message.contains("d1") && !message.contains("d2"), "{message}");
}

#[test]
fn error_messages_name_the_files() {
    let mixed = DatasetError::MixedGames(vec![
        GameFiles {
            map_name: "A".into(),
            map_size: "S".into(),
            underground: true,
            files: vec!["a1".into(), "a2".into()],
        },
        GameFiles {
            map_name: "B".into(),
            map_size: "M".into(),
            underground: false,
            files: vec!["b1".into()],
        },
    ]);
    assert_eq!(
        mixed.to_string(),
        "the saves come from 2 different games; select the saves of one game:\n\
         - map \"A\" (S with underground): a1, a2\n\
         - map \"B\" (M): b1"
    );
    let unreadable = DatasetError::Unreadable(vec![h3dataset::UnreadableFile {
        file: "x".into(),
        reason: "bad".into(),
    }]);
    assert_eq!(
        unreadable.to_string(),
        "these files could not be read as saves:\n- x: bad"
    );
}

#[test]
fn json_has_the_shape_the_frontend_expects() {
    let dataset = dataset_from("resources", &["[hotseat] 112.GM8", "[hotseat] 113.GM5"]).unwrap();
    let json = serde_json::to_value(&dataset).unwrap();
    assert_eq!(json["missingDays"], json!([]));
    assert_eq!(json["missingDaysWarning"], Value::Null);
    let keys = |v: &Value| v.as_object().unwrap().keys().cloned().collect::<BTreeSet<_>>();
    let set = |names: &[&str]| names.iter().map(|s| (*s).to_string()).collect::<BTreeSet<_>>();
    assert_eq!(
        keys(&json),
        set(&[
            "map",
            "days",
            "missingDays",
            "missingDaysWarning",
            "playerStats",
            "heroStats",
            "players",
            "heroes"
        ])
    );
    assert_eq!(
        keys(&json["map"]),
        set(&["name", "description", "size", "tiles", "underground"])
    );
    assert_eq!(
        keys(&json["days"][0]),
        set(&["day", "month", "week", "dayOfWeek", "label", "file"])
    );
    assert_eq!(keys(&json["playerStats"][0]), set(&["id", "name", "group"]));
    assert_eq!(
        keys(&json["players"][0]),
        set(&["id", "name", "color", "eliminatedDay", "values"])
    );
    assert_eq!(keys(&json["heroes"][0]), set(&["id", "name", "owner", "values"]));
    assert_eq!(json["map"]["size"], "S");
    assert_eq!(
        json["days"][1],
        json!({"day": 3, "month": 1, "week": 1, "dayOfWeek": 3, "label": "1-1-3", "file": "[hotseat] 113.GM5"})
    );
    let blue = json["players"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == "blue")
        .unwrap();
    assert_eq!(blue["name"], "Blue");
    assert_eq!(blue["color"], "#3050f8");
    assert_eq!(blue["eliminatedDay"], 3);
    assert_eq!(blue["values"]["gold"], json!([6700.0, null]));
    let red = json["players"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == "red")
        .unwrap();
    assert_eq!(red["eliminatedDay"], Value::Null);
}

#[test]
fn errors_serialize_with_a_kind_and_details() {
    let error = DatasetError::DuplicateDays(vec![DayFiles {
        date: "Month 1, Week 1, Day 1".into(),
        files: vec!["a".into(), "b".into()],
    }]);
    assert_eq!(
        serde_json::to_value(&error).unwrap(),
        json!({"kind": "duplicateDays", "details": [{"date": "Month 1, Week 1, Day 1", "files": ["a", "b"]}]})
    );
    assert_eq!(
        serde_json::to_value(DatasetError::NoSaves).unwrap(),
        json!({"kind": "noSaves"})
    );
    let mixed = DatasetError::MixedGames(vec![GameFiles {
        map_name: "A".into(),
        map_size: "S".into(),
        underground: false,
        files: vec![],
    }]);
    assert_eq!(
        serde_json::to_value(&mixed).unwrap(),
        json!({"kind": "mixedGames", "details": [{"mapName": "A", "mapSize": "S", "underground": false, "files": []}]})
    );
}

fn experience_files(days: &[&str]) -> Vec<String> {
    days.iter().map(|d| format!("[hotseat] {d}.GM2")).collect()
}

#[test]
fn missing_days_are_on_the_x_axis_with_no_values() {
    // Days 2, 5 and 6 have no save.
    let files = experience_files(&["111", "113", "114", "117", "121"]);
    let names: Vec<&str> = files.iter().map(String::as_str).collect();
    let dataset = dataset_from("experience", &names).unwrap();
    check_shape(&dataset);
    let days: Vec<(u32, Option<&str>)> = dataset.days.iter().map(|d| (d.day, d.file.as_deref())).collect();
    assert_eq!(
        days,
        [
            (1, Some("[hotseat] 111.GM2")),
            (2, None),
            (3, Some("[hotseat] 113.GM2")),
            (4, Some("[hotseat] 114.GM2")),
            (5, None),
            (6, None),
            (7, Some("[hotseat] 117.GM2")),
            (8, Some("[hotseat] 121.GM2")),
        ]
    );
    let labels: Vec<&str> = dataset.days.iter().map(|d| d.label.as_str()).collect();
    assert_eq!(
        labels,
        ["1-1-1", "1-1-2", "1-1-3", "1-1-4", "1-1-5", "1-1-6", "1-1-7", "1-2-1"]
    );
    // Values from info.txt on the days with a save, nothing on the others.
    let giselle = dataset.heroes.iter().find(|h| h.name == "Giselle").unwrap();
    let level = |v: [Option<u32>; 8]| v.map(|x| x.map(f64::from)).to_vec();
    assert_eq!(
        giselle.values["level"],
        level([Some(2), None, Some(9), Some(10), None, None, Some(16), Some(18)])
    );
    let purple = Some("purple".to_string());
    assert_eq!(
        giselle.owner,
        [
            purple.clone(),
            None,
            purple.clone(),
            purple.clone(),
            None,
            None,
            purple.clone(),
            purple
        ]
    );
    assert_eq!(
        values(&dataset, "teal", "totalExperience"),
        level([
            Some(1800),
            None,
            Some(61_800),
            Some(91_800),
            None,
            None,
            Some(112_800),
            Some(120_720)
        ])
    );
    // A gap is not an elimination.
    assert!(dataset.players.iter().all(|p| p.eliminated_day.is_none()));
}

#[test]
fn missing_days_are_listed_as_runs_with_a_warning() {
    let files = experience_files(&["111", "113", "114", "117", "121"]);
    let names: Vec<&str> = files.iter().map(String::as_str).collect();
    let dataset = dataset_from("experience", &names).unwrap();
    assert_eq!(
        dataset.missing_days,
        [
            DayRange {
                first: 2,
                last: 2,
                label: "Month 1, Week 1, Day 2".into()
            },
            DayRange {
                first: 5,
                last: 6,
                label: "Month 1, Week 1, Day 5 to Month 1, Week 1, Day 6".into()
            },
        ]
    );
    assert_eq!(
        dataset.missing_days_warning.as_deref(),
        Some(
            "There are no saves for 3 days between the first and the last save:\n\
             - Month 1, Week 1, Day 2\n\
             - Month 1, Week 1, Day 5 to Month 1, Week 1, Day 6\n\
             These days are left empty in the charts. Continue?"
        )
    );
    // One missing day, across the end of a week.
    let files = experience_files(&["116", "121"]);
    let names: Vec<&str> = files.iter().map(String::as_str).collect();
    let dataset = dataset_from("experience", &names).unwrap();
    check_shape(&dataset);
    assert_eq!(
        dataset.missing_days,
        [DayRange {
            first: 7,
            last: 7,
            label: "Month 1, Week 1, Day 7".into()
        }]
    );
    assert!(
        dataset
            .missing_days_warning
            .unwrap()
            .starts_with("There are no saves for 1 day between")
    );
}

#[test]
fn consecutive_saves_have_no_missing_days() {
    // Day 7 to day 8 crosses into week 2 without a gap.
    let files = experience_files(&["116", "117", "121"]);
    let names: Vec<&str> = files.iter().map(String::as_str).collect();
    let dataset = dataset_from("experience", &names).unwrap();
    assert_eq!(dataset.missing_days, []);
    assert_eq!(dataset.missing_days_warning, None);
    assert_eq!(dataset.days.len(), 3);
}

#[test]
fn elimination_is_the_first_saved_day_out_of_the_game() {
    // Day 2 is missing; blue, tan and orange are out of the game on day 3.
    let dataset = dataset_from("resources", &["[hotseat] 111.GM8", "[hotseat] 113.GM5"]).unwrap();
    check_shape(&dataset);
    for id in ["blue", "tan", "orange"] {
        assert_eq!(player(&dataset, id).eliminated_day, Some(3), "{id}");
    }
    assert_eq!(values(&dataset, "blue", "gold"), [Some(3400.0), None, None]);
    assert_eq!(values(&dataset, "red", "gold"), [Some(2500.0), None, Some(10_700.0)]);
}

/// A parsed save with its date changed, for spans the test saves do not have.
fn save_on(date: Date) -> h3save::SaveFile {
    let mut save = h3save::SaveFile::read(test_data("experience").join("[hotseat] 111.GM2")).unwrap();
    save.date = date;
    save
}

#[test]
fn a_span_of_at_most_max_days_is_allowed() {
    let mut builder = Builder::new();
    builder.add_save("first", &save_on(Date::FIRST));
    builder.add_save("last", &save_on(Date::from_days_elapsed(MAX_DAYS - 1).unwrap()));
    let dataset = builder.finish().unwrap();
    assert_eq!(dataset.days.len(), usize::try_from(MAX_DAYS).unwrap());
    assert_eq!(
        dataset.missing_days,
        [DayRange {
            first: 2,
            last: MAX_DAYS - 1,
            label: "Month 1, Week 1, Day 2 to Month 120, Week 4, Day 6".into()
        }]
    );
    check_shape(&dataset);
}

#[test]
fn a_longer_span_is_an_error_naming_the_first_and_last_save() {
    let mut builder = Builder::new();
    builder.add_save("last", &save_on(Date::from_days_elapsed(MAX_DAYS).unwrap()));
    builder.add_save("first", &save_on(Date::FIRST));
    let error = builder.finish().unwrap_err();
    assert_eq!(
        error,
        DatasetError::TooManyDays {
            days: MAX_DAYS + 1,
            first: DayFiles {
                date: "Month 1, Week 1, Day 1".into(),
                files: vec!["first".into()]
            },
            last: DayFiles {
                date: "Month 121, Week 1, Day 1".into(),
                files: vec!["last".into()]
            },
        }
    );
    let message = error.to_string();
    assert!(
        message.contains("first") && message.contains("last") && message.contains("3361 days"),
        "{message}"
    );
    assert_eq!(
        serde_json::to_value(&error).unwrap(),
        json!({"kind": "tooManyDays", "details": {
            "days": 3361,
            "first": {"date": "Month 1, Week 1, Day 1", "files": ["first"]},
            "last": {"date": "Month 121, Week 1, Day 1", "files": ["last"]},
        }})
    );
}

#[test]
fn missing_days_serialize_with_null_files() {
    let dataset = dataset_from("resources", &["[hotseat] 111.GM8", "[hotseat] 113.GM5"]).unwrap();
    let json = serde_json::to_value(&dataset).unwrap();
    assert_eq!(json["days"][1]["file"], Value::Null);
    assert_eq!(
        json["missingDays"],
        json!([{"first": 2, "last": 2, "label": "Month 1, Week 1, Day 2"}])
    );
    assert!(
        json["missingDaysWarning"]
            .as_str()
            .unwrap()
            .contains("Month 1, Week 1, Day 2")
    );
}

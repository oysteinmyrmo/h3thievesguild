// Ported from the homm3tools C++ test test-town-names.cpp. Expected values are unchanged.

#![allow(clippy::needless_range_loop)]

mod common;

#[allow(unused_imports)]
use common::{assert_creature, assert_date, data_dir, find_hero, find_town, open};
#[allow(unused_imports)]
use h3save::stats;
#[allow(unused_imports)]
use h3save::{
    Artifact, ArtifactSlot, Color, Creature, Hero, MapSize, Orientation, SaveFile, Skill, SkillLevel, SkillSlot, Spell,
    Town,
};

use h3save::town::TOWN_NAMES;

fn town_exists(save: &SaveFile, town_name: &str) -> bool {
    save.towns.iter().any(|town| town.name == town_name)
}

// All Castle, Rampart and Towner town names should be present once each.
#[test]
fn towns_names_castle_rampart_tower() {
    let save = open(&data_dir("town_names").join("01").join("[hotseat] 111.GM3"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    // Castle
    for i in 0..=15 {
        assert!(town_exists(&save, TOWN_NAMES[i]));
    }

    // Rampart
    for i in 16..=31 {
        assert!(town_exists(&save, TOWN_NAMES[i]));
    }

    // Tower
    for i in 32..=47 {
        assert!(town_exists(&save, TOWN_NAMES[i]));
    }
}

// All Inferno, Necropolis and Dungeon town names should be present once each.
#[test]
fn towns_names_inferno_necro_dungeon() {
    let save = open(&data_dir("town_names").join("02").join("[hotseat] 111.GM3"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    // Inferno
    for i in 48..=63 {
        assert!(town_exists(&save, TOWN_NAMES[i]));
    }

    // Necro
    for i in 64..=79 {
        assert!(town_exists(&save, TOWN_NAMES[i]));
    }

    // Dungeon
    for i in 80..=95 {
        assert!(town_exists(&save, TOWN_NAMES[i]));
    }
}

// All Stronghold and Fortress town names should be present once each.
#[test]
fn towns_names_stronghold_fortress() {
    let save = open(&data_dir("town_names").join("03").join("[hotseat] 111.GM2"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    // Stronghold
    for i in 96..=111 {
        assert!(town_exists(&save, TOWN_NAMES[i]));
    }

    // Fortress
    for i in 112..=127 {
        assert!(town_exists(&save, TOWN_NAMES[i]));
    }
}

// All Conflux and Cove town names should be present once each.
#[test]
fn towns_names_conflux_cove() {
    let save = open(&data_dir("town_names").join("04").join("[hotseat] 111.GM2"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    // Conflux
    for i in 128..=143 {
        assert!(town_exists(&save, TOWN_NAMES[i]));
    }

    // Cove
    for i in 144..=159 {
        assert!(town_exists(&save, TOWN_NAMES[i]));
    }
}

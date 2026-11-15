// Ported from the homm3tools C++ test test-prison.cpp. Expected values are unchanged.

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

#[test]
fn prison_day1() {
    let save = open(&data_dir("prison").join("[hotseat] 111.GM2"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    let hero = find_hero(&save, "Spint");
    assert_eq!(hero.x, 14);
    assert_eq!(hero.y, 13);
    assert_eq!(hero.z, 1);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 1);
    assert!(!hero.prison());
    assert!(!hero.garrison());

    let hero = find_hero(&save, "Solmyr");
    assert_eq!(hero.x, 19);
    assert_eq!(hero.y, 13);
    assert_eq!(hero.z, 1);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 1);
    assert!(!hero.prison());
    assert!(!hero.garrison());

    let hero = find_hero(&save, "Sandro");
    assert_eq!(hero.x, 13);
    assert_eq!(hero.y, 13);
    assert_eq!(hero.z, 1);
    assert_eq!(hero.player, 0xFF);
    assert_eq!(hero.on_map, 0); // In prison = 0, but position is set!
    assert!(hero.prison());
    assert!(!hero.garrison());
}

#[test]
fn prison_day2() {
    let save = open(&data_dir("prison").join("[hotseat] 112.GM2"));
    assert_date(&save, 1, 1, 2);
    assert!(save.valid());

    let hero = find_hero(&save, "Spint");
    assert_eq!(hero.x, 14);
    assert_eq!(hero.y, 13);
    assert_eq!(hero.z, 1);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 1);
    assert!(!hero.prison());
    assert!(!hero.garrison());

    let hero = find_hero(&save, "Solmyr");
    assert_eq!(hero.x, 19);
    assert_eq!(hero.y, 13);
    assert_eq!(hero.z, 1);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 1);
    assert!(!hero.prison());
    assert!(!hero.garrison());

    // Sandro is released, same position, but now player and on_map is changed.
    let hero = find_hero(&save, "Sandro");
    assert_eq!(hero.x, 13);
    assert_eq!(hero.y, 13);
    assert_eq!(hero.z, 1);
    assert_eq!(hero.player, 0); // Belongs to red player now.
    assert_eq!(hero.on_map, 1); // Is now on map.
    assert!(!hero.prison());
    assert!(!hero.garrison());
}

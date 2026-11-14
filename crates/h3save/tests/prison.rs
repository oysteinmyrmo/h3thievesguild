// Ported from the homm3tools C++ test test-prison.cpp. Expected values are unchanged.

#[macro_use]
mod common;

#[allow(unused_imports)]
use common::{data_dir, find_hero, find_town, open, open_with_towns, test_creature};
#[allow(unused_imports)]
use h3save::stats;
#[allow(unused_imports)]
use h3save::{
    Artifact, ArtifactSlot, Color, Creature, Hero, MapSize, Orientation, SaveFile, Skill, SkillLevel, SkillSlot, Spell,
    Town,
};

#[test]
fn prison_day1() {
    let _guard = common::Guard::new("prison-day1");
    let save = open(&data_dir("prison").join("[hotseat] 111.GM2"));
    require!(save.valid());

    let hero = find_hero(&save, "Spint");
    require_eq!(hero.x, 14);
    require_eq!(hero.y, 13);
    require_eq!(hero.z, 1);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 1);
    require!(!hero.prison());
    require!(!hero.garrison());

    let hero = find_hero(&save, "Solmyr");
    require_eq!(hero.x, 19);
    require_eq!(hero.y, 13);
    require_eq!(hero.z, 1);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 1);
    require!(!hero.prison());
    require!(!hero.garrison());

    let hero = find_hero(&save, "Sandro");
    require_eq!(hero.x, 13);
    require_eq!(hero.y, 13);
    require_eq!(hero.z, 1);
    require_eq!(hero.player, 0xFF);
    require_eq!(hero.on_map, 0); // In prison = 0, but position is set!
    require!(hero.prison());
    require!(!hero.garrison());
}

#[test]
fn prison_day2() {
    let _guard = common::Guard::new("prison-day2");
    let save = open(&data_dir("prison").join("[hotseat] 112.GM2"));
    require!(save.valid());

    let hero = find_hero(&save, "Spint");
    require_eq!(hero.x, 14);
    require_eq!(hero.y, 13);
    require_eq!(hero.z, 1);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 1);
    require!(!hero.prison());
    require!(!hero.garrison());

    let hero = find_hero(&save, "Solmyr");
    require_eq!(hero.x, 19);
    require_eq!(hero.y, 13);
    require_eq!(hero.z, 1);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 1);
    require!(!hero.prison());
    require!(!hero.garrison());

    // Sandro is released, same position, but now player and on_map is changed.
    let hero = find_hero(&save, "Sandro");
    require_eq!(hero.x, 13);
    require_eq!(hero.y, 13);
    require_eq!(hero.z, 1);
    require_eq!(hero.player, 0); // Belongs to red player now.
    require_eq!(hero.on_map, 1); // Is now on map.
    require!(!hero.prison());
    require!(!hero.garrison());
}

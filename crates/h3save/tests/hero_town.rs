// Ported from the homm3tools C++ test test-hero-town.cpp. Expected values are unchanged.

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
fn hero_town_day1() {
    let save = open(&data_dir("hero_town").join("[hotseat] 111.GM3"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    let hero = find_hero(&save, "Orrin");
    assert_eq!(hero.x, 9);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Valeska");
    assert_eq!(hero.x, 12);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Edric");
    assert_eq!(hero.x, 15);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Mephala");
    assert_eq!(hero.x, 18);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Ufretin");
    assert_eq!(hero.x, 21);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Jenova");
    assert_eq!(hero.x, 24);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Piquedram");
    assert_eq!(hero.x, 27);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 2);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Thane");
    assert_eq!(hero.x, 30);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 2);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Josephine");
    assert_eq!(hero.x, 33);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 2);
    assert_eq!(hero.on_map, 1);
}

// - Second hero goes to town.
#[test]
fn hero_town_day2() {
    let save = open(&data_dir("hero_town").join("[hotseat] 112.GM3"));
    assert_date(&save, 1, 1, 2);
    assert!(save.valid());

    let hero = find_hero(&save, "Orrin");
    assert_eq!(hero.x, 9);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Valeska");
    assert_eq!(hero.x, 12);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Edric");
    assert_eq!(hero.x, 15);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Mephala");
    assert_eq!(hero.x, 18);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Ufretin");
    assert_eq!(hero.x, 21);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Jenova");
    assert_eq!(hero.x, 24);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Piquedram");
    assert_eq!(hero.x, 27);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 2);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Thane");
    assert_eq!(hero.x, 30);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 2);
    assert_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Josephine");
    assert_eq!(hero.x, 33);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 2);
    assert_eq!(hero.on_map, 1);
}

// - Hero in town swaps to visiting position.
// - First hero goes to town.
#[test]
fn hero_town_day3() {
    let save = open(&data_dir("hero_town").join("[hotseat] 113.GM3"));
    assert_date(&save, 1, 1, 3);
    assert!(save.valid());

    let hero = find_hero(&save, "Orrin");
    assert_eq!(hero.x, 12);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Valeska");
    assert_eq!(hero.x, 12);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 0); // Visiting

    let hero = find_hero(&save, "Edric");
    assert_eq!(hero.x, 15);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Mephala");
    assert_eq!(hero.x, 21);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Ufretin");
    assert_eq!(hero.x, 21);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 0); // Visiting

    let hero = find_hero(&save, "Jenova");
    assert_eq!(hero.x, 24);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Piquedram");
    assert_eq!(hero.x, 30);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 2);
    assert_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Thane");
    assert_eq!(hero.x, 30);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 2);
    assert_eq!(hero.on_map, 0); // Visiting

    let hero = find_hero(&save, "Josephine");
    assert_eq!(hero.x, 33);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 2);
    assert_eq!(hero.on_map, 1);
}

// - Swap visiting hero and hero in town.
#[test]
fn hero_town_day4() {
    let save = open(&data_dir("hero_town").join("[hotseat] 114.GM3"));
    assert_date(&save, 1, 1, 4);
    assert!(save.valid());

    let hero = find_hero(&save, "Orrin");
    assert_eq!(hero.x, 12);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 0); // Visiting

    let hero = find_hero(&save, "Valeska");
    assert_eq!(hero.x, 12);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Edric");
    assert_eq!(hero.x, 15);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 0);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Mephala");
    assert_eq!(hero.x, 21);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 0); // Visiting

    let hero = find_hero(&save, "Ufretin");
    assert_eq!(hero.x, 21);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Jenova");
    assert_eq!(hero.x, 24);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 1);
    assert_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Piquedram");
    assert_eq!(hero.x, 30);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 2);
    assert_eq!(hero.on_map, 0); // Visiting

    let hero = find_hero(&save, "Thane");
    assert_eq!(hero.x, 30);
    assert_eq!(hero.y, 9);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 2);
    assert_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Josephine");
    assert_eq!(hero.x, 33);
    assert_eq!(hero.y, 12);
    assert_eq!(hero.z, 0);
    assert_eq!(hero.player, 2);
    assert_eq!(hero.on_map, 1);
}

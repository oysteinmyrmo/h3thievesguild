// Ported from the homm3tools C++ test test-hero-town.cpp. Expected values are unchanged.

#[macro_use]
mod common;

#[allow(unused_imports)]
use common::{data_dir, find_hero, find_town, open, test_creature};
#[allow(unused_imports)]
use h3save::stats;
#[allow(unused_imports)]
use h3save::{
    Artifact, ArtifactSlot, Color, Creature, Hero, MapSize, Orientation, SaveFile, Skill, SkillLevel, SkillSlot, Spell,
    Town,
};

#[test]
fn hero_town_day1() {
    let _guard = common::Guard::new("hero-town-day1");
    let save = open(&data_dir("hero_town").join("[hotseat] 111.GM3"));
    require!(save.valid());

    let hero = find_hero(&save, "Orrin");
    require_eq!(hero.x, 9);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Valeska");
    require_eq!(hero.x, 12);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Edric");
    require_eq!(hero.x, 15);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Mephala");
    require_eq!(hero.x, 18);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Ufretin");
    require_eq!(hero.x, 21);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Jenova");
    require_eq!(hero.x, 24);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Piquedram");
    require_eq!(hero.x, 27);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 2);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Thane");
    require_eq!(hero.x, 30);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 2);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Josephine");
    require_eq!(hero.x, 33);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 2);
    require_eq!(hero.on_map, 1);
}

// - Second hero goes to town.
#[test]
fn hero_town_day2() {
    let _guard = common::Guard::new("hero-town-day2");
    let save = open(&data_dir("hero_town").join("[hotseat] 112.GM3"));
    require!(save.valid());

    let hero = find_hero(&save, "Orrin");
    require_eq!(hero.x, 9);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Valeska");
    require_eq!(hero.x, 12);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Edric");
    require_eq!(hero.x, 15);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Mephala");
    require_eq!(hero.x, 18);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Ufretin");
    require_eq!(hero.x, 21);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Jenova");
    require_eq!(hero.x, 24);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Piquedram");
    require_eq!(hero.x, 27);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 2);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Thane");
    require_eq!(hero.x, 30);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 2);
    require_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Josephine");
    require_eq!(hero.x, 33);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 2);
    require_eq!(hero.on_map, 1);
}

// - Hero in town swaps to visiting position.
// - First hero goes to town.
#[test]
fn hero_town_day3() {
    let _guard = common::Guard::new("hero-town-day3");
    let save = open(&data_dir("hero_town").join("[hotseat] 113.GM3"));
    require!(save.valid());

    let hero = find_hero(&save, "Orrin");
    require_eq!(hero.x, 12);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Valeska");
    require_eq!(hero.x, 12);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 0); // Visiting

    let hero = find_hero(&save, "Edric");
    require_eq!(hero.x, 15);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Mephala");
    require_eq!(hero.x, 21);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Ufretin");
    require_eq!(hero.x, 21);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 0); // Visiting

    let hero = find_hero(&save, "Jenova");
    require_eq!(hero.x, 24);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Piquedram");
    require_eq!(hero.x, 30);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 2);
    require_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Thane");
    require_eq!(hero.x, 30);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 2);
    require_eq!(hero.on_map, 0); // Visiting

    let hero = find_hero(&save, "Josephine");
    require_eq!(hero.x, 33);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 2);
    require_eq!(hero.on_map, 1);
}

// - Swap visiting hero and hero in town.
#[test]
fn hero_town_day4() {
    let _guard = common::Guard::new("hero-town-day4");
    let save = open(&data_dir("hero_town").join("[hotseat] 114.GM3"));
    require!(save.valid());

    let hero = find_hero(&save, "Orrin");
    require_eq!(hero.x, 12);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 0); // Visiting

    let hero = find_hero(&save, "Valeska");
    require_eq!(hero.x, 12);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Edric");
    require_eq!(hero.x, 15);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 0);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Mephala");
    require_eq!(hero.x, 21);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 0); // Visiting

    let hero = find_hero(&save, "Ufretin");
    require_eq!(hero.x, 21);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Jenova");
    require_eq!(hero.x, 24);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 1);
    require_eq!(hero.on_map, 1);

    let hero = find_hero(&save, "Piquedram");
    require_eq!(hero.x, 30);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 2);
    require_eq!(hero.on_map, 0); // Visiting

    let hero = find_hero(&save, "Thane");
    require_eq!(hero.x, 30);
    require_eq!(hero.y, 9);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 2);
    require_eq!(hero.on_map, 1); // In town

    let hero = find_hero(&save, "Josephine");
    require_eq!(hero.x, 33);
    require_eq!(hero.y, 12);
    require_eq!(hero.z, 0);
    require_eq!(hero.player, 2);
    require_eq!(hero.on_map, 1);
}

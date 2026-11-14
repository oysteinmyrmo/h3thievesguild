// Ported from the homm3tools C++ test test-savefile.cpp. Expected values are unchanged.

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

use h3save::hero::HERO_COUNT;
use h3save::player::players;

#[test]
fn savefile_s() {
    let _guard = common::Guard::new("savefile-S");
    let save = open_with_towns(&data_dir("savefile").join("S").join("[hotseat] 111.GM2"), "RedsTown", 2);

    require!(save.valid());
    require_eq!(usize::try_from(save.hero_count).unwrap(), HERO_COUNT);
    require_eq!(save.map_size, MapSize::S);
    require_eq!(save.has_underground, 0);
    require_eq!(save.map_name_size, 20);
    require_eq!(save.map_name, "SaveGameTester_Map_S");
    require_eq!(save.description_size, 0);
    require_eq!(save.description, "");
    require_eq!(save.towns.len(), 2);

    let players = players(&save);

    let red = &players[0];
    require_eq!(red.color(), Color::Red);
    require_eq!(red.towns().len(), 1);
    require_eq!(red.heroes().len(), 1);

    let town = &red.towns()[0];
    require_eq!(town.name, "RedsTown");
    require_eq!(town.x, 2);
    require_eq!(town.y, 3);
    require_eq!(town.z, 0);

    let hero = &red.heroes()[0];
    require!(hero.name[..5] == *b"Orrin");
    require_eq!(hero.x, 2);
    require_eq!(hero.y, 4);
    require_eq!(hero.z, 0);

    let blue = &players[1];
    require_eq!(blue.color(), Color::Blue);
    require_eq!(blue.towns().len(), 1);
    require_eq!(blue.heroes().len(), 1);

    let town = &blue.towns()[0];
    require_eq!(town.name, "BluesTown");
    require_eq!(town.x, 33);
    require_eq!(town.y, 34);
    require_eq!(town.z, 0);

    let hero = &blue.heroes()[0];
    require!(hero.name[..7] == *b"Mephala");
    require_eq!(hero.x, 33);
    require_eq!(hero.y, 35);
    require_eq!(hero.z, 0);

    require_eq!(players[2].color(), Color::Tan);
    require_eq!(players[3].color(), Color::Green);
    require_eq!(players[4].color(), Color::Orange);
    require_eq!(players[5].color(), Color::Purple);
    require_eq!(players[6].color(), Color::Teal);
    require_eq!(players[7].color(), Color::Pink);

    require_eq!(players[2].alive(), false);
    require_eq!(players[3].alive(), false);
    require_eq!(players[4].alive(), false);
    require_eq!(players[5].alive(), false);
    require_eq!(players[6].alive(), false);
    require_eq!(players[7].alive(), false);
}

#[test]
fn savefile_s_u() {
    let _guard = common::Guard::new("savefile-S_U");
    let save = open_with_towns(
        &data_dir("savefile").join("S_U").join("[hotseat] 111.GM2"),
        "RedsTown",
        4,
    );

    require!(save.valid());
    require_eq!(usize::try_from(save.hero_count).unwrap(), HERO_COUNT);
    require_eq!(save.map_size, MapSize::S);
    require_eq!(save.has_underground, 1);
    require_eq!(save.map_name_size, 22);
    require_eq!(save.map_name, "SaveGameTester_Map_S_U");
    require_eq!(save.description_size, 0);
    require_eq!(save.description, "");
    require_eq!(save.towns.len(), 4);

    let players = players(&save);

    let red = &players[0];
    require_eq!(red.color(), Color::Red);
    require_eq!(red.towns().len(), 2);
    require_eq!(red.heroes().len(), 2);

    let town = &red.towns()[0];
    require_eq!(town.name, "RedsTown");
    require_eq!(town.x, 2);
    require_eq!(town.y, 3);
    require_eq!(town.z, 0);

    let town = &red.towns()[1];
    require_eq!(town.name, "RedsTown_U");
    require_eq!(town.x, 33);
    require_eq!(town.y, 3);
    require_eq!(town.z, 1);

    let hero = &red.heroes()[0];
    require!(hero.name[..5] == *b"Orrin");
    require_eq!(hero.x, 2);
    require_eq!(hero.y, 4);
    require_eq!(hero.z, 0);

    let hero = &red.heroes()[1];
    require!(hero.name[..4] == *b"Rion");
    require_eq!(hero.x, 33);
    require_eq!(hero.y, 4);
    require_eq!(hero.z, 1);

    let blue = &players[1];
    require_eq!(blue.color(), Color::Blue);
    require_eq!(blue.towns().len(), 2);
    require_eq!(blue.heroes().len(), 2);

    let town = &blue.towns()[0];
    require_eq!(town.name, "BluesTown");
    require_eq!(town.x, 33);
    require_eq!(town.y, 34);
    require_eq!(town.z, 0);

    let town = &blue.towns()[1];
    require_eq!(town.name, "BluesTown_U");
    require_eq!(town.x, 2);
    require_eq!(town.y, 34);
    require_eq!(town.z, 1);

    let hero = &blue.heroes()[0];
    require!(hero.name[..7] == *b"Mephala");
    require_eq!(hero.x, 33);
    require_eq!(hero.y, 35);
    require_eq!(hero.z, 0);

    let hero = &blue.heroes()[1];
    require!(hero.name[..8] == *b"Coronius");
    require_eq!(hero.x, 2);
    require_eq!(hero.y, 35);
    require_eq!(hero.z, 1);

    require_eq!(players[2].color(), Color::Tan);
    require_eq!(players[3].color(), Color::Green);
    require_eq!(players[4].color(), Color::Orange);
    require_eq!(players[5].color(), Color::Purple);
    require_eq!(players[6].color(), Color::Teal);
    require_eq!(players[7].color(), Color::Pink);

    require_eq!(players[2].alive(), false);
    require_eq!(players[3].alive(), false);
    require_eq!(players[4].alive(), false);
    require_eq!(players[5].alive(), false);
    require_eq!(players[6].alive(), false);
    require_eq!(players[7].alive(), false);
}

#[test]
fn savefile_m() {
    let _guard = common::Guard::new("savefile-M");
    let save = open_with_towns(&data_dir("savefile").join("M").join("[hotseat] 111.GM2"), "RedsTown", 2);

    require!(save.valid());
    require_eq!(usize::try_from(save.hero_count).unwrap(), HERO_COUNT);
    require_eq!(save.map_size, MapSize::M);
    require_eq!(save.has_underground, 0);
    require_eq!(save.map_name_size, 20);
    require_eq!(save.map_name, "SaveGameTester_Map_M");
    require_eq!(save.description_size, 0);
    require_eq!(save.description, "");
    require_eq!(save.towns.len(), 2);

    let players = players(&save);

    let red = &players[0];
    require_eq!(red.color(), Color::Red);
    require_eq!(red.towns().len(), 1);
    require_eq!(red.heroes().len(), 1);

    let town = &red.towns()[0];
    require_eq!(town.name, "RedsTown");
    require_eq!(town.x, 2);
    require_eq!(town.y, 3);
    require_eq!(town.z, 0);

    let hero = &red.heroes()[0];
    require!(hero.name[..5] == *b"Orrin");
    require_eq!(hero.x, 2);
    require_eq!(hero.y, 4);
    require_eq!(hero.z, 0);

    let blue = &players[1];
    require_eq!(blue.color(), Color::Blue);
    require_eq!(blue.towns().len(), 1);
    require_eq!(blue.heroes().len(), 1);

    let town = &blue.towns()[0];
    require_eq!(town.name, "Bath'iere"); // Forgot to change to "BluesTown".
    require_eq!(town.x, 69);
    require_eq!(town.y, 70);
    require_eq!(town.z, 0);

    let hero = &blue.heroes()[0];
    require!(hero.name[..7] == *b"Mephala");
    require_eq!(hero.x, 69);
    require_eq!(hero.y, 71);
    require_eq!(hero.z, 0);

    require_eq!(players[2].color(), Color::Tan);
    require_eq!(players[3].color(), Color::Green);
    require_eq!(players[4].color(), Color::Orange);
    require_eq!(players[5].color(), Color::Purple);
    require_eq!(players[6].color(), Color::Teal);
    require_eq!(players[7].color(), Color::Pink);

    require_eq!(players[2].alive(), false);
    require_eq!(players[3].alive(), false);
    require_eq!(players[4].alive(), false);
    require_eq!(players[5].alive(), false);
    require_eq!(players[6].alive(), false);
    require_eq!(players[7].alive(), false);
}

#[test]
fn savefile_m_u() {
    let _guard = common::Guard::new("savefile-M_U");
    let save = open_with_towns(
        &data_dir("savefile").join("M_U").join("[hotseat] 111.GM2"),
        "RedsTown",
        4,
    );

    require!(save.valid());
    require_eq!(usize::try_from(save.hero_count).unwrap(), HERO_COUNT);
    require_eq!(save.map_size, MapSize::M);
    require_eq!(save.has_underground, 1);
    require_eq!(save.map_name_size, 22);
    require_eq!(save.map_name, "SaveGameTester_Map_M_U");
    require_eq!(save.description_size, 0);
    require_eq!(save.description, "");
    require_eq!(save.towns.len(), 4);

    let players = players(&save);

    let red = &players[0];
    require_eq!(red.color(), Color::Red);
    require_eq!(red.towns().len(), 2);
    require_eq!(red.heroes().len(), 2);

    let town = &red.towns()[0];
    require_eq!(town.name, "RedsTown");
    require_eq!(town.x, 2);
    require_eq!(town.y, 3);
    require_eq!(town.z, 0);

    let town = &red.towns()[1];
    require_eq!(town.name, "RedsTown_U");
    require_eq!(town.x, 69);
    require_eq!(town.y, 3);
    require_eq!(town.z, 1);

    let hero = &red.heroes()[0];
    require!(hero.name[..5] == *b"Orrin");
    require_eq!(hero.x, 2);
    require_eq!(hero.y, 4);
    require_eq!(hero.z, 0);

    let hero = &red.heroes()[1];
    require!(hero.name[..4] == *b"Rion");
    require_eq!(hero.x, 69);
    require_eq!(hero.y, 4);
    require_eq!(hero.z, 1);

    let blue = &players[1];
    require_eq!(blue.color(), Color::Blue);
    require_eq!(blue.towns().len(), 2);
    require_eq!(blue.heroes().len(), 2);

    let town = &blue.towns()[0];
    require_eq!(town.name, "Emerald Moor"); // Forgot to change to "BluesTown"
    require_eq!(town.x, 69);
    require_eq!(town.y, 70);
    require_eq!(town.z, 0);

    let town = &blue.towns()[1];
    require_eq!(town.name, "BluesTown_U");
    require_eq!(town.x, 2);
    require_eq!(town.y, 70);
    require_eq!(town.z, 1);

    let hero = &blue.heroes()[0];
    require!(hero.name[..7] == *b"Mephala");
    require_eq!(hero.x, 69);
    require_eq!(hero.y, 71);
    require_eq!(hero.z, 0);

    let hero = &blue.heroes()[1];
    require!(hero.name[..8] == *b"Coronius");
    require_eq!(hero.x, 2);
    require_eq!(hero.y, 71);
    require_eq!(hero.z, 1);

    require_eq!(players[2].color(), Color::Tan);
    require_eq!(players[3].color(), Color::Green);
    require_eq!(players[4].color(), Color::Orange);
    require_eq!(players[5].color(), Color::Purple);
    require_eq!(players[6].color(), Color::Teal);
    require_eq!(players[7].color(), Color::Pink);

    require_eq!(players[2].alive(), false);
    require_eq!(players[3].alive(), false);
    require_eq!(players[4].alive(), false);
    require_eq!(players[5].alive(), false);
    require_eq!(players[6].alive(), false);
    require_eq!(players[7].alive(), false);
}

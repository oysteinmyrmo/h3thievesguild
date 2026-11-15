// Ported from the homm3tools C++ test test-savefile.cpp. Expected values are unchanged.

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

use h3save::hero::HERO_COUNT;
use h3save::player::players;

#[test]
fn savefile_s() {
    let save = open(&data_dir("savefile").join("S").join("[hotseat] 111.GM2"));
    assert_date(&save, 1, 1, 1);

    assert!(save.valid());
    assert_eq!(usize::try_from(save.hero_count).unwrap(), HERO_COUNT);
    assert_eq!(save.map_size, MapSize::S);
    assert_eq!(save.has_underground, 0);
    assert_eq!(save.map_name_size, 20);
    assert_eq!(save.map_name, "SaveGameTester_Map_S");
    assert_eq!(save.description_size, 0);
    assert_eq!(save.description, "");
    assert_eq!(save.towns.len(), 2);

    let players = players(&save);

    let red = &players[0];
    assert_eq!(red.color(), Color::Red);
    assert_eq!(red.towns().len(), 1);
    assert_eq!(red.heroes().len(), 1);

    let town = &red.towns()[0];
    assert_eq!(town.name, "RedsTown");
    assert_eq!(town.x, 2);
    assert_eq!(town.y, 3);
    assert_eq!(town.z, 0);

    let hero = &red.heroes()[0];
    assert!(hero.name[..5] == *b"Orrin");
    assert_eq!(hero.x, 2);
    assert_eq!(hero.y, 4);
    assert_eq!(hero.z, 0);

    let blue = &players[1];
    assert_eq!(blue.color(), Color::Blue);
    assert_eq!(blue.towns().len(), 1);
    assert_eq!(blue.heroes().len(), 1);

    let town = &blue.towns()[0];
    assert_eq!(town.name, "BluesTown");
    assert_eq!(town.x, 33);
    assert_eq!(town.y, 34);
    assert_eq!(town.z, 0);

    let hero = &blue.heroes()[0];
    assert!(hero.name[..7] == *b"Mephala");
    assert_eq!(hero.x, 33);
    assert_eq!(hero.y, 35);
    assert_eq!(hero.z, 0);

    assert_eq!(players[2].color(), Color::Tan);
    assert_eq!(players[3].color(), Color::Green);
    assert_eq!(players[4].color(), Color::Orange);
    assert_eq!(players[5].color(), Color::Purple);
    assert_eq!(players[6].color(), Color::Teal);
    assert_eq!(players[7].color(), Color::Pink);

    assert!(!players[2].alive());
    assert!(!players[3].alive());
    assert!(!players[4].alive());
    assert!(!players[5].alive());
    assert!(!players[6].alive());
    assert!(!players[7].alive());
}

#[test]
fn savefile_s_u() {
    let save = open(&data_dir("savefile").join("S_U").join("[hotseat] 111.GM2"));
    assert_date(&save, 1, 1, 1);

    assert!(save.valid());
    assert_eq!(usize::try_from(save.hero_count).unwrap(), HERO_COUNT);
    assert_eq!(save.map_size, MapSize::S);
    assert_eq!(save.has_underground, 1);
    assert_eq!(save.map_name_size, 22);
    assert_eq!(save.map_name, "SaveGameTester_Map_S_U");
    assert_eq!(save.description_size, 0);
    assert_eq!(save.description, "");
    assert_eq!(save.towns.len(), 4);

    let players = players(&save);

    let red = &players[0];
    assert_eq!(red.color(), Color::Red);
    assert_eq!(red.towns().len(), 2);
    assert_eq!(red.heroes().len(), 2);

    let town = &red.towns()[0];
    assert_eq!(town.name, "RedsTown");
    assert_eq!(town.x, 2);
    assert_eq!(town.y, 3);
    assert_eq!(town.z, 0);

    let town = &red.towns()[1];
    assert_eq!(town.name, "RedsTown_U");
    assert_eq!(town.x, 33);
    assert_eq!(town.y, 3);
    assert_eq!(town.z, 1);

    let hero = &red.heroes()[0];
    assert!(hero.name[..5] == *b"Orrin");
    assert_eq!(hero.x, 2);
    assert_eq!(hero.y, 4);
    assert_eq!(hero.z, 0);

    let hero = &red.heroes()[1];
    assert!(hero.name[..4] == *b"Rion");
    assert_eq!(hero.x, 33);
    assert_eq!(hero.y, 4);
    assert_eq!(hero.z, 1);

    let blue = &players[1];
    assert_eq!(blue.color(), Color::Blue);
    assert_eq!(blue.towns().len(), 2);
    assert_eq!(blue.heroes().len(), 2);

    let town = &blue.towns()[0];
    assert_eq!(town.name, "BluesTown");
    assert_eq!(town.x, 33);
    assert_eq!(town.y, 34);
    assert_eq!(town.z, 0);

    let town = &blue.towns()[1];
    assert_eq!(town.name, "BluesTown_U");
    assert_eq!(town.x, 2);
    assert_eq!(town.y, 34);
    assert_eq!(town.z, 1);

    let hero = &blue.heroes()[0];
    assert!(hero.name[..7] == *b"Mephala");
    assert_eq!(hero.x, 33);
    assert_eq!(hero.y, 35);
    assert_eq!(hero.z, 0);

    let hero = &blue.heroes()[1];
    assert!(hero.name[..8] == *b"Coronius");
    assert_eq!(hero.x, 2);
    assert_eq!(hero.y, 35);
    assert_eq!(hero.z, 1);

    assert_eq!(players[2].color(), Color::Tan);
    assert_eq!(players[3].color(), Color::Green);
    assert_eq!(players[4].color(), Color::Orange);
    assert_eq!(players[5].color(), Color::Purple);
    assert_eq!(players[6].color(), Color::Teal);
    assert_eq!(players[7].color(), Color::Pink);

    assert!(!players[2].alive());
    assert!(!players[3].alive());
    assert!(!players[4].alive());
    assert!(!players[5].alive());
    assert!(!players[6].alive());
    assert!(!players[7].alive());
}

#[test]
fn savefile_m() {
    let save = open(&data_dir("savefile").join("M").join("[hotseat] 111.GM2"));
    assert_date(&save, 1, 1, 1);

    assert!(save.valid());
    assert_eq!(usize::try_from(save.hero_count).unwrap(), HERO_COUNT);
    assert_eq!(save.map_size, MapSize::M);
    assert_eq!(save.has_underground, 0);
    assert_eq!(save.map_name_size, 20);
    assert_eq!(save.map_name, "SaveGameTester_Map_M");
    assert_eq!(save.description_size, 0);
    assert_eq!(save.description, "");
    assert_eq!(save.towns.len(), 2);

    let players = players(&save);

    let red = &players[0];
    assert_eq!(red.color(), Color::Red);
    assert_eq!(red.towns().len(), 1);
    assert_eq!(red.heroes().len(), 1);

    let town = &red.towns()[0];
    assert_eq!(town.name, "RedsTown");
    assert_eq!(town.x, 2);
    assert_eq!(town.y, 3);
    assert_eq!(town.z, 0);

    let hero = &red.heroes()[0];
    assert!(hero.name[..5] == *b"Orrin");
    assert_eq!(hero.x, 2);
    assert_eq!(hero.y, 4);
    assert_eq!(hero.z, 0);

    let blue = &players[1];
    assert_eq!(blue.color(), Color::Blue);
    assert_eq!(blue.towns().len(), 1);
    assert_eq!(blue.heroes().len(), 1);

    let town = &blue.towns()[0];
    assert_eq!(town.name, "Bath'iere"); // Forgot to change to "BluesTown".
    assert_eq!(town.x, 69);
    assert_eq!(town.y, 70);
    assert_eq!(town.z, 0);

    let hero = &blue.heroes()[0];
    assert!(hero.name[..7] == *b"Mephala");
    assert_eq!(hero.x, 69);
    assert_eq!(hero.y, 71);
    assert_eq!(hero.z, 0);

    assert_eq!(players[2].color(), Color::Tan);
    assert_eq!(players[3].color(), Color::Green);
    assert_eq!(players[4].color(), Color::Orange);
    assert_eq!(players[5].color(), Color::Purple);
    assert_eq!(players[6].color(), Color::Teal);
    assert_eq!(players[7].color(), Color::Pink);

    assert!(!players[2].alive());
    assert!(!players[3].alive());
    assert!(!players[4].alive());
    assert!(!players[5].alive());
    assert!(!players[6].alive());
    assert!(!players[7].alive());
}

#[test]
fn savefile_m_u() {
    let save = open(&data_dir("savefile").join("M_U").join("[hotseat] 111.GM2"));
    assert_date(&save, 1, 1, 1);

    assert!(save.valid());
    assert_eq!(usize::try_from(save.hero_count).unwrap(), HERO_COUNT);
    assert_eq!(save.map_size, MapSize::M);
    assert_eq!(save.has_underground, 1);
    assert_eq!(save.map_name_size, 22);
    assert_eq!(save.map_name, "SaveGameTester_Map_M_U");
    assert_eq!(save.description_size, 0);
    assert_eq!(save.description, "");
    assert_eq!(save.towns.len(), 4);

    let players = players(&save);

    let red = &players[0];
    assert_eq!(red.color(), Color::Red);
    assert_eq!(red.towns().len(), 2);
    assert_eq!(red.heroes().len(), 2);

    let town = &red.towns()[0];
    assert_eq!(town.name, "RedsTown");
    assert_eq!(town.x, 2);
    assert_eq!(town.y, 3);
    assert_eq!(town.z, 0);

    let town = &red.towns()[1];
    assert_eq!(town.name, "RedsTown_U");
    assert_eq!(town.x, 69);
    assert_eq!(town.y, 3);
    assert_eq!(town.z, 1);

    let hero = &red.heroes()[0];
    assert!(hero.name[..5] == *b"Orrin");
    assert_eq!(hero.x, 2);
    assert_eq!(hero.y, 4);
    assert_eq!(hero.z, 0);

    let hero = &red.heroes()[1];
    assert!(hero.name[..4] == *b"Rion");
    assert_eq!(hero.x, 69);
    assert_eq!(hero.y, 4);
    assert_eq!(hero.z, 1);

    let blue = &players[1];
    assert_eq!(blue.color(), Color::Blue);
    assert_eq!(blue.towns().len(), 2);
    assert_eq!(blue.heroes().len(), 2);

    let town = &blue.towns()[0];
    assert_eq!(town.name, "Emerald Moor"); // Forgot to change to "BluesTown"
    assert_eq!(town.x, 69);
    assert_eq!(town.y, 70);
    assert_eq!(town.z, 0);

    let town = &blue.towns()[1];
    assert_eq!(town.name, "BluesTown_U");
    assert_eq!(town.x, 2);
    assert_eq!(town.y, 70);
    assert_eq!(town.z, 1);

    let hero = &blue.heroes()[0];
    assert!(hero.name[..7] == *b"Mephala");
    assert_eq!(hero.x, 69);
    assert_eq!(hero.y, 71);
    assert_eq!(hero.z, 0);

    let hero = &blue.heroes()[1];
    assert!(hero.name[..8] == *b"Coronius");
    assert_eq!(hero.x, 2);
    assert_eq!(hero.y, 71);
    assert_eq!(hero.z, 1);

    assert_eq!(players[2].color(), Color::Tan);
    assert_eq!(players[3].color(), Color::Green);
    assert_eq!(players[4].color(), Color::Orange);
    assert_eq!(players[5].color(), Color::Purple);
    assert_eq!(players[6].color(), Color::Teal);
    assert_eq!(players[7].color(), Color::Pink);

    assert!(!players[2].alive());
    assert!(!players[3].alive());
    assert!(!players[4].alive());
    assert!(!players[5].alive());
    assert!(!players[6].alive());
    assert!(!players[7].alive());
}

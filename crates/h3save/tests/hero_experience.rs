// Ported from the homm3tools C++ test test-hero-experience.cpp. Expected values are unchanged.

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
fn hero_experience_day1() {
    let save = open(&data_dir("experience").join("[hotseat] 111.GM2"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    assert_eq!(save.towns.len(), 2);
    assert_eq!(save.towns[0].name, "Emerald Moor");
    assert_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    assert_eq!(hero.level, 2);
    assert_eq!(hero.experience, 1500);
    assert_eq!(hero.attack, 1);
    assert_eq!(hero.defense, 3);
    assert_eq!(hero.power, 2);
    assert_eq!(hero.knowledge, 1);

    let hero = find_hero(&save, "Thant");
    assert_eq!(hero.level, 2);
    assert_eq!(hero.experience, 1800);
    assert_eq!(hero.attack, 1);
    assert_eq!(hero.defense, 0);
    assert_eq!(hero.power, 2);
    assert_eq!(hero.knowledge, 3);
}

#[test]
fn hero_experience_day2() {
    let save = open(&data_dir("experience").join("[hotseat] 112.GM2"));
    assert_date(&save, 1, 1, 2);
    assert!(save.valid());

    assert_eq!(save.towns.len(), 2);
    assert_eq!(save.towns[0].name, "Emerald Moor");
    assert_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    assert_eq!(hero.level, 7);
    assert_eq!(hero.experience, 9000);
    assert_eq!(hero.attack, 4);
    assert_eq!(hero.defense, 4);
    assert_eq!(hero.power, 2);
    assert_eq!(hero.knowledge, 2);

    let hero = find_hero(&save, "Thant");
    assert_eq!(hero.level, 12);
    assert_eq!(hero.experience, 21800);
    assert_eq!(hero.attack, 2);
    assert_eq!(hero.defense, 2);
    assert_eq!(hero.power, 5);
    assert_eq!(hero.knowledge, 7);
}

#[test]
fn hero_experience_day3() {
    let save = open(&data_dir("experience").join("[hotseat] 113.GM2"));
    assert_date(&save, 1, 1, 3);
    assert!(save.valid());

    assert_eq!(save.towns.len(), 2);
    assert_eq!(save.towns[0].name, "Emerald Moor");
    assert_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    assert_eq!(hero.level, 9);
    assert_eq!(hero.experience, 13000);
    assert_eq!(hero.attack, 4);
    assert_eq!(hero.defense, 5);
    assert_eq!(hero.power, 3);
    assert_eq!(hero.knowledge, 2);

    let hero = find_hero(&save, "Thant");
    assert_eq!(hero.level, 18);
    assert_eq!(hero.experience, 61800);
    assert_eq!(hero.attack, 3);
    assert_eq!(hero.defense, 5);
    assert_eq!(hero.power, 6);
    assert_eq!(hero.knowledge, 8);
}

#[test]
fn hero_experience_day4() {
    let save = open(&data_dir("experience").join("[hotseat] 114.GM2"));
    assert_date(&save, 1, 1, 4);
    assert!(save.valid());

    assert_eq!(save.towns.len(), 2);
    assert_eq!(save.towns[0].name, "Emerald Moor");
    assert_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    assert_eq!(hero.level, 10);
    assert_eq!(hero.experience, 17000);
    assert_eq!(hero.attack, 4);
    assert_eq!(hero.defense, 5);
    assert_eq!(hero.power, 4);
    assert_eq!(hero.knowledge, 2);

    let hero = find_hero(&save, "Thant");
    assert_eq!(hero.level, 20);
    assert_eq!(hero.experience, 91800);
    assert_eq!(hero.attack, 4);
    assert_eq!(hero.defense, 5);
    assert_eq!(hero.power, 7);
    assert_eq!(hero.knowledge, 8);
}

#[test]
fn hero_experience_day5() {
    let save = open(&data_dir("experience").join("[hotseat] 115.GM2"));
    assert_date(&save, 1, 1, 5);
    assert!(save.valid());

    assert_eq!(save.towns.len(), 2);
    assert_eq!(save.towns[0].name, "Emerald Moor");
    assert_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    assert_eq!(hero.level, 13);
    assert_eq!(hero.experience, 26000);
    assert_eq!(hero.attack, 5);
    assert_eq!(hero.defense, 6);
    assert_eq!(hero.power, 5);
    assert_eq!(hero.knowledge, 2);

    let hero = find_hero(&save, "Thant");
    assert_eq!(hero.level, 21);
    assert_eq!(hero.experience, 101_800);
    assert_eq!(hero.attack, 5);
    assert_eq!(hero.defense, 5);
    assert_eq!(hero.power, 7);
    assert_eq!(hero.knowledge, 8);
}

#[test]
fn hero_experience_day6() {
    let save = open(&data_dir("experience").join("[hotseat] 116.GM2"));
    assert_date(&save, 1, 1, 6);
    assert!(save.valid());

    assert_eq!(save.towns.len(), 2);
    assert_eq!(save.towns[0].name, "Emerald Moor");
    assert_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    assert_eq!(hero.level, 14);
    assert_eq!(hero.experience, 29750);
    assert_eq!(hero.attack, 5);
    assert_eq!(hero.defense, 6);
    assert_eq!(hero.power, 5);
    assert_eq!(hero.knowledge, 3);

    let hero = find_hero(&save, "Thant");
    assert_eq!(hero.level, 21);
    assert_eq!(hero.experience, 107_800);
    assert_eq!(hero.attack, 5);
    assert_eq!(hero.defense, 5);
    assert_eq!(hero.power, 7);
    assert_eq!(hero.knowledge, 8);
}

#[test]
fn hero_experience_day7() {
    let save = open(&data_dir("experience").join("[hotseat] 117.GM2"));
    assert_date(&save, 1, 1, 7);
    assert!(save.valid());

    assert_eq!(save.towns.len(), 2);
    assert_eq!(save.towns[0].name, "Emerald Moor");
    assert_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    assert_eq!(hero.level, 16);
    assert_eq!(hero.experience, 41750);
    assert_eq!(hero.attack, 6);
    assert_eq!(hero.defense, 7);
    assert_eq!(hero.power, 5);
    assert_eq!(hero.knowledge, 3);

    let hero = find_hero(&save, "Thant");
    assert_eq!(hero.level, 21);
    assert_eq!(hero.experience, 112_800);
    assert_eq!(hero.attack, 5);
    assert_eq!(hero.defense, 5);
    assert_eq!(hero.power, 7);
    assert_eq!(hero.knowledge, 8);
}

#[test]
fn hero_experience_day8() {
    let save = open(&data_dir("experience").join("[hotseat] 121.GM2"));
    assert_date(&save, 1, 2, 1);
    assert!(save.valid());

    assert_eq!(save.towns.len(), 2);
    assert_eq!(save.towns[0].name, "Emerald Moor");
    assert_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    assert_eq!(hero.level, 18);
    assert_eq!(hero.experience, 62375);
    assert_eq!(hero.attack, 7);
    assert_eq!(hero.defense, 7);
    assert_eq!(hero.power, 6);
    assert_eq!(hero.knowledge, 3);

    let hero = find_hero(&save, "Thant");
    assert_eq!(hero.level, 22);
    assert_eq!(hero.experience, 120_720);
    assert_eq!(hero.attack, 5);
    assert_eq!(hero.defense, 5);
    assert_eq!(hero.power, 7);
    assert_eq!(hero.knowledge, 9);
}

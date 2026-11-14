// Ported from the homm3tools C++ test test-hero-experience.cpp. Expected values are unchanged.

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
fn hero_experience_day1() {
    let _guard = common::Guard::new("hero-experience-day1");
    let save = open(data_dir("experience").join("[hotseat] 111.GM2"));
    require!(save.valid());

    require_eq!(save.towns.len(), 2);
    require_eq!(save.towns[0].name, "Emerald Moor");
    require_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    require_eq!(hero.level, 2);
    require_eq!(hero.experience, 1500);
    require_eq!(hero.attack, 1);
    require_eq!(hero.defense, 3);
    require_eq!(hero.power, 2);
    require_eq!(hero.knowledge, 1);

    let hero = find_hero(&save, "Thant");
    require_eq!(hero.level, 2);
    require_eq!(hero.experience, 1800);
    require_eq!(hero.attack, 1);
    require_eq!(hero.defense, 0);
    require_eq!(hero.power, 2);
    require_eq!(hero.knowledge, 3);
}

#[test]
fn hero_experience_day2() {
    let _guard = common::Guard::new("hero-experience-day2");
    let save = open(data_dir("experience").join("[hotseat] 112.GM2"));
    require!(save.valid());

    require_eq!(save.towns.len(), 2);
    require_eq!(save.towns[0].name, "Emerald Moor");
    require_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    require_eq!(hero.level, 7);
    require_eq!(hero.experience, 9000);
    require_eq!(hero.attack, 4);
    require_eq!(hero.defense, 4);
    require_eq!(hero.power, 2);
    require_eq!(hero.knowledge, 2);

    let hero = find_hero(&save, "Thant");
    require_eq!(hero.level, 12);
    require_eq!(hero.experience, 21800);
    require_eq!(hero.attack, 2);
    require_eq!(hero.defense, 2);
    require_eq!(hero.power, 5);
    require_eq!(hero.knowledge, 7);
}

#[test]
fn hero_experience_day3() {
    let _guard = common::Guard::new("hero-experience-day3");
    let save = open(data_dir("experience").join("[hotseat] 113.GM2"));
    require!(save.valid());

    require_eq!(save.towns.len(), 2);
    require_eq!(save.towns[0].name, "Emerald Moor");
    require_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    require_eq!(hero.level, 9);
    require_eq!(hero.experience, 13000);
    require_eq!(hero.attack, 4);
    require_eq!(hero.defense, 5);
    require_eq!(hero.power, 3);
    require_eq!(hero.knowledge, 2);

    let hero = find_hero(&save, "Thant");
    require_eq!(hero.level, 18);
    require_eq!(hero.experience, 61800);
    require_eq!(hero.attack, 3);
    require_eq!(hero.defense, 5);
    require_eq!(hero.power, 6);
    require_eq!(hero.knowledge, 8);
}

#[test]
fn hero_experience_day4() {
    let _guard = common::Guard::new("hero-experience-day4");
    let save = open(data_dir("experience").join("[hotseat] 114.GM2"));
    require!(save.valid());

    require_eq!(save.towns.len(), 2);
    require_eq!(save.towns[0].name, "Emerald Moor");
    require_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    require_eq!(hero.level, 10);
    require_eq!(hero.experience, 17000);
    require_eq!(hero.attack, 4);
    require_eq!(hero.defense, 5);
    require_eq!(hero.power, 4);
    require_eq!(hero.knowledge, 2);

    let hero = find_hero(&save, "Thant");
    require_eq!(hero.level, 20);
    require_eq!(hero.experience, 91800);
    require_eq!(hero.attack, 4);
    require_eq!(hero.defense, 5);
    require_eq!(hero.power, 7);
    require_eq!(hero.knowledge, 8);
}

#[test]
fn hero_experience_day5() {
    let _guard = common::Guard::new("hero-experience-day5");
    let save = open(data_dir("experience").join("[hotseat] 115.GM2"));
    require!(save.valid());

    require_eq!(save.towns.len(), 2);
    require_eq!(save.towns[0].name, "Emerald Moor");
    require_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    require_eq!(hero.level, 13);
    require_eq!(hero.experience, 26000);
    require_eq!(hero.attack, 5);
    require_eq!(hero.defense, 6);
    require_eq!(hero.power, 5);
    require_eq!(hero.knowledge, 2);

    let hero = find_hero(&save, "Thant");
    require_eq!(hero.level, 21);
    require_eq!(hero.experience, 101800);
    require_eq!(hero.attack, 5);
    require_eq!(hero.defense, 5);
    require_eq!(hero.power, 7);
    require_eq!(hero.knowledge, 8);
}

#[test]
fn hero_experience_day6() {
    let _guard = common::Guard::new("hero-experience-day6");
    let save = open(data_dir("experience").join("[hotseat] 116.GM2"));
    require!(save.valid());

    require_eq!(save.towns.len(), 2);
    require_eq!(save.towns[0].name, "Emerald Moor");
    require_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    require_eq!(hero.level, 14);
    require_eq!(hero.experience, 29750);
    require_eq!(hero.attack, 5);
    require_eq!(hero.defense, 6);
    require_eq!(hero.power, 5);
    require_eq!(hero.knowledge, 3);

    let hero = find_hero(&save, "Thant");
    require_eq!(hero.level, 21);
    require_eq!(hero.experience, 107800);
    require_eq!(hero.attack, 5);
    require_eq!(hero.defense, 5);
    require_eq!(hero.power, 7);
    require_eq!(hero.knowledge, 8);
}

#[test]
fn hero_experience_day7() {
    let _guard = common::Guard::new("hero-experience-day7");
    let save = open(data_dir("experience").join("[hotseat] 117.GM2"));
    require!(save.valid());

    require_eq!(save.towns.len(), 2);
    require_eq!(save.towns[0].name, "Emerald Moor");
    require_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    require_eq!(hero.level, 16);
    require_eq!(hero.experience, 41750);
    require_eq!(hero.attack, 6);
    require_eq!(hero.defense, 7);
    require_eq!(hero.power, 5);
    require_eq!(hero.knowledge, 3);

    let hero = find_hero(&save, "Thant");
    require_eq!(hero.level, 21);
    require_eq!(hero.experience, 112800);
    require_eq!(hero.attack, 5);
    require_eq!(hero.defense, 5);
    require_eq!(hero.power, 7);
    require_eq!(hero.knowledge, 8);
}

#[test]
fn hero_experience_day8() {
    let _guard = common::Guard::new("hero-experience-day8");
    let save = open(data_dir("experience").join("[hotseat] 121.GM2"));
    require!(save.valid());

    require_eq!(save.towns.len(), 2);
    require_eq!(save.towns[0].name, "Emerald Moor");
    require_eq!(save.towns[1].name, "Cessacioun");

    let hero = find_hero(&save, "Giselle");
    require_eq!(hero.level, 18);
    require_eq!(hero.experience, 62375);
    require_eq!(hero.attack, 7);
    require_eq!(hero.defense, 7);
    require_eq!(hero.power, 6);
    require_eq!(hero.knowledge, 3);

    let hero = find_hero(&save, "Thant");
    require_eq!(hero.level, 22);
    require_eq!(hero.experience, 120720);
    require_eq!(hero.attack, 5);
    require_eq!(hero.defense, 5);
    require_eq!(hero.power, 7);
    require_eq!(hero.knowledge, 9);
}

// Ported from the homm3tools C++ test test-primary-skills.cpp. Expected values are unchanged.

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
fn primary_skills_day1() {
    let save = open(&data_dir("primary_skills").join("[hotseat] 111.GM2"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    let hero = find_hero(&save, "Clavius");
    assert_eq!(hero.attack, 1);
    assert_eq!(hero.defense, 2);
    assert_eq!(hero.power, 3);
    assert_eq!(hero.knowledge, 4);
    assert_eq!(hero.attack_in_game(), 1);
    assert_eq!(hero.defense_in_game(), 2);
    assert_eq!(hero.power_in_game(), 3);
    assert_eq!(hero.knowledge_in_game(), 4);

    let hero = find_hero(&save, "Nagash");
    assert_eq!(hero.attack, 10);
    assert_eq!(hero.defense, 20);
    assert_eq!(hero.power, 30);
    assert_eq!(hero.knowledge, 40);
    assert_eq!(hero.attack_in_game(), 10);
    assert_eq!(hero.defense_in_game(), 20);
    assert_eq!(hero.power_in_game(), 30);
    assert_eq!(hero.knowledge_in_game(), 40);

    let hero = find_hero(&save, "Erdamon");
    assert_eq!(hero.attack, 99);
    assert_eq!(hero.defense, 99);
    assert_eq!(hero.power, 99);
    assert_eq!(hero.knowledge, 99);
    assert_eq!(hero.attack_in_game(), 99);
    assert_eq!(hero.defense_in_game(), 99);
    assert_eq!(hero.power_in_game(), 99);
    assert_eq!(hero.knowledge_in_game(), 99);

    let hero = find_hero(&save, "Theodorus");
    assert_eq!(hero.attack, 99);
    assert_eq!(hero.defense, 99);
    assert_eq!(hero.power, 99);
    assert_eq!(hero.knowledge, 99);
    assert_eq!(hero.attack_in_game(), 99);
    assert_eq!(hero.defense_in_game(), 99);
    assert_eq!(hero.power_in_game(), 99);
    assert_eq!(hero.knowledge_in_game(), 99);
}

#[test]
fn primary_skills_day2() {
    let save = open(&data_dir("primary_skills").join("[hotseat] 112.GM2"));
    assert_date(&save, 1, 1, 2);
    assert!(save.valid());

    let hero = find_hero(&save, "Clavius");
    assert_eq!(hero.attack, 1);
    assert_eq!(hero.defense, 2);
    assert_eq!(hero.power, 12);
    assert_eq!(hero.knowledge, 4);
    assert_eq!(hero.attack_in_game(), 1);
    assert_eq!(hero.defense_in_game(), 2);
    assert_eq!(hero.power_in_game(), 12);
    assert_eq!(hero.knowledge_in_game(), 4);

    let hero = find_hero(&save, "Nagash");
    assert_eq!(hero.attack, 10);
    assert_eq!(hero.defense, 20);
    assert_eq!(hero.power, 30);
    assert_eq!(hero.knowledge, 49);
    assert_eq!(hero.attack_in_game(), 10);
    assert_eq!(hero.defense_in_game(), 20);
    assert_eq!(hero.power_in_game(), 30);
    assert_eq!(hero.knowledge_in_game(), 49);

    let hero = find_hero(&save, "Erdamon");
    assert_eq!(hero.attack, 101);
    assert_eq!(hero.defense, 105);
    assert_eq!(hero.power, 99);
    assert_eq!(hero.knowledge, 99);
    assert_eq!(hero.attack_in_game(), 99);
    assert_eq!(hero.defense_in_game(), 99);
    assert_eq!(hero.power_in_game(), 99);
    assert_eq!(hero.knowledge_in_game(), 99);

    let hero = find_hero(&save, "Theodorus");
    assert_eq!(hero.attack, 99);
    assert_eq!(hero.defense, 99);
    assert_eq!(hero.power, 99);
    assert_eq!(hero.knowledge, 107);
    assert_eq!(hero.attack_in_game(), 99);
    assert_eq!(hero.defense_in_game(), 99);
    assert_eq!(hero.power_in_game(), 99);
    assert_eq!(hero.knowledge_in_game(), 99);
}

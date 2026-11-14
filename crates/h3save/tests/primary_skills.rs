// Ported from the homm3tools C++ test test-primary-skills.cpp. Expected values are unchanged.

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
fn primary_skills_day1() {
    let _guard = common::Guard::new("primary-skills-day1");
    let save = open(&data_dir("primary_skills").join("[hotseat] 111.GM2"));
    require!(save.valid());

    let hero = find_hero(&save, "Clavius");
    require_eq!(hero.attack, 1);
    require_eq!(hero.defense, 2);
    require_eq!(hero.power, 3);
    require_eq!(hero.knowledge, 4);
    require_eq!(hero.attack_in_game(), 1);
    require_eq!(hero.defense_in_game(), 2);
    require_eq!(hero.power_in_game(), 3);
    require_eq!(hero.knowledge_in_game(), 4);

    let hero = find_hero(&save, "Nagash");
    require_eq!(hero.attack, 10);
    require_eq!(hero.defense, 20);
    require_eq!(hero.power, 30);
    require_eq!(hero.knowledge, 40);
    require_eq!(hero.attack_in_game(), 10);
    require_eq!(hero.defense_in_game(), 20);
    require_eq!(hero.power_in_game(), 30);
    require_eq!(hero.knowledge_in_game(), 40);

    let hero = find_hero(&save, "Erdamon");
    require_eq!(hero.attack, 99);
    require_eq!(hero.defense, 99);
    require_eq!(hero.power, 99);
    require_eq!(hero.knowledge, 99);
    require_eq!(hero.attack_in_game(), 99);
    require_eq!(hero.defense_in_game(), 99);
    require_eq!(hero.power_in_game(), 99);
    require_eq!(hero.knowledge_in_game(), 99);

    let hero = find_hero(&save, "Theodorus");
    require_eq!(hero.attack, 99);
    require_eq!(hero.defense, 99);
    require_eq!(hero.power, 99);
    require_eq!(hero.knowledge, 99);
    require_eq!(hero.attack_in_game(), 99);
    require_eq!(hero.defense_in_game(), 99);
    require_eq!(hero.power_in_game(), 99);
    require_eq!(hero.knowledge_in_game(), 99);
}

#[test]
fn primary_skills_day2() {
    let _guard = common::Guard::new("primary-skills-day2");
    let save = open(&data_dir("primary_skills").join("[hotseat] 112.GM2"));
    require!(save.valid());

    let hero = find_hero(&save, "Clavius");
    require_eq!(hero.attack, 1);
    require_eq!(hero.defense, 2);
    require_eq!(hero.power, 12);
    require_eq!(hero.knowledge, 4);
    require_eq!(hero.attack_in_game(), 1);
    require_eq!(hero.defense_in_game(), 2);
    require_eq!(hero.power_in_game(), 12);
    require_eq!(hero.knowledge_in_game(), 4);

    let hero = find_hero(&save, "Nagash");
    require_eq!(hero.attack, 10);
    require_eq!(hero.defense, 20);
    require_eq!(hero.power, 30);
    require_eq!(hero.knowledge, 49);
    require_eq!(hero.attack_in_game(), 10);
    require_eq!(hero.defense_in_game(), 20);
    require_eq!(hero.power_in_game(), 30);
    require_eq!(hero.knowledge_in_game(), 49);

    let hero = find_hero(&save, "Erdamon");
    require_eq!(hero.attack, 101);
    require_eq!(hero.defense, 105);
    require_eq!(hero.power, 99);
    require_eq!(hero.knowledge, 99);
    require_eq!(hero.attack_in_game(), 99);
    require_eq!(hero.defense_in_game(), 99);
    require_eq!(hero.power_in_game(), 99);
    require_eq!(hero.knowledge_in_game(), 99);

    let hero = find_hero(&save, "Theodorus");
    require_eq!(hero.attack, 99);
    require_eq!(hero.defense, 99);
    require_eq!(hero.power, 99);
    require_eq!(hero.knowledge, 107);
    require_eq!(hero.attack_in_game(), 99);
    require_eq!(hero.defense_in_game(), 99);
    require_eq!(hero.power_in_game(), 99);
    require_eq!(hero.knowledge_in_game(), 99);
}

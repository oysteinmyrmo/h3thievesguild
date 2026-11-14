// Ported from the homm3tools C++ test test-kingdom-army-strength.cpp. Expected values are unchanged.

#[macro_use]
mod common;

#[allow(unused_imports)]
use common::{data_dir, find_hero, find_town, open, test_creature, test_date};
#[allow(unused_imports)]
use h3save::stats;
#[allow(unused_imports)]
use h3save::{
    Artifact, ArtifactSlot, Color, Creature, Hero, MapSize, Orientation, SaveFile, Skill, SkillLevel, SkillSlot, Spell,
    Town,
};

use h3save::player::players;

const GREEN: u8 = 3;
const ORANGE: u8 = 4;
const PURPLE: u8 = 5;

#[test]
fn kingdom_army_strength_day1() {
    let _guard = common::Guard::new("kingdom-army-strength-day1");
    let save = open(&data_dir("kingdom_army_strength").join("[hotseat] 111.GM3"));
    test_date(&save, 1, 1, 1);
    require!(save.valid());

    let players = players(&save);

    let player = &players[usize::from(GREEN)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), 5019);

    let hero = find_hero(&save, "Rion");
    require_eq!(hero.player, GREEN);
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 0);
    require_eq!(hero.defense, 0);
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), 5019);

    let town = find_town(&save, "GreensTown");
    require_eq!(town.owner, GREEN);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);

    let player = &players[usize::from(ORANGE)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), 5019);

    let hero = find_hero(&save, "Adelaide");
    require_eq!(hero.player, ORANGE);
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 0);
    require_eq!(hero.defense, 0);
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), 5019);

    let town = find_town(&save, "OrangesTown");
    require_eq!(town.owner, ORANGE);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);

    let player = &players[usize::from(PURPLE)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), 5019);

    let hero = find_hero(&save, "Ingham");
    require_eq!(hero.player, PURPLE);
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 0);
    require_eq!(hero.defense, 0);
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), 5019);

    let town = find_town(&save, "PurplesTown");
    require_eq!(town.owner, PURPLE);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);
}

// Rion +1 attack.
// Adelaide +1 defense.
#[test]
fn kingdom_army_strength_day2() {
    let _guard = common::Guard::new("kingdom-army-strength-day2");
    let save = open(&data_dir("kingdom_army_strength").join("[hotseat] 112.GM3"));
    test_date(&save, 1, 1, 2);
    require!(save.valid());

    let players = players(&save);

    let player = &players[usize::from(GREEN)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), 5142);

    let hero = find_hero(&save, "Rion");
    require_eq!(hero.player, GREEN);
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 1); // +1 attack
    require_eq!(hero.defense, 0);
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), 5142);

    let town = find_town(&save, "GreensTown");
    require_eq!(town.owner, GREEN);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);

    let player = &players[usize::from(ORANGE)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), 5142);

    let hero = find_hero(&save, "Adelaide");
    require_eq!(hero.player, ORANGE);
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 0);
    require_eq!(hero.defense, 1); // +1 defense
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), 5142);

    let town = find_town(&save, "OrangesTown");
    require_eq!(town.owner, ORANGE);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);

    let player = &players[usize::from(PURPLE)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), 5019);

    let hero = find_hero(&save, "Ingham");
    require_eq!(hero.player, PURPLE);
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 0);
    require_eq!(hero.defense, 0);
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), 5019);

    let town = find_town(&save, "PurplesTown");
    require_eq!(town.owner, PURPLE);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);
}

// Ingham +1 attack, +1 defense.
#[test]
fn kingdom_army_strength_day3() {
    let _guard = common::Guard::new("kingdom-army-strength-day3");
    let save = open(&data_dir("kingdom_army_strength").join("[hotseat] 113.GM3"));
    test_date(&save, 1, 1, 3);
    require!(save.valid());

    let players = players(&save);

    let player = &players[usize::from(GREEN)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), 5142);

    let hero = find_hero(&save, "Rion");
    require_eq!(hero.player, GREEN);
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 1); // +1 attack
    require_eq!(hero.defense, 0);
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), 5142);

    let town = find_town(&save, "GreensTown");
    require_eq!(town.owner, GREEN);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);

    let player = &players[usize::from(ORANGE)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), 5142);

    let hero = find_hero(&save, "Adelaide");
    require_eq!(hero.player, ORANGE);
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 0);
    require_eq!(hero.defense, 1); // +1 defense
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), 5142);

    let town = find_town(&save, "OrangesTown");
    require_eq!(town.owner, ORANGE);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);

    let player = &players[usize::from(PURPLE)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), 5269);

    let hero = find_hero(&save, "Ingham");
    require_eq!(hero.player, PURPLE);
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 1); // +1 attack
    require_eq!(hero.defense, 1); // +1 defense
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), 5269);

    let town = find_town(&save, "PurplesTown");
    require_eq!(town.owner, PURPLE);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);
}

// Green recruits 14 halberdiers in town. Rion is visiting.
// Orange recruits 14 halberdiers in town. Adelaide is visiting.
#[test]
fn kingdom_army_strength_day4() {
    let _guard = common::Guard::new("kingdom-army-strength-day4");
    let save = open(&data_dir("kingdom_army_strength").join("[hotseat] 114.GM3"));
    test_date(&save, 1, 1, 4);
    require!(save.valid());

    let players = players(&save);

    let player = &players[usize::from(GREEN)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), 6752);

    let hero = find_hero(&save, "Rion");
    require_eq!(hero.player, GREEN);
    require_eq!(hero.on_map, 1); // Visiting means that the hero is on the map.
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 1);
    require_eq!(hero.defense, 0);
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), 5142);

    let town = find_town(&save, "GreensTown");
    require_eq!(town.owner, GREEN);
    test_creature(town, 0, Creature::HALBERDIER, 14);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 1610);

    let player = &players[usize::from(ORANGE)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), 6752);

    let hero = find_hero(&save, "Adelaide");
    require_eq!(hero.player, ORANGE);
    require_eq!(hero.on_map, 1); // Visiting means that the hero is on the map.
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 0);
    require_eq!(hero.defense, 1);
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), 5142);

    let town = find_town(&save, "OrangesTown");
    require_eq!(town.owner, ORANGE);
    test_creature(town, 0, Creature::HALBERDIER, 14);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 1610);

    let player = &players[usize::from(PURPLE)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), 5269);

    let hero = find_hero(&save, "Ingham");
    require_eq!(hero.player, PURPLE);
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 1);
    require_eq!(hero.defense, 1);
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), 5269);

    let town = find_town(&save, "PurplesTown");
    require_eq!(town.owner, PURPLE);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);
}

// Green moves Rion to garrison.
// Orange moves Adelaide to garrison.
// Purple recruits 14 halberdiers in town. Moves Ingham to garrison.
#[test]
fn kingdom_army_strength_day5() {
    let _guard = common::Guard::new("kingdom-army-strength-day5");
    let save = open(&data_dir("kingdom_army_strength").join("[hotseat] 115.GM3"));
    test_date(&save, 1, 1, 5);
    require!(save.valid());

    let players = players(&save);

    let expected_kas: u64 = stats::ai_value(Creature::ANGEL, 1) + stats::ai_value(Creature::HALBERDIER, 14);
    require_eq!(expected_kas, 6629);

    let player = &players[usize::from(GREEN)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), expected_kas);

    let hero = find_hero(&save, "Rion");
    require_eq!(hero.player, GREEN);
    require_eq!(hero.on_map, 0);
    require!(hero.garrison());
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::HALBERDIER, 14);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 1);
    require_eq!(hero.defense, 0);
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), expected_kas); // When in garrison, attack/defense does not modify army strength.

    let town = find_town(&save, "GreensTown");
    require_eq!(town.owner, GREEN);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);

    let player = &players[usize::from(ORANGE)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), expected_kas);

    let hero = find_hero(&save, "Adelaide");
    require_eq!(hero.player, ORANGE);
    require_eq!(hero.on_map, 0);
    require!(hero.garrison());
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::HALBERDIER, 14);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 0);
    require_eq!(hero.defense, 1);
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), expected_kas); // When in garrison, attack/defense does not modify army strength.

    let town = find_town(&save, "OrangesTown");
    require_eq!(town.owner, ORANGE);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);

    let player = &players[usize::from(PURPLE)];
    require!(player.alive());
    require_eq!(player.heroes().len(), 1);
    require_eq!(player.towns().len(), 1);
    require_eq!(player.kingdom_army_strength(), expected_kas);

    let hero = find_hero(&save, "Ingham");
    require_eq!(hero.player, PURPLE);
    require_eq!(hero.on_map, 0);
    require!(hero.garrison());
    test_creature(hero, 0, Creature::ANGEL, 1);
    test_creature(hero, 1, Creature::HALBERDIER, 14);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(hero.attack, 1);
    require_eq!(hero.defense, 1);
    require_eq!(hero.power, 1);
    require_eq!(hero.knowledge, 1);
    require_eq!(hero.army_strength(), expected_kas); // When in garrison, attack/defense does not modify army strength.

    let town = find_town(&save, "PurplesTown");
    require_eq!(town.owner, PURPLE);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);
    require_eq!(town.army_strength(), 0);
}

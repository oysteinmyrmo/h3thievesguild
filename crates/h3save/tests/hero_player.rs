// Ported from the homm3tools C++ test test-hero-player.cpp. Expected values are unchanged.

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
fn hero_player_01() {
    let save = open(&data_dir("hero_player").join("[hotseat] 111.01.GM8"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    let hero = find_hero(&save, "Orrin");
    assert_eq!(hero.player, 0);

    let hero = find_hero(&save, "Valeska");
    assert_eq!(hero.player, 1);

    let hero = find_hero(&save, "Edric");
    assert_eq!(hero.player, 2);

    let hero = find_hero(&save, "Sylvia");
    assert_eq!(hero.player, 3);

    let hero = find_hero(&save, "Lord Haart");
    assert_eq!(hero.player, 4);

    let hero = find_hero(&save, "Sorsha");
    assert_eq!(hero.player, 5);

    let hero = find_hero(&save, "Christian");
    assert_eq!(hero.player, 6);

    let hero = find_hero(&save, "Tyris");
    assert_eq!(hero.player, 7);
}

#[test]
fn hero_player_02() {
    let save = open(&data_dir("hero_player").join("[hotseat] 111.02.GM8"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    let hero = find_hero(&save, "Orrin");
    assert_eq!(hero.player, 7);

    let hero = find_hero(&save, "Valeska");
    assert_eq!(hero.player, 0);

    let hero = find_hero(&save, "Edric");
    assert_eq!(hero.player, 1);

    let hero = find_hero(&save, "Sylvia");
    assert_eq!(hero.player, 2);

    let hero = find_hero(&save, "Lord Haart");
    assert_eq!(hero.player, 3);

    let hero = find_hero(&save, "Sorsha");
    assert_eq!(hero.player, 4);

    let hero = find_hero(&save, "Christian");
    assert_eq!(hero.player, 5);

    let hero = find_hero(&save, "Tyris");
    assert_eq!(hero.player, 6);
}

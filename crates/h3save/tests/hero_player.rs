// Ported from the homm3tools C++ test test-hero-player.cpp. Expected values are unchanged.

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

#[test]
fn hero_player_01() {
    let _guard = common::Guard::new("hero-player-01");
    let save = open(&data_dir("hero_player").join("[hotseat] 111.01.GM8"));
    test_date(&save, 1, 1, 1);
    require!(save.valid());

    let hero = find_hero(&save, "Orrin");
    require_eq!(hero.player, 0);

    let hero = find_hero(&save, "Valeska");
    require_eq!(hero.player, 1);

    let hero = find_hero(&save, "Edric");
    require_eq!(hero.player, 2);

    let hero = find_hero(&save, "Sylvia");
    require_eq!(hero.player, 3);

    let hero = find_hero(&save, "Lord Haart");
    require_eq!(hero.player, 4);

    let hero = find_hero(&save, "Sorsha");
    require_eq!(hero.player, 5);

    let hero = find_hero(&save, "Christian");
    require_eq!(hero.player, 6);

    let hero = find_hero(&save, "Tyris");
    require_eq!(hero.player, 7);
}

#[test]
fn hero_player_02() {
    let _guard = common::Guard::new("hero-player-02");
    let save = open(&data_dir("hero_player").join("[hotseat] 111.02.GM8"));
    test_date(&save, 1, 1, 1);
    require!(save.valid());

    let hero = find_hero(&save, "Orrin");
    require_eq!(hero.player, 7);

    let hero = find_hero(&save, "Valeska");
    require_eq!(hero.player, 0);

    let hero = find_hero(&save, "Edric");
    require_eq!(hero.player, 1);

    let hero = find_hero(&save, "Sylvia");
    require_eq!(hero.player, 2);

    let hero = find_hero(&save, "Lord Haart");
    require_eq!(hero.player, 3);

    let hero = find_hero(&save, "Sorsha");
    require_eq!(hero.player, 4);

    let hero = find_hero(&save, "Christian");
    require_eq!(hero.player, 5);

    let hero = find_hero(&save, "Tyris");
    require_eq!(hero.player, 6);
}

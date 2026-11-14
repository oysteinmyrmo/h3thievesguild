// Ported from the homm3tools C++ test test-spell-points.cpp. Expected values are unchanged.

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
fn spell_points_day1() {
    let _guard = common::Guard::new("spell-points-day1");
    let save = open(data_dir("spell_points").join("[hotseat] 111.GM2"));
    require!(save.valid());

    let hero = find_hero(&save, "Sephinroth");
    require_eq!(hero.spell_points, 360);

    let hero = find_hero(&save, "Sorsha");
    require_eq!(hero.spell_points, 10);
}

#[test]
fn spell_points_day2() {
    let _guard = common::Guard::new("spell-points-day2");
    let save = open(data_dir("spell_points").join("[hotseat] 112.GM2"));
    require!(save.valid());

    let hero = find_hero(&save, "Sephinroth");
    require_eq!(hero.spell_points, 344);

    let hero = find_hero(&save, "Sorsha");
    require_eq!(hero.spell_points, 4);
}

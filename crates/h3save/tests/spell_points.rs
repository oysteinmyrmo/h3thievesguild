// Ported from the homm3tools C++ test test-spell-points.cpp. Expected values are unchanged.

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
fn spell_points_day1() {
    let save = open(&data_dir("spell_points").join("[hotseat] 111.GM2"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    let hero = find_hero(&save, "Sephinroth");
    assert_eq!(hero.spell_points, 360);

    let hero = find_hero(&save, "Sorsha");
    assert_eq!(hero.spell_points, 10);
}

#[test]
fn spell_points_day2() {
    let save = open(&data_dir("spell_points").join("[hotseat] 112.GM2"));
    assert_date(&save, 1, 1, 2);
    assert!(save.valid());

    let hero = find_hero(&save, "Sephinroth");
    assert_eq!(hero.spell_points, 344);

    let hero = find_hero(&save, "Sorsha");
    assert_eq!(hero.spell_points, 4);
}

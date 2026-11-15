// Ported from the homm3tools C++ test test-hero-movement.cpp. Expected values are unchanged.

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

// - Orrin starts at [2,2,0], facing E.
// - Valeska starts at [2,4,0], facing E.
#[test]
fn hero_movement_start() {
    let save = open(&data_dir("movement").join("start.GM2"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    let orrin = &save.heroes[0];
    assert_eq!(orrin.x, 2);
    assert_eq!(orrin.y, 2);
    assert_eq!(orrin.z, 0);
    assert_eq!(orrin.orientation, Orientation::E);
    assert_eq!(orrin.movement_remaining, 1560);

    let valeska = &save.heroes[1];
    assert_eq!(valeska.x, 2);
    assert_eq!(valeska.y, 4);
    assert_eq!(valeska.z, 0);
    assert_eq!(valeska.orientation, Orientation::E);
    assert_eq!(valeska.movement_remaining, 1630);
}

// - Orrin moved to [3,2,0], facing E.
// - Valeska moved to [1,4,0], facing W.
#[test]
fn hero_movement_day1() {
    let save = open(&data_dir("movement").join("[hotseat] 111.GM2"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    let orrin = &save.heroes[0];
    assert_eq!(orrin.x, 3); // Moved 1E
    assert_eq!(orrin.y, 2);
    assert_eq!(orrin.z, 0);
    assert_eq!(orrin.orientation, Orientation::E);
    assert_eq!(orrin.movement_remaining, 1460);

    let valeska = &save.heroes[1];
    assert_eq!(valeska.x, 1); // Moved 1W
    assert_eq!(valeska.y, 4);
    assert_eq!(valeska.z, 0);
    assert_eq!(valeska.orientation, Orientation::W);
    assert_eq!(valeska.movement_remaining, 1530);
}

// - Orrin sets destination to [5,4,0].
// - Valeska sets destination to [2,3,0].
#[test]
fn hero_movement_day2() {
    let save = open(&data_dir("movement").join("[hotseat] 112.GM2"));
    assert_date(&save, 1, 1, 2);
    assert!(save.valid());

    let orrin = &save.heroes[0];
    assert_eq!(orrin.x, 3);
    assert_eq!(orrin.y, 2);
    assert_eq!(orrin.z, 0);
    assert_eq!(orrin.orientation, Orientation::E);
    assert_eq!(orrin.dest_x, 5);
    assert_eq!(orrin.dest_y, 4);
    assert_eq!(orrin.dest_z, 0);
    assert_eq!(orrin.movement_remaining, 1560);

    let valeska = &save.heroes[1];
    assert_eq!(valeska.x, 1);
    assert_eq!(valeska.y, 4);
    assert_eq!(valeska.z, 0);
    assert_eq!(valeska.orientation, Orientation::W);
    assert_eq!(valeska.dest_x, 2);
    assert_eq!(valeska.dest_y, 3);
    assert_eq!(valeska.dest_z, 0);
    assert_eq!(valeska.movement_remaining, 1630);
}

// - Orrin sets destination to [5,0,0], facing NW.
// - Valeska sets destination to [0,1,0], facing S.
#[test]
fn hero_movement_day3() {
    let save = open(&data_dir("movement").join("[hotseat] 113.GM2"));
    assert_date(&save, 1, 1, 3);
    assert!(save.valid());

    let orrin = &save.heroes[0];
    assert_eq!(orrin.x, 3);
    assert_eq!(orrin.y, 2);
    assert_eq!(orrin.z, 0);
    assert_eq!(orrin.orientation, Orientation::NW);
    assert_eq!(orrin.dest_x, 5);
    assert_eq!(orrin.dest_y, 0);
    assert_eq!(orrin.dest_z, 0);
    assert_eq!(orrin.movement_remaining, 1078);

    let valeska = &save.heroes[1];
    assert_eq!(valeska.x, 1);
    assert_eq!(valeska.y, 4);
    assert_eq!(valeska.z, 0);
    assert_eq!(valeska.orientation, Orientation::S);
    assert_eq!(valeska.dest_x, 0);
    assert_eq!(valeska.dest_y, 1);
    assert_eq!(valeska.dest_z, 0);
    assert_eq!(valeska.movement_remaining, 1430);
}

// - Orrin spent all his movement, except 150 movement points.
// - Valeska spent all her movement, except 630 movement points.
#[test]
fn hero_movement_day4() {
    let save = open(&data_dir("movement").join("[hotseat] 114.GM2"));
    assert_date(&save, 1, 1, 4);
    assert!(save.valid());

    let orrin = &save.heroes[0];
    assert_eq!(orrin.x, 3);
    assert_eq!(orrin.y, 2);
    assert_eq!(orrin.z, 0);
    assert_eq!(orrin.orientation, Orientation::NW);
    assert_eq!(orrin.dest_x, 5);
    assert_eq!(orrin.dest_y, 0);
    assert_eq!(orrin.dest_z, 0);
    assert_eq!(orrin.movement_remaining, 150);

    let valeska = &save.heroes[1];
    assert_eq!(valeska.x, 1);
    assert_eq!(valeska.y, 4);
    assert_eq!(valeska.z, 0);
    assert_eq!(valeska.orientation, Orientation::S);
    assert_eq!(valeska.dest_x, 0);
    assert_eq!(valeska.dest_y, 1);
    assert_eq!(valeska.dest_z, 0);
    assert_eq!(valeska.movement_remaining, 630);
}

// - Orrin sets target to Valeska at [1,4,0].
// - Valeska sets target to Orrin at [3,2,0].
#[test]
fn hero_movement_day5() {
    let save = open(&data_dir("movement").join("[hotseat] 115.GM2"));
    assert_date(&save, 1, 1, 5);
    assert!(save.valid());

    let orrin = &save.heroes[0];
    assert_eq!(orrin.x, 3);
    assert_eq!(orrin.y, 2);
    assert_eq!(orrin.z, 0);
    assert_eq!(orrin.orientation, Orientation::NW);
    assert_eq!(orrin.dest_x, 1);
    assert_eq!(orrin.dest_y, 4);
    assert_eq!(orrin.dest_z, 0);
    assert_eq!(orrin.movement_remaining, 1560);

    let valeska = &save.heroes[1];
    assert_eq!(valeska.x, 1);
    assert_eq!(valeska.y, 4);
    assert_eq!(valeska.z, 0);
    assert_eq!(valeska.orientation, Orientation::S);
    assert_eq!(valeska.dest_x, 3);
    assert_eq!(valeska.dest_y, 2);
    assert_eq!(valeska.dest_z, 0);
    assert_eq!(valeska.movement_remaining, 1630);
}

// - Orrin now faces N.
// - Valeska now faces SE.
#[test]
fn hero_movement_day6() {
    let save = open(&data_dir("movement").join("[hotseat] 116.GM2"));
    assert_date(&save, 1, 1, 6);
    assert!(save.valid());

    let orrin = &save.heroes[0];
    assert_eq!(orrin.x, 3);
    assert_eq!(orrin.y, 2);
    assert_eq!(orrin.z, 0);
    assert_eq!(orrin.orientation, Orientation::N);
    assert_eq!(orrin.dest_x, 1);
    assert_eq!(orrin.dest_y, 4);
    assert_eq!(orrin.dest_z, 0);
    assert_eq!(orrin.movement_remaining, 1360);

    let valeska = &save.heroes[1];
    assert_eq!(valeska.x, 1);
    assert_eq!(valeska.y, 4);
    assert_eq!(valeska.z, 0);
    assert_eq!(valeska.orientation, Orientation::SE);
    assert_eq!(valeska.dest_x, 3);
    assert_eq!(valeska.dest_y, 2);
    assert_eq!(valeska.dest_z, 0);
    assert_eq!(valeska.movement_remaining, 1348);
}

// - Orrin now faces SW.
// - OrrinPlayer loses because he has no town (Valeska cannot move).
#[test]
fn hero_movement_day7() {
    let save = open(&data_dir("movement").join("[hotseat] 117.GM2"));
    assert_date(&save, 1, 1, 7);
    assert!(save.valid());

    let orrin = &save.heroes[0];
    assert_eq!(orrin.x, 3);
    assert_eq!(orrin.y, 2);
    assert_eq!(orrin.z, 0);
    assert_eq!(orrin.orientation, Orientation::SW);
    assert_eq!(orrin.dest_x, 1);
    assert_eq!(orrin.dest_y, 4);
    assert_eq!(orrin.dest_z, 0);
    assert_eq!(orrin.movement_remaining, 1219);

    let valeska = &save.heroes[1];
    assert_eq!(valeska.x, 1);
    assert_eq!(valeska.y, 4);
    assert_eq!(valeska.z, 0);
    assert_eq!(valeska.orientation, Orientation::SE);
    assert_eq!(valeska.dest_x, 3);
    assert_eq!(valeska.dest_y, 2);
    assert_eq!(valeska.dest_z, 0);
    assert_eq!(valeska.movement_remaining, 1348);
}

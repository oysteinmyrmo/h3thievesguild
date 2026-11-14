// Ported from the homm3tools C++ test test-hero-movement-gigantic.cpp. Expected values are unchanged.

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

// - Orrin starts at [245,246,0], facing E.
// - Valeska starts at [249,250,0], facing E.
#[test]
fn hero_movement_gigantic_start() {
    let _guard = common::Guard::new("hero-movement-gigantic-start");
    let save = open(&data_dir("movement_gigantic").join("start.GM2"));
    require!(save.valid());

    let orrin = &save.heroes[0];
    require_eq!(orrin.x, 245);
    require_eq!(orrin.y, 246);
    require_eq!(orrin.z, 0);
    require_eq!(orrin.orientation, Orientation::E);
    require_eq!(orrin.movement_remaining, 1560);

    // Note: For some reason, Valeska's position was not saved at this point.
    let valeska = &save.heroes[1];
    require_eq!(valeska.x, 0xFFFF);
    require_eq!(valeska.y, 0xFFFF);
    require_eq!(valeska.z, 0xFFFF);
    require_eq!(valeska.orientation, Orientation::E);
    require_eq!(valeska.movement_remaining, 1630);
}

// - Orrin moved to [245,247,0], facing S.
// - Valeska moved to [248,249,0], facing NW.
#[test]
fn hero_movement_gigantic_day1() {
    let _guard = common::Guard::new("hero-movement-gigantic-day1");
    let save = open(&data_dir("movement_gigantic").join("[hotseat] 111.GM2"));
    require!(save.valid());

    let orrin = &save.heroes[0];
    require_eq!(orrin.x, 245);
    require_eq!(orrin.y, 247); // Moved 1S
    require_eq!(orrin.z, 0);
    require_eq!(orrin.orientation, Orientation::S);
    require_eq!(orrin.movement_remaining, 1385);

    let valeska = &save.heroes[1];
    require_eq!(valeska.x, 248); // Moved 1NW
    require_eq!(valeska.y, 249); // Moved 1NW
    require_eq!(valeska.z, 0);
    require_eq!(valeska.orientation, Orientation::NW);
    require_eq!(valeska.movement_remaining, 1383);
}

// - Orrin sets target position to [241,248,0].
// - Valeska sets target position to [249,250,0].
#[test]
fn hero_movement_gigantic_day2() {
    let _guard = common::Guard::new("hero-movement-gigantic-day2");
    let save = open(&data_dir("movement_gigantic").join("[hotseat] 112.GM2"));
    require!(save.valid());

    let orrin = &save.heroes[0];
    require_eq!(orrin.x, 245);
    require_eq!(orrin.y, 247);
    require_eq!(orrin.z, 0);
    require_eq!(orrin.orientation, Orientation::S);
    require_eq!(orrin.dest_x, 241);
    require_eq!(orrin.dest_y, 248);
    require_eq!(orrin.dest_z, 0);
    require_eq!(orrin.movement_remaining, 1560);

    let valeska = &save.heroes[1];
    require_eq!(valeska.x, 248);
    require_eq!(valeska.y, 249);
    require_eq!(valeska.z, 0);
    require_eq!(valeska.orientation, Orientation::NW);
    require_eq!(valeska.dest_x, 249);
    require_eq!(valeska.dest_y, 250);
    require_eq!(valeska.dest_z, 0);
    require_eq!(valeska.movement_remaining, 1630);
}

// - Orrin now faces W
// - Valeska now faces SW
#[test]
fn hero_movement_gigantic_day3() {
    let _guard = common::Guard::new("hero-movement-gigantic-day3");
    let save = open(&data_dir("movement_gigantic").join("[hotseat] 113.GM2"));
    require!(save.valid());

    let orrin = &save.heroes[0];
    require_eq!(orrin.x, 245);
    require_eq!(orrin.y, 247);
    require_eq!(orrin.z, 0);
    require_eq!(orrin.orientation, Orientation::W);
    require_eq!(orrin.dest_x, 241);
    require_eq!(orrin.dest_y, 248);
    require_eq!(orrin.dest_z, 0);
    require_eq!(orrin.movement_remaining, 1210);

    let valeska = &save.heroes[1];
    require_eq!(valeska.x, 248);
    require_eq!(valeska.y, 249);
    require_eq!(valeska.z, 0);
    require_eq!(valeska.orientation, Orientation::SW);
    require_eq!(valeska.dest_x, 249);
    require_eq!(valeska.dest_y, 250);
    require_eq!(valeska.dest_z, 0);
    require_eq!(valeska.movement_remaining, 1136);
}

// - Orrin spent all his movement, except 160 movement points.
// - Valeska spent all her movement, except 539 movement points.
#[test]
fn hero_movement_gigantic_day4() {
    let _guard = common::Guard::new("hero-movement-gigantic-day4");
    let save = open(&data_dir("movement_gigantic").join("[hotseat] 114.GM2"));
    require!(save.valid());

    let orrin = &save.heroes[0];
    require_eq!(orrin.x, 245);
    require_eq!(orrin.y, 247);
    require_eq!(orrin.z, 0);
    require_eq!(orrin.orientation, Orientation::W);
    require_eq!(orrin.dest_x, 241);
    require_eq!(orrin.dest_y, 248);
    require_eq!(orrin.dest_z, 0);
    require_eq!(orrin.movement_remaining, 160);

    let valeska = &save.heroes[1];
    require_eq!(valeska.x, 248);
    require_eq!(valeska.y, 249);
    require_eq!(valeska.z, 0);
    require_eq!(valeska.orientation, Orientation::SW);
    require_eq!(valeska.dest_x, 249);
    require_eq!(valeska.dest_y, 250);
    require_eq!(valeska.dest_z, 0);
    require_eq!(valeska.movement_remaining, 539);
}

// - Orrin sets target to Valeska at [248,249,0].
// - Valeska sets target to Orrin at [245,247,0].
#[test]
fn hero_movement_gigantic_day5() {
    let _guard = common::Guard::new("hero-movement-gigantic-day5");
    let save = open(&data_dir("movement_gigantic").join("[hotseat] 115.GM2"));
    require!(save.valid());

    let orrin = &save.heroes[0];
    require_eq!(orrin.x, 245);
    require_eq!(orrin.y, 247);
    require_eq!(orrin.z, 0);
    require_eq!(orrin.orientation, Orientation::W);
    require_eq!(orrin.dest_x, 248);
    require_eq!(orrin.dest_y, 249);
    require_eq!(orrin.dest_z, 0);
    require_eq!(orrin.movement_remaining, 1560);

    let valeska = &save.heroes[1];
    require_eq!(valeska.x, 248);
    require_eq!(valeska.y, 249);
    require_eq!(valeska.z, 0);
    require_eq!(valeska.orientation, Orientation::SW);
    require_eq!(valeska.dest_x, 245);
    require_eq!(valeska.dest_y, 247);
    require_eq!(valeska.dest_z, 0);
    require_eq!(valeska.movement_remaining, 1630);
}

// - Orrin now faces N.
// - Valeska now faces SE.
#[test]
fn hero_movement_gigantic_day6() {
    let _guard = common::Guard::new("hero-movement-gigantic-day6");
    let save = open(&data_dir("movement_gigantic").join("[hotseat] 116.GM2"));
    require!(save.valid());

    let orrin = &save.heroes[0];
    require_eq!(orrin.x, 245);
    require_eq!(orrin.y, 247);
    require_eq!(orrin.z, 0);
    require_eq!(orrin.orientation, Orientation::N);
    require_eq!(orrin.dest_x, 248);
    require_eq!(orrin.dest_y, 249);
    require_eq!(orrin.dest_z, 0);
    require_eq!(orrin.movement_remaining, 1210);

    let valeska = &save.heroes[1];
    require_eq!(valeska.x, 248);
    require_eq!(valeska.y, 249);
    require_eq!(valeska.z, 0);
    require_eq!(valeska.orientation, Orientation::SE);
    require_eq!(valeska.dest_x, 245);
    require_eq!(valeska.dest_y, 247);
    require_eq!(valeska.dest_z, 0);
    require_eq!(valeska.movement_remaining, 1136);
}

// - Orrin now faces E.
// - OrrinPlayer loses because he has no town (Valeska cannot move).
#[test]
fn hero_movement_gigantic_day7() {
    let _guard = common::Guard::new("hero-movement-gigantic-day7");
    let save = open(&data_dir("movement_gigantic").join("[hotseat] 117.GM2"));
    require!(save.valid());

    let orrin = &save.heroes[0];
    require_eq!(orrin.x, 245);
    require_eq!(orrin.y, 247);
    require_eq!(orrin.z, 0);
    require_eq!(orrin.orientation, Orientation::E);
    require_eq!(orrin.dest_x, 248);
    require_eq!(orrin.dest_y, 249);
    require_eq!(orrin.dest_z, 0);
    require_eq!(orrin.movement_remaining, 1210);

    let valeska = &save.heroes[1];
    require_eq!(valeska.x, 248);
    require_eq!(valeska.y, 249);
    require_eq!(valeska.z, 0);
    require_eq!(valeska.orientation, Orientation::SE);
    require_eq!(valeska.dest_x, 245);
    require_eq!(valeska.dest_y, 247);
    require_eq!(valeska.dest_z, 0);
    require_eq!(valeska.movement_remaining, 1136);
}

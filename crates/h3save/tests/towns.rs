// Ported from the homm3tools C++ test test-towns.cpp. Expected values are unchanged.

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

fn test_town_constants(towns: &[Town]) {
    require_eq!(towns.len(), 4);

    let town = &towns[0];
    require_eq!(town.name, "RedsTown");
    require_eq!(town.name.len(), usize::from(town.name_size));
    require_eq!(town.x, 12);
    require_eq!(town.y, 7);
    require_eq!(town.z, 0);

    let town = &towns[1];
    require_eq!(town.name, "BluesTown");
    require_eq!(town.name.len(), usize::from(town.name_size));
    require_eq!(town.x, 18);
    require_eq!(town.y, 7);
    require_eq!(town.z, 0);

    let town = &towns[2];
    require_eq!(town.name, "BUUUUUUURN");
    require_eq!(town.name.len(), usize::from(town.name_size));
    require_eq!(town.x, 12);
    require_eq!(town.y, 14);
    require_eq!(town.z, 0);

    let town = &towns[3];
    require_eq!(town.name, "LOOKOUT");
    require_eq!(town.name.len(), usize::from(town.name_size));
    require_eq!(town.x, 18);
    require_eq!(town.y, 14);
    require_eq!(town.z, 0);
}

#[test]
fn towns_day1() {
    let _guard = common::Guard::new("towns-day1");
    let save = open_with_towns(data_dir("towns").join("[hotseat] 111.GM2"), "RedsTown", 4);
    require!(save.valid());
    test_town_constants(&save.towns);

    let town = find_town(&save, "RedsTown");
    require_eq!(town.owner, 0);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);

    let town = find_town(&save, "BluesTown");
    require_eq!(town.owner, 1);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);

    let town = find_town(&save, "BUUUUUUURN");
    require_eq!(town.owner, h3save::town::NO_OWNER);
    test_creature(town, 0, Creature::MAGOG, 1);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::MAGOG, 2);

    let town = find_town(&save, "LOOKOUT");
    require_eq!(town.owner, h3save::town::NO_OWNER);
    test_creature(town, 0, Creature::GREMLIN, 1);
    test_creature(town, 1, Creature::GREMLIN, 2);
    test_creature(town, 2, Creature::GREMLIN, 3);
    test_creature(town, 3, Creature::GREMLIN, 4);
    test_creature(town, 4, Creature::GREMLIN, 5);
    test_creature(town, 5, Creature::GREMLIN, 6);
    test_creature(town, 6, Creature::GREMLIN, 7);
}

// - Red takes BluesTown.
// - Leaves [griffin, centaur, dwarf, empty, dwarf, centaur, griffin]/[1,1,1,0,1,1,1] in garrison.
// - Edric stands in town with [pikeman, griffin,0,0,0,0]/[15,2,0,0,0,0,0]
#[test]
fn towns_day2() {
    let _guard = common::Guard::new("towns-day2");
    let save = open_with_towns(data_dir("towns").join("[hotseat] 112.GM2"), "RedsTown", 4);
    require!(save.valid());
    test_town_constants(&save.towns);

    let town = find_town(&save, "RedsTown");
    require_eq!(town.owner, 0);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);

    let town = find_town(&save, "BluesTown");
    require_eq!(town.owner, 0); // Changed owner
    test_creature(town, 0, Creature::GRIFFIN, 1);
    test_creature(town, 1, Creature::CENTAUR, 1);
    test_creature(town, 2, Creature::DWARF, 1);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::DWARF, 1);
    test_creature(town, 5, Creature::CENTAUR, 1);
    test_creature(town, 6, Creature::GRIFFIN, 1);

    // Edric stands in BluesTown.
    let hero = find_hero(&save, "Edric");
    require_eq!(hero.on_map, 1);
    require_eq!(hero.x, u16::from(town.x));
    require_eq!(hero.y, u16::from(town.y));
    require_eq!(hero.z, u16::from(town.z));
    test_creature(hero, 0, Creature::PIKEMAN, 15);
    test_creature(hero, 1, Creature::GRIFFIN, 2);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);

    let town = find_town(&save, "BUUUUUUURN");
    test_creature(town, 0, Creature::MAGOG, 1);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::MAGOG, 2);

    let town = find_town(&save, "LOOKOUT");
    test_creature(town, 0, Creature::GREMLIN, 1);
    test_creature(town, 1, Creature::GREMLIN, 2);
    test_creature(town, 2, Creature::GREMLIN, 3);
    test_creature(town, 3, Creature::GREMLIN, 4);
    test_creature(town, 4, Creature::GREMLIN, 5);
    test_creature(town, 5, Creature::GREMLIN, 6);
    test_creature(town, 6, Creature::GREMLIN, 7);
}

// - Blue takes RedsTown.
// - Leaves [centaur, empty, archer, dwarf, archer, dwarf, centaur]/[2,0,5,1,4,1,2] in garrison.
// - Kyrre stands in town with [woodelf, centaur, 0, 0, 0, 0, woodelf]/[1,19,0,0,0,0,1].
#[test]
fn towns_day3() {
    let _guard = common::Guard::new("towns-day3");
    let save = open_with_towns(data_dir("towns").join("[hotseat] 113.GM2"), "RedsTown", 4);
    require!(save.valid());
    test_town_constants(&save.towns);

    let town = find_town(&save, "RedsTown");
    require_eq!(town.owner, 1); // Changed owner
    test_creature(town, 0, Creature::CENTAUR, 2);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::ARCHER, 5);
    test_creature(town, 3, Creature::DWARF, 1);
    test_creature(town, 4, Creature::ARCHER, 4);
    test_creature(town, 5, Creature::DWARF, 1);
    test_creature(town, 6, Creature::CENTAUR, 2);

    // Kyrre stands in RedsTown.
    let hero = find_hero(&save, "Kyrre");
    require_eq!(hero.on_map, 1);
    require_eq!(hero.x, u16::from(town.x));
    require_eq!(hero.y, u16::from(town.y));
    require_eq!(hero.z, u16::from(town.z));
    test_creature(hero, 0, Creature::WOOD_ELF, 1);
    test_creature(hero, 1, Creature::CENTAUR, 19);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::WOOD_ELF, 1);

    let town = find_town(&save, "BluesTown");
    require_eq!(town.owner, 0); // Changed owner
    test_creature(town, 0, Creature::GRIFFIN, 1);
    test_creature(town, 1, Creature::CENTAUR, 1);
    test_creature(town, 2, Creature::DWARF, 1);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::DWARF, 1);
    test_creature(town, 5, Creature::CENTAUR, 1);
    test_creature(town, 6, Creature::GRIFFIN, 1);

    // Edric stands in BluesTown.
    let hero = find_hero(&save, "Edric");
    require_eq!(hero.on_map, 1);
    require_eq!(hero.x, u16::from(town.x));
    require_eq!(hero.y, u16::from(town.y));
    require_eq!(hero.z, u16::from(town.z));
    test_creature(hero, 0, Creature::PIKEMAN, 15);
    test_creature(hero, 1, Creature::GRIFFIN, 2);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);

    let town = find_town(&save, "BUUUUUUURN");
    test_creature(town, 0, Creature::MAGOG, 1);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::MAGOG, 2);

    let town = find_town(&save, "LOOKOUT");
    test_creature(town, 0, Creature::GREMLIN, 1);
    test_creature(town, 1, Creature::GREMLIN, 2);
    test_creature(town, 2, Creature::GREMLIN, 3);
    test_creature(town, 3, Creature::GREMLIN, 4);
    test_creature(town, 4, Creature::GREMLIN, 5);
    test_creature(town, 5, Creature::GREMLIN, 6);
    test_creature(town, 6, Creature::GREMLIN, 7);
}

// - Red takes BUUUUUUURN.
// - Blue takes LOOKOUT.
// - Edric stands in BUUUUUUURN with [pikeman, griffin,0,0,0,0]/[15,2,0,0,0,0,0]
// - BUUUUUUURN has garrison [imp, imp, imp, gog, 0, gog, gog]/[2,2,2,2,0,1,1]
// - Kyrre stands in LOOKOUT with [woodelf, centaur, 0, 0, centaur, 0, woodelf]/[1,8,0,0,8,0,1].
// - LOOKOUT has garrison [gremlin, gremlin, 0, gremlin, gremlin, 0, stonegargoyle], [2,2,0,4,2,0,9].
#[test]
fn towns_day4() {
    let _guard = common::Guard::new("towns-day4");
    let save = open_with_towns(data_dir("towns").join("[hotseat] 114.GM2"), "RedsTown", 4);
    require!(save.valid());
    test_town_constants(&save.towns);

    let town = find_town(&save, "RedsTown");
    require_eq!(town.owner, 1);
    test_creature(town, 0, Creature::CENTAUR, 2);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::ARCHER, 5);
    test_creature(town, 3, Creature::DWARF, 1);
    test_creature(town, 4, Creature::ARCHER, 4);
    test_creature(town, 5, Creature::DWARF, 1);
    test_creature(town, 6, Creature::CENTAUR, 2);

    let town = find_town(&save, "BluesTown");
    require_eq!(town.owner, 0);
    test_creature(town, 0, Creature::GRIFFIN, 1);
    test_creature(town, 1, Creature::CENTAUR, 1);
    test_creature(town, 2, Creature::DWARF, 1);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::DWARF, 1);
    test_creature(town, 5, Creature::CENTAUR, 1);
    test_creature(town, 6, Creature::GRIFFIN, 1);

    let town = find_town(&save, "BUUUUUUURN");
    require_eq!(town.owner, 0); // Changed owner
    test_creature(town, 0, Creature::IMP, 2);
    test_creature(town, 1, Creature::IMP, 2);
    test_creature(town, 2, Creature::IMP, 2);
    test_creature(town, 3, Creature::GOG, 2);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::GOG, 1);
    test_creature(town, 6, Creature::GOG, 1);

    // Edric stands in BUUUUUUURN.
    let hero = find_hero(&save, "Edric");
    require_eq!(hero.on_map, 1);
    require_eq!(hero.x, u16::from(town.x));
    require_eq!(hero.y, u16::from(town.y));
    require_eq!(hero.z, u16::from(town.z));
    test_creature(hero, 0, Creature::PIKEMAN, 15);
    test_creature(hero, 1, Creature::GRIFFIN, 2);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);

    let town = find_town(&save, "LOOKOUT");
    require_eq!(town.owner, 1); // Changed owner
    test_creature(town, 0, Creature::GREMLIN, 2);
    test_creature(town, 1, Creature::GREMLIN, 2);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::GREMLIN, 4);
    test_creature(town, 4, Creature::GREMLIN, 2);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::STONE_GARGOYLE, 9);

    // Kyrre stands in LOOKOUT.
    let hero = find_hero(&save, "Kyrre");
    require_eq!(hero.on_map, 1);
    require_eq!(hero.x, u16::from(town.x));
    require_eq!(hero.y, u16::from(town.y));
    require_eq!(hero.z, u16::from(town.z));
    test_creature(hero, 0, Creature::WOOD_ELF, 1);
    test_creature(hero, 1, Creature::CENTAUR, 8);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::CENTAUR, 8);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::WOOD_ELF, 1);
}

// - Edric changes to visiting hero.
// - Kyrre changes to visiting hero.
#[test]
fn towns_day5() {
    let _guard = common::Guard::new("towns-day5");
    let save = open_with_towns(data_dir("towns").join("[hotseat] 115.GM2"), "RedsTown", 4);
    require!(save.valid());
    test_town_constants(&save.towns);

    let town = find_town(&save, "RedsTown");
    require_eq!(town.owner, 1);
    test_creature(town, 0, Creature::CENTAUR, 2);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::ARCHER, 5);
    test_creature(town, 3, Creature::DWARF, 1);
    test_creature(town, 4, Creature::ARCHER, 4);
    test_creature(town, 5, Creature::DWARF, 1);
    test_creature(town, 6, Creature::CENTAUR, 2);

    let town = find_town(&save, "BluesTown");
    require_eq!(town.owner, 0);
    test_creature(town, 0, Creature::GRIFFIN, 1);
    test_creature(town, 1, Creature::CENTAUR, 1);
    test_creature(town, 2, Creature::DWARF, 1);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::DWARF, 1);
    test_creature(town, 5, Creature::CENTAUR, 1);
    test_creature(town, 6, Creature::GRIFFIN, 1);

    // BUUUUUUURN has no creatures in garrison since Edric is visiting.
    let town = find_town(&save, "BUUUUUUURN");
    require_eq!(town.owner, 0);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);

    // Edric is visiting BUUUUUUURN.
    let hero = find_hero(&save, "Edric");
    require_eq!(hero.on_map, 0);
    require_eq!(hero.x, u16::from(town.x));
    require_eq!(hero.y, u16::from(town.y));
    require_eq!(hero.z, u16::from(town.z));
    test_creature(hero, 0, Creature::PIKEMAN, 8);
    test_creature(hero, 1, Creature::GRIFFIN, 2);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::GOG, 4);
    test_creature(hero, 4, Creature::IMP, 6);
    test_creature(hero, 5, Creature::PIKEMAN, 7);
    test_creature(hero, 6, Creature::EMPTY, 0);

    // LOOKOUT has no creatures in garrison since Edric is visiting.
    let town = find_town(&save, "LOOKOUT");
    require_eq!(town.owner, 1);
    test_creature(town, 0, Creature::EMPTY, 0);
    test_creature(town, 1, Creature::EMPTY, 0);
    test_creature(town, 2, Creature::EMPTY, 0);
    test_creature(town, 3, Creature::EMPTY, 0);
    test_creature(town, 4, Creature::EMPTY, 0);
    test_creature(town, 5, Creature::EMPTY, 0);
    test_creature(town, 6, Creature::EMPTY, 0);

    // Kyrre is visiting LOOKOUT.
    let hero = find_hero(&save, "Kyrre");
    require_eq!(hero.on_map, 0);
    require_eq!(hero.x, u16::from(town.x));
    require_eq!(hero.y, u16::from(town.y));
    require_eq!(hero.z, u16::from(town.z));
    test_creature(hero, 0, Creature::WOOD_ELF, 1);
    test_creature(hero, 1, Creature::CENTAUR, 8);
    test_creature(hero, 2, Creature::GREMLIN, 10);
    test_creature(hero, 3, Creature::STONE_GARGOYLE, 9);
    test_creature(hero, 4, Creature::CENTAUR, 8);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::WOOD_ELF, 1);
}

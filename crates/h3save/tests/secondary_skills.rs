// Ported from the homm3tools C++ test test-secondary-skills.cpp. Expected values are unchanged.

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

use h3save::skills::SKILL_SLOT_COUNT;

/// Slots not given are expected to be empty, as with a partly initialised `std::array` in C++.
fn test_skills(hero: &Hero, required: &[SkillSlot]) {
    let skills = hero.skills();
    let mut expected = [SkillSlot::default(); SKILL_SLOT_COUNT];
    expected[..required.len()].copy_from_slice(required);
    for i in 0..skills.len() {
        assert_eq!(skills[i], expected[i]);
    }
}

#[test]
fn secondary_skills_day1() {
    let save = open(&data_dir("secondary_skills").join("[hotseat] 111.GM5"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    let hero = find_hero(&save, "Edric");
    assert_eq!(hero.skill_count, 2);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::LEADERSHIP, SkillLevel::BASIC),
            SkillSlot::new(Skill::ARMORER, SkillLevel::BASIC),
            // Remaining skill slots are unused
        ],
    );

    let hero = find_hero(&save, "Axsis");
    assert_eq!(hero.skill_count, 2);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::WISDOM, SkillLevel::BASIC),
            SkillSlot::new(Skill::MYSTICISM, SkillLevel::BASIC),
            // Remaining skill slots are unused
        ],
    );

    let hero = find_hero(&save, "Leena");
    assert_eq!(hero.skill_count, 2);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::ESTATES, SkillLevel::BASIC),
            SkillSlot::new(Skill::PATHFINDING, SkillLevel::BASIC),
            // Remaining skill slots are unused
        ],
    );

    let hero = find_hero(&save, "Lacus");
    assert_eq!(hero.skill_count, 1);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::TACTICS, SkillLevel::ADVANCED),
            // Remaining skill slots are unused
        ],
    );

    let hero = find_hero(&save, "Septienna");
    assert_eq!(hero.skill_count, 2);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::NECROMANCY, SkillLevel::BASIC),
            SkillSlot::new(Skill::SCHOLAR, SkillLevel::BASIC),
            // Remaining skill slots are unused
        ],
    );
}

#[test]
fn secondary_skills_day2() {
    let save = open(&data_dir("secondary_skills").join("[hotseat] 112.GM5"));
    assert_date(&save, 1, 1, 2);
    assert!(save.valid());

    let hero = find_hero(&save, "Edric");
    assert_eq!(hero.skill_count, 5);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::LEADERSHIP, SkillLevel::BASIC),
            SkillSlot::new(Skill::ARMORER, SkillLevel::BASIC),
            SkillSlot::new(Skill::AIR_MAGIC, SkillLevel::BASIC),
            SkillSlot::new(Skill::ARCHERY, SkillLevel::BASIC),
            SkillSlot::new(Skill::ARTILLERY, SkillLevel::BASIC),
            // Remaining skill slots are unused
        ],
    );

    let hero = find_hero(&save, "Axsis");
    assert_eq!(hero.skill_count, 5);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::WISDOM, SkillLevel::BASIC),
            SkillSlot::new(Skill::MYSTICISM, SkillLevel::BASIC),
            SkillSlot::new(Skill::EAGLE_EYE, SkillLevel::BASIC),
            SkillSlot::new(Skill::EARTH_MAGIC, SkillLevel::BASIC),
            SkillSlot::new(Skill::ESTATES, SkillLevel::BASIC),
            // Remaining skill slots are unused
        ],
    );

    let hero = find_hero(&save, "Leena");
    assert_eq!(hero.skill_count, 5);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::ESTATES, SkillLevel::BASIC),
            SkillSlot::new(Skill::PATHFINDING, SkillLevel::BASIC),
            SkillSlot::new(Skill::INTERFERENCE, SkillLevel::BASIC),
            SkillSlot::new(Skill::LEADERSHIP, SkillLevel::BASIC),
            SkillSlot::new(Skill::LEARNING, SkillLevel::BASIC),
            // Remaining skill slots are unused
        ],
    );

    let hero = find_hero(&save, "Lacus");
    assert_eq!(hero.skill_count, 4);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::TACTICS, SkillLevel::ADVANCED),
            SkillSlot::new(Skill::NAVIGATION, SkillLevel::BASIC),
            SkillSlot::new(Skill::NECROMANCY, SkillLevel::BASIC),
            SkillSlot::new(Skill::OFFENSE, SkillLevel::BASIC),
            // Remaining skill slots are unused
        ],
    );

    let hero = find_hero(&save, "Septienna");
    assert_eq!(hero.skill_count, 5);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::NECROMANCY, SkillLevel::BASIC),
            SkillSlot::new(Skill::SCHOLAR, SkillLevel::BASIC),
            SkillSlot::new(Skill::SCOUTING, SkillLevel::BASIC),
            SkillSlot::new(Skill::SORCERY, SkillLevel::BASIC),
            SkillSlot::new(Skill::TACTICS, SkillLevel::BASIC),
            // Remaining skill slots are unused
        ],
    );
}

#[test]
fn secondary_skills_day3() {
    let save = open(&data_dir("secondary_skills").join("[hotseat] 113.GM5"));
    assert_date(&save, 1, 1, 3);
    assert!(save.valid());

    let hero = find_hero(&save, "Edric");
    assert_eq!(hero.skill_count, 8);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::LEADERSHIP, SkillLevel::EXPERT),
            SkillSlot::new(Skill::ARMORER, SkillLevel::ADVANCED),
            SkillSlot::new(Skill::AIR_MAGIC, SkillLevel::EXPERT),
            SkillSlot::new(Skill::ARCHERY, SkillLevel::EXPERT),
            SkillSlot::new(Skill::ARTILLERY, SkillLevel::EXPERT),
            SkillSlot::new(Skill::BALLISTICS, SkillLevel::EXPERT),
            SkillSlot::new(Skill::DIPLOMACY, SkillLevel::EXPERT),
            SkillSlot::new(Skill::OFFENSE, SkillLevel::EXPERT),
        ],
    );

    let hero = find_hero(&save, "Axsis");
    assert_eq!(hero.skill_count, 8);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::WISDOM, SkillLevel::EXPERT),
            SkillSlot::new(Skill::MYSTICISM, SkillLevel::ADVANCED),
            SkillSlot::new(Skill::EAGLE_EYE, SkillLevel::BASIC),
            SkillSlot::new(Skill::EARTH_MAGIC, SkillLevel::EXPERT),
            SkillSlot::new(Skill::ESTATES, SkillLevel::EXPERT),
            SkillSlot::new(Skill::FIRE_MAGIC, SkillLevel::EXPERT),
            SkillSlot::new(Skill::FIRST_AID, SkillLevel::EXPERT),
            SkillSlot::new(Skill::INTELLIGENCE, SkillLevel::EXPERT),
        ],
    );

    let hero = find_hero(&save, "Leena");
    assert_eq!(hero.skill_count, 8);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::ESTATES, SkillLevel::EXPERT),
            SkillSlot::new(Skill::PATHFINDING, SkillLevel::EXPERT),
            SkillSlot::new(Skill::INTERFERENCE, SkillLevel::EXPERT),
            SkillSlot::new(Skill::LEADERSHIP, SkillLevel::EXPERT),
            SkillSlot::new(Skill::LEARNING, SkillLevel::BASIC),
            SkillSlot::new(Skill::LOGISTICS, SkillLevel::EXPERT),
            SkillSlot::new(Skill::LUCK, SkillLevel::EXPERT),
            SkillSlot::new(Skill::MYSTICISM, SkillLevel::ADVANCED),
        ],
    );

    let hero = find_hero(&save, "Lacus");
    assert_eq!(hero.skill_count, 7);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::TACTICS, SkillLevel::EXPERT),
            SkillSlot::new(Skill::NAVIGATION, SkillLevel::EXPERT),
            SkillSlot::new(Skill::NECROMANCY, SkillLevel::EXPERT),
            SkillSlot::new(Skill::OFFENSE, SkillLevel::EXPERT),
            SkillSlot::new(Skill::PATHFINDING, SkillLevel::EXPERT),
            SkillSlot::new(Skill::SCHOLAR, SkillLevel::EXPERT),
            SkillSlot::new(Skill::ARCHERY, SkillLevel::ADVANCED),
            SkillSlot::new(Skill::NONE, SkillLevel::NONE),
        ],
    );

    let hero = find_hero(&save, "Septienna");
    assert_eq!(hero.skill_count, 8);
    test_skills(
        hero,
        &[
            SkillSlot::new(Skill::NECROMANCY, SkillLevel::BASIC),
            SkillSlot::new(Skill::SCHOLAR, SkillLevel::EXPERT),
            SkillSlot::new(Skill::SCOUTING, SkillLevel::EXPERT),
            SkillSlot::new(Skill::SORCERY, SkillLevel::EXPERT),
            SkillSlot::new(Skill::TACTICS, SkillLevel::ADVANCED),
            SkillSlot::new(Skill::WATER_MAGIC, SkillLevel::EXPERT),
            SkillSlot::new(Skill::WISDOM, SkillLevel::EXPERT),
            SkillSlot::new(Skill::LOGISTICS, SkillLevel::ADVANCED),
        ],
    );
}

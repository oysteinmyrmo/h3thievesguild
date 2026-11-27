// Saves from HotA 1.7, before Bulwark: 198 heroes, one skill fewer (no Runes) and so one byte
// less per hero record, and the date closer to the hero array.
//
// The two saves are the first and last day of a seven-player game on a medium map.

mod common;

use common::{Decompressed, assert_date, data_dir, find_hero, find_town, open};
use h3save::player::players;
use h3save::savefile::{Error, Format, find_heroes};
use h3save::{Color, Creature, Hero, MapSize, SaveFile, Skill, SkillLevel, SkillSlot, TownType};

fn first_day() -> SaveFile {
    open(&data_dir("hota_1_7/[hotseat] 111.GM7"))
}

fn last_day() -> SaveFile {
    open(&data_dir("hota_1_7/[hotseat] 317.GM4"))
}

/// A hero's learned skills in slot order.
fn learned(hero: &Hero) -> Vec<SkillSlot> {
    hero.skills().into_iter().filter(|s| !s.is_empty()).collect()
}

#[test]
fn the_saves_read_as_hota_1_7() {
    for (save, (month, week, day)) in [(first_day(), (1, 1, 1)), (last_day(), (3, 1, 7))] {
        assert_eq!(save.format, Format::HOTA_1_7);
        assert_eq!(save.hero_count, 198);
        assert_eq!(save.heroes.len(), 198);
        assert_eq!(save.map_size, MapSize::M);
        assert_date(&save, month, week, day);
    }
}

#[test]
fn the_towns_read() {
    let expected = [
        ("Kildare", TownType::CASTLE),
        ("Westland Pier", TownType::COVE),
        ("Daemon Gate", TownType::INFERNO),
        ("Backwater", TownType::FORTRESS),
        ("Scar", TownType::DUNGEON),
        ("Marishen", TownType::RAMPART),
        ("Drago Breach", TownType::STRONGHOLD),
    ];
    let first = first_day();
    let last = last_day();
    for (owner, (name, town_type)) in (0..).zip(expected) {
        assert_eq!(find_town(&first, name).town_type, town_type, "{name}");
        assert_eq!(find_town(&first, name).owner, owner, "{name} on the first day");
        // By the last day orange (4) has taken every town but green's (3).
        let last_owner = if owner == 3 { 3 } else { 4 };
        assert_eq!(find_town(&last, name).owner, last_owner, "{name} on the last day");
    }
    assert_eq!(first.towns.len(), 7);
    assert_eq!(last.towns.len(), 7);
}

#[test]
fn every_player_starts_with_one_town_and_their_heroes() {
    let save = first_day();
    let expected: [(Color, u32, &[&str]); 7] = [
        (Color::Red, 4310, &["Orrin", "Cuthbert", "Clavius"]),
        (Color::Blue, 7850, &["Derek", "Cassiopeia"]),
        (Color::Tan, 4250, &["Fiona", "Ayden", "Dury"]),
        (Color::Green, 5710, &["Rosic", "Tiva"]),
        (Color::Orange, 3000, &["Arlach", "Damacon"]),
        (Color::Purple, 2300, &["Uland", "Elleshar", "Giselle"]),
        (Color::Teal, 4000, &["Yog", "Krellion"]),
    ];
    let all = players(&save);
    let alive: Vec<_> = all.iter().filter(|p| p.alive()).collect();
    assert_eq!(alive.len(), 7);
    for (player, (color, gold, heroes)) in alive.into_iter().zip(expected) {
        assert_eq!(player.color(), color);
        assert_eq!(player.gold(), gold, "{color:?} gold");
        assert_eq!(player.towns().len(), 1, "{color:?} towns");
        let names: Vec<String> = player.heroes().iter().map(Hero::name).collect();
        assert_eq!(names, heroes, "{color:?} heroes");
    }
}

#[test]
fn orange_holds_six_towns_on_the_last_day() {
    let save = last_day();
    let all = players(&save);
    let alive: Vec<_> = all.iter().filter(|p| p.alive()).collect();
    assert_eq!(alive.len(), 2);
    let (green, orange) = (alive[0], alive[1]);
    assert_eq!(green.color(), Color::Green);
    assert_eq!(green.gold(), 24_900);
    assert_eq!(green.heroes(), []);
    assert_eq!(green.towns().len(), 1);
    assert_eq!(orange.color(), Color::Orange);
    assert_eq!(orange.gold(), 39_881);
    assert_eq!(orange.towns().len(), 6);
    let names: Vec<String> = orange.heroes().iter().map(Hero::name).collect();
    assert_eq!(
        names,
        [
            "Caitlin",
            "Aine",
            "Arlach",
            "Damacon",
            "Shakti",
            "Jaegar",
            "Malekith",
            "Geon",
            "Sephinroth",
            "Gerwulf",
            "Bertram"
        ]
    );
}

#[test]
fn a_player_with_only_an_empty_town_has_no_best_creature() {
    let save = last_day();
    let all = players(&save);
    let green = &all[Color::Green.index()];
    assert!(green.alive());
    assert!(green.towns().iter().all(|t| t.army_strength() == 0));
    assert_eq!(green.best_creature(), None);
    assert_eq!(green.best_creature_ai_value(), 0);
    let orange = &all[Color::Orange.index()];
    assert_eq!(orange.best_creature(), Some(Creature::BLACK_DRAGON));
    assert_eq!(orange.best_creature_ai_value(), 8721);
}

#[test]
fn orrin_has_his_starting_skills() {
    let save = first_day();
    let orrin = find_hero(&save, "Orrin");
    assert_eq!(orrin.player, 0);
    assert_eq!((orrin.level, orrin.experience), (1, 83));
    assert_eq!(
        (orrin.attack, orrin.defense, orrin.power, orrin.knowledge),
        (2, 2, 1, 1)
    );
    assert_eq!(
        learned(orrin),
        [
            SkillSlot::new(Skill::LEADERSHIP, SkillLevel::BASIC),
            SkillSlot::new(Skill::ARCHERY, SkillLevel::BASIC),
        ]
    );
}

#[test]
fn interference_the_last_skill_before_runes_reads() {
    // Interference is the last entry of the 1.7 skill tables, right before where 1.8 has Runes.
    let save = last_day();
    let arlach = find_hero(&save, "Arlach");
    assert_eq!(arlach.player, 4);
    assert_eq!((arlach.level, arlach.experience), (18, 64_760));
    assert_eq!(
        (arlach.attack, arlach.defense, arlach.power, arlach.knowledge),
        (19, 19, 6, 9)
    );
    assert_eq!(
        learned(arlach),
        [
            SkillSlot::new(Skill::OFFENSE, SkillLevel::EXPERT),
            SkillSlot::new(Skill::ARTILLERY, SkillLevel::EXPERT),
            SkillSlot::new(Skill::EARTH_MAGIC, SkillLevel::EXPERT),
            SkillSlot::new(Skill::BALLISTICS, SkillLevel::EXPERT),
            SkillSlot::new(Skill::WISDOM, SkillLevel::EXPERT),
            SkillSlot::new(Skill::INTERFERENCE, SkillLevel::EXPERT),
            SkillSlot::new(Skill::PATHFINDING, SkillLevel::BASIC),
        ]
    );
    let first = first_day();
    let giselle = find_hero(&first, "Giselle");
    assert_eq!(
        learned(giselle),
        [SkillSlot::new(Skill::INTERFERENCE, SkillLevel::ADVANCED)]
    );
}

#[test]
fn no_hero_has_runes() {
    let runes = usize::from(Skill::RUNES.0);
    for save in [first_day(), last_day()] {
        for hero in &save.heroes {
            assert_eq!(hero.skill_levels[runes], SkillLevel::NONE, "{}", hero.name());
            assert_eq!(hero.skill_slots[runes], 0, "{}", hero.name());
        }
    }
}

#[test]
fn skill_slots_and_levels_agree_for_every_hero() {
    // A shift in the skill slot table would put slots on unlearned skills or repeat them.
    for save in [first_day(), last_day()] {
        for hero in &save.heroes {
            let name = hero.name();
            let mut slots: Vec<u8> = Vec::new();
            for (level, &slot) in hero.skill_levels.iter().zip(&hero.skill_slots) {
                assert_eq!(*level == SkillLevel::NONE, slot == 0, "{name}");
                if slot != 0 {
                    slots.push(slot);
                }
            }
            slots.sort_unstable();
            let expected: Vec<u8> = (1..=u8::try_from(slots.len()).unwrap()).collect();
            assert_eq!(slots, expected, "{name}: slots");
        }
    }
}

#[test]
fn heroes_are_only_found_with_their_own_format() {
    let v17 = Decompressed::read(&data_dir("hota_1_7/[hotseat] 111.GM7"));
    let v18 = Decompressed::read(&data_dir("bulwark/test-bulwark.GM2"));
    assert_eq!(v17.format, Format::HOTA_1_7);
    assert_eq!(v18.format, Format::HOTA_1_8);
    assert!(matches!(
        find_heroes(&v17.data, v17.hero_count, Format::HOTA_1_8),
        Err(Error::HeroesNotFound)
    ));
    assert!(matches!(
        find_heroes(&v18.data, v18.hero_count, Format::HOTA_1_7),
        Err(Error::HeroesNotFound)
    ));
}

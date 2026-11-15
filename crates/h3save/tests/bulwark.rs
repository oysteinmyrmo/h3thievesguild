// Bulwark, the town added in HotA 1.8: its creatures, heroes, town names and the Runes skill.
//
// The save has two Bulwark heroes. Vehr (red) has every basic Bulwark creature and Oidana (blue)
// every upgraded one, each stack as many creatures as the creature's level. AI values are from
// https://heroes.thelazy.net/index.php/List_of_creatures, heroes and town names from
// https://heroes.thelazy.net/index.php/Bulwark.

mod common;

use std::collections::BTreeSet;

use common::{assert_creature, assert_date, data_dir, find_hero, open};
use h3save::player::players;
use h3save::town::TOWN_NAMES;
use h3save::{Creature, SaveFile, Skill, SkillLevel, SkillSlot, TownType, stats};

const BASIC: [Creature; 7] = [
    Creature::KOBOLD,
    Creature::MOUNTAIN_RAM,
    Creature::SNOW_ELF,
    Creature::YETI,
    Creature::SHAMAN,
    Creature::MAMMOTH,
    Creature::JOTUNN,
];

const UPGRADED: [Creature; 7] = [
    Creature::KOBOLD_FOREMAN,
    Creature::ARGALI,
    Creature::STEEL_ELF,
    Creature::YETI_RUNEMASTER,
    Creature::GREAT_SHAMAN,
    Creature::WAR_MAMMOTH,
    Creature::JOTUNN_WARLORD,
];

fn save() -> SaveFile {
    open(&data_dir("bulwark").join("test-bulwark.GM2"))
}

const BULWARK_TOWN_NAMES: [&str; 16] = [
    "Alfheim",
    "Anabar",
    "Borea",
    "Ellesmere",
    "Everwinter",
    "Halsmere",
    "Jotunngard",
    "Ketpall",
    "Meghion",
    "Stonefang",
    "Thulekh",
    "Tundara",
    "Varand",
    "White Mouth",
    "Windskalir",
    "Winterkill",
];

#[test]
fn the_save_reads() {
    let save = save();
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());
    assert_eq!(save.map_name, "SaveGameTester_Bulwark");
    assert_eq!(save.hero_count, 215);
}

#[test]
fn vehr_has_every_basic_creature_as_many_as_its_level() {
    let save = save();
    let hero = find_hero(&save, "Vehr");
    assert_eq!(hero.player, 0);
    for (slot, (creature, count)) in BASIC.into_iter().zip(1..=7).enumerate() {
        assert_creature(hero, slot, creature, count);
    }
    // Kobold 54 + 2 Mountain Rams 228 + 3 Snow Elves 370 + 4 Yetis 504 + 5 Shamans 685
    // + 6 Mammoths 1359 + 7 Jotunns 4180.
    assert_eq!(stats::hero_ai_value(hero), 44_475);
    // On the map with attack 4 and defense 3: 44475 * sqrt(1.20 * 1.15), rounded down.
    assert_eq!((hero.attack, hero.defense, hero.power, hero.knowledge), (4, 3, 2, 1));
    assert_eq!(stats::hero_army_strength(hero), 52_246);
}

#[test]
fn oidana_has_every_upgraded_creature_as_many_as_its_level() {
    let save = save();
    let hero = find_hero(&save, "Oidana");
    assert_eq!(hero.player, 1);
    // From the highest level down: 7 Jotunn Warlords first, 1 Kobold Foreman last.
    for (slot, (creature, count)) in UPGRADED.into_iter().rev().zip((1..=7).rev()).enumerate() {
        assert_creature(hero, slot, creature, count);
    }
    // 7 Jotunn Warlords 6694 + 6 War Mammoths 1601 + 5 Great Shamans 818 + 4 Yeti Runemasters 751
    // + 3 Steel Elves 526 + 2 Argalis 250 + 1 Kobold Foreman 84.
    assert_eq!(stats::hero_ai_value(hero), 65_720);
    // Attack 1 and defense 2: 65720 * sqrt(1.05 * 1.10), rounded down.
    assert_eq!((hero.attack, hero.defense, hero.power, hero.knowledge), (1, 2, 3, 4));
    assert_eq!(stats::hero_army_strength(hero), 70_629);
}

#[test]
fn both_heroes_have_runes() {
    let save = save();
    assert_eq!(Skill::RUNES.name(), Some("Runes"));
    for (name, level) in [("Vehr", SkillLevel::ADVANCED), ("Oidana", SkillLevel::EXPERT)] {
        let hero = find_hero(&save, name);
        assert_eq!(hero.skill_levels[usize::from(Skill::RUNES.0)], level);
        let skills = hero.skills();
        assert_eq!(skills[0], SkillSlot::new(Skill::RUNES, level));
        // Runes is the only skill.
        for slot in &skills[1..] {
            assert_eq!(*slot, SkillSlot::default());
        }
        let learned = hero.skill_levels.iter().filter(|l| **l != SkillLevel::NONE).count();
        assert_eq!(learned, 1);
    }
}

#[test]
fn kingdom_army_strength_counts_the_bulwark_creatures() {
    let save = save();
    let players = players(&save);
    // The towns have no garrisons, so each kingdom is its hero.
    assert_eq!(players[0].kingdom_army_strength(), 52_246);
    assert_eq!(players[1].kingdom_army_strength(), 70_629);
    assert_eq!(players[0].best_creature_ai_value(), 4180);
    assert_eq!(players[1].best_creature_ai_value(), 6694);
}

#[test]
fn the_bulwark_heroes_are_in_the_save() {
    let save = save();
    // Chieftains, then Elders, as on the wiki. They are the last 17 hero records.
    let chieftains = [
        "Dhuin", "Oidana", "Neia", "Eikthurn", "Creyle", "Spadum", "Kynr", "Ergon",
    ];
    let elders = [
        "Kriv", "Glacius", "Sial", "Dalton", "Biarma", "Haugir", "Akka", "Vehr", "Allora",
    ];
    let expected: BTreeSet<&str> = chieftains.into_iter().chain(elders).collect();
    let found: Vec<String> = save.heroes[198..].iter().map(h3save::Hero::name).collect();
    assert_eq!(found.iter().map(String::as_str).collect::<BTreeSet<_>>(), expected);
    assert_eq!(found.len(), 17);
    // The Bulwark heroes nobody has hired start with Kobolds, Mountain Rams and Snow Elves.
    let starting = [
        Creature::KOBOLD,
        Creature::MOUNTAIN_RAM,
        Creature::SNOW_ELF,
        Creature::EMPTY,
    ];
    for hero in save.heroes[198..].iter().filter(|h| h.player == 0xFF) {
        for creature in &hero.creatures {
            assert!(starting.contains(creature));
        }
    }
}

#[test]
fn every_town_is_a_bulwark_town_with_its_default_names_used_first() {
    let save = save();
    assert_eq!(save.towns.len(), 20);
    for town in &save.towns {
        assert_eq!(town.town_type, TownType::BULWARK);
        assert_eq!(town.town_type.name(), Some("Bulwark"));
    }
    // All 16 Bulwark names are used once. The map has 20 Bulwark towns, so the other 4 get
    // names of other town types.
    let names: BTreeSet<&str> = save.towns.iter().map(|t| t.name.as_str()).collect();
    assert_eq!(names.len(), 20);
    for name in BULWARK_TOWN_NAMES {
        assert!(names.contains(name));
    }
    // The town names list has every Bulwark name.
    for name in BULWARK_TOWN_NAMES {
        assert!(TOWN_NAMES.contains(&name));
    }
    let others: BTreeSet<&str> = names
        .iter()
        .copied()
        .filter(|n| !BULWARK_TOWN_NAMES.contains(n))
        .collect();
    assert_eq!(
        others,
        BTreeSet::from(["Bath'iere", "Blackquarter", "Vluchton", "Watergate"])
    );
    for name in others {
        assert!(TOWN_NAMES.contains(&name));
    }
    // Red owns 8 towns and blue 12, as placed on the map.
    let owned = |player: u8| save.towns.iter().filter(|t| t.owner == player).count();
    assert_eq!((owned(0), owned(1)), (8, 12));
}

#[test]
fn creature_names_and_ids() {
    let names = [
        "Kobold",
        "KoboldForeman",
        "MountainRam",
        "Argali",
        "SnowElf",
        "SteelElf",
        "Yeti",
        "YetiRunemaster",
        "Shaman",
        "GreatShaman",
        "Mammoth",
        "WarMammoth",
        "Jotunn",
        "JotunnWarlord",
    ];
    // IDs 186 to 199: each basic creature, then its upgrade.
    let creatures: Vec<Creature> = BASIC.into_iter().zip(UPGRADED).flat_map(|(b, u)| [b, u]).collect();
    for ((id, name), creature) in (186u32..).zip(names).zip(creatures) {
        assert_eq!(creature, Creature(id));
        assert!(creature.is_known());
        assert_eq!(creature.name(), Some(name));
    }
    assert!(!Creature(200).is_known());
}

#[test]
fn creature_ai_values() {
    let basic = [54, 228, 370, 504, 685, 1359, 4180];
    let upgraded = [84, 250, 526, 751, 818, 1601, 6694];
    for (creature, value) in BASIC.into_iter().zip(basic).chain(UPGRADED.into_iter().zip(upgraded)) {
        assert_eq!(creature.ai_value(), Some(value));
    }
}

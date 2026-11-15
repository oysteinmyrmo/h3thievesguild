// Factory, the town added in HotA 1.7: its creatures, heroes and town names.
//
// Factory has two level 7 creatures, so the save has three Factory heroes. Floribert (red) has
// every basic creature except the Dreadnought, Agar (blue) every upgraded one except the
// Juggernaut, and Morton (tan) 7 Dreadnoughts and 7 Juggernauts. Floribert's and Agar's stacks
// are as many creatures as the creature's level. AI values are from
// https://heroes.thelazy.net/index.php/List_of_creatures, heroes and town names from
// https://heroes.thelazy.net/index.php/Factory.

#[macro_use]
mod common;

use std::collections::BTreeSet;

use common::{data_dir, find_hero, open, test_creature, test_date};
use h3save::player::players;
use h3save::town::TOWN_NAMES;
use h3save::{Creature, Hero, SaveFile, TownType, stats};

/// The basic creatures of levels 1 to 7, without the Dreadnought.
const BASIC: [Creature; 7] = [
    Creature::HALFLING,
    Creature::MECHANIC,
    Creature::ARMADILLO,
    Creature::AUTOMATON,
    Creature::SANDWORM,
    Creature::GUNSLINGER,
    Creature::COUATL,
];

/// The upgraded creatures of levels 1 to 7, without the Juggernaut.
const UPGRADED: [Creature; 7] = [
    Creature::HALFLING_GRENADIER,
    Creature::ENGINEER,
    Creature::BELLWETHER_ARMADILLO,
    Creature::SENTINEL_AUTOMATON,
    Creature::OLGOI_KHORKHOI,
    Creature::BOUNTY_HUNTER,
    Creature::CRIMSON_COUATL,
];

const FACTORY_TOWN_NAMES: [&str; 16] = [
    "Arcadia",
    "Ardon",
    "Aurichalcum",
    "Burton",
    "Corakstone",
    "Dardentor",
    "Endurance",
    "Fort Rotwang",
    "Kergar",
    "Mount Copper",
    "New Dolere",
    "Prospero",
    "Ridder",
    "Salda",
    "Volta",
    "Vulcan",
];

fn save() -> SaveFile {
    open(&data_dir("factory").join("test-factory.GM3"))
}

#[test]
fn the_save_reads() {
    let _guard = common::Guard::new("factory-save");
    let save = save();
    test_date(&save, 1, 1, 1);
    require!(save.valid());
    require_eq!(save.map_name, "SaveGameTester_Factory");
    require_eq!(save.hero_count, 215);
}

#[test]
fn floribert_has_every_basic_creature_but_the_dreadnought() {
    let _guard = common::Guard::new("factory-floribert");
    let save = save();
    let hero = find_hero(&save, "Floribert");
    require_eq!(hero.player, 0);
    for (slot, (creature, count)) in BASIC.into_iter().zip(1..=7).enumerate() {
        test_creature(hero, slot, creature, count);
    }
    // Halfling 75 + 2 Mechanics 186 + 3 Armadillos 198 + 4 Automatons 669 + 5 Sandworms 991
    // + 6 Gunslingers 1351 + 7 Couatls 3574.
    require_eq!(stats::hero_ai_value(hero), 41_796);
    // On the map with attack 3 and defense 1: 41796 * sqrt(1.15 * 1.05), rounded down.
    require_eq!((hero.attack, hero.defense), (3, 1));
    require_eq!(stats::hero_army_strength(hero), 45_928);
}

#[test]
fn agar_has_every_upgraded_creature_but_the_juggernaut() {
    let _guard = common::Guard::new("factory-agar");
    let save = save();
    let hero = find_hero(&save, "Agar");
    require_eq!(hero.player, 1);
    for (slot, (creature, count)) in UPGRADED.into_iter().zip(1..=7).enumerate() {
        test_creature(hero, slot, creature, count);
    }
    // Halfling Grenadier 95 + 2 Engineers 278 + 3 Bellwether Armadillos 256
    // + 4 Sentinel Automatons 947 + 5 Olgoi-Khorkhois 1220 + 6 Bounty Hunters 1454
    // + 7 Crimson Couatls 5341.
    require_eq!(stats::hero_ai_value(hero), 57_418);
    // Attack 0 and defense 1: 57418 * sqrt(1.00 * 1.05), rounded down.
    require_eq!((hero.attack, hero.defense), (0, 1));
    require_eq!(stats::hero_army_strength(hero), 58_835);
}

#[test]
fn morton_has_dreadnoughts_and_juggernauts() {
    let _guard = common::Guard::new("factory-morton");
    let save = save();
    let hero = find_hero(&save, "Morton");
    require_eq!(hero.player, 2);
    test_creature(hero, 0, Creature::DREADNOUGHT, 7);
    test_creature(hero, 1, Creature::JUGGERNAUT, 7);
    for slot in 2..7 {
        test_creature(hero, slot, Creature::EMPTY, 0);
    }
    // 7 Dreadnoughts 3879 + 7 Juggernauts 6433.
    require_eq!(stats::hero_ai_value(hero), 72_184);
    // Attack 3 and defense 1: 72184 * sqrt(1.15 * 1.05), rounded down.
    require_eq!((hero.attack, hero.defense), (3, 1));
    require_eq!(stats::hero_army_strength(hero), 79_320);
}

#[test]
fn kingdom_army_strength_counts_the_factory_creatures() {
    let _guard = common::Guard::new("factory-kingdom");
    let save = save();
    let players = players(&save);
    // The towns have no garrisons, so each kingdom is its hero.
    require_eq!(players[0].kingdom_army_strength(), 45_928);
    require_eq!(players[1].kingdom_army_strength(), 58_835);
    require_eq!(players[2].kingdom_army_strength(), 79_320);
    require_eq!(players[0].best_creature_ai_value(), 3574);
    require_eq!(players[1].best_creature_ai_value(), 5341);
    require_eq!(players[2].best_creature_ai_value(), 6433);
}

#[test]
fn the_factory_heroes_are_in_the_save() {
    let _guard = common::Guard::new("factory-heroes");
    let save = save();
    // Mercenaries, then Artificers. They are hero records 179 to 197.
    let mercenaries = [
        "Henrietta",
        "Sam",
        "Tancred",
        "Melchior",
        "Floribert",
        "Wynona",
        "Dury",
        "Morton",
        "Tavin",
        "Murdoch",
    ];
    let artificers = [
        "Celestine",
        "Todd",
        "Agar",
        "Bertram",
        "Wrathmont",
        "Ziph",
        "Victoria",
        "Eanswythe",
        "Frederick",
    ];
    let expected: BTreeSet<&str> = mercenaries.into_iter().chain(artificers).collect();
    let factory = &save.heroes[179..198];
    let found: BTreeSet<String> = factory.iter().map(Hero::name).collect();
    require_eq!(found.iter().map(String::as_str).collect::<BTreeSet<_>>(), expected);
    require_eq!(factory.len(), 19);
    // The Factory heroes nobody has hired start with Halflings, Mechanics, Armadillos and
    // Automatons.
    let starting = [
        Creature::HALFLING,
        Creature::MECHANIC,
        Creature::ARMADILLO,
        Creature::AUTOMATON,
        Creature::EMPTY,
    ];
    for hero in factory.iter().filter(|h| h.player == 0xFF) {
        for creature in &hero.creatures {
            require!(starting.contains(creature));
        }
    }
}

#[test]
fn every_town_is_a_factory_town_with_a_factory_name() {
    let _guard = common::Guard::new("factory-towns");
    let save = save();
    require_eq!(save.towns.len(), 16);
    for town in &save.towns {
        require_eq!(town.town_type, TownType::FACTORY);
        require_eq!(town.town_type.name(), Some("Factory"));
    }
    // 16 Factory towns use all 16 Factory names once.
    let names: BTreeSet<&str> = save.towns.iter().map(|t| t.name.as_str()).collect();
    require_eq!(names, BTreeSet::from(FACTORY_TOWN_NAMES));
    for name in FACTORY_TOWN_NAMES {
        require!(TOWN_NAMES.contains(&name));
    }
    // Red and blue own 8 towns each, as placed on the map.
    let owned = |player: u8| save.towns.iter().filter(|t| t.owner == player).count();
    require_eq!((owned(0), owned(1), owned(2)), (8, 8, 0));
}

#[test]
fn town_names_are_16_per_town_type() {
    let _guard = common::Guard::new("factory-town-names");
    // Castle through Cove, Factory and Bulwark.
    require_eq!(TOWN_NAMES.len(), 12 * 16);
    // Names are unique, except that the game names both a Rampart and a Stronghold town
    // Strongglen, as the town name saves show.
    let repeated: BTreeSet<&str> = TOWN_NAMES
        .iter()
        .copied()
        .filter(|name| TOWN_NAMES.iter().filter(|n| *n == name).count() > 1)
        .collect();
    require_eq!(repeated, BTreeSet::from(["Strongglen"]));
    // Factory is town type 10, so its names are the 11th group of 16.
    require_eq!(
        TOWN_NAMES[usize::from(TownType::FACTORY.0) * 16..][..16],
        FACTORY_TOWN_NAMES
    );
}

#[test]
fn creature_names_and_ids() {
    let _guard = common::Guard::new("factory-creature-names");
    // The Halfling is the neutral creature 138; the other Factory creatures are 171 to 185.
    require_eq!(Creature::HALFLING, Creature(138));
    require_eq!(Creature::HALFLING.name(), Some("Halfling"));
    let names = [
        "HalflingGrenadier",
        "Mechanic",
        "Engineer",
        "Armadillo",
        "BellwetherArmadillo",
        "Automaton",
        "SentinelAutomaton",
        "Sandworm",
        "OlgoiKhorkhoi",
        "Gunslinger",
        "BountyHunter",
        "Couatl",
        "CrimsonCouatl",
        "Dreadnought",
        "Juggernaut",
    ];
    // After the Grenadier, each basic creature, then its upgrade.
    let mut creatures = vec![Creature::HALFLING_GRENADIER];
    creatures.extend(BASIC[1..].iter().zip(&UPGRADED[1..]).flat_map(|(&b, &u)| [b, u]));
    creatures.extend([Creature::DREADNOUGHT, Creature::JUGGERNAUT]);
    for ((id, name), creature) in (171u32..).zip(names).zip(creatures) {
        require_eq!(creature, Creature(id));
        require!(creature.is_known());
        require_eq!(creature.name(), Some(name));
    }
}

#[test]
fn creature_ai_values() {
    let _guard = common::Guard::new("factory-creature-ai-values");
    let basic = [75, 186, 198, 669, 991, 1351, 3574];
    let upgraded = [95, 278, 256, 947, 1220, 1454, 5341];
    for (creature, value) in BASIC.into_iter().zip(basic).chain(UPGRADED.into_iter().zip(upgraded)) {
        require_eq!(creature.ai_value(), Some(value));
    }
    require_eq!(Creature::DREADNOUGHT.ai_value(), Some(3879));
    require_eq!(Creature::JUGGERNAUT.ai_value(), Some(6433));
}

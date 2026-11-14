// Ported from the homm3tools C++ test test-creatures.cpp. Expected values are unchanged.

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
fn creatures_day1() {
    let _guard = common::Guard::new("creatures-day1");
    let save = open(data_dir("creatures").join("[hotseat] 111.GM8"));
    require!(save.valid());

    let hero = find_hero(&save, "Lord Haart");
    test_creature(hero, 0, Creature::PIKEMAN, 1);
    test_creature(hero, 1, Creature::ARCHER, 2);
    test_creature(hero, 2, Creature::GRIFFIN, 3);
    test_creature(hero, 3, Creature::SWORDSMAN, 4);
    test_creature(hero, 4, Creature::MONK, 5);
    test_creature(hero, 5, Creature::CAVALIER, 6);
    test_creature(hero, 6, Creature::ANGEL, 7);
    require_eq!(stats::hero_ai_value(hero), 52884);
    require_eq!(stats::hero_army_strength(hero), 58172);

    let hero = find_hero(&save, "Sanya");
    test_creature(hero, 0, Creature::HALBERDIER, 1);
    test_creature(hero, 1, Creature::MARKSMAN, 2);
    test_creature(hero, 2, Creature::ROYAL_GRIFFIN, 3);
    test_creature(hero, 3, Creature::CRUSADER, 4);
    test_creature(hero, 4, Creature::ZEALOT, 5);
    test_creature(hero, 5, Creature::CHAMPION, 6);
    test_creature(hero, 6, Creature::ARCH_ANGEL, 7);
    require_eq!(stats::hero_ai_value(hero), 81961);
    require_eq!(stats::hero_army_strength(hero), 83985);

    let hero = find_hero(&save, "Kyrre");
    test_creature(hero, 0, Creature::CENTAUR, 1);
    test_creature(hero, 1, Creature::DWARF, 2);
    test_creature(hero, 2, Creature::WOOD_ELF, 3);
    test_creature(hero, 3, Creature::PEGASUS, 4);
    test_creature(hero, 4, Creature::DENDROID_GUARD, 5);
    test_creature(hero, 5, Creature::UNICORN, 6);
    test_creature(hero, 6, Creature::GREEN_DRAGON, 7);
    require_eq!(stats::hero_ai_value(hero), 50675);
    require_eq!(stats::hero_army_strength(hero), 55684);

    let hero = find_hero(&save, "Elleshar");
    test_creature(hero, 0, Creature::CENTAUR_CAPTAIN, 1);
    test_creature(hero, 1, Creature::BATTLE_DWARF, 2);
    test_creature(hero, 2, Creature::GRAND_ELF, 3);
    test_creature(hero, 3, Creature::SILVER_PEGASUS, 4);
    test_creature(hero, 4, Creature::DENDROID_SOLDIER, 5);
    test_creature(hero, 5, Creature::WAR_UNICORN, 6);
    test_creature(hero, 6, Creature::GOLD_DRAGON, 7);
    require_eq!(stats::hero_ai_value(hero), 80163);
    require_eq!(stats::hero_army_strength(hero), 84075);

    let hero = find_hero(&save, "Piquedram");
    test_creature(hero, 0, Creature::GREMLIN, 1);
    test_creature(hero, 1, Creature::STONE_GARGOYLE, 2);
    test_creature(hero, 2, Creature::STONE_GOLEM, 3);
    test_creature(hero, 3, Creature::MAGE, 4);
    test_creature(hero, 4, Creature::GENIE, 5);
    test_creature(hero, 5, Creature::NAGA, 6);
    test_creature(hero, 6, Creature::GIANT, 7);
    require_eq!(stats::hero_ai_value(hero), 45946);
    require_eq!(stats::hero_army_strength(hero), 48243);

    let hero = find_hero(&save, "Halon");
    test_creature(hero, 0, Creature::MASTER_GREMLIN, 1);
    test_creature(hero, 1, Creature::OBSIDIAN_GARGOYLE, 2);
    test_creature(hero, 2, Creature::IRON_GOLEM, 3);
    test_creature(hero, 3, Creature::ARCH_MAGE, 4);
    test_creature(hero, 4, Creature::MASTER_GENIE, 5);
    test_creature(hero, 5, Creature::NAGA_QUEEN, 6);
    test_creature(hero, 6, Creature::TITAN, 7);
    require_eq!(stats::hero_ai_value(hero), 78674);
    require_eq!(stats::hero_army_strength(hero), 78674);

    let hero = find_hero(&save, "Pyre");
    test_creature(hero, 0, Creature::IMP, 1);
    test_creature(hero, 1, Creature::GOG, 2);
    test_creature(hero, 2, Creature::HELL_HOUND, 3);
    test_creature(hero, 3, Creature::DEMON, 4);
    test_creature(hero, 4, Creature::PIT_FIEND, 5);
    test_creature(hero, 5, Creature::EFREETI, 6);
    test_creature(hero, 6, Creature::DEVIL, 7);
    require_eq!(stats::hero_ai_value(hero), 52771);
    require_eq!(stats::hero_army_strength(hero), 58048);

    let hero = find_hero(&save, "Calid");
    test_creature(hero, 0, Creature::FAMILIAR, 1);
    test_creature(hero, 1, Creature::MAGOG, 2);
    test_creature(hero, 2, Creature::CERBERUS, 3);
    test_creature(hero, 3, Creature::HORNED_DEMON, 4);
    test_creature(hero, 4, Creature::PIT_LORD, 5);
    test_creature(hero, 5, Creature::EFREET_SULTAN, 6);
    test_creature(hero, 6, Creature::ARCH_DEVIL, 7);
    require_eq!(stats::hero_ai_value(hero), 73619);
    require_eq!(stats::hero_army_strength(hero), 77299);

    let hero = find_hero(&save, "Isra");
    test_creature(hero, 0, Creature::SKELETON, 1);
    test_creature(hero, 1, Creature::WALKING_DEAD, 2);
    test_creature(hero, 2, Creature::WIGHT, 3);
    test_creature(hero, 3, Creature::VAMPIRE, 4);
    test_creature(hero, 4, Creature::LICH, 5);
    test_creature(hero, 5, Creature::BLACK_KNIGHT, 6);
    test_creature(hero, 6, Creature::BONE_DRAGON, 7);
    require_eq!(stats::hero_ai_value(hero), 43710);
    require_eq!(stats::hero_army_strength(hero), 46975);

    let hero = find_hero(&save, "Vidomina");
    test_creature(hero, 0, Creature::SKELETON_WARRIOR, 1);
    test_creature(hero, 1, Creature::ZOMBIE, 2);
    test_creature(hero, 2, Creature::WRAITH, 3);
    test_creature(hero, 3, Creature::VAMPIRE_LORD, 4);
    test_creature(hero, 4, Creature::POWER_LICH, 5);
    test_creature(hero, 5, Creature::DREAD_KNIGHT, 6);
    test_creature(hero, 6, Creature::GHOST_DRAGON, 7);
    require_eq!(stats::hero_ai_value(hero), 56977);
    require_eq!(stats::hero_army_strength(hero), 58384);

    let hero = find_hero(&save, "Dace");
    test_creature(hero, 0, Creature::TROGLODYTE, 1);
    test_creature(hero, 1, Creature::HARPY, 2);
    test_creature(hero, 2, Creature::BEHOLDER, 3);
    test_creature(hero, 3, Creature::MEDUSA, 4);
    test_creature(hero, 4, Creature::MINOTAUR, 5);
    test_creature(hero, 5, Creature::MANTICORE, 6);
    test_creature(hero, 6, Creature::RED_DRAGON, 7);
    require_eq!(stats::hero_ai_value(hero), 49814);
    require_eq!(stats::hero_army_strength(hero), 54795);

    let hero = find_hero(&save, "Deemer");
    test_creature(hero, 0, Creature::INFERNAL_TROGLODYTE, 1);
    test_creature(hero, 1, Creature::HARPY_HAG, 2);
    test_creature(hero, 2, Creature::EVIL_EYE, 3);
    test_creature(hero, 3, Creature::MEDUSA_QUEEN, 4);
    test_creature(hero, 4, Creature::MINOTAUR_KING, 5);
    test_creature(hero, 5, Creature::SCORPICORE, 6);
    test_creature(hero, 6, Creature::BLACK_DRAGON, 7);
    require_eq!(stats::hero_ai_value(hero), 79890);
    require_eq!(stats::hero_army_strength(hero), 79890);

    let hero = find_hero(&save, "Yog");
    test_creature(hero, 0, Creature::GOBLIN, 1);
    test_creature(hero, 1, Creature::WOLF_RIDER, 2);
    test_creature(hero, 2, Creature::ORC, 3);
    test_creature(hero, 3, Creature::OGRE, 4);
    test_creature(hero, 4, Creature::ROC, 5);
    test_creature(hero, 5, Creature::CYCLOPS, 6);
    test_creature(hero, 6, Creature::BEHEMOTH, 7);
    require_eq!(stats::hero_ai_value(hero), 37425);
    require_eq!(stats::hero_army_strength(hero), 40997);

    let hero = find_hero(&save, "Saurug");
    test_creature(hero, 0, Creature::HOBGOBLIN, 1);
    test_creature(hero, 1, Creature::WOLF_RAIDER, 2);
    test_creature(hero, 2, Creature::ORC_CHIEFTAIN, 3);
    test_creature(hero, 3, Creature::OGRE_MAGE, 4);
    test_creature(hero, 4, Creature::THUNDERBIRD, 5);
    test_creature(hero, 5, Creature::CYCLOPS_KING, 6);
    test_creature(hero, 6, Creature::ANCIENT_BEHEMOTH, 7);
    require_eq!(stats::hero_ai_value(hero), 61256);
    require_eq!(stats::hero_army_strength(hero), 65832);

    let hero = find_hero(&save, "Gerwulf");
    test_creature(hero, 0, Creature::GNOLL, 1);
    test_creature(hero, 1, Creature::LIZARDMAN, 2);
    test_creature(hero, 2, Creature::SERPENT_FLY, 3);
    test_creature(hero, 3, Creature::BASILISK, 4);
    test_creature(hero, 4, Creature::GORGON, 5);
    test_creature(hero, 5, Creature::WYVERN, 6);
    test_creature(hero, 6, Creature::HYDRA, 7);
    require_eq!(stats::hero_ai_value(hero), 44710);
    require_eq!(stats::hero_army_strength(hero), 48977);

    let hero = find_hero(&save, "Merist");
    test_creature(hero, 0, Creature::GNOLL_MARAUDER, 1);
    test_creature(hero, 1, Creature::LIZARD_WARRIOR, 2);
    test_creature(hero, 2, Creature::DRAGON_FLY, 3);
    test_creature(hero, 3, Creature::GREATER_BASILISK, 4);
    test_creature(hero, 4, Creature::MIGHTY_GORGON, 5);
    test_creature(hero, 5, Creature::WYVERN_MONARCH, 6);
    test_creature(hero, 6, Creature::CHAOS_HYDRA, 7);
    require_eq!(stats::hero_ai_value(hero), 59959);
    require_eq!(stats::hero_army_strength(hero), 61439);

    let hero = find_hero(&save, "Thunar");
    test_creature(hero, 0, Creature::PIXIE, 1);
    test_creature(hero, 1, Creature::AIR_ELEMENTAL, 2);
    test_creature(hero, 2, Creature::WATER_ELEMENTAL, 3);
    test_creature(hero, 3, Creature::FIRE_ELEMENTAL, 4);
    test_creature(hero, 4, Creature::EARTH_ELEMENTAL, 5);
    test_creature(hero, 5, Creature::PSYCHIC_ELEMENTAL, 6);
    test_creature(hero, 6, Creature::FIREBIRD, 7);
    require_eq!(stats::hero_ai_value(hero), 45108);
    require_eq!(stats::hero_army_strength(hero), 49567);

    let hero = find_hero(&save, "Inteus");
    test_creature(hero, 0, Creature::SPRITE, 1);
    test_creature(hero, 1, Creature::STORM_ELEMENTAL, 2);
    test_creature(hero, 2, Creature::ICE_ELEMENTAL, 3);
    test_creature(hero, 3, Creature::ENERGY_ELEMENTAL, 4);
    test_creature(hero, 4, Creature::MAGMA_ELEMENTAL, 5);
    test_creature(hero, 5, Creature::MAGIC_ELEMENTAL, 6);
    test_creature(hero, 6, Creature::PHOENIX, 7);
    require_eq!(stats::hero_ai_value(hero), 65656);
    require_eq!(stats::hero_army_strength(hero), 65656);

    let hero = find_hero(&save, "Jeremy");
    test_creature(hero, 0, Creature::NYMPH, 1);
    test_creature(hero, 1, Creature::CREW_MATE, 2);
    test_creature(hero, 2, Creature::PIRATE, 3);
    test_creature(hero, 3, Creature::STORMBIRD, 4);
    test_creature(hero, 4, Creature::SEA_WITCH, 5);
    test_creature(hero, 5, Creature::NIX, 6);
    test_creature(hero, 6, Creature::SEA_SERPENT, 7);
    require_eq!(stats::hero_ai_value(hero), 43422);
    require_eq!(stats::hero_army_strength(hero), 46564);

    let hero = find_hero(&save, "Astra");
    test_creature(hero, 0, Creature::OCEANID, 1);
    test_creature(hero, 1, Creature::SEAMAN, 2);
    test_creature(hero, 2, Creature::CORSAIR, 3);
    test_creature(hero, 3, Creature::AYSSID, 4);
    test_creature(hero, 4, Creature::SORCERESS, 5);
    test_creature(hero, 5, Creature::NIX_WARRIOR, 6);
    test_creature(hero, 6, Creature::HASPID, 7);
    require_eq!(stats::hero_ai_value(hero), 71720);
    require_eq!(stats::hero_army_strength(hero), 75220);

    let hero = find_hero(&save, "Dargem");
    test_creature(hero, 0, Creature::SEA_DOG, 1234);
    test_creature(hero, 1, Creature::EMPTY, 0);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(stats::hero_ai_value(hero), 742868);
    require_eq!(stats::hero_army_strength(hero), 779126);

    let hero = find_hero(&save, "Corkes");
    test_creature(hero, 0, Creature::HALFLING, 1);
    test_creature(hero, 1, Creature::PEASANT, 2);
    test_creature(hero, 2, Creature::BOAR, 3);
    test_creature(hero, 3, Creature::ROGUE, 4);
    test_creature(hero, 4, Creature::LEPRECHAUN, 5);
    test_creature(hero, 5, Creature::MUMMY, 6);
    test_creature(hero, 6, Creature::NOMAD, 7);
    require_eq!(stats::hero_ai_value(hero), 6155);
    require_eq!(stats::hero_army_strength(hero), 6600);

    let hero = find_hero(&save, "Zilare");
    test_creature(hero, 0, Creature::SHARPSHOOTER, 1);
    test_creature(hero, 1, Creature::SATYR, 2);
    test_creature(hero, 2, Creature::STEEL_GOLEM, 3);
    test_creature(hero, 3, Creature::GOLD_GOLEM, 4);
    test_creature(hero, 4, Creature::TROLL, 5);
    test_creature(hero, 5, Creature::FANGARM, 6);
    test_creature(hero, 6, Creature::DIAMOND_GOLEM, 7);
    require_eq!(stats::hero_ai_value(hero), 21931);
    require_eq!(stats::hero_army_strength(hero), 23001);

    let hero = find_hero(&save, "Spint");
    test_creature(hero, 0, Creature::ENCHANTER, 1);
    test_creature(hero, 1, Creature::AZURE_DRAGON, 2);
    test_creature(hero, 2, Creature::CRYSTAL_DRAGON, 3);
    test_creature(hero, 3, Creature::FAERIE_DRAGON, 4);
    test_creature(hero, 4, Creature::RUST_DRAGON, 5);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(stats::hero_ai_value(hero), 531083);
    require_eq!(stats::hero_army_strength(hero), 557004);
}

#[test]
fn creatures_day2() {
    let _guard = common::Guard::new("creatures-day2");
    let save = open(data_dir("creatures").join("[hotseat] 112.GM8"));
    require!(save.valid());

    let hero = find_hero(&save, "Lord Haart");
    test_creature(hero, 0, Creature::ANGEL, 7);
    test_creature(hero, 1, Creature::CAVALIER, 6);
    test_creature(hero, 2, Creature::MONK, 5);
    test_creature(hero, 3, Creature::SWORDSMAN, 4);
    test_creature(hero, 4, Creature::GRIFFIN, 3);
    test_creature(hero, 5, Creature::ARCHER, 2);
    test_creature(hero, 6, Creature::PIKEMAN, 1);
    require_eq!(stats::hero_ai_value(hero), 52884);
    require_eq!(stats::hero_army_strength(hero), 58172);

    let hero = find_hero(&save, "Sanya");
    test_creature(hero, 0, Creature::ARCH_ANGEL, 7);
    test_creature(hero, 1, Creature::CHAMPION, 6);
    test_creature(hero, 2, Creature::ZEALOT, 5);
    test_creature(hero, 3, Creature::CRUSADER, 4);
    test_creature(hero, 4, Creature::ROYAL_GRIFFIN, 3);
    test_creature(hero, 5, Creature::MARKSMAN, 2);
    test_creature(hero, 6, Creature::HALBERDIER, 1);
    require_eq!(stats::hero_ai_value(hero), 81961);
    require_eq!(stats::hero_army_strength(hero), 83985);

    let hero = find_hero(&save, "Kyrre");
    test_creature(hero, 0, Creature::GREEN_DRAGON, 7);
    test_creature(hero, 1, Creature::UNICORN, 6);
    test_creature(hero, 2, Creature::DENDROID_GUARD, 5);
    test_creature(hero, 3, Creature::PEGASUS, 4);
    test_creature(hero, 4, Creature::WOOD_ELF, 3);
    test_creature(hero, 5, Creature::DWARF, 2);
    test_creature(hero, 6, Creature::CENTAUR, 1);
    require_eq!(stats::hero_ai_value(hero), 50675);
    require_eq!(stats::hero_army_strength(hero), 55684);

    let hero = find_hero(&save, "Elleshar");
    test_creature(hero, 0, Creature::GOLD_DRAGON, 7);
    test_creature(hero, 1, Creature::WAR_UNICORN, 6);
    test_creature(hero, 2, Creature::DENDROID_SOLDIER, 5);
    test_creature(hero, 3, Creature::SILVER_PEGASUS, 4);
    test_creature(hero, 4, Creature::GRAND_ELF, 3);
    test_creature(hero, 5, Creature::BATTLE_DWARF, 2);
    test_creature(hero, 6, Creature::CENTAUR_CAPTAIN, 1);
    require_eq!(stats::hero_ai_value(hero), 80163);
    require_eq!(stats::hero_army_strength(hero), 84075);

    let hero = find_hero(&save, "Piquedram");
    test_creature(hero, 0, Creature::GIANT, 7);
    test_creature(hero, 1, Creature::NAGA, 6);
    test_creature(hero, 2, Creature::GENIE, 5);
    test_creature(hero, 3, Creature::MAGE, 4);
    test_creature(hero, 4, Creature::STONE_GOLEM, 3);
    test_creature(hero, 5, Creature::STONE_GARGOYLE, 2);
    test_creature(hero, 6, Creature::GREMLIN, 1);
    require_eq!(stats::hero_ai_value(hero), 45946);
    require_eq!(stats::hero_army_strength(hero), 48243);

    let hero = find_hero(&save, "Halon");
    test_creature(hero, 0, Creature::TITAN, 7);
    test_creature(hero, 1, Creature::NAGA_QUEEN, 6);
    test_creature(hero, 2, Creature::MASTER_GENIE, 5);
    test_creature(hero, 3, Creature::ARCH_MAGE, 4);
    test_creature(hero, 4, Creature::IRON_GOLEM, 3);
    test_creature(hero, 5, Creature::OBSIDIAN_GARGOYLE, 2);
    test_creature(hero, 6, Creature::MASTER_GREMLIN, 1);
    require_eq!(stats::hero_ai_value(hero), 78674);
    require_eq!(stats::hero_army_strength(hero), 78674);

    let hero = find_hero(&save, "Pyre");
    test_creature(hero, 0, Creature::DEVIL, 7);
    test_creature(hero, 1, Creature::EFREETI, 6);
    test_creature(hero, 2, Creature::PIT_FIEND, 5);
    test_creature(hero, 3, Creature::DEMON, 4);
    test_creature(hero, 4, Creature::HELL_HOUND, 3);
    test_creature(hero, 5, Creature::GOG, 2);
    test_creature(hero, 6, Creature::IMP, 1);
    require_eq!(stats::hero_ai_value(hero), 52771);
    require_eq!(stats::hero_army_strength(hero), 58048);

    let hero = find_hero(&save, "Calid");
    test_creature(hero, 0, Creature::ARCH_DEVIL, 7);
    test_creature(hero, 1, Creature::EFREET_SULTAN, 6);
    test_creature(hero, 2, Creature::PIT_LORD, 5);
    test_creature(hero, 3, Creature::HORNED_DEMON, 4);
    test_creature(hero, 4, Creature::CERBERUS, 3);
    test_creature(hero, 5, Creature::MAGOG, 2);
    test_creature(hero, 6, Creature::FAMILIAR, 1);
    require_eq!(stats::hero_ai_value(hero), 73619);
    require_eq!(stats::hero_army_strength(hero), 77299);

    let hero = find_hero(&save, "Isra");
    test_creature(hero, 0, Creature::BONE_DRAGON, 7);
    test_creature(hero, 1, Creature::BLACK_KNIGHT, 6);
    test_creature(hero, 2, Creature::LICH, 5);
    test_creature(hero, 3, Creature::VAMPIRE, 4);
    test_creature(hero, 4, Creature::WIGHT, 3);
    test_creature(hero, 5, Creature::WALKING_DEAD, 2);
    test_creature(hero, 6, Creature::SKELETON, 1);
    require_eq!(stats::hero_ai_value(hero), 43710);
    require_eq!(stats::hero_army_strength(hero), 46975);

    let hero = find_hero(&save, "Vidomina");
    test_creature(hero, 0, Creature::GHOST_DRAGON, 7);
    test_creature(hero, 1, Creature::DREAD_KNIGHT, 6);
    test_creature(hero, 2, Creature::POWER_LICH, 5);
    test_creature(hero, 3, Creature::VAMPIRE_LORD, 4);
    test_creature(hero, 4, Creature::WRAITH, 3);
    test_creature(hero, 5, Creature::ZOMBIE, 2);
    test_creature(hero, 6, Creature::SKELETON_WARRIOR, 1);
    require_eq!(stats::hero_ai_value(hero), 56977);
    require_eq!(stats::hero_army_strength(hero), 58384);

    let hero = find_hero(&save, "Dace");
    test_creature(hero, 0, Creature::RED_DRAGON, 7);
    test_creature(hero, 1, Creature::MANTICORE, 6);
    test_creature(hero, 2, Creature::MINOTAUR, 5);
    test_creature(hero, 3, Creature::MEDUSA, 4);
    test_creature(hero, 4, Creature::BEHOLDER, 3);
    test_creature(hero, 5, Creature::HARPY, 2);
    test_creature(hero, 6, Creature::TROGLODYTE, 1);
    require_eq!(stats::hero_ai_value(hero), 49814);
    require_eq!(stats::hero_army_strength(hero), 54795);

    let hero = find_hero(&save, "Deemer");
    test_creature(hero, 0, Creature::BLACK_DRAGON, 7);
    test_creature(hero, 1, Creature::SCORPICORE, 6);
    test_creature(hero, 2, Creature::MINOTAUR_KING, 5);
    test_creature(hero, 3, Creature::MEDUSA_QUEEN, 4);
    test_creature(hero, 4, Creature::EVIL_EYE, 3);
    test_creature(hero, 5, Creature::HARPY_HAG, 2);
    test_creature(hero, 6, Creature::INFERNAL_TROGLODYTE, 1);
    require_eq!(stats::hero_ai_value(hero), 79890);
    require_eq!(stats::hero_army_strength(hero), 79890);

    let hero = find_hero(&save, "Yog");
    test_creature(hero, 0, Creature::BEHEMOTH, 7);
    test_creature(hero, 1, Creature::CYCLOPS, 6);
    test_creature(hero, 2, Creature::ROC, 5);
    test_creature(hero, 3, Creature::OGRE, 4);
    test_creature(hero, 4, Creature::ORC, 3);
    test_creature(hero, 5, Creature::WOLF_RIDER, 2);
    test_creature(hero, 6, Creature::GOBLIN, 1);
    require_eq!(stats::hero_ai_value(hero), 37425);
    require_eq!(stats::hero_army_strength(hero), 40997);

    let hero = find_hero(&save, "Saurug");
    test_creature(hero, 0, Creature::ANCIENT_BEHEMOTH, 7);
    test_creature(hero, 1, Creature::CYCLOPS_KING, 6);
    test_creature(hero, 2, Creature::THUNDERBIRD, 5);
    test_creature(hero, 3, Creature::OGRE_MAGE, 4);
    test_creature(hero, 4, Creature::ORC_CHIEFTAIN, 3);
    test_creature(hero, 5, Creature::WOLF_RAIDER, 2);
    test_creature(hero, 6, Creature::HOBGOBLIN, 1);
    require_eq!(stats::hero_ai_value(hero), 61256);
    require_eq!(stats::hero_army_strength(hero), 65832);

    let hero = find_hero(&save, "Gerwulf");
    test_creature(hero, 0, Creature::HYDRA, 7);
    test_creature(hero, 1, Creature::WYVERN, 6);
    test_creature(hero, 2, Creature::GORGON, 5);
    test_creature(hero, 3, Creature::BASILISK, 4);
    test_creature(hero, 4, Creature::SERPENT_FLY, 3);
    test_creature(hero, 5, Creature::LIZARDMAN, 2);
    test_creature(hero, 6, Creature::GNOLL, 1);
    require_eq!(stats::hero_ai_value(hero), 44710);
    require_eq!(stats::hero_army_strength(hero), 48977);

    let hero = find_hero(&save, "Merist");
    test_creature(hero, 0, Creature::CHAOS_HYDRA, 7);
    test_creature(hero, 1, Creature::WYVERN_MONARCH, 6);
    test_creature(hero, 2, Creature::MIGHTY_GORGON, 5);
    test_creature(hero, 3, Creature::GREATER_BASILISK, 4);
    test_creature(hero, 4, Creature::DRAGON_FLY, 3);
    test_creature(hero, 5, Creature::LIZARD_WARRIOR, 2);
    test_creature(hero, 6, Creature::GNOLL_MARAUDER, 1);
    require_eq!(stats::hero_ai_value(hero), 59959);
    require_eq!(stats::hero_army_strength(hero), 61439);

    let hero = find_hero(&save, "Thunar");
    test_creature(hero, 0, Creature::FIREBIRD, 7);
    test_creature(hero, 1, Creature::PSYCHIC_ELEMENTAL, 6);
    test_creature(hero, 2, Creature::EARTH_ELEMENTAL, 5);
    test_creature(hero, 3, Creature::FIRE_ELEMENTAL, 4);
    test_creature(hero, 4, Creature::WATER_ELEMENTAL, 3);
    test_creature(hero, 5, Creature::AIR_ELEMENTAL, 2);
    test_creature(hero, 6, Creature::PIXIE, 1);
    require_eq!(stats::hero_ai_value(hero), 45108);
    require_eq!(stats::hero_army_strength(hero), 49567);

    let hero = find_hero(&save, "Inteus");
    test_creature(hero, 0, Creature::PHOENIX, 7);
    test_creature(hero, 1, Creature::MAGIC_ELEMENTAL, 6);
    test_creature(hero, 2, Creature::MAGMA_ELEMENTAL, 5);
    test_creature(hero, 3, Creature::ENERGY_ELEMENTAL, 4);
    test_creature(hero, 4, Creature::ICE_ELEMENTAL, 3);
    test_creature(hero, 5, Creature::STORM_ELEMENTAL, 2);
    test_creature(hero, 6, Creature::SPRITE, 1);
    require_eq!(stats::hero_ai_value(hero), 65656);
    require_eq!(stats::hero_army_strength(hero), 65656);

    let hero = find_hero(&save, "Jeremy");
    test_creature(hero, 0, Creature::SEA_SERPENT, 7);
    test_creature(hero, 1, Creature::NIX, 6);
    test_creature(hero, 2, Creature::SEA_WITCH, 5);
    test_creature(hero, 3, Creature::STORMBIRD, 4);
    test_creature(hero, 4, Creature::PIRATE, 3);
    test_creature(hero, 5, Creature::CREW_MATE, 2);
    test_creature(hero, 6, Creature::NYMPH, 1);
    require_eq!(stats::hero_ai_value(hero), 43422);
    require_eq!(stats::hero_army_strength(hero), 46564);

    let hero = find_hero(&save, "Astra");
    test_creature(hero, 0, Creature::HASPID, 7);
    test_creature(hero, 1, Creature::NIX_WARRIOR, 6);
    test_creature(hero, 2, Creature::SORCERESS, 5);
    test_creature(hero, 3, Creature::AYSSID, 4);
    test_creature(hero, 4, Creature::CORSAIR, 3);
    test_creature(hero, 5, Creature::SEAMAN, 2);
    test_creature(hero, 6, Creature::OCEANID, 1);
    require_eq!(stats::hero_ai_value(hero), 71720);
    require_eq!(stats::hero_army_strength(hero), 75220);

    let hero = find_hero(&save, "Dargem");
    test_creature(hero, 0, Creature::SEA_DOG, 617);
    test_creature(hero, 1, Creature::SEA_DOG, 617);
    test_creature(hero, 2, Creature::EMPTY, 0);
    test_creature(hero, 3, Creature::EMPTY, 0);
    test_creature(hero, 4, Creature::EMPTY, 0);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(stats::hero_ai_value(hero), 742868);
    require_eq!(stats::hero_army_strength(hero), 779126);

    let hero = find_hero(&save, "Corkes");
    test_creature(hero, 0, Creature::NOMAD, 7);
    test_creature(hero, 1, Creature::MUMMY, 6);
    test_creature(hero, 2, Creature::LEPRECHAUN, 5);
    test_creature(hero, 3, Creature::ROGUE, 4);
    test_creature(hero, 4, Creature::BOAR, 3);
    test_creature(hero, 5, Creature::PEASANT, 2);
    test_creature(hero, 6, Creature::HALFLING, 1);
    require_eq!(stats::hero_ai_value(hero), 6155);
    require_eq!(stats::hero_army_strength(hero), 6600);

    let hero = find_hero(&save, "Zilare");
    test_creature(hero, 0, Creature::DIAMOND_GOLEM, 7);
    test_creature(hero, 1, Creature::FANGARM, 6);
    test_creature(hero, 2, Creature::TROLL, 5);
    test_creature(hero, 3, Creature::GOLD_GOLEM, 4);
    test_creature(hero, 4, Creature::STEEL_GOLEM, 3);
    test_creature(hero, 5, Creature::SATYR, 2);
    test_creature(hero, 6, Creature::SHARPSHOOTER, 1);
    require_eq!(stats::hero_ai_value(hero), 21931);
    require_eq!(stats::hero_army_strength(hero), 23001);

    let hero = find_hero(&save, "Spint");
    test_creature(hero, 0, Creature::RUST_DRAGON, 5);
    test_creature(hero, 1, Creature::FAERIE_DRAGON, 4);
    test_creature(hero, 2, Creature::CRYSTAL_DRAGON, 3);
    test_creature(hero, 3, Creature::AZURE_DRAGON, 2);
    test_creature(hero, 4, Creature::ENCHANTER, 1);
    test_creature(hero, 5, Creature::EMPTY, 0);
    test_creature(hero, 6, Creature::EMPTY, 0);
    require_eq!(stats::hero_ai_value(hero), 531083);
    require_eq!(stats::hero_army_strength(hero), 557004);
}

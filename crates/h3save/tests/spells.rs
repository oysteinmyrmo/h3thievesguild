// Ported from the homm3tools C++ test test-spells.cpp. Expected values are unchanged.

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

use h3save::spells::SPELL_COUNT;

fn contains(spells: &[Spell], arg: Spell) -> bool {
    spells.contains(&arg)
}

/// Verify spells in spell book (not artifacts).
fn verify_spell_book(hero: &Hero, expected_spells: &[Spell]) {
    for i in 0..SPELL_COUNT {
        let spell = Spell(i as u8);
        let exists = hero.spellbook[i] != 0;
        let expected = contains(expected_spells, spell);
        require_eq!(expected, exists);
        require_eq!(hero.has_spell_in_spellbook(spell), expected);
    }
}

/// Verify all spells, both in spell book and from artifacts.
fn verify_spells(hero: &Hero, expected_spells: &[Spell]) {
    for i in 0..SPELL_COUNT {
        let spell = Spell(i as u8);
        let exists = hero.spells[i] != 0;
        let expected = contains(expected_spells, spell);
        require_eq!(expected, exists);
        require_eq!(hero.has_spell(spell), expected);
    }
}

/// Verify spells from artifacts. Expects spells that are not in the spell book, except the
/// speciality spell, which is added to the spell book by default (e.g. Curse for Ingham).
fn verify_spells_from_artifacts_with_speciality(hero: &Hero, expected_spells: &[Spell], speciality: Spell) {
    for i in 0..SPELL_COUNT {
        let spell = Spell(i as u8);
        let exists = hero.spells[i] != 0;
        let expected = contains(expected_spells, spell);
        require_eq!(expected, exists);
        require_eq!(hero.has_spell(spell), expected);

        if spell != speciality {
            require!(!hero.has_spell_in_spellbook(spell)); // This function verifies spells from artifacts only.
        }
    }
}

/// Verify spells from artifacts. Expects spells that are not in the spell book.
fn verify_spells_from_artifacts(hero: &Hero, expected_spells: &[Spell]) {
    for i in 0..SPELL_COUNT {
        let spell = Spell(i as u8);
        let exists = hero.spells[i] != 0;
        let expected = contains(expected_spells, spell);
        require_eq!(expected, exists);
        require_eq!(hero.has_spell(spell), expected);
        require!(!hero.has_spell_in_spellbook(spell)); // This function verifies spells from artifacts only.
    }
}

// Only heroes with spell speciality have spells on day 1.
#[test]
fn spells_day1() {
    let _guard = common::Guard::new("spells-day1");
    let save = open(data_dir("spells").join("[hotseat] 111.GM3"));
    require!(save.valid());

    let no_spells: [Spell; 0] = [];

    {
        let hero = find_hero(&save, "Sanya");
        verify_spell_book(hero, &no_spells);
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Rion");
        verify_spell_book(hero, &no_spells);
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Cuthbert");
        verify_spell_book(hero, &no_spells);
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Loynis");
        let spells = [
            Spell::PRAYER, // Speciality
        ];
        verify_spell_book(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Adela");
        let spells = [
            Spell::BLESS, // Speciality
        ];
        verify_spell_book(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Ingham");
        let spells = [Spell::CURSE];
        verify_spell_book(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Adelaide");
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Caitlin");
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Elleshar");
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Coronius");
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Alagar");
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Gem");
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Aeris");
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Malcom");
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Uland");
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Melodia");
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Theodorus");
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Halon");
        verify_spells(hero, &no_spells);
    }

    {
        let hero = find_hero(&save, "Daremyth");
        verify_spells(hero, &no_spells);
    }

    {
        // Solmyr had his precious Chain Lightning stripped from the game!
        let hero = find_hero(&save, "Solmyr");
        verify_spells(hero, &no_spells);
    }
}

#[test]
fn spells_day2_sanya() {
    let _guard = common::Guard::new("spells-day2-sanya");
    let save = open(data_dir("spells").join("[hotseat] 112.GM3"));
    require!(save.valid());

    {
        let hero = find_hero(&save, "Sanya");
        let spells = [
            Spell::BLESS,
            Spell::BLOODLUST,
            Spell::CURE,
            Spell::CURSE,
            Spell::DISPEL,
            Spell::HASTE,
            Spell::MAGIC_ARROW,
            Spell::PROTECTION_FROM_WATER,
            Spell::PROTECTION_FROM_FIRE,
            Spell::SHIELD,
            Spell::SLOW,
            Spell::STONE_SKIN,
            Spell::SUMMON_BOAT,
            Spell::VIEW_AIR,
            Spell::VIEW_EARTH,
        ];
        verify_spell_book(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Rion");
        let spells = [
            Spell::BLIND,
            Spell::DEATH_RIPPLE,
            Spell::DISGUISE,
            Spell::DISRUPTING_RAY,
            Spell::FIRE_WALL,
            Spell::FORTUNE,
            Spell::ICE_BOLT,
            Spell::LIGHTNING_BOLT,
            Spell::PRECISION,
            Spell::PROTECTION_FROM_AIR,
            Spell::QUICKSAND,
            Spell::REMOVE_OBSTACLE,
            Spell::SCUTTLE_BOAT,
            Spell::VISIONS,
            Spell::WEAKNESS,
        ];
        verify_spell_book(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Cuthbert");
        let spells = [
            Spell::AIR_SHIELD,
            Spell::ANIMATE_DEAD,
            Spell::ANTI_MAGIC,
            Spell::DESTROY_UNDEAD,
            Spell::EARTHQUAKE,
            Spell::FIREBALL,
            Spell::FORCE_FIELD,
            Spell::FORGETFULNESS,
            Spell::FROST_RING,
            Spell::HYPNOTIZE,
            Spell::LAND_MINE,
            Spell::MIRTH,
            Spell::MISFORTUNE,
            Spell::PROTECTION_FROM_EARTH,
            Spell::TELEPORT,
        ];
        verify_spell_book(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Loynis");
        let spells = [
            Spell::PRAYER,
            Spell::ARMAGEDDON,
            Spell::BERSERK,
            Spell::CHAIN_LIGHTNING,
            Spell::CLONE,
            Spell::COUNTERSTRIKE,
            Spell::FIRE_SHIELD,
            Spell::FRENZY,
            Spell::INFERNO,
            Spell::METEOR_SHOWER,
            Spell::RESURRECTION,
            Spell::SLAYER,
            Spell::SORROW,
            Spell::TOWN_PORTAL,
            Spell::WATER_WALK,
        ];
        verify_spell_book(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Adela");
        let spells = [
            Spell::DIMENSION_DOOR,
            Spell::FLY,
            Spell::IMPLOSION,
            Spell::MAGIC_MIRROR,
            Spell::SACRIFICE,
            Spell::SUMMON_AIR_ELEMENTAL,
            Spell::SUMMON_EARTH_ELEMENTAL,
            Spell::SUMMON_FIRE_ELEMENTAL,
            Spell::SUMMON_WATER_ELEMENTAL,
            Spell::BLESS, // From speciality.
        ];
        verify_spell_book(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        // All spells, provided by artifacts only.
        let hero = find_hero(&save, "Ingham");
        let speciality = Spell::CURSE;
        let spells = [
            Spell::AIR_SHIELD,
            Spell::ANIMATE_DEAD,
            Spell::ANTI_MAGIC,
            Spell::ARMAGEDDON,
            Spell::BERSERK,
            Spell::BLESS,
            Spell::BLIND,
            Spell::BLOODLUST,
            Spell::CHAIN_LIGHTNING,
            Spell::CLONE,
            Spell::COUNTERSTRIKE,
            Spell::CURE,
            Spell::CURSE,
            Spell::DEATH_RIPPLE,
            Spell::DESTROY_UNDEAD,
            Spell::DIMENSION_DOOR,
            Spell::DISGUISE,
            Spell::DISPEL,
            Spell::DISRUPTING_RAY,
            Spell::EARTHQUAKE,
            Spell::FIRE_SHIELD,
            Spell::FIRE_WALL,
            Spell::FIREBALL,
            Spell::FLY,
            Spell::FORCE_FIELD,
            Spell::FORGETFULNESS,
            Spell::FORTUNE,
            Spell::FRENZY,
            Spell::FROST_RING,
            Spell::HASTE,
            Spell::HYPNOTIZE,
            Spell::ICE_BOLT,
            Spell::IMPLOSION,
            Spell::INFERNO,
            Spell::LAND_MINE,
            Spell::LIGHTNING_BOLT,
            Spell::MAGIC_ARROW,
            Spell::MAGIC_MIRROR,
            Spell::METEOR_SHOWER,
            Spell::MIRTH,
            Spell::MISFORTUNE,
            Spell::PRAYER,
            Spell::PRECISION,
            Spell::PROTECTION_FROM_AIR,
            Spell::PROTECTION_FROM_EARTH,
            Spell::PROTECTION_FROM_FIRE,
            Spell::PROTECTION_FROM_WATER,
            Spell::QUICKSAND,
            Spell::REMOVE_OBSTACLE,
            Spell::RESURRECTION,
            Spell::SACRIFICE,
            Spell::SCUTTLE_BOAT,
            Spell::SHIELD,
            Spell::SLAYER,
            Spell::SLOW,
            Spell::SORROW,
            Spell::STONE_SKIN,
            Spell::SUMMON_AIR_ELEMENTAL,
            Spell::SUMMON_BOAT,
            Spell::SUMMON_EARTH_ELEMENTAL,
            Spell::SUMMON_FIRE_ELEMENTAL,
            Spell::SUMMON_WATER_ELEMENTAL,
            Spell::TELEPORT,
            Spell::TITANS_LIGHTNING_BOLT,
            Spell::TOWN_PORTAL,
            Spell::VIEW_AIR,
            Spell::VIEW_EARTH,
            Spell::VISIONS,
            Spell::WATER_WALK,
            Spell::WEAKNESS,
        ];
        verify_spells_from_artifacts_with_speciality(hero, &spells, speciality);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Adelaide");
        let spells = [
            Spell::SUMMON_AIR_ELEMENTAL,
            Spell::AIR_SHIELD,
            Spell::ANIMATE_DEAD,
            Spell::ANTI_MAGIC,
            Spell::ARMAGEDDON,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Caitlin");
        let spells = [
            Spell::BERSERK,
            Spell::BLESS,
            Spell::BLIND,
            Spell::BLOODLUST,
            Spell::CHAIN_LIGHTNING,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Elleshar");
        let spells = [
            Spell::CLONE,
            Spell::COUNTERSTRIKE,
            Spell::CURE,
            Spell::CURSE,
            Spell::DEATH_RIPPLE,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Coronius");
        let spells = [
            Spell::DESTROY_UNDEAD,
            Spell::DIMENSION_DOOR,
            Spell::DISGUISE,
            Spell::DISPEL,
            Spell::DISRUPTING_RAY,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Alagar");
        let spells = [
            Spell::SUMMON_EARTH_ELEMENTAL,
            Spell::EARTHQUAKE,
            Spell::SUMMON_FIRE_ELEMENTAL,
            Spell::FIRE_SHIELD,
            Spell::FIRE_WALL,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Gem");
        let spells = [
            Spell::FIREBALL,
            Spell::FLY,
            Spell::FORCE_FIELD,
            Spell::FORGETFULNESS,
            Spell::FORTUNE,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Aeris");
        let spells = [
            Spell::FRENZY,
            Spell::FROST_RING,
            Spell::HASTE,
            Spell::HYPNOTIZE,
            Spell::ICE_BOLT,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Malcom");
        let spells = [
            Spell::IMPLOSION,
            Spell::INFERNO,
            Spell::LAND_MINE,
            Spell::LIGHTNING_BOLT,
            Spell::MAGIC_ARROW,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Uland");
        let spells = [
            Spell::MAGIC_MIRROR,
            Spell::METEOR_SHOWER,
            Spell::MIRTH,
            Spell::MISFORTUNE,
            Spell::PRAYER,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Melodia");
        let spells = [
            Spell::PRECISION,
            Spell::PROTECTION_FROM_EARTH,
            Spell::PROTECTION_FROM_WATER,
            Spell::PROTECTION_FROM_AIR,
            Spell::PROTECTION_FROM_FIRE,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Theodorus");
        let spells = [
            Spell::QUICKSAND,
            Spell::REMOVE_OBSTACLE,
            Spell::RESURRECTION,
            Spell::SACRIFICE,
            Spell::SCUTTLE_BOAT,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Halon");
        let spells = [
            Spell::SHIELD,
            Spell::SLAYER,
            Spell::SLOW,
            Spell::SORROW,
            Spell::STONE_SKIN,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        let hero = find_hero(&save, "Daremyth");
        let spells = [
            Spell::SUMMON_BOAT,
            Spell::TELEPORT,
            Spell::TOWN_PORTAL,
            Spell::VIEW_AIR,
            Spell::VIEW_EARTH,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }

    {
        // Solmyr had his precious Chain Lightning stripped from the game!
        let hero = find_hero(&save, "Solmyr");
        let spells = [
            Spell::VISIONS,
            Spell::SUMMON_WATER_ELEMENTAL,
            Spell::WATER_WALK,
            Spell::WEAKNESS,
        ];
        verify_spells_from_artifacts(hero, &spells);
        verify_spells(hero, &spells);
    }
}

// Ported from the homm3tools C++ test test-artifacts.cpp. Expected values are unchanged.

#![expect(clippy::too_many_lines, reason = "long test cases, ported from the C++ tests")]

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

fn verify_no_artifacts(hero: &Hero) {
    assert_eq!(hero.helm(), Artifact::NONE);
    assert_eq!(hero.cloak(), Artifact::NONE);
    assert_eq!(hero.neck(), Artifact::NONE);
    assert_eq!(hero.weapon(), Artifact::NONE);
    assert_eq!(hero.shield(), Artifact::NONE);
    assert_eq!(hero.armor(), Artifact::NONE);
    assert_eq!(hero.ring_left(), Artifact::NONE);
    assert_eq!(hero.ring_right(), Artifact::NONE);
    assert_eq!(hero.boots(), Artifact::NONE);
    assert_eq!(hero.misc01(), Artifact::NONE);
    assert_eq!(hero.misc02(), Artifact::NONE);
    assert_eq!(hero.misc03(), Artifact::NONE);
    assert_eq!(hero.misc04(), Artifact::NONE);
    assert_eq!(hero.misc05(), Artifact::NONE);
    assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
}

fn verify_catapult_only(hero: &Hero) {
    assert_eq!(hero.ballista(), Artifact::NONE);
    assert_eq!(hero.ammo_cart(), Artifact::NONE);
    assert_eq!(hero.first_aid_tent(), Artifact::NONE);
    assert_eq!(hero.catapult(), Artifact::CATAPULT);
    assert_eq!(hero.spell_book(), Artifact::NONE);
}

fn verify_catapult_and_spell_book(hero: &Hero) {
    assert_eq!(hero.ballista(), Artifact::NONE);
    assert_eq!(hero.ammo_cart(), Artifact::NONE);
    assert_eq!(hero.first_aid_tent(), Artifact::NONE);
    assert_eq!(hero.catapult(), Artifact::CATAPULT);
    assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);
}

#[test]
fn artifacts_day1() {
    let save = open(&data_dir("artifacts").join("[hotseat] 111.GM8"));
    assert_date(&save, 1, 1, 1);
    assert!(save.valid());

    {
        let hero = find_hero(&save, "Orrin");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Mephala");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Piquedram");
        verify_no_artifacts(hero);
        verify_catapult_and_spell_book(hero);
    }

    {
        let hero = find_hero(&save, "Fiona");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Valeska");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Ufretin");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Thane");
        verify_no_artifacts(hero);
        verify_catapult_and_spell_book(hero);
    }

    {
        let hero = find_hero(&save, "Jeremy");
        verify_no_artifacts(hero);
        assert_eq!(hero.ballista(), Artifact::CANNON);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Edric");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Clancy");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Josephine");
        verify_no_artifacts(hero);
        verify_catapult_and_spell_book(hero);
    }

    {
        let hero = find_hero(&save, "Sylvia");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Jenova");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Neela");
        verify_no_artifacts(hero);
        verify_catapult_and_spell_book(hero);
    }

    {
        let hero = find_hero(&save, "Lord Haart");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Ryland");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        // NOTE! "Torosar" is saved as "Torosar " in the save file.
        let hero = find_hero(&save, "Torosar ");
        verify_no_artifacts(hero);
        assert_eq!(hero.ballista(), Artifact::BALLISTA);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);
    }

    {
        let hero = find_hero(&save, "Sorsha");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Ivor");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Fafner");
        verify_no_artifacts(hero);
        verify_catapult_and_spell_book(hero);
    }

    {
        let hero = find_hero(&save, "Tyris");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Kyrre");
        verify_no_artifacts(hero);
        verify_catapult_only(hero);
    }

    {
        let hero = find_hero(&save, "Rissa");
        verify_no_artifacts(hero);
        verify_catapult_and_spell_book(hero);
    }

    {
        let hero = find_hero(&save, "Christian");
        verify_no_artifacts(hero);
        assert_eq!(hero.ballista(), Artifact::BALLISTA);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Iona");
        verify_no_artifacts(hero);
        verify_catapult_and_spell_book(hero);
    }

    {
        let hero = find_hero(&save, "Giselle");
        verify_no_artifacts(hero);
        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);
    }
}

#[test]
fn artifacts_day2() {
    let save = open(&data_dir("artifacts").join("[hotseat] 112.GM8"));
    assert_date(&save, 1, 1, 2);
    assert!(save.valid());

    {
        let hero = find_hero(&save, "Orrin");
        assert_eq!(hero.helm(), Artifact::ADMIRALS_HAT);
        assert_eq!(hero.cloak(), Artifact::NONE); // Blocked by Cornucopia
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Admiral's Hat
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Cornucopia
        assert_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Cornucopia
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::BOW_OF_THE_SHARPSHOOTER);
        assert_eq!(hero.misc02(), Artifact::CORNUCOPIA);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        assert_eq!(hero.misc05(), Artifact::NONE); // Blocked by Bow of the Sharpshooter

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::IRONFIST_OF_THE_OGRE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::GOLDEN_GOOSE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::WIZARDS_WELL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::RING_OF_THE_MAGI);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::DIPLOMATS_CLOAK);
        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY06),
            Artifact::PENDANT_OF_REFLECTION
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY07), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Mephala");
        assert_eq!(hero.helm(), Artifact::CROWN_OF_THE_SUPREME_MAGI);
        assert_eq!(hero.cloak(), Artifact::CAPE_OF_VELOCITY);
        assert_eq!(hero.neck(), Artifact::NECKLACE_OF_SWIFTNESS);
        assert_eq!(hero.weapon(), Artifact::OGRES_CLUB_OF_HAVOC);
        assert_eq!(hero.shield(), Artifact::TARG_OF_THE_RAMPAGING_OGRE);
        assert_eq!(hero.armor(), Artifact::TUNIC_OF_THE_CYCLOPS_KING);
        assert_eq!(hero.ring_left(), Artifact::RING_OF_OBLIVION);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::BOOTS_OF_SPEED);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::CAPE_OF_SILENCE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Piquedram");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::AMBASSADORS_SASH);
        assert_eq!(hero.neck(), Artifact::STATESMANS_MEDAL);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::DIPLOMATS_RING);
        assert_eq!(hero.ring_right(), Artifact::RING_OF_THE_WAYFARER);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::EQUESTRIANS_GLOVES);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Fiona");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::SURCOAT_OF_COUNTERPOISE);
        assert_eq!(hero.neck(), Artifact::GARNITURE_OF_INTERFERENCE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::BOOTS_OF_POLARITY);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::BALLISTA);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Valeska");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::CLOAK_OF_THE_UNDEAD_KING);
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Cloak of the Undead King
        assert_eq!(hero.weapon(), Artifact::ARMAGEDDONS_BLADE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Elixir of Life
        assert_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Elixir of Life
        assert_eq!(hero.boots(), Artifact::NONE); // Blocked by Cloak of the Undead King
        assert_eq!(hero.misc01(), Artifact::VIAL_OF_DRAGON_BLOOD);
        assert_eq!(hero.misc02(), Artifact::HORN_OF_THE_ABYSS);
        assert_eq!(hero.misc03(), Artifact::ELIXIR_OF_LIFE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::STATUE_OF_LEGION);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::ANGELIC_ALLIANCE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::ARMOR_OF_THE_DAMNED);
        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY04),
            Artifact::POWER_OF_THE_DRAGON_FATHER
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::TITANS_THUNDER);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Ufretin");
        assert_eq!(hero.helm(), Artifact::HELLSTORM_HELMET);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_NEGATIVITY);
        assert_eq!(hero.weapon(), Artifact::SWORD_OF_HELLFIRE);
        assert_eq!(hero.shield(), Artifact::SHIELD_OF_THE_DAMNED);
        assert_eq!(hero.armor(), Artifact::BREASTPLATE_OF_BRIMSTONE);
        assert_eq!(hero.ring_left(), Artifact::RING_OF_INFINITE_GEMS);
        assert_eq!(hero.ring_right(), Artifact::EVERSMOKING_RING_OF_SULFUR);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::EVERPOURING_VIAL_OF_MERCURY);
        assert_eq!(hero.misc02(), Artifact::INEXHAUSTIBLE_CART_OF_ORE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::PENDANT_OF_DISPASSION
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Thane");
        verify_no_artifacts(hero);
        verify_catapult_and_spell_book(hero);
    }

    {
        let hero = find_hero(&save, "Jeremy");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::CANNON);
        assert_eq!(hero.ammo_cart(), Artifact::AMMO_CART);
        assert_eq!(hero.first_aid_tent(), Artifact::FIRST_AID_TENT);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Edric");
        assert_eq!(hero.helm(), Artifact::SPELLBINDERS_HAT);
        assert_eq!(hero.cloak(), Artifact::CAPE_OF_CONJURING);
        assert_eq!(hero.neck(), Artifact::COLLAR_OF_CONJURING);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::RING_OF_CONJURING);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::ORB_OF_FIRMAMENT);
        assert_eq!(hero.misc02(), Artifact::ORB_OF_SILT);
        assert_eq!(hero.misc03(), Artifact::ORB_OF_TEMPESTUOUS_FIRE);
        assert_eq!(hero.misc04(), Artifact::ORB_OF_DRIVING_RAIN);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Clancy");
        assert_eq!(hero.helm(), Artifact::CROWN_OF_THE_FIVE_SEAS);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_SECOND_SIGHT);
        assert_eq!(hero.weapon(), Artifact::TRIDENT_OF_DOMINION);
        assert_eq!(hero.shield(), Artifact::SHIELD_OF_NAVAL_GLORY);
        assert_eq!(hero.armor(), Artifact::ROYAL_ARMOR_OF_NIX);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::ENDLESS_SACK_OF_GOLD);
        assert_eq!(hero.misc02(), Artifact::ENDLESS_BAG_OF_GOLD);
        assert_eq!(hero.misc03(), Artifact::ENDLESS_PURSE_OF_GOLD);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::PENDANT_OF_TOTAL_RECALL
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Josephine");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::MISC01),
            Spell::SUMMON_AIR_ELEMENTAL
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::AIR_SHIELD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::ANIMATE_DEAD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::ANTI_MAGIC);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::ARMAGEDDON);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::BERSERK);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::BLESS);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::BLIND);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::BLOODLUST);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY05),
            Spell::CHAIN_LIGHTNING
        );
    }

    {
        let hero = find_hero(&save, "Sylvia");
        assert_eq!(hero.helm(), Artifact::SEA_CAPTAINS_HAT);
        assert_eq!(hero.cloak(), Artifact::ANGEL_WINGS);
        assert_eq!(hero.neck(), Artifact::NECKLACE_OF_OCEAN_GUIDANCE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::PLATE_OF_DYING_LIGHT);
        assert_eq!(hero.ring_left(), Artifact::SEAL_OF_SUNSET);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::CHARM_OF_MANA);
        assert_eq!(hero.misc02(), Artifact::TALISMAN_OF_MANA);
        assert_eq!(hero.misc03(), Artifact::MYSTIC_ORB_OF_MANA);
        assert_eq!(hero.misc04(), Artifact::SHACKLES_OF_WAR);
        assert_eq!(hero.misc05(), Artifact::CHARM_OF_ECLIPSE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Jenova");
        assert_eq!(hero.helm(), Artifact::THUNDER_HELMET);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_HOLINESS);
        assert_eq!(hero.weapon(), Artifact::TITANS_GLADIUS);
        assert_eq!(hero.shield(), Artifact::SENTINELS_SHIELD);
        assert_eq!(hero.armor(), Artifact::TITANS_CUIRASS);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::LEGS_OF_LEGION);
        assert_eq!(hero.misc02(), Artifact::LOINS_OF_LEGION);
        assert_eq!(hero.misc03(), Artifact::TORSO_OF_LEGION);
        assert_eq!(hero.misc04(), Artifact::ARMS_OF_LEGION);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::PENDANT_OF_COURAGE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Neela");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::CLONE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::COUNTERSTRIKE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::CURE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::CURSE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::DEATH_RIPPLE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::DESTROY_UNDEAD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::DIMENSION_DOOR);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::DISGUISE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::DISPEL);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::DISRUPTING_RAY);
    }

    {
        let hero = find_hero(&save, "Lord Haart");
        assert_eq!(hero.helm(), Artifact::HELM_OF_THE_ALABASTER_UNICORN);
        assert_eq!(hero.cloak(), Artifact::RECANTERS_CLOAK);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::CENTAURS_AXE);
        assert_eq!(hero.shield(), Artifact::SHIELD_OF_THE_DWARVEN_LORDS);
        assert_eq!(hero.armor(), Artifact::BREASTPLATE_OF_PETRIFIED_WOOD);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPIRIT_OF_OPRESSION);
        assert_eq!(hero.misc02(), Artifact::HOURGLASS_OF_THE_EVIL_HOUR);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Ryland");
        assert_eq!(hero.helm(), Artifact::HELM_OF_HEAVENLY_ENLIGHTENMENT);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::CELESTIAL_NECKLACE_OF_BLISS);
        assert_eq!(hero.weapon(), Artifact::SWORD_OF_JUDGEMENT);
        assert_eq!(hero.shield(), Artifact::LIONS_SHIELD_OF_COURAGE);
        assert_eq!(hero.armor(), Artifact::ARMOR_OF_WONDER);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::SANDALS_OF_THE_SAINT);
        assert_eq!(hero.misc01(), Artifact::ORB_OF_INHIBITION);
        assert_eq!(hero.misc02(), Artifact::RUNES_OF_IMMINENCY);
        assert_eq!(hero.misc03(), Artifact::DEMONS_HORSESHOE);
        assert_eq!(hero.misc04(), Artifact::SHAMANS_PUPPET);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // NOTE! "Torosar" is saved as "Torosar " in the save file.
        let hero = find_hero(&save, "Torosar ");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::MISC01),
            Spell::SUMMON_EARTH_ELEMENTAL
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::EARTHQUAKE);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::MISC03),
            Spell::SUMMON_FIRE_ELEMENTAL
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::FIRE_SHIELD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::FIRE_WALL);

        assert_eq!(hero.ballista(), Artifact::BALLISTA);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::FIREBALL);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::FLY);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::FORCE_FIELD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::FORGETFULNESS);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::FORTUNE);
    }

    {
        let hero = find_hero(&save, "Sorsha");
        assert_eq!(hero.helm(), Artifact::SKULL_HELMET);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::BLACKSHARD_OF_THE_DEAD_KNIGHT);
        assert_eq!(hero.shield(), Artifact::SHIELD_OF_THE_YAWNING_DEAD);
        assert_eq!(hero.armor(), Artifact::RIB_CAGE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::BOOTS_OF_LEVITATION);
        assert_eq!(hero.misc01(), Artifact::TOME_OF_FIRE_MAGIC);
        assert_eq!(hero.misc02(), Artifact::TOME_OF_AIR_MAGIC);
        assert_eq!(hero.misc03(), Artifact::TOME_OF_WATER_MAGIC);
        assert_eq!(hero.misc04(), Artifact::TOME_OF_EARTH_MAGIC);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Ivor");
        assert_eq!(hero.helm(), Artifact::CROWN_OF_DRAGON_TOOTH);
        assert_eq!(hero.cloak(), Artifact::DRAGON_WING_TABARD);
        assert_eq!(hero.neck(), Artifact::NECKLACE_OF_DRAGONTEETH);
        assert_eq!(hero.weapon(), Artifact::RED_DRAGON_FLAME_TONGUE);
        assert_eq!(hero.shield(), Artifact::DRAGON_SCALE_SHIELD);
        assert_eq!(hero.armor(), Artifact::DRAGON_SCALE_ARMOR);
        assert_eq!(hero.ring_left(), Artifact::QUIET_EYE_OF_THE_DRAGON);
        assert_eq!(hero.ring_right(), Artifact::STILL_EYE_OF_THE_DRAGON);
        assert_eq!(hero.boots(), Artifact::DRAGONBONE_GREAVES);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Fafner");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::FRENZY);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::FROST_RING);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::HASTE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::HYPNOTIZE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::ICE_BOLT);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::IMPLOSION);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::INFERNO);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::LAND_MINE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::LIGHTNING_BOLT);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::MAGIC_ARROW);
    }

    {
        let hero = find_hero(&save, "Tyris");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::SUMMON_BOAT);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::TELEPORT);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::TOWN_PORTAL);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::VIEW_AIR);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::VIEW_EARTH);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::NONE);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::VISIONS);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY02),
            Spell::SUMMON_WATER_ELEMENTAL
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::WATER_WALK);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::WEAKNESS);
    }

    {
        let hero = find_hero(&save, "Kyrre");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_LIFE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::CLOVER_OF_FORTUNE);
        assert_eq!(hero.misc02(), Artifact::CARDS_OF_PROPHECY);
        assert_eq!(hero.misc03(), Artifact::LADYBIRD_OF_LUCK);
        assert_eq!(hero.misc04(), Artifact::BADGE_OF_COURAGE);
        assert_eq!(hero.misc05(), Artifact::CREST_OF_VALOR);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::GLYPH_OF_GALLANTRY);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPECULUM);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPYGLASS);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::PENDANT_OF_DEATH);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::PENDANT_OF_FREE_WILL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Rissa");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::MAGIC_MIRROR);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::METEOR_SHOWER);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::MIRTH);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::MISFORTUNE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::PRAYER);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::PRECISION);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY02),
            Spell::PROTECTION_FROM_EARTH
        );
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY03),
            Spell::PROTECTION_FROM_WATER
        );
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY04),
            Spell::PROTECTION_FROM_AIR
        );
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY05),
            Spell::PROTECTION_FROM_FIRE
        );
    }

    {
        let hero = find_hero(&save, "Christian");
        assert_eq!(hero.helm(), Artifact::HELM_OF_CHAOS);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::GREATER_GNOLLS_FLAIL);
        assert_eq!(hero.shield(), Artifact::BUCKLER_OF_THE_GNOLL_KING);
        assert_eq!(hero.armor(), Artifact::SCALES_OF_THE_GREATER_BASILISK);
        assert_eq!(hero.ring_left(), Artifact::RING_OF_VITALITY);
        assert_eq!(hero.ring_right(), Artifact::RING_OF_LIFE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::GOLDEN_BOW);
        assert_eq!(hero.misc02(), Artifact::SPHERE_OF_PERMANENCE);
        assert_eq!(hero.misc03(), Artifact::ORB_OF_VULNERABILITY);
        assert_eq!(hero.misc04(), Artifact::VIAL_OF_LIFEBLOOED);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::BALLISTA);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Iona");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::QUICKSAND);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::REMOVE_OBSTACLE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::RESURRECTION);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::SACRIFICE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::SCUTTLE_BOAT);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::SHIELD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::SLAYER);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::SLOW);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::SORROW);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::STONE_SKIN);
    }

    {
        let hero = find_hero(&save, "Giselle");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::VAMPIRES_COWL);
        assert_eq!(hero.neck(), Artifact::AMULET_OF_THE_UNDERTAKER);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::DEAD_MANS_BOOTS);
        assert_eq!(hero.misc01(), Artifact::BOW_OF_ELVEN_CHERRY_WOOD);
        assert_eq!(hero.misc02(), Artifact::BOWSTRING_OF_THE_UNICORNS_MANE);
        assert_eq!(hero.misc03(), Artifact::ANGEL_FEATHER_ARROWS);
        assert_eq!(hero.misc04(), Artifact::BIRD_OF_PERCEPTION);
        assert_eq!(hero.misc05(), Artifact::STOIC_WATCHMAN);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::EMBLEM_OF_COGNIZANCE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }
}

// Same as day 2, but with some minor differences.
#[test]
fn artifacts_day3() {
    let save = open(&data_dir("artifacts").join("[hotseat] 113.GM8"));
    assert_date(&save, 1, 1, 3);
    assert!(save.valid());

    {
        let hero = find_hero(&save, "Orrin");
        assert_eq!(hero.helm(), Artifact::ADMIRALS_HAT);
        assert_eq!(hero.cloak(), Artifact::NONE); // Blocked by Cornucopia
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Admiral's Hat
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Cornucopia
        assert_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Cornucopia
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::BOW_OF_THE_SHARPSHOOTER);
        assert_eq!(hero.misc02(), Artifact::CORNUCOPIA);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        assert_eq!(hero.misc05(), Artifact::NONE); // Blocked by Bow of the Sharpshooter

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        // Diplomat's Cloak and Ring of the Magi are swapped. The inventory is also shuffled/scrolled
        // so that the artifact in the first slot is noe Wizards Well.
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::WIZARDS_WELL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::DIPLOMATS_CLOAK); // Swapped
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::RING_OF_THE_MAGI); // Swapped
        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY04),
            Artifact::PENDANT_OF_REFLECTION
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::IRONFIST_OF_THE_OGRE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::GOLDEN_GOOSE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY07), Artifact::NONE);
    }

    {
        // AssemblesIronfist of the Ogre.
        let hero = find_hero(&save, "Mephala");
        assert_eq!(hero.helm(), Artifact::NONE); // Blocked by Ironfist of the Ogre
        assert_eq!(hero.cloak(), Artifact::CAPE_OF_SILENCE); // Swapped
        assert_eq!(hero.neck(), Artifact::NECKLACE_OF_SWIFTNESS);
        assert_eq!(hero.weapon(), Artifact::IRONFIST_OF_THE_OGRE);
        assert_eq!(hero.shield(), Artifact::NONE); // Blocked by Ironfist of the Ogre
        assert_eq!(hero.armor(), Artifact::NONE); // Blocked by Ironfist of the Ogre
        assert_eq!(hero.ring_left(), Artifact::RING_OF_OBLIVION);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::BOOTS_OF_SPEED);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::CAPE_OF_VELOCITY); // Swapped
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // Assemble Diplomat's Cloak.
        let hero = find_hero(&save, "Piquedram");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::DIPLOMATS_CLOAK);
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Diplomat's Cloak
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Diplomat's Cloak
        assert_eq!(hero.ring_right(), Artifact::RING_OF_THE_WAYFARER);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::EQUESTRIANS_GLOVES);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Fiona");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE); // Blocked by Pendant of Reflection
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_REFLECTION);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE); // Blocked by Pendant of Reflection
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::BALLISTA);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Valeska");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::CLOAK_OF_THE_UNDEAD_KING);
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Cloak of the Undead King
        assert_eq!(hero.weapon(), Artifact::ARMAGEDDONS_BLADE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Elixir of Life
        assert_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Elixir of Life
        assert_eq!(hero.boots(), Artifact::NONE); // Blocked by Cloak of the Undead King
        assert_eq!(hero.misc01(), Artifact::VIAL_OF_DRAGON_BLOOD);
        assert_eq!(hero.misc02(), Artifact::HORN_OF_THE_ABYSS);
        assert_eq!(hero.misc03(), Artifact::ELIXIR_OF_LIFE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        // Inventory01 was moved to Inventory05.
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::ANGELIC_ALLIANCE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::ARMOR_OF_THE_DAMNED);
        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY03),
            Artifact::POWER_OF_THE_DRAGON_FATHER
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::TITANS_THUNDER);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::STATUE_OF_LEGION); // Moved from Inventory01
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);
    }

    {
        // Swap neck slot item.
        let hero = find_hero(&save, "Ufretin");
        assert_eq!(hero.helm(), Artifact::HELLSTORM_HELMET);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_DISPASSION); // Swapped
        assert_eq!(hero.weapon(), Artifact::SWORD_OF_HELLFIRE);
        assert_eq!(hero.shield(), Artifact::SHIELD_OF_THE_DAMNED);
        assert_eq!(hero.armor(), Artifact::BREASTPLATE_OF_BRIMSTONE);
        assert_eq!(hero.ring_left(), Artifact::RING_OF_INFINITE_GEMS);
        assert_eq!(hero.ring_right(), Artifact::EVERSMOKING_RING_OF_SULFUR);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::EVERPOURING_VIAL_OF_MERCURY);
        assert_eq!(hero.misc02(), Artifact::INEXHAUSTIBLE_CART_OF_ORE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::PENDANT_OF_NEGATIVITY
        ); // Swapped
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // Thane now has the Grail.
        let hero = find_hero(&save, "Thane");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::GRAIL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Jeremy");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::CANNON);
        assert_eq!(hero.ammo_cart(), Artifact::AMMO_CART);
        assert_eq!(hero.first_aid_tent(), Artifact::FIRST_AID_TENT);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // Assemble Ring of the Magi.
        let hero = find_hero(&save, "Edric");
        assert_eq!(hero.helm(), Artifact::SPELLBINDERS_HAT);
        assert_eq!(hero.cloak(), Artifact::NONE); // Blocked by Ring of the Magi
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Ring of the Magi
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::RING_OF_THE_MAGI);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::ORB_OF_FIRMAMENT);
        assert_eq!(hero.misc02(), Artifact::ORB_OF_SILT);
        assert_eq!(hero.misc03(), Artifact::ORB_OF_TEMPESTUOUS_FIRE);
        assert_eq!(hero.misc04(), Artifact::ORB_OF_DRIVING_RAIN);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // Assemble Golden Goose.
        let hero = find_hero(&save, "Clancy");
        assert_eq!(hero.helm(), Artifact::CROWN_OF_THE_FIVE_SEAS);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_SECOND_SIGHT);
        assert_eq!(hero.weapon(), Artifact::TRIDENT_OF_DOMINION);
        assert_eq!(hero.shield(), Artifact::SHIELD_OF_NAVAL_GLORY);
        assert_eq!(hero.armor(), Artifact::ROYAL_ARMOR_OF_NIX);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::GOLDEN_GOOSE);
        assert_eq!(hero.misc02(), Artifact::NONE); // Blocked by Golden Goose
        assert_eq!(hero.misc03(), Artifact::NONE); // Blocked by Golden Goose
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::PENDANT_OF_TOTAL_RECALL
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // Swap Bless and Air Shield, put Air Shield in inventory04
        let hero = find_hero(&save, "Josephine");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::MISC01),
            Spell::SUMMON_AIR_ELEMENTAL
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::BLESS); // Swapped
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::ANIMATE_DEAD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::ANTI_MAGIC);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::ARMAGEDDON);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::BERSERK);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::BLIND);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::BLOODLUST);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::AIR_SHIELD);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY05),
            Spell::CHAIN_LIGHTNING
        );
    }

    {
        // Assemble Wizard's Well.
        let hero = find_hero(&save, "Sylvia");
        assert_eq!(hero.helm(), Artifact::SEA_CAPTAINS_HAT);
        assert_eq!(hero.cloak(), Artifact::ANGEL_WINGS);
        assert_eq!(hero.neck(), Artifact::NECKLACE_OF_OCEAN_GUIDANCE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::PLATE_OF_DYING_LIGHT);
        assert_eq!(hero.ring_left(), Artifact::SEAL_OF_SUNSET);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::WIZARDS_WELL);
        assert_eq!(hero.misc02(), Artifact::NONE); // Blocked by Wizard's Well
        assert_eq!(hero.misc03(), Artifact::NONE); // Blocked by Wizard's Well
        assert_eq!(hero.misc04(), Artifact::SHACKLES_OF_WAR);
        assert_eq!(hero.misc05(), Artifact::CHARM_OF_ECLIPSE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // Add Head of Legion to Misc05
        // Add Wayfarer's Boots to Boots
        let hero = find_hero(&save, "Jenova");
        assert_eq!(hero.helm(), Artifact::THUNDER_HELMET);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_HOLINESS);
        assert_eq!(hero.weapon(), Artifact::TITANS_GLADIUS);
        assert_eq!(hero.shield(), Artifact::SENTINELS_SHIELD);
        assert_eq!(hero.armor(), Artifact::TITANS_CUIRASS);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::WAYFARERS_BOOTS);
        assert_eq!(hero.misc01(), Artifact::LEGS_OF_LEGION);
        assert_eq!(hero.misc02(), Artifact::LOINS_OF_LEGION);
        assert_eq!(hero.misc03(), Artifact::TORSO_OF_LEGION);
        assert_eq!(hero.misc04(), Artifact::ARMS_OF_LEGION);
        assert_eq!(hero.misc05(), Artifact::HEAD_OF_LEGION);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::PENDANT_OF_COURAGE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Neela");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::CLONE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::COUNTERSTRIKE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::CURE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::CURSE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::DEATH_RIPPLE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::DESTROY_UNDEAD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::DIMENSION_DOOR);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::DISGUISE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::DISPEL);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::DISRUPTING_RAY);
    }

    {
        let hero = find_hero(&save, "Lord Haart");
        assert_eq!(hero.helm(), Artifact::HELM_OF_THE_ALABASTER_UNICORN);
        assert_eq!(hero.cloak(), Artifact::RECANTERS_CLOAK);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::CENTAURS_AXE);
        assert_eq!(hero.shield(), Artifact::NONE); // Moved shield to inventory.
        assert_eq!(hero.armor(), Artifact::BREASTPLATE_OF_PETRIFIED_WOOD);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPIRIT_OF_OPRESSION);
        assert_eq!(hero.misc02(), Artifact::HOURGLASS_OF_THE_EVIL_HOUR);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::SHIELD_OF_THE_DWARVEN_LORDS
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // Assemble Angelic Alliance.
        let hero = find_hero(&save, "Ryland");
        assert_eq!(hero.helm(), Artifact::NONE); // Blocked by Angelic Alliance
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Angelic Alliance
        assert_eq!(hero.weapon(), Artifact::ANGELIC_ALLIANCE);
        assert_eq!(hero.shield(), Artifact::NONE); // Blocked by Angelic Alliance
        assert_eq!(hero.armor(), Artifact::NONE); // Blocked by Angelic Alliance
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE); // Blocked by Angelic Alliance
        assert_eq!(hero.misc01(), Artifact::ORB_OF_INHIBITION);
        assert_eq!(hero.misc02(), Artifact::RUNES_OF_IMMINENCY);
        assert_eq!(hero.misc03(), Artifact::DEMONS_HORSESHOE);
        assert_eq!(hero.misc04(), Artifact::SHAMANS_PUPPET);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // NOTE! "Torosar" is saved as "Torosar " in the save file.
        // No changes.
        let hero = find_hero(&save, "Torosar ");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::MISC01),
            Spell::SUMMON_EARTH_ELEMENTAL
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::EARTHQUAKE);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::MISC03),
            Spell::SUMMON_FIRE_ELEMENTAL
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::FIRE_SHIELD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::FIRE_WALL);

        assert_eq!(hero.ballista(), Artifact::BALLISTA);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::FIREBALL);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::FLY);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::FORCE_FIELD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::FORGETFULNESS);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::FORTUNE);
    }

    {
        // Assemble Armor of the Damned.
        let hero = find_hero(&save, "Sorsha");
        assert_eq!(hero.helm(), Artifact::NONE); // Blocked by Armor of the Damned
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE); // Blocked by Armor of the Damned
        assert_eq!(hero.shield(), Artifact::NONE); // Blocked by Armor of the Damned
        assert_eq!(hero.armor(), Artifact::ARMOR_OF_THE_DAMNED);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::BOOTS_OF_LEVITATION);
        assert_eq!(hero.misc01(), Artifact::TOME_OF_FIRE_MAGIC);
        assert_eq!(hero.misc02(), Artifact::TOME_OF_AIR_MAGIC);
        assert_eq!(hero.misc03(), Artifact::TOME_OF_WATER_MAGIC);
        assert_eq!(hero.misc04(), Artifact::TOME_OF_EARTH_MAGIC);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Ivor");
        assert_eq!(hero.helm(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.cloak(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.weapon(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.shield(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.armor(), Artifact::POWER_OF_THE_DRAGON_FATHER);
        assert_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.boots(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Fafner");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::FRENZY);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::FROST_RING);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::HASTE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::HYPNOTIZE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::ICE_BOLT);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::IMPLOSION);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::INFERNO);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::LAND_MINE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::LIGHTNING_BOLT);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::MAGIC_ARROW);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Tyris");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::SUMMON_BOAT);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::TELEPORT);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::TOWN_PORTAL);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::VIEW_AIR);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::VIEW_EARTH);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::NONE);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::VISIONS);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY02),
            Spell::SUMMON_WATER_ELEMENTAL
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::WATER_WALK);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::WEAKNESS);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Kyrre");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_LIFE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::CLOVER_OF_FORTUNE);
        assert_eq!(hero.misc02(), Artifact::CARDS_OF_PROPHECY);
        assert_eq!(hero.misc03(), Artifact::LADYBIRD_OF_LUCK);
        assert_eq!(hero.misc04(), Artifact::BADGE_OF_COURAGE);
        assert_eq!(hero.misc05(), Artifact::CREST_OF_VALOR);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::GLYPH_OF_GALLANTRY);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPECULUM);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPYGLASS);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::PENDANT_OF_DEATH);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::PENDANT_OF_FREE_WILL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Rissa");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::MAGIC_MIRROR);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::METEOR_SHOWER);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::MIRTH);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::MISFORTUNE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::PRAYER);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::PRECISION);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY02),
            Spell::PROTECTION_FROM_EARTH
        );
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY03),
            Spell::PROTECTION_FROM_WATER
        );
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY04),
            Spell::PROTECTION_FROM_AIR
        );
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY05),
            Spell::PROTECTION_FROM_FIRE
        );
    }

    {
        // Asemble Elixir of Life.
        let hero = find_hero(&save, "Christian");
        assert_eq!(hero.helm(), Artifact::HELM_OF_CHAOS);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::GREATER_GNOLLS_FLAIL);
        assert_eq!(hero.shield(), Artifact::BUCKLER_OF_THE_GNOLL_KING);
        assert_eq!(hero.armor(), Artifact::SCALES_OF_THE_GREATER_BASILISK);
        assert_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Elixir of Life
        assert_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Elixir of Life
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::GOLDEN_BOW);
        assert_eq!(hero.misc02(), Artifact::SPHERE_OF_PERMANENCE);
        assert_eq!(hero.misc03(), Artifact::ORB_OF_VULNERABILITY);
        assert_eq!(hero.misc04(), Artifact::ELIXIR_OF_LIFE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::BALLISTA);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Iona");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::QUICKSAND);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::REMOVE_OBSTACLE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::RESURRECTION);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::SACRIFICE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::SCUTTLE_BOAT);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::SHIELD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::SLAYER);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::SLOW);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::SORROW);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::STONE_SKIN);
    }

    {
        // Assemble Bow of the Sharpshooter.
        let hero = find_hero(&save, "Giselle");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::VAMPIRES_COWL);
        assert_eq!(hero.neck(), Artifact::AMULET_OF_THE_UNDERTAKER);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::DEAD_MANS_BOOTS);
        assert_eq!(hero.misc01(), Artifact::BOW_OF_THE_SHARPSHOOTER);
        assert_eq!(hero.misc02(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        assert_eq!(hero.misc03(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        assert_eq!(hero.misc04(), Artifact::BIRD_OF_PERCEPTION);
        assert_eq!(hero.misc05(), Artifact::STOIC_WATCHMAN);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::EMBLEM_OF_COGNIZANCE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }
}

// Same as day 3, but with some minor differences.
#[test]
fn artifacts_day4() {
    let save = open(&data_dir("artifacts").join("[hotseat] 114.GM8"));
    assert_date(&save, 1, 1, 4);
    assert!(save.valid());

    {
        // No changes.
        let hero = find_hero(&save, "Orrin");
        assert_eq!(hero.helm(), Artifact::ADMIRALS_HAT);
        assert_eq!(hero.cloak(), Artifact::NONE); // Blocked by Cornucopia
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Admiral's Hat
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Cornucopia
        assert_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Cornucopia
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::BOW_OF_THE_SHARPSHOOTER);
        assert_eq!(hero.misc02(), Artifact::CORNUCOPIA);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        assert_eq!(hero.misc05(), Artifact::NONE); // Blocked by Bow of the Sharpshooter

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        // Diplomat's Cloak and Ring of the Magi are swapped. The inventory is also shuffled/scrolled
        // so that the artifact in the first slot is noe Wizards Well.
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::WIZARDS_WELL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::DIPLOMATS_CLOAK); // Swapped
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::RING_OF_THE_MAGI); // Swapped
        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY04),
            Artifact::PENDANT_OF_REFLECTION
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::IRONFIST_OF_THE_OGRE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::GOLDEN_GOOSE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY07), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Mephala");
        assert_eq!(hero.helm(), Artifact::NONE); // Blocked by Ironfist of the Ogre
        assert_eq!(hero.cloak(), Artifact::CAPE_OF_SILENCE); // Swapped
        assert_eq!(hero.neck(), Artifact::NECKLACE_OF_SWIFTNESS);
        assert_eq!(hero.weapon(), Artifact::IRONFIST_OF_THE_OGRE);
        assert_eq!(hero.shield(), Artifact::NONE); // Blocked by Ironfist of the Ogre
        assert_eq!(hero.armor(), Artifact::NONE); // Blocked by Ironfist of the Ogre
        assert_eq!(hero.ring_left(), Artifact::RING_OF_OBLIVION);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::BOOTS_OF_SPEED);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::CAPE_OF_VELOCITY); // Swapped
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Piquedram");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::DIPLOMATS_CLOAK);
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Diplomat's Cloak
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Diplomat's Cloak
        assert_eq!(hero.ring_right(), Artifact::RING_OF_THE_WAYFARER);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::EQUESTRIANS_GLOVES);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Fiona");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE); // Blocked by Pendant of Reflection
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_REFLECTION);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE); // Blocked by Pendant of Reflection
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::BALLISTA);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Valeska");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::CLOAK_OF_THE_UNDEAD_KING);
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Cloak of the Undead King
        assert_eq!(hero.weapon(), Artifact::ARMAGEDDONS_BLADE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Elixir of Life
        assert_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Elixir of Life
        assert_eq!(hero.boots(), Artifact::NONE); // Blocked by Cloak of the Undead King
        assert_eq!(hero.misc01(), Artifact::VIAL_OF_DRAGON_BLOOD);
        assert_eq!(hero.misc02(), Artifact::HORN_OF_THE_ABYSS);
        assert_eq!(hero.misc03(), Artifact::ELIXIR_OF_LIFE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        // Inventory01 was moved to Inventory05.
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::ANGELIC_ALLIANCE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::ARMOR_OF_THE_DAMNED);
        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY03),
            Artifact::POWER_OF_THE_DRAGON_FATHER
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::TITANS_THUNDER);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::STATUE_OF_LEGION); // Moved from Inventory01
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);
    }

    {
        // Pick up two more artifacts.
        let hero = find_hero(&save, "Ufretin");
        assert_eq!(hero.helm(), Artifact::HELLSTORM_HELMET);
        assert_eq!(hero.cloak(), Artifact::EVERFLOWING_CRYSTAL_CLOAK); // Added
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_DISPASSION);
        assert_eq!(hero.weapon(), Artifact::SWORD_OF_HELLFIRE);
        assert_eq!(hero.shield(), Artifact::SHIELD_OF_THE_DAMNED);
        assert_eq!(hero.armor(), Artifact::BREASTPLATE_OF_BRIMSTONE);
        assert_eq!(hero.ring_left(), Artifact::RING_OF_INFINITE_GEMS);
        assert_eq!(hero.ring_right(), Artifact::EVERSMOKING_RING_OF_SULFUR);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::EVERPOURING_VIAL_OF_MERCURY);
        assert_eq!(hero.misc02(), Artifact::INEXHAUSTIBLE_CART_OF_ORE);
        assert_eq!(hero.misc03(), Artifact::INEXHAUSTIBLE_CART_OF_LUMBER); // Added
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::PENDANT_OF_NEGATIVITY
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Thane");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::GRAIL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Jeremy");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::CANNON);
        assert_eq!(hero.ammo_cart(), Artifact::AMMO_CART);
        assert_eq!(hero.first_aid_tent(), Artifact::FIRST_AID_TENT);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Edric");
        assert_eq!(hero.helm(), Artifact::SPELLBINDERS_HAT);
        assert_eq!(hero.cloak(), Artifact::NONE); // Blocked by Ring of the Magi
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Ring of the Magi
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::RING_OF_THE_MAGI);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::ORB_OF_FIRMAMENT);
        assert_eq!(hero.misc02(), Artifact::ORB_OF_SILT);
        assert_eq!(hero.misc03(), Artifact::ORB_OF_TEMPESTUOUS_FIRE);
        assert_eq!(hero.misc04(), Artifact::ORB_OF_DRIVING_RAIN);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Clancy");
        assert_eq!(hero.helm(), Artifact::CROWN_OF_THE_FIVE_SEAS);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_SECOND_SIGHT);
        assert_eq!(hero.weapon(), Artifact::TRIDENT_OF_DOMINION);
        assert_eq!(hero.shield(), Artifact::SHIELD_OF_NAVAL_GLORY);
        assert_eq!(hero.armor(), Artifact::ROYAL_ARMOR_OF_NIX);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::GOLDEN_GOOSE);
        assert_eq!(hero.misc02(), Artifact::NONE); // Blocked by Golden Goose
        assert_eq!(hero.misc03(), Artifact::NONE); // Blocked by Golden Goose
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::PENDANT_OF_TOTAL_RECALL
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // Josephine was defeated.
        let hero = find_hero(&save, "Josephine");
        assert_eq!(hero.player, 0xFF);
        assert_eq!(hero.on_map, 0);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Sylvia");
        assert_eq!(hero.helm(), Artifact::SEA_CAPTAINS_HAT);
        assert_eq!(hero.cloak(), Artifact::ANGEL_WINGS);
        assert_eq!(hero.neck(), Artifact::NECKLACE_OF_OCEAN_GUIDANCE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::PLATE_OF_DYING_LIGHT);
        assert_eq!(hero.ring_left(), Artifact::SEAL_OF_SUNSET);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::WIZARDS_WELL);
        assert_eq!(hero.misc02(), Artifact::NONE); // Blocked by Wizard's Well
        assert_eq!(hero.misc03(), Artifact::NONE); // Blocked by Wizard's Well
        assert_eq!(hero.misc04(), Artifact::SHACKLES_OF_WAR);
        assert_eq!(hero.misc05(), Artifact::CHARM_OF_ECLIPSE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // - Assemble Statue of Legion
        // - Assemble Titan's Thunder. This gives Jenova a spell book (!).
        // - Gives Pendant of Courage to Neela, who is now full of scrolls. The pendant disappears!
        let hero = find_hero(&save, "Jenova");
        assert_eq!(hero.helm(), Artifact::NONE); // Blocked by Titan's Thunder
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_HOLINESS);
        assert_eq!(hero.weapon(), Artifact::TITANS_THUNDER);
        assert_eq!(hero.shield(), Artifact::NONE); // Blocked by Titan's Thunder
        assert_eq!(hero.armor(), Artifact::NONE); // Blocked by Titan's Thunder
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::WAYFARERS_BOOTS);
        assert_eq!(hero.misc01(), Artifact::STATUE_OF_LEGION);
        assert_eq!(hero.misc02(), Artifact::NONE); // Blocked by Statue of Legion
        assert_eq!(hero.misc03(), Artifact::NONE); // Blocked by Statue of Legion
        assert_eq!(hero.misc04(), Artifact::NONE); // Blocked by Statue of Legion
        assert_eq!(hero.misc05(), Artifact::NONE); // Blocked by Statue of Legion

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK); // Spell Book gained by assembling Titan's Thunder!

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // Neela used her Armorer skill to kill off all other heroes with scrolls.
        // She now posesses all 69 scrolls, filling up the entire inventory.
        // Any artifacts gained after this is therefore lost.
        let hero = find_hero(&save, "Neela");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        assert_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::CLONE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::COUNTERSTRIKE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::CURE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::CURSE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::DEATH_RIPPLE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        // Loop through all inventory items and verify that they are scrolls.
        let start = usize::from(ArtifactSlot::INVENTORY01.0);
        for i in start..start + 64 {
            assert_eq!(hero.artifact_at(i), Artifact::SPELL_SCROLL);
        }

        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::BLIND);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::BERSERK);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY03),
            Spell::SUMMON_EARTH_ELEMENTAL
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::EARTHQUAKE);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY05),
            Spell::SUMMON_FIRE_ELEMENTAL
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY06), Spell::FIRE_SHIELD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY07), Spell::FIRE_WALL);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY08), Spell::FORTUNE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY09), Spell::FORGETFULNESS);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY10), Spell::FORCE_FIELD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY11), Spell::FLY);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY12), Spell::FIREBALL);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY13), Spell::FRENZY);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY14), Spell::FROST_RING);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY15), Spell::HASTE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY16), Spell::HYPNOTIZE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY17), Spell::ICE_BOLT);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY18), Spell::MAGIC_ARROW);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY19), Spell::LIGHTNING_BOLT);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY20), Spell::LAND_MINE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY21), Spell::INFERNO);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY22), Spell::IMPLOSION);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY23), Spell::MAGIC_MIRROR);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY24), Spell::METEOR_SHOWER);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY25), Spell::MIRTH);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY26), Spell::MISFORTUNE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY27), Spell::PRAYER);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY28),
            Spell::PROTECTION_FROM_FIRE
        );
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY29),
            Spell::PROTECTION_FROM_AIR
        );
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY30),
            Spell::PROTECTION_FROM_WATER
        );
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY31),
            Spell::PROTECTION_FROM_EARTH
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY32), Spell::PRECISION);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY33), Spell::QUICKSAND);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY34),
            Spell::REMOVE_OBSTACLE
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY35), Spell::RESURRECTION);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY36), Spell::SACRIFICE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY37), Spell::SCUTTLE_BOAT);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY38), Spell::STONE_SKIN);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY39), Spell::SORROW);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY40), Spell::SLOW);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY41), Spell::SLAYER);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY42), Spell::SHIELD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY43), Spell::SUMMON_BOAT);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY44), Spell::TELEPORT);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY45), Spell::TOWN_PORTAL);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY46), Spell::VIEW_AIR);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY47), Spell::VIEW_EARTH);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY48), Spell::WEAKNESS);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY49), Spell::WATER_WALK);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY50),
            Spell::SUMMON_WATER_ELEMENTAL
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY51), Spell::VISIONS);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY52), Spell::DESTROY_UNDEAD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY53), Spell::DIMENSION_DOOR);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY54), Spell::DISGUISE);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY55), Spell::DISPEL);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY56), Spell::DISRUPTING_RAY);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY57),
            Spell::SUMMON_AIR_ELEMENTAL
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY58), Spell::BLESS);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY59), Spell::ANIMATE_DEAD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY60), Spell::ANTI_MAGIC);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY61), Spell::ARMAGEDDON);
        assert_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY62),
            Spell::CHAIN_LIGHTNING
        );
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY63), Spell::AIR_SHIELD);
        assert_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY64), Spell::BLOODLUST);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Lord Haart");
        assert_eq!(hero.helm(), Artifact::HELM_OF_THE_ALABASTER_UNICORN);
        assert_eq!(hero.cloak(), Artifact::RECANTERS_CLOAK);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::CENTAURS_AXE);
        assert_eq!(hero.shield(), Artifact::NONE); // Moved shield to inventory.
        assert_eq!(hero.armor(), Artifact::BREASTPLATE_OF_PETRIFIED_WOOD);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::SPIRIT_OF_OPRESSION);
        assert_eq!(hero.misc02(), Artifact::HOURGLASS_OF_THE_EVIL_HOUR);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::SHIELD_OF_THE_DWARVEN_LORDS
        );
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // - Adds Left ring: Ring of Suppression
        // - Adds Misc05: Hideous Mask
        // - Adds inventory01: Pendant of Downfall
        let hero = find_hero(&save, "Ryland");
        assert_eq!(hero.helm(), Artifact::NONE); // Blocked by Angelic Alliance
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Angelic Alliance
        assert_eq!(hero.weapon(), Artifact::ANGELIC_ALLIANCE);
        assert_eq!(hero.shield(), Artifact::NONE); // Blocked by Angelic Alliance
        assert_eq!(hero.armor(), Artifact::NONE); // Blocked by Angelic Alliance
        assert_eq!(hero.ring_left(), Artifact::RING_OF_SUPPRESSION);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE); // Blocked by Angelic Alliance
        assert_eq!(hero.misc01(), Artifact::ORB_OF_INHIBITION);
        assert_eq!(hero.misc02(), Artifact::RUNES_OF_IMMINENCY);
        assert_eq!(hero.misc03(), Artifact::DEMONS_HORSESHOE);
        assert_eq!(hero.misc04(), Artifact::SHAMANS_PUPPET);
        assert_eq!(hero.misc05(), Artifact::HIDEOUS_MASK);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::PENDANT_OF_DOWNFALL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // NOTE! "Torosar" is saved as "Torosar " in the save file.
        // Torosar was defeated.
        let hero = find_hero(&save, "Torosar ");
        assert_eq!(hero.player, 0xFF);
        assert_eq!(hero.on_map, 0);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Sorsha");
        assert_eq!(hero.helm(), Artifact::NONE); // Blocked by Armor of the Damned
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::NONE); // Blocked by Armor of the Damned
        assert_eq!(hero.shield(), Artifact::NONE); // Blocked by Armor of the Damned
        assert_eq!(hero.armor(), Artifact::ARMOR_OF_THE_DAMNED);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::BOOTS_OF_LEVITATION);
        assert_eq!(hero.misc01(), Artifact::TOME_OF_FIRE_MAGIC);
        assert_eq!(hero.misc02(), Artifact::TOME_OF_AIR_MAGIC);
        assert_eq!(hero.misc03(), Artifact::TOME_OF_WATER_MAGIC);
        assert_eq!(hero.misc04(), Artifact::TOME_OF_EARTH_MAGIC);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Ivor");
        assert_eq!(hero.helm(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.cloak(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.neck(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.weapon(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.shield(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.armor(), Artifact::POWER_OF_THE_DRAGON_FATHER);
        assert_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.boots(), Artifact::NONE); // Blocked by Power of the Dragon Father
        assert_eq!(hero.misc01(), Artifact::NONE);
        assert_eq!(hero.misc02(), Artifact::NONE);
        assert_eq!(hero.misc03(), Artifact::NONE);
        assert_eq!(hero.misc04(), Artifact::NONE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // Fafner was defeated.
        let hero = find_hero(&save, "Fafner");
        assert_eq!(hero.player, 0xFF);
        assert_eq!(hero.on_map, 0);
    }

    {
        // Tyris was defeated.
        let hero = find_hero(&save, "Tyris");
        assert_eq!(hero.player, 0xFF);
        assert_eq!(hero.on_map, 0);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Kyrre");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::PENDANT_OF_LIFE);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::CLOVER_OF_FORTUNE);
        assert_eq!(hero.misc02(), Artifact::CARDS_OF_PROPHECY);
        assert_eq!(hero.misc03(), Artifact::LADYBIRD_OF_LUCK);
        assert_eq!(hero.misc04(), Artifact::BADGE_OF_COURAGE);
        assert_eq!(hero.misc05(), Artifact::CREST_OF_VALOR);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::GLYPH_OF_GALLANTRY);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPECULUM);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPYGLASS);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::PENDANT_OF_DEATH);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::PENDANT_OF_FREE_WILL);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);
    }

    {
        // Rissa was defeated.
        let hero = find_hero(&save, "Rissa");
        assert_eq!(hero.player, 0xFF);
        assert_eq!(hero.on_map, 0);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Christian");
        assert_eq!(hero.helm(), Artifact::HELM_OF_CHAOS);
        assert_eq!(hero.cloak(), Artifact::NONE);
        assert_eq!(hero.neck(), Artifact::NONE);
        assert_eq!(hero.weapon(), Artifact::GREATER_GNOLLS_FLAIL);
        assert_eq!(hero.shield(), Artifact::BUCKLER_OF_THE_GNOLL_KING);
        assert_eq!(hero.armor(), Artifact::SCALES_OF_THE_GREATER_BASILISK);
        assert_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Elixir of Life
        assert_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Elixir of Life
        assert_eq!(hero.boots(), Artifact::NONE);
        assert_eq!(hero.misc01(), Artifact::GOLDEN_BOW);
        assert_eq!(hero.misc02(), Artifact::SPHERE_OF_PERMANENCE);
        assert_eq!(hero.misc03(), Artifact::ORB_OF_VULNERABILITY);
        assert_eq!(hero.misc04(), Artifact::ELIXIR_OF_LIFE);
        assert_eq!(hero.misc05(), Artifact::NONE);

        assert_eq!(hero.ballista(), Artifact::BALLISTA);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // Iona was defeated.
        let hero = find_hero(&save, "Iona");
        assert_eq!(hero.player, 0xFF);
        assert_eq!(hero.on_map, 0);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Giselle");
        assert_eq!(hero.helm(), Artifact::NONE);
        assert_eq!(hero.cloak(), Artifact::VAMPIRES_COWL);
        assert_eq!(hero.neck(), Artifact::AMULET_OF_THE_UNDERTAKER);
        assert_eq!(hero.weapon(), Artifact::NONE);
        assert_eq!(hero.shield(), Artifact::NONE);
        assert_eq!(hero.armor(), Artifact::NONE);
        assert_eq!(hero.ring_left(), Artifact::NONE);
        assert_eq!(hero.ring_right(), Artifact::NONE);
        assert_eq!(hero.boots(), Artifact::DEAD_MANS_BOOTS);
        assert_eq!(hero.misc01(), Artifact::BOW_OF_THE_SHARPSHOOTER);
        assert_eq!(hero.misc02(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        assert_eq!(hero.misc03(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        assert_eq!(hero.misc04(), Artifact::BIRD_OF_PERCEPTION);
        assert_eq!(hero.misc05(), Artifact::STOIC_WATCHMAN);

        assert_eq!(hero.ballista(), Artifact::NONE);
        assert_eq!(hero.ammo_cart(), Artifact::NONE);
        assert_eq!(hero.first_aid_tent(), Artifact::NONE);
        assert_eq!(hero.catapult(), Artifact::CATAPULT);
        assert_eq!(hero.spell_book(), Artifact::NONE);

        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::EMBLEM_OF_COGNIZANCE);
        assert_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }
}

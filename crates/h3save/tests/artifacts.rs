// Ported from the homm3tools C++ test test-artifacts.cpp. Expected values are unchanged.

#![expect(clippy::too_many_lines, reason = "one-to-one ports of long C++ test cases")]

#[macro_use]
mod common;

#[allow(unused_imports)]
use common::{data_dir, find_hero, find_town, open, test_creature};
#[allow(unused_imports)]
use h3save::stats;
#[allow(unused_imports)]
use h3save::{
    Artifact, ArtifactSlot, Color, Creature, Hero, MapSize, Orientation, SaveFile, Skill, SkillLevel, SkillSlot, Spell,
    Town,
};

fn verify_no_artifacts(hero: &Hero) {
    require_eq!(hero.helm(), Artifact::NONE);
    require_eq!(hero.cloak(), Artifact::NONE);
    require_eq!(hero.neck(), Artifact::NONE);
    require_eq!(hero.weapon(), Artifact::NONE);
    require_eq!(hero.shield(), Artifact::NONE);
    require_eq!(hero.armor(), Artifact::NONE);
    require_eq!(hero.ring_left(), Artifact::NONE);
    require_eq!(hero.ring_right(), Artifact::NONE);
    require_eq!(hero.boots(), Artifact::NONE);
    require_eq!(hero.misc01(), Artifact::NONE);
    require_eq!(hero.misc02(), Artifact::NONE);
    require_eq!(hero.misc03(), Artifact::NONE);
    require_eq!(hero.misc04(), Artifact::NONE);
    require_eq!(hero.misc05(), Artifact::NONE);
    require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
}

fn verify_catapult_only(hero: &Hero) {
    require_eq!(hero.ballista(), Artifact::NONE);
    require_eq!(hero.ammo_cart(), Artifact::NONE);
    require_eq!(hero.first_aid_tent(), Artifact::NONE);
    require_eq!(hero.catapult(), Artifact::CATAPULT);
    require_eq!(hero.spell_book(), Artifact::NONE);
}

fn verify_catapult_and_spell_book(hero: &Hero) {
    require_eq!(hero.ballista(), Artifact::NONE);
    require_eq!(hero.ammo_cart(), Artifact::NONE);
    require_eq!(hero.first_aid_tent(), Artifact::NONE);
    require_eq!(hero.catapult(), Artifact::CATAPULT);
    require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);
}

#[test]
fn artifacts_day1() {
    let _guard = common::Guard::new("artifacts-day1");
    let save = open(&data_dir("artifacts").join("[hotseat] 111.GM8"));
    require!(save.valid());

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
        require_eq!(hero.ballista(), Artifact::CANNON);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);
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
        require_eq!(hero.ballista(), Artifact::BALLISTA);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);
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
        require_eq!(hero.ballista(), Artifact::BALLISTA);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Iona");
        verify_no_artifacts(hero);
        verify_catapult_and_spell_book(hero);
    }

    {
        let hero = find_hero(&save, "Giselle");
        verify_no_artifacts(hero);
        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);
    }
}

#[test]
fn artifacts_day2() {
    let _guard = common::Guard::new("artifacts-day2");
    let save = open(&data_dir("artifacts").join("[hotseat] 112.GM8"));
    require!(save.valid());

    {
        let hero = find_hero(&save, "Orrin");
        require_eq!(hero.helm(), Artifact::ADMIRALS_HAT);
        require_eq!(hero.cloak(), Artifact::NONE); // Blocked by Cornucopia
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Admiral's Hat
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Cornucopia
        require_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Cornucopia
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::BOW_OF_THE_SHARPSHOOTER);
        require_eq!(hero.misc02(), Artifact::CORNUCOPIA);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        require_eq!(hero.misc05(), Artifact::NONE); // Blocked by Bow of the Sharpshooter

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::IRONFIST_OF_THE_OGRE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::GOLDEN_GOOSE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::WIZARDS_WELL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::RING_OF_THE_MAGI);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::DIPLOMATS_CLOAK);
        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY06),
            Artifact::PENDANT_OF_REFLECTION
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY07), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Mephala");
        require_eq!(hero.helm(), Artifact::CROWN_OF_THE_SUPREME_MAGI);
        require_eq!(hero.cloak(), Artifact::CAPE_OF_VELOCITY);
        require_eq!(hero.neck(), Artifact::NECKLACE_OF_SWIFTNESS);
        require_eq!(hero.weapon(), Artifact::OGRES_CLUB_OF_HAVOC);
        require_eq!(hero.shield(), Artifact::TARG_OF_THE_RAMPAGING_OGRE);
        require_eq!(hero.armor(), Artifact::TUNIC_OF_THE_CYCLOPS_KING);
        require_eq!(hero.ring_left(), Artifact::RING_OF_OBLIVION);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::BOOTS_OF_SPEED);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::CAPE_OF_SILENCE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Piquedram");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::AMBASSADORS_SASH);
        require_eq!(hero.neck(), Artifact::STATESMANS_MEDAL);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::DIPLOMATS_RING);
        require_eq!(hero.ring_right(), Artifact::RING_OF_THE_WAYFARER);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::EQUESTRIANS_GLOVES);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Fiona");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::SURCOAT_OF_COUNTERPOISE);
        require_eq!(hero.neck(), Artifact::GARNITURE_OF_INTERFERENCE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::BOOTS_OF_POLARITY);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::BALLISTA);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Valeska");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::CLOAK_OF_THE_UNDEAD_KING);
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Cloak of the Undead King
        require_eq!(hero.weapon(), Artifact::ARMAGEDDONS_BLADE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Elixir of Life
        require_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Elixir of Life
        require_eq!(hero.boots(), Artifact::NONE); // Blocked by Cloak of the Undead King
        require_eq!(hero.misc01(), Artifact::VIAL_OF_DRAGON_BLOOD);
        require_eq!(hero.misc02(), Artifact::HORN_OF_THE_ABYSS);
        require_eq!(hero.misc03(), Artifact::ELIXIR_OF_LIFE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::STATUE_OF_LEGION);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::ANGELIC_ALLIANCE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::ARMOR_OF_THE_DAMNED);
        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY04),
            Artifact::POWER_OF_THE_DRAGON_FATHER
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::TITANS_THUNDER);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Ufretin");
        require_eq!(hero.helm(), Artifact::HELLSTORM_HELMET);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::PENDANT_OF_NEGATIVITY);
        require_eq!(hero.weapon(), Artifact::SWORD_OF_HELLFIRE);
        require_eq!(hero.shield(), Artifact::SHIELD_OF_THE_DAMNED);
        require_eq!(hero.armor(), Artifact::BREASTPLATE_OF_BRIMSTONE);
        require_eq!(hero.ring_left(), Artifact::RING_OF_INFINITE_GEMS);
        require_eq!(hero.ring_right(), Artifact::EVERSMOKING_RING_OF_SULFUR);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::EVERPOURING_VIAL_OF_MERCURY);
        require_eq!(hero.misc02(), Artifact::INEXHAUSTIBLE_CART_OF_ORE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::PENDANT_OF_DISPASSION
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Thane");
        verify_no_artifacts(hero);
        verify_catapult_and_spell_book(hero);
    }

    {
        let hero = find_hero(&save, "Jeremy");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::CANNON);
        require_eq!(hero.ammo_cart(), Artifact::AMMO_CART);
        require_eq!(hero.first_aid_tent(), Artifact::FIRST_AID_TENT);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Edric");
        require_eq!(hero.helm(), Artifact::SPELLBINDERS_HAT);
        require_eq!(hero.cloak(), Artifact::CAPE_OF_CONJURING);
        require_eq!(hero.neck(), Artifact::COLLAR_OF_CONJURING);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::RING_OF_CONJURING);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::ORB_OF_FIRMAMENT);
        require_eq!(hero.misc02(), Artifact::ORB_OF_SILT);
        require_eq!(hero.misc03(), Artifact::ORB_OF_TEMPESTUOUS_FIRE);
        require_eq!(hero.misc04(), Artifact::ORB_OF_DRIVING_RAIN);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Clancy");
        require_eq!(hero.helm(), Artifact::CROWN_OF_THE_FIVE_SEAS);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::PENDANT_OF_SECOND_SIGHT);
        require_eq!(hero.weapon(), Artifact::TRIDENT_OF_DOMINION);
        require_eq!(hero.shield(), Artifact::SHIELD_OF_NAVAL_GLORY);
        require_eq!(hero.armor(), Artifact::ROYAL_ARMOR_OF_NIX);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::ENDLESS_SACK_OF_GOLD);
        require_eq!(hero.misc02(), Artifact::ENDLESS_BAG_OF_GOLD);
        require_eq!(hero.misc03(), Artifact::ENDLESS_PURSE_OF_GOLD);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::PENDANT_OF_TOTAL_RECALL
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Josephine");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::MISC01),
            Spell::SUMMON_AIR_ELEMENTAL
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::AIR_SHIELD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::ANIMATE_DEAD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::ANTI_MAGIC);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::ARMAGEDDON);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::BERSERK);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::BLESS);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::BLIND);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::BLOODLUST);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY05),
            Spell::CHAIN_LIGHTNING
        );
    }

    {
        let hero = find_hero(&save, "Sylvia");
        require_eq!(hero.helm(), Artifact::SEA_CAPTAINS_HAT);
        require_eq!(hero.cloak(), Artifact::ANGEL_WINGS);
        require_eq!(hero.neck(), Artifact::NECKLACE_OF_OCEAN_GUIDANCE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::PLATE_OF_DYING_LIGHT);
        require_eq!(hero.ring_left(), Artifact::SEAL_OF_SUNSET);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::CHARM_OF_MANA);
        require_eq!(hero.misc02(), Artifact::TALISMAN_OF_MANA);
        require_eq!(hero.misc03(), Artifact::MYSTIC_ORB_OF_MANA);
        require_eq!(hero.misc04(), Artifact::SHACKLES_OF_WAR);
        require_eq!(hero.misc05(), Artifact::CHARM_OF_ECLIPSE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Jenova");
        require_eq!(hero.helm(), Artifact::THUNDER_HELMET);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::PENDANT_OF_HOLINESS);
        require_eq!(hero.weapon(), Artifact::TITANS_GLADIUS);
        require_eq!(hero.shield(), Artifact::SENTINELS_SHIELD);
        require_eq!(hero.armor(), Artifact::TITANS_CUIRASS);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::LEGS_OF_LEGION);
        require_eq!(hero.misc02(), Artifact::LOINS_OF_LEGION);
        require_eq!(hero.misc03(), Artifact::TORSO_OF_LEGION);
        require_eq!(hero.misc04(), Artifact::ARMS_OF_LEGION);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::PENDANT_OF_COURAGE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Neela");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::CLONE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::COUNTERSTRIKE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::CURE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::CURSE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::DEATH_RIPPLE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::DESTROY_UNDEAD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::DIMENSION_DOOR);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::DISGUISE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::DISPEL);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::DISRUPTING_RAY);
    }

    {
        let hero = find_hero(&save, "Lord Haart");
        require_eq!(hero.helm(), Artifact::HELM_OF_THE_ALABASTER_UNICORN);
        require_eq!(hero.cloak(), Artifact::RECANTERS_CLOAK);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::CENTAURS_AXE);
        require_eq!(hero.shield(), Artifact::SHIELD_OF_THE_DWARVEN_LORDS);
        require_eq!(hero.armor(), Artifact::BREASTPLATE_OF_PETRIFIED_WOOD);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPIRIT_OF_OPRESSION);
        require_eq!(hero.misc02(), Artifact::HOURGLASS_OF_THE_EVIL_HOUR);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Ryland");
        require_eq!(hero.helm(), Artifact::HELM_OF_HEAVENLY_ENLIGHTENMENT);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::CELESTIAL_NECKLACE_OF_BLISS);
        require_eq!(hero.weapon(), Artifact::SWORD_OF_JUDGEMENT);
        require_eq!(hero.shield(), Artifact::LIONS_SHIELD_OF_COURAGE);
        require_eq!(hero.armor(), Artifact::ARMOR_OF_WONDER);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::SANDALS_OF_THE_SAINT);
        require_eq!(hero.misc01(), Artifact::ORB_OF_INHIBITION);
        require_eq!(hero.misc02(), Artifact::RUNES_OF_IMMINENCY);
        require_eq!(hero.misc03(), Artifact::DEMONS_HORSESHOE);
        require_eq!(hero.misc04(), Artifact::SHAMANS_PUPPET);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // NOTE! "Torosar" is saved as "Torosar " in the save file.
        let hero = find_hero(&save, "Torosar ");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::MISC01),
            Spell::SUMMON_EARTH_ELEMENTAL
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::EARTHQUAKE);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::MISC03),
            Spell::SUMMON_FIRE_ELEMENTAL
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::FIRE_SHIELD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::FIRE_WALL);

        require_eq!(hero.ballista(), Artifact::BALLISTA);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::FIREBALL);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::FLY);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::FORCE_FIELD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::FORGETFULNESS);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::FORTUNE);
    }

    {
        let hero = find_hero(&save, "Sorsha");
        require_eq!(hero.helm(), Artifact::SKULL_HELMET);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::BLACKSHARD_OF_THE_DEAD_KNIGHT);
        require_eq!(hero.shield(), Artifact::SHIELD_OF_THE_YAWNING_DEAD);
        require_eq!(hero.armor(), Artifact::RIB_CAGE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::BOOTS_OF_LEVITATION);
        require_eq!(hero.misc01(), Artifact::TOME_OF_FIRE_MAGIC);
        require_eq!(hero.misc02(), Artifact::TOME_OF_AIR_MAGIC);
        require_eq!(hero.misc03(), Artifact::TOME_OF_WATER_MAGIC);
        require_eq!(hero.misc04(), Artifact::TOME_OF_EARTH_MAGIC);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Ivor");
        require_eq!(hero.helm(), Artifact::CROWN_OF_DRAGON_TOOTH);
        require_eq!(hero.cloak(), Artifact::DRAGON_WING_TABARD);
        require_eq!(hero.neck(), Artifact::NECKLACE_OF_DRAGONTEETH);
        require_eq!(hero.weapon(), Artifact::RED_DRAGON_FLAME_TONGUE);
        require_eq!(hero.shield(), Artifact::DRAGON_SCALE_SHIELD);
        require_eq!(hero.armor(), Artifact::DRAGON_SCALE_ARMOR);
        require_eq!(hero.ring_left(), Artifact::QUIET_EYE_OF_THE_DRAGON);
        require_eq!(hero.ring_right(), Artifact::STILL_EYE_OF_THE_DRAGON);
        require_eq!(hero.boots(), Artifact::DRAGONBONE_GREAVES);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Fafner");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::FRENZY);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::FROST_RING);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::HASTE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::HYPNOTIZE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::ICE_BOLT);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::IMPLOSION);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::INFERNO);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::LAND_MINE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::LIGHTNING_BOLT);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::MAGIC_ARROW);
    }

    {
        let hero = find_hero(&save, "Tyris");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::SUMMON_BOAT);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::TELEPORT);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::TOWN_PORTAL);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::VIEW_AIR);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::VIEW_EARTH);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::NONE);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::VISIONS);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY02),
            Spell::SUMMON_WATER_ELEMENTAL
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::WATER_WALK);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::WEAKNESS);
    }

    {
        let hero = find_hero(&save, "Kyrre");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::PENDANT_OF_LIFE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::CLOVER_OF_FORTUNE);
        require_eq!(hero.misc02(), Artifact::CARDS_OF_PROPHECY);
        require_eq!(hero.misc03(), Artifact::LADYBIRD_OF_LUCK);
        require_eq!(hero.misc04(), Artifact::BADGE_OF_COURAGE);
        require_eq!(hero.misc05(), Artifact::CREST_OF_VALOR);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::GLYPH_OF_GALLANTRY);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPECULUM);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPYGLASS);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::PENDANT_OF_DEATH);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::PENDANT_OF_FREE_WILL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Rissa");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::MAGIC_MIRROR);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::METEOR_SHOWER);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::MIRTH);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::MISFORTUNE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::PRAYER);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::PRECISION);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY02),
            Spell::PROTECTION_FROM_EARTH
        );
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY03),
            Spell::PROTECTION_FROM_WATER
        );
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY04),
            Spell::PROTECTION_FROM_AIR
        );
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY05),
            Spell::PROTECTION_FROM_FIRE
        );
    }

    {
        let hero = find_hero(&save, "Christian");
        require_eq!(hero.helm(), Artifact::HELM_OF_CHAOS);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::GREATER_GNOLLS_FLAIL);
        require_eq!(hero.shield(), Artifact::BUCKLER_OF_THE_GNOLL_KING);
        require_eq!(hero.armor(), Artifact::SCALES_OF_THE_GREATER_BASILISK);
        require_eq!(hero.ring_left(), Artifact::RING_OF_VITALITY);
        require_eq!(hero.ring_right(), Artifact::RING_OF_LIFE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::GOLDEN_BOW);
        require_eq!(hero.misc02(), Artifact::SPHERE_OF_PERMANENCE);
        require_eq!(hero.misc03(), Artifact::ORB_OF_VULNERABILITY);
        require_eq!(hero.misc04(), Artifact::VIAL_OF_LIFEBLOOED);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::BALLISTA);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Iona");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::QUICKSAND);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::REMOVE_OBSTACLE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::RESURRECTION);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::SACRIFICE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::SCUTTLE_BOAT);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::SHIELD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::SLAYER);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::SLOW);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::SORROW);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::STONE_SKIN);
    }

    {
        let hero = find_hero(&save, "Giselle");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::VAMPIRES_COWL);
        require_eq!(hero.neck(), Artifact::AMULET_OF_THE_UNDERTAKER);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::DEAD_MANS_BOOTS);
        require_eq!(hero.misc01(), Artifact::BOW_OF_ELVEN_CHERRY_WOOD);
        require_eq!(hero.misc02(), Artifact::BOWSTRING_OF_THE_UNICORNS_MANE);
        require_eq!(hero.misc03(), Artifact::ANGEL_FEATHER_ARROWS);
        require_eq!(hero.misc04(), Artifact::BIRD_OF_PERCEPTION);
        require_eq!(hero.misc05(), Artifact::STOIC_WATCHMAN);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::EMBLEM_OF_COGNIZANCE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }
}

// Same as day 2, but with some minor differences.
#[test]
fn artifacts_day3() {
    let _guard = common::Guard::new("artifacts-day3");
    let save = open(&data_dir("artifacts").join("[hotseat] 113.GM8"));
    require!(save.valid());

    {
        let hero = find_hero(&save, "Orrin");
        require_eq!(hero.helm(), Artifact::ADMIRALS_HAT);
        require_eq!(hero.cloak(), Artifact::NONE); // Blocked by Cornucopia
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Admiral's Hat
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Cornucopia
        require_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Cornucopia
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::BOW_OF_THE_SHARPSHOOTER);
        require_eq!(hero.misc02(), Artifact::CORNUCOPIA);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        require_eq!(hero.misc05(), Artifact::NONE); // Blocked by Bow of the Sharpshooter

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        // Diplomat's Cloak and Ring of the Magi are swapped. The inventory is also shuffled/scrolled
        // so that the artifact in the first slot is noe Wizards Well.
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::WIZARDS_WELL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::DIPLOMATS_CLOAK); // Swapped
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::RING_OF_THE_MAGI); // Swapped
        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY04),
            Artifact::PENDANT_OF_REFLECTION
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::IRONFIST_OF_THE_OGRE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::GOLDEN_GOOSE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY07), Artifact::NONE);
    }

    {
        // AssemblesIronfist of the Ogre.
        let hero = find_hero(&save, "Mephala");
        require_eq!(hero.helm(), Artifact::NONE); // Blocked by Ironfist of the Ogre
        require_eq!(hero.cloak(), Artifact::CAPE_OF_SILENCE); // Swapped
        require_eq!(hero.neck(), Artifact::NECKLACE_OF_SWIFTNESS);
        require_eq!(hero.weapon(), Artifact::IRONFIST_OF_THE_OGRE);
        require_eq!(hero.shield(), Artifact::NONE); // Blocked by Ironfist of the Ogre
        require_eq!(hero.armor(), Artifact::NONE); // Blocked by Ironfist of the Ogre
        require_eq!(hero.ring_left(), Artifact::RING_OF_OBLIVION);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::BOOTS_OF_SPEED);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::CAPE_OF_VELOCITY); // Swapped
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // Assemble Diplomat's Cloak.
        let hero = find_hero(&save, "Piquedram");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::DIPLOMATS_CLOAK);
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Diplomat's Cloak
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Diplomat's Cloak
        require_eq!(hero.ring_right(), Artifact::RING_OF_THE_WAYFARER);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::EQUESTRIANS_GLOVES);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Fiona");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE); // Blocked by Pendant of Reflection
        require_eq!(hero.neck(), Artifact::PENDANT_OF_REFLECTION);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE); // Blocked by Pendant of Reflection
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::BALLISTA);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Valeska");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::CLOAK_OF_THE_UNDEAD_KING);
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Cloak of the Undead King
        require_eq!(hero.weapon(), Artifact::ARMAGEDDONS_BLADE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Elixir of Life
        require_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Elixir of Life
        require_eq!(hero.boots(), Artifact::NONE); // Blocked by Cloak of the Undead King
        require_eq!(hero.misc01(), Artifact::VIAL_OF_DRAGON_BLOOD);
        require_eq!(hero.misc02(), Artifact::HORN_OF_THE_ABYSS);
        require_eq!(hero.misc03(), Artifact::ELIXIR_OF_LIFE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        // Inventory01 was moved to Inventory05.
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::ANGELIC_ALLIANCE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::ARMOR_OF_THE_DAMNED);
        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY03),
            Artifact::POWER_OF_THE_DRAGON_FATHER
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::TITANS_THUNDER);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::STATUE_OF_LEGION); // Moved from Inventory01
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);
    }

    {
        // Swap neck slot item.
        let hero = find_hero(&save, "Ufretin");
        require_eq!(hero.helm(), Artifact::HELLSTORM_HELMET);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::PENDANT_OF_DISPASSION); // Swapped
        require_eq!(hero.weapon(), Artifact::SWORD_OF_HELLFIRE);
        require_eq!(hero.shield(), Artifact::SHIELD_OF_THE_DAMNED);
        require_eq!(hero.armor(), Artifact::BREASTPLATE_OF_BRIMSTONE);
        require_eq!(hero.ring_left(), Artifact::RING_OF_INFINITE_GEMS);
        require_eq!(hero.ring_right(), Artifact::EVERSMOKING_RING_OF_SULFUR);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::EVERPOURING_VIAL_OF_MERCURY);
        require_eq!(hero.misc02(), Artifact::INEXHAUSTIBLE_CART_OF_ORE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::PENDANT_OF_NEGATIVITY
        ); // Swapped
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // Thane now has the Grail.
        let hero = find_hero(&save, "Thane");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::GRAIL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Jeremy");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::CANNON);
        require_eq!(hero.ammo_cart(), Artifact::AMMO_CART);
        require_eq!(hero.first_aid_tent(), Artifact::FIRST_AID_TENT);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // Assemble Ring of the Magi.
        let hero = find_hero(&save, "Edric");
        require_eq!(hero.helm(), Artifact::SPELLBINDERS_HAT);
        require_eq!(hero.cloak(), Artifact::NONE); // Blocked by Ring of the Magi
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Ring of the Magi
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::RING_OF_THE_MAGI);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::ORB_OF_FIRMAMENT);
        require_eq!(hero.misc02(), Artifact::ORB_OF_SILT);
        require_eq!(hero.misc03(), Artifact::ORB_OF_TEMPESTUOUS_FIRE);
        require_eq!(hero.misc04(), Artifact::ORB_OF_DRIVING_RAIN);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // Assemble Golden Goose.
        let hero = find_hero(&save, "Clancy");
        require_eq!(hero.helm(), Artifact::CROWN_OF_THE_FIVE_SEAS);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::PENDANT_OF_SECOND_SIGHT);
        require_eq!(hero.weapon(), Artifact::TRIDENT_OF_DOMINION);
        require_eq!(hero.shield(), Artifact::SHIELD_OF_NAVAL_GLORY);
        require_eq!(hero.armor(), Artifact::ROYAL_ARMOR_OF_NIX);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::GOLDEN_GOOSE);
        require_eq!(hero.misc02(), Artifact::NONE); // Blocked by Golden Goose
        require_eq!(hero.misc03(), Artifact::NONE); // Blocked by Golden Goose
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::PENDANT_OF_TOTAL_RECALL
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // Swap Bless and Air Shield, put Air Shield in inventory04
        let hero = find_hero(&save, "Josephine");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::MISC01),
            Spell::SUMMON_AIR_ELEMENTAL
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::BLESS); // Swapped
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::ANIMATE_DEAD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::ANTI_MAGIC);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::ARMAGEDDON);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::BERSERK);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::BLIND);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::BLOODLUST);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::AIR_SHIELD);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY05),
            Spell::CHAIN_LIGHTNING
        );
    }

    {
        // Assemble Wizard's Well.
        let hero = find_hero(&save, "Sylvia");
        require_eq!(hero.helm(), Artifact::SEA_CAPTAINS_HAT);
        require_eq!(hero.cloak(), Artifact::ANGEL_WINGS);
        require_eq!(hero.neck(), Artifact::NECKLACE_OF_OCEAN_GUIDANCE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::PLATE_OF_DYING_LIGHT);
        require_eq!(hero.ring_left(), Artifact::SEAL_OF_SUNSET);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::WIZARDS_WELL);
        require_eq!(hero.misc02(), Artifact::NONE); // Blocked by Wizard's Well
        require_eq!(hero.misc03(), Artifact::NONE); // Blocked by Wizard's Well
        require_eq!(hero.misc04(), Artifact::SHACKLES_OF_WAR);
        require_eq!(hero.misc05(), Artifact::CHARM_OF_ECLIPSE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // Add Head of Legion to Misc05
        // Add Wayfarer's Boots to Boots
        let hero = find_hero(&save, "Jenova");
        require_eq!(hero.helm(), Artifact::THUNDER_HELMET);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::PENDANT_OF_HOLINESS);
        require_eq!(hero.weapon(), Artifact::TITANS_GLADIUS);
        require_eq!(hero.shield(), Artifact::SENTINELS_SHIELD);
        require_eq!(hero.armor(), Artifact::TITANS_CUIRASS);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::WAYFARERS_BOOTS);
        require_eq!(hero.misc01(), Artifact::LEGS_OF_LEGION);
        require_eq!(hero.misc02(), Artifact::LOINS_OF_LEGION);
        require_eq!(hero.misc03(), Artifact::TORSO_OF_LEGION);
        require_eq!(hero.misc04(), Artifact::ARMS_OF_LEGION);
        require_eq!(hero.misc05(), Artifact::HEAD_OF_LEGION);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::PENDANT_OF_COURAGE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Neela");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::CLONE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::COUNTERSTRIKE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::CURE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::CURSE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::DEATH_RIPPLE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::DESTROY_UNDEAD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::DIMENSION_DOOR);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::DISGUISE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::DISPEL);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::DISRUPTING_RAY);
    }

    {
        let hero = find_hero(&save, "Lord Haart");
        require_eq!(hero.helm(), Artifact::HELM_OF_THE_ALABASTER_UNICORN);
        require_eq!(hero.cloak(), Artifact::RECANTERS_CLOAK);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::CENTAURS_AXE);
        require_eq!(hero.shield(), Artifact::NONE); // Moved shield to inventory.
        require_eq!(hero.armor(), Artifact::BREASTPLATE_OF_PETRIFIED_WOOD);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPIRIT_OF_OPRESSION);
        require_eq!(hero.misc02(), Artifact::HOURGLASS_OF_THE_EVIL_HOUR);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::SHIELD_OF_THE_DWARVEN_LORDS
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // Assemble Angelic Alliance.
        let hero = find_hero(&save, "Ryland");
        require_eq!(hero.helm(), Artifact::NONE); // Blocked by Angelic Alliance
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Angelic Alliance
        require_eq!(hero.weapon(), Artifact::ANGELIC_ALLIANCE);
        require_eq!(hero.shield(), Artifact::NONE); // Blocked by Angelic Alliance
        require_eq!(hero.armor(), Artifact::NONE); // Blocked by Angelic Alliance
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE); // Blocked by Angelic Alliance
        require_eq!(hero.misc01(), Artifact::ORB_OF_INHIBITION);
        require_eq!(hero.misc02(), Artifact::RUNES_OF_IMMINENCY);
        require_eq!(hero.misc03(), Artifact::DEMONS_HORSESHOE);
        require_eq!(hero.misc04(), Artifact::SHAMANS_PUPPET);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // NOTE! "Torosar" is saved as "Torosar " in the save file.
        // No changes.
        let hero = find_hero(&save, "Torosar ");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::MISC01),
            Spell::SUMMON_EARTH_ELEMENTAL
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::EARTHQUAKE);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::MISC03),
            Spell::SUMMON_FIRE_ELEMENTAL
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::FIRE_SHIELD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::FIRE_WALL);

        require_eq!(hero.ballista(), Artifact::BALLISTA);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::FIREBALL);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::FLY);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::FORCE_FIELD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::FORGETFULNESS);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::FORTUNE);
    }

    {
        // Assemble Armor of the Damned.
        let hero = find_hero(&save, "Sorsha");
        require_eq!(hero.helm(), Artifact::NONE); // Blocked by Armor of the Damned
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE); // Blocked by Armor of the Damned
        require_eq!(hero.shield(), Artifact::NONE); // Blocked by Armor of the Damned
        require_eq!(hero.armor(), Artifact::ARMOR_OF_THE_DAMNED);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::BOOTS_OF_LEVITATION);
        require_eq!(hero.misc01(), Artifact::TOME_OF_FIRE_MAGIC);
        require_eq!(hero.misc02(), Artifact::TOME_OF_AIR_MAGIC);
        require_eq!(hero.misc03(), Artifact::TOME_OF_WATER_MAGIC);
        require_eq!(hero.misc04(), Artifact::TOME_OF_EARTH_MAGIC);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        let hero = find_hero(&save, "Ivor");
        require_eq!(hero.helm(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.cloak(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.weapon(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.shield(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.armor(), Artifact::POWER_OF_THE_DRAGON_FATHER);
        require_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.boots(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Fafner");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::FRENZY);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::FROST_RING);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::HASTE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::HYPNOTIZE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::ICE_BOLT);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::IMPLOSION);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::INFERNO);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::LAND_MINE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::LIGHTNING_BOLT);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::MAGIC_ARROW);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Tyris");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::SUMMON_BOAT);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::TELEPORT);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::TOWN_PORTAL);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::VIEW_AIR);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::VIEW_EARTH);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::NONE);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::VISIONS);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY02),
            Spell::SUMMON_WATER_ELEMENTAL
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::WATER_WALK);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::WEAKNESS);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Kyrre");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::PENDANT_OF_LIFE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::CLOVER_OF_FORTUNE);
        require_eq!(hero.misc02(), Artifact::CARDS_OF_PROPHECY);
        require_eq!(hero.misc03(), Artifact::LADYBIRD_OF_LUCK);
        require_eq!(hero.misc04(), Artifact::BADGE_OF_COURAGE);
        require_eq!(hero.misc05(), Artifact::CREST_OF_VALOR);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::GLYPH_OF_GALLANTRY);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPECULUM);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPYGLASS);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::PENDANT_OF_DEATH);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::PENDANT_OF_FREE_WILL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Rissa");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::MAGIC_MIRROR);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::METEOR_SHOWER);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::MIRTH);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::MISFORTUNE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::PRAYER);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::PRECISION);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY02),
            Spell::PROTECTION_FROM_EARTH
        );
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY03),
            Spell::PROTECTION_FROM_WATER
        );
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY04),
            Spell::PROTECTION_FROM_AIR
        );
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY05),
            Spell::PROTECTION_FROM_FIRE
        );
    }

    {
        // Asemble Elixir of Life.
        let hero = find_hero(&save, "Christian");
        require_eq!(hero.helm(), Artifact::HELM_OF_CHAOS);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::GREATER_GNOLLS_FLAIL);
        require_eq!(hero.shield(), Artifact::BUCKLER_OF_THE_GNOLL_KING);
        require_eq!(hero.armor(), Artifact::SCALES_OF_THE_GREATER_BASILISK);
        require_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Elixir of Life
        require_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Elixir of Life
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::GOLDEN_BOW);
        require_eq!(hero.misc02(), Artifact::SPHERE_OF_PERMANENCE);
        require_eq!(hero.misc03(), Artifact::ORB_OF_VULNERABILITY);
        require_eq!(hero.misc04(), Artifact::ELIXIR_OF_LIFE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::BALLISTA);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Iona");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::QUICKSAND);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::REMOVE_OBSTACLE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::RESURRECTION);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::SACRIFICE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::SCUTTLE_BOAT);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::SPELL_SCROLL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::SHIELD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::SLAYER);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY03), Spell::SLOW);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::SORROW);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY05), Spell::STONE_SKIN);
    }

    {
        // Assemble Bow of the Sharpshooter.
        let hero = find_hero(&save, "Giselle");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::VAMPIRES_COWL);
        require_eq!(hero.neck(), Artifact::AMULET_OF_THE_UNDERTAKER);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::DEAD_MANS_BOOTS);
        require_eq!(hero.misc01(), Artifact::BOW_OF_THE_SHARPSHOOTER);
        require_eq!(hero.misc02(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        require_eq!(hero.misc03(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        require_eq!(hero.misc04(), Artifact::BIRD_OF_PERCEPTION);
        require_eq!(hero.misc05(), Artifact::STOIC_WATCHMAN);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::EMBLEM_OF_COGNIZANCE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }
}

// Same as day 3, but with some minor differences.
#[test]
fn artifacts_day4() {
    let _guard = common::Guard::new("artifacts-day4");
    let save = open(&data_dir("artifacts").join("[hotseat] 114.GM8"));
    require!(save.valid());

    {
        // No changes.
        let hero = find_hero(&save, "Orrin");
        require_eq!(hero.helm(), Artifact::ADMIRALS_HAT);
        require_eq!(hero.cloak(), Artifact::NONE); // Blocked by Cornucopia
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Admiral's Hat
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Cornucopia
        require_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Cornucopia
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::BOW_OF_THE_SHARPSHOOTER);
        require_eq!(hero.misc02(), Artifact::CORNUCOPIA);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        require_eq!(hero.misc05(), Artifact::NONE); // Blocked by Bow of the Sharpshooter

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        // Diplomat's Cloak and Ring of the Magi are swapped. The inventory is also shuffled/scrolled
        // so that the artifact in the first slot is noe Wizards Well.
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::WIZARDS_WELL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::DIPLOMATS_CLOAK); // Swapped
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::RING_OF_THE_MAGI); // Swapped
        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY04),
            Artifact::PENDANT_OF_REFLECTION
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::IRONFIST_OF_THE_OGRE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::GOLDEN_GOOSE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY07), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Mephala");
        require_eq!(hero.helm(), Artifact::NONE); // Blocked by Ironfist of the Ogre
        require_eq!(hero.cloak(), Artifact::CAPE_OF_SILENCE); // Swapped
        require_eq!(hero.neck(), Artifact::NECKLACE_OF_SWIFTNESS);
        require_eq!(hero.weapon(), Artifact::IRONFIST_OF_THE_OGRE);
        require_eq!(hero.shield(), Artifact::NONE); // Blocked by Ironfist of the Ogre
        require_eq!(hero.armor(), Artifact::NONE); // Blocked by Ironfist of the Ogre
        require_eq!(hero.ring_left(), Artifact::RING_OF_OBLIVION);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::BOOTS_OF_SPEED);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::CAPE_OF_VELOCITY); // Swapped
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Piquedram");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::DIPLOMATS_CLOAK);
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Diplomat's Cloak
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Diplomat's Cloak
        require_eq!(hero.ring_right(), Artifact::RING_OF_THE_WAYFARER);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::EQUESTRIANS_GLOVES);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Fiona");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE); // Blocked by Pendant of Reflection
        require_eq!(hero.neck(), Artifact::PENDANT_OF_REFLECTION);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE); // Blocked by Pendant of Reflection
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::BALLISTA);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Valeska");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::CLOAK_OF_THE_UNDEAD_KING);
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Cloak of the Undead King
        require_eq!(hero.weapon(), Artifact::ARMAGEDDONS_BLADE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Elixir of Life
        require_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Elixir of Life
        require_eq!(hero.boots(), Artifact::NONE); // Blocked by Cloak of the Undead King
        require_eq!(hero.misc01(), Artifact::VIAL_OF_DRAGON_BLOOD);
        require_eq!(hero.misc02(), Artifact::HORN_OF_THE_ABYSS);
        require_eq!(hero.misc03(), Artifact::ELIXIR_OF_LIFE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        // Inventory01 was moved to Inventory05.
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::ANGELIC_ALLIANCE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::ARMOR_OF_THE_DAMNED);
        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY03),
            Artifact::POWER_OF_THE_DRAGON_FATHER
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::TITANS_THUNDER);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::STATUE_OF_LEGION); // Moved from Inventory01
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);
    }

    {
        // Pick up two more artifacts.
        let hero = find_hero(&save, "Ufretin");
        require_eq!(hero.helm(), Artifact::HELLSTORM_HELMET);
        require_eq!(hero.cloak(), Artifact::EVERFLOWING_CRYSTAL_CLOAK); // Added
        require_eq!(hero.neck(), Artifact::PENDANT_OF_DISPASSION);
        require_eq!(hero.weapon(), Artifact::SWORD_OF_HELLFIRE);
        require_eq!(hero.shield(), Artifact::SHIELD_OF_THE_DAMNED);
        require_eq!(hero.armor(), Artifact::BREASTPLATE_OF_BRIMSTONE);
        require_eq!(hero.ring_left(), Artifact::RING_OF_INFINITE_GEMS);
        require_eq!(hero.ring_right(), Artifact::EVERSMOKING_RING_OF_SULFUR);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::EVERPOURING_VIAL_OF_MERCURY);
        require_eq!(hero.misc02(), Artifact::INEXHAUSTIBLE_CART_OF_ORE);
        require_eq!(hero.misc03(), Artifact::INEXHAUSTIBLE_CART_OF_LUMBER); // Added
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::PENDANT_OF_NEGATIVITY
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Thane");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::GRAIL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Jeremy");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::CANNON);
        require_eq!(hero.ammo_cart(), Artifact::AMMO_CART);
        require_eq!(hero.first_aid_tent(), Artifact::FIRST_AID_TENT);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Edric");
        require_eq!(hero.helm(), Artifact::SPELLBINDERS_HAT);
        require_eq!(hero.cloak(), Artifact::NONE); // Blocked by Ring of the Magi
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Ring of the Magi
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::RING_OF_THE_MAGI);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::ORB_OF_FIRMAMENT);
        require_eq!(hero.misc02(), Artifact::ORB_OF_SILT);
        require_eq!(hero.misc03(), Artifact::ORB_OF_TEMPESTUOUS_FIRE);
        require_eq!(hero.misc04(), Artifact::ORB_OF_DRIVING_RAIN);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Clancy");
        require_eq!(hero.helm(), Artifact::CROWN_OF_THE_FIVE_SEAS);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::PENDANT_OF_SECOND_SIGHT);
        require_eq!(hero.weapon(), Artifact::TRIDENT_OF_DOMINION);
        require_eq!(hero.shield(), Artifact::SHIELD_OF_NAVAL_GLORY);
        require_eq!(hero.armor(), Artifact::ROYAL_ARMOR_OF_NIX);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::GOLDEN_GOOSE);
        require_eq!(hero.misc02(), Artifact::NONE); // Blocked by Golden Goose
        require_eq!(hero.misc03(), Artifact::NONE); // Blocked by Golden Goose
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::PENDANT_OF_TOTAL_RECALL
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // Josephine was defeated.
        let hero = find_hero(&save, "Josephine");
        require_eq!(hero.player, 0xFF);
        require_eq!(hero.on_map, 0);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Sylvia");
        require_eq!(hero.helm(), Artifact::SEA_CAPTAINS_HAT);
        require_eq!(hero.cloak(), Artifact::ANGEL_WINGS);
        require_eq!(hero.neck(), Artifact::NECKLACE_OF_OCEAN_GUIDANCE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::PLATE_OF_DYING_LIGHT);
        require_eq!(hero.ring_left(), Artifact::SEAL_OF_SUNSET);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::WIZARDS_WELL);
        require_eq!(hero.misc02(), Artifact::NONE); // Blocked by Wizard's Well
        require_eq!(hero.misc03(), Artifact::NONE); // Blocked by Wizard's Well
        require_eq!(hero.misc04(), Artifact::SHACKLES_OF_WAR);
        require_eq!(hero.misc05(), Artifact::CHARM_OF_ECLIPSE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // - Assemble Statue of Legion
        // - Assemble Titan's Thunder. This gives Jenova a spell book (!).
        // - Gives Pendant of Courage to Neela, who is now full of scrolls. The pendant disappears!
        let hero = find_hero(&save, "Jenova");
        require_eq!(hero.helm(), Artifact::NONE); // Blocked by Titan's Thunder
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::PENDANT_OF_HOLINESS);
        require_eq!(hero.weapon(), Artifact::TITANS_THUNDER);
        require_eq!(hero.shield(), Artifact::NONE); // Blocked by Titan's Thunder
        require_eq!(hero.armor(), Artifact::NONE); // Blocked by Titan's Thunder
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::WAYFARERS_BOOTS);
        require_eq!(hero.misc01(), Artifact::STATUE_OF_LEGION);
        require_eq!(hero.misc02(), Artifact::NONE); // Blocked by Statue of Legion
        require_eq!(hero.misc03(), Artifact::NONE); // Blocked by Statue of Legion
        require_eq!(hero.misc04(), Artifact::NONE); // Blocked by Statue of Legion
        require_eq!(hero.misc05(), Artifact::NONE); // Blocked by Statue of Legion

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK); // Spell Book gained by assembling Titan's Thunder!

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // Neela used her Armorer skill to kill off all other heroes with scrolls.
        // She now posesses all 69 scrolls, filling up the entire inventory.
        // Any artifacts gained after this is therefore lost.
        let hero = find_hero(&save, "Neela");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc02(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc03(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc04(), Artifact::SPELL_SCROLL);
        require_eq!(hero.misc05(), Artifact::SPELL_SCROLL);

        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC01), Spell::CLONE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC02), Spell::COUNTERSTRIKE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC03), Spell::CURE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC04), Spell::CURSE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::MISC05), Spell::DEATH_RIPPLE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::SPELL_BOOK);

        // Loop through all inventory items and verify that they are scrolls.
        let start = usize::from(ArtifactSlot::INVENTORY01.0);
        for i in start..start + 64 {
            require_eq!(hero.artifact_at(i), Artifact::SPELL_SCROLL);
        }

        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY01), Spell::BLIND);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY02), Spell::BERSERK);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY03),
            Spell::SUMMON_EARTH_ELEMENTAL
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY04), Spell::EARTHQUAKE);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY05),
            Spell::SUMMON_FIRE_ELEMENTAL
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY06), Spell::FIRE_SHIELD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY07), Spell::FIRE_WALL);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY08), Spell::FORTUNE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY09), Spell::FORGETFULNESS);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY10), Spell::FORCE_FIELD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY11), Spell::FLY);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY12), Spell::FIREBALL);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY13), Spell::FRENZY);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY14), Spell::FROST_RING);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY15), Spell::HASTE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY16), Spell::HYPNOTIZE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY17), Spell::ICE_BOLT);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY18), Spell::MAGIC_ARROW);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY19), Spell::LIGHTNING_BOLT);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY20), Spell::LAND_MINE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY21), Spell::INFERNO);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY22), Spell::IMPLOSION);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY23), Spell::MAGIC_MIRROR);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY24), Spell::METEOR_SHOWER);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY25), Spell::MIRTH);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY26), Spell::MISFORTUNE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY27), Spell::PRAYER);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY28),
            Spell::PROTECTION_FROM_FIRE
        );
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY29),
            Spell::PROTECTION_FROM_AIR
        );
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY30),
            Spell::PROTECTION_FROM_WATER
        );
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY31),
            Spell::PROTECTION_FROM_EARTH
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY32), Spell::PRECISION);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY33), Spell::QUICKSAND);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY34),
            Spell::REMOVE_OBSTACLE
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY35), Spell::RESURRECTION);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY36), Spell::SACRIFICE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY37), Spell::SCUTTLE_BOAT);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY38), Spell::STONE_SKIN);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY39), Spell::SORROW);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY40), Spell::SLOW);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY41), Spell::SLAYER);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY42), Spell::SHIELD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY43), Spell::SUMMON_BOAT);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY44), Spell::TELEPORT);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY45), Spell::TOWN_PORTAL);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY46), Spell::VIEW_AIR);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY47), Spell::VIEW_EARTH);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY48), Spell::WEAKNESS);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY49), Spell::WATER_WALK);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY50),
            Spell::SUMMON_WATER_ELEMENTAL
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY51), Spell::VISIONS);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY52), Spell::DESTROY_UNDEAD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY53), Spell::DIMENSION_DOOR);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY54), Spell::DISGUISE);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY55), Spell::DISPEL);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY56), Spell::DISRUPTING_RAY);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY57),
            Spell::SUMMON_AIR_ELEMENTAL
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY58), Spell::BLESS);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY59), Spell::ANIMATE_DEAD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY60), Spell::ANTI_MAGIC);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY61), Spell::ARMAGEDDON);
        require_eq!(
            hero.spell_from_scroll(ArtifactSlot::INVENTORY62),
            Spell::CHAIN_LIGHTNING
        );
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY63), Spell::AIR_SHIELD);
        require_eq!(hero.spell_from_scroll(ArtifactSlot::INVENTORY64), Spell::BLOODLUST);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Lord Haart");
        require_eq!(hero.helm(), Artifact::HELM_OF_THE_ALABASTER_UNICORN);
        require_eq!(hero.cloak(), Artifact::RECANTERS_CLOAK);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::CENTAURS_AXE);
        require_eq!(hero.shield(), Artifact::NONE); // Moved shield to inventory.
        require_eq!(hero.armor(), Artifact::BREASTPLATE_OF_PETRIFIED_WOOD);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::SPIRIT_OF_OPRESSION);
        require_eq!(hero.misc02(), Artifact::HOURGLASS_OF_THE_EVIL_HOUR);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(
            hero.artifact(ArtifactSlot::INVENTORY01),
            Artifact::SHIELD_OF_THE_DWARVEN_LORDS
        );
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // - Adds Left ring: Ring of Suppression
        // - Adds Misc05: Hideous Mask
        // - Adds inventory01: Pendant of Downfall
        let hero = find_hero(&save, "Ryland");
        require_eq!(hero.helm(), Artifact::NONE); // Blocked by Angelic Alliance
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Angelic Alliance
        require_eq!(hero.weapon(), Artifact::ANGELIC_ALLIANCE);
        require_eq!(hero.shield(), Artifact::NONE); // Blocked by Angelic Alliance
        require_eq!(hero.armor(), Artifact::NONE); // Blocked by Angelic Alliance
        require_eq!(hero.ring_left(), Artifact::RING_OF_SUPPRESSION);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE); // Blocked by Angelic Alliance
        require_eq!(hero.misc01(), Artifact::ORB_OF_INHIBITION);
        require_eq!(hero.misc02(), Artifact::RUNES_OF_IMMINENCY);
        require_eq!(hero.misc03(), Artifact::DEMONS_HORSESHOE);
        require_eq!(hero.misc04(), Artifact::SHAMANS_PUPPET);
        require_eq!(hero.misc05(), Artifact::HIDEOUS_MASK);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::PENDANT_OF_DOWNFALL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }

    {
        // NOTE! "Torosar" is saved as "Torosar " in the save file.
        // Torosar was defeated.
        let hero = find_hero(&save, "Torosar ");
        require_eq!(hero.player, 0xFF);
        require_eq!(hero.on_map, 0);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Sorsha");
        require_eq!(hero.helm(), Artifact::NONE); // Blocked by Armor of the Damned
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::NONE); // Blocked by Armor of the Damned
        require_eq!(hero.shield(), Artifact::NONE); // Blocked by Armor of the Damned
        require_eq!(hero.armor(), Artifact::ARMOR_OF_THE_DAMNED);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::BOOTS_OF_LEVITATION);
        require_eq!(hero.misc01(), Artifact::TOME_OF_FIRE_MAGIC);
        require_eq!(hero.misc02(), Artifact::TOME_OF_AIR_MAGIC);
        require_eq!(hero.misc03(), Artifact::TOME_OF_WATER_MAGIC);
        require_eq!(hero.misc04(), Artifact::TOME_OF_EARTH_MAGIC);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Ivor");
        require_eq!(hero.helm(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.cloak(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.neck(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.weapon(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.shield(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.armor(), Artifact::POWER_OF_THE_DRAGON_FATHER);
        require_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.boots(), Artifact::NONE); // Blocked by Power of the Dragon Father
        require_eq!(hero.misc01(), Artifact::NONE);
        require_eq!(hero.misc02(), Artifact::NONE);
        require_eq!(hero.misc03(), Artifact::NONE);
        require_eq!(hero.misc04(), Artifact::NONE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // Fafner was defeated.
        let hero = find_hero(&save, "Fafner");
        require_eq!(hero.player, 0xFF);
        require_eq!(hero.on_map, 0);
    }

    {
        // Tyris was defeated.
        let hero = find_hero(&save, "Tyris");
        require_eq!(hero.player, 0xFF);
        require_eq!(hero.on_map, 0);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Kyrre");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::PENDANT_OF_LIFE);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::CLOVER_OF_FORTUNE);
        require_eq!(hero.misc02(), Artifact::CARDS_OF_PROPHECY);
        require_eq!(hero.misc03(), Artifact::LADYBIRD_OF_LUCK);
        require_eq!(hero.misc04(), Artifact::BADGE_OF_COURAGE);
        require_eq!(hero.misc05(), Artifact::CREST_OF_VALOR);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::GLYPH_OF_GALLANTRY);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::SPECULUM);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY03), Artifact::SPYGLASS);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY04), Artifact::PENDANT_OF_DEATH);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY05), Artifact::PENDANT_OF_FREE_WILL);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY06), Artifact::NONE);
    }

    {
        // Rissa was defeated.
        let hero = find_hero(&save, "Rissa");
        require_eq!(hero.player, 0xFF);
        require_eq!(hero.on_map, 0);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Christian");
        require_eq!(hero.helm(), Artifact::HELM_OF_CHAOS);
        require_eq!(hero.cloak(), Artifact::NONE);
        require_eq!(hero.neck(), Artifact::NONE);
        require_eq!(hero.weapon(), Artifact::GREATER_GNOLLS_FLAIL);
        require_eq!(hero.shield(), Artifact::BUCKLER_OF_THE_GNOLL_KING);
        require_eq!(hero.armor(), Artifact::SCALES_OF_THE_GREATER_BASILISK);
        require_eq!(hero.ring_left(), Artifact::NONE); // Blocked by Elixir of Life
        require_eq!(hero.ring_right(), Artifact::NONE); // Blocked by Elixir of Life
        require_eq!(hero.boots(), Artifact::NONE);
        require_eq!(hero.misc01(), Artifact::GOLDEN_BOW);
        require_eq!(hero.misc02(), Artifact::SPHERE_OF_PERMANENCE);
        require_eq!(hero.misc03(), Artifact::ORB_OF_VULNERABILITY);
        require_eq!(hero.misc04(), Artifact::ELIXIR_OF_LIFE);
        require_eq!(hero.misc05(), Artifact::NONE);

        require_eq!(hero.ballista(), Artifact::BALLISTA);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::NONE);
    }

    {
        // Iona was defeated.
        let hero = find_hero(&save, "Iona");
        require_eq!(hero.player, 0xFF);
        require_eq!(hero.on_map, 0);
    }

    {
        // No changes.
        let hero = find_hero(&save, "Giselle");
        require_eq!(hero.helm(), Artifact::NONE);
        require_eq!(hero.cloak(), Artifact::VAMPIRES_COWL);
        require_eq!(hero.neck(), Artifact::AMULET_OF_THE_UNDERTAKER);
        require_eq!(hero.weapon(), Artifact::NONE);
        require_eq!(hero.shield(), Artifact::NONE);
        require_eq!(hero.armor(), Artifact::NONE);
        require_eq!(hero.ring_left(), Artifact::NONE);
        require_eq!(hero.ring_right(), Artifact::NONE);
        require_eq!(hero.boots(), Artifact::DEAD_MANS_BOOTS);
        require_eq!(hero.misc01(), Artifact::BOW_OF_THE_SHARPSHOOTER);
        require_eq!(hero.misc02(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        require_eq!(hero.misc03(), Artifact::NONE); // Blocked by Bow of the Sharpshooter
        require_eq!(hero.misc04(), Artifact::BIRD_OF_PERCEPTION);
        require_eq!(hero.misc05(), Artifact::STOIC_WATCHMAN);

        require_eq!(hero.ballista(), Artifact::NONE);
        require_eq!(hero.ammo_cart(), Artifact::NONE);
        require_eq!(hero.first_aid_tent(), Artifact::NONE);
        require_eq!(hero.catapult(), Artifact::CATAPULT);
        require_eq!(hero.spell_book(), Artifact::NONE);

        require_eq!(hero.artifact(ArtifactSlot::INVENTORY01), Artifact::EMBLEM_OF_COGNIZANCE);
        require_eq!(hero.artifact(ArtifactSlot::INVENTORY02), Artifact::NONE);
    }
}

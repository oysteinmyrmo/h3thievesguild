// Ported from the homm3tools C++ test test-resources.cpp. Expected values are unchanged.

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

use h3save::player::players;

#[test]
fn resources_day1() {
    let save = open(&data_dir("resources").join("[hotseat] 111.GM8"));
    assert_date(&save, 1, 1, 1);
    let players = players(&save);

    {
        // Red
        let player = &players[0];
        let player_data = &save.players[0];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 5);
        assert_eq!(player.ore(), 5);
        assert_eq!(player.sulfur(), 0);
        assert_eq!(player.crystal(), 0);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 2500);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 5);
        assert_eq!(player_data.ore, 5);
        assert_eq!(player_data.sulfur, 0);
        assert_eq!(player_data.crystal, 0);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 2500);
    }

    {
        // Blue
        let player = &players[1];
        let player_data = &save.players[1];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 0);
        assert_eq!(player.ore(), 5);
        assert_eq!(player.sulfur(), 0);
        assert_eq!(player.crystal(), 0);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 3400);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 0);
        assert_eq!(player_data.ore, 5);
        assert_eq!(player_data.sulfur, 0);
        assert_eq!(player_data.crystal, 0);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 3400);
    }

    {
        // Tan
        let player = &players[2];
        let player_data = &save.players[2];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 5);
        assert_eq!(player.ore(), 5);
        assert_eq!(player.sulfur(), 0);
        assert_eq!(player.crystal(), 0);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 2500);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 5);
        assert_eq!(player_data.ore, 5);
        assert_eq!(player_data.sulfur, 0);
        assert_eq!(player_data.crystal, 0);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 2500);
    }

    {
        // Green
        let player = &players[3];
        let player_data = &save.players[3];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 0);
        assert_eq!(player.ore(), 5);
        assert_eq!(player.sulfur(), 0);
        assert_eq!(player.crystal(), 0);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 3400);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 0);
        assert_eq!(player_data.ore, 5);
        assert_eq!(player_data.sulfur, 0);
        assert_eq!(player_data.crystal, 0);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 3400);
    }

    {
        // Orange
        let player = &players[4];
        let player_data = &save.players[4];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 3);
        assert_eq!(player.ore(), 5);
        assert_eq!(player.sulfur(), 0);
        assert_eq!(player.crystal(), 0);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 2500);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 3);
        assert_eq!(player_data.ore, 5);
        assert_eq!(player_data.sulfur, 0);
        assert_eq!(player_data.crystal, 0);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 2500);
    }

    {
        // Purple
        let player = &players[5];
        let player_data = &save.players[5];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 0);
        assert_eq!(player.ore(), 5);
        assert_eq!(player.sulfur(), 0);
        assert_eq!(player.crystal(), 0);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 3300);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 0);
        assert_eq!(player_data.ore, 5);
        assert_eq!(player_data.sulfur, 0);
        assert_eq!(player_data.crystal, 0);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 3300);
    }

    {
        // Teal
        let player = &players[6];
        let player_data = &save.players[6];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 6);
        assert_eq!(player.ore(), 5);
        assert_eq!(player.sulfur(), 0);
        assert_eq!(player.crystal(), 0);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 2500);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 6);
        assert_eq!(player_data.ore, 5);
        assert_eq!(player_data.sulfur, 0);
        assert_eq!(player_data.crystal, 0);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 2500);
    }

    {
        // Pink
        let player = &players[7];
        let player_data = &save.players[7];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 0);
        assert_eq!(player.ore(), 5);
        assert_eq!(player.sulfur(), 0);
        assert_eq!(player.crystal(), 0);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 3500);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 0);
        assert_eq!(player_data.ore, 5);
        assert_eq!(player_data.sulfur, 0);
        assert_eq!(player_data.crystal, 0);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 3500);
    }
}

#[test]
fn resources_day2() {
    let save = open(&data_dir("resources").join("[hotseat] 112.GM8"));
    assert_date(&save, 1, 1, 2);
    let players = players(&save);

    {
        // Red
        let player = &players[0];
        let player_data = &save.players[0];

        assert_eq!(player.wood(), 15);
        assert_eq!(player.mercury(), 17);
        assert_eq!(player.ore(), 16);
        assert_eq!(player.sulfur(), 5);
        assert_eq!(player.crystal(), 5);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 6700);

        assert_eq!(player_data.wood, 15);
        assert_eq!(player_data.mercury, 17);
        assert_eq!(player_data.ore, 16);
        assert_eq!(player_data.sulfur, 5);
        assert_eq!(player_data.crystal, 5);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 6700);
    }

    {
        // Blue
        let player = &players[1];
        let player_data = &save.players[1];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 12);
        assert_eq!(player.ore(), 23);
        assert_eq!(player.sulfur(), 0);
        assert_eq!(player.crystal(), 8);
        assert_eq!(player.gems(), 11);
        assert_eq!(player.gold(), 6700);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 12);
        assert_eq!(player_data.ore, 23);
        assert_eq!(player_data.sulfur, 0);
        assert_eq!(player_data.crystal, 8);
        assert_eq!(player_data.gems, 11);
        assert_eq!(player_data.gold, 6700);
    }

    {
        // Tan
        let player = &players[2];
        let player_data = &save.players[2];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 5);
        assert_eq!(player.ore(), 15);
        assert_eq!(player.sulfur(), 18);
        assert_eq!(player.crystal(), 6);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 3100);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 5);
        assert_eq!(player_data.ore, 15);
        assert_eq!(player_data.sulfur, 18);
        assert_eq!(player_data.crystal, 6);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 3100);
    }

    {
        // Green
        let player = &players[3];
        let player_data = &save.players[3];

        assert_eq!(player.wood(), 5);
        assert_eq!(player.mercury(), 10);
        assert_eq!(player.ore(), 10);
        assert_eq!(player.sulfur(), 14);
        assert_eq!(player.crystal(), 0);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 6800);

        assert_eq!(player_data.wood, 5);
        assert_eq!(player_data.mercury, 10);
        assert_eq!(player_data.ore, 10);
        assert_eq!(player_data.sulfur, 14);
        assert_eq!(player_data.crystal, 0);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 6800);
    }

    {
        // Orange
        let player = &players[4];
        let player_data = &save.players[4];

        assert_eq!(player.wood(), 5);
        assert_eq!(player.mercury(), 3);
        assert_eq!(player.ore(), 5);
        assert_eq!(player.sulfur(), 0);
        assert_eq!(player.crystal(), 6);
        assert_eq!(player.gems(), 12);
        assert_eq!(player.gold(), 7250);

        assert_eq!(player_data.wood, 5);
        assert_eq!(player_data.mercury, 3);
        assert_eq!(player_data.ore, 5);
        assert_eq!(player_data.sulfur, 0);
        assert_eq!(player_data.crystal, 6);
        assert_eq!(player_data.gems, 12);
        assert_eq!(player_data.gold, 7250);
    }

    {
        // Purple
        let player = &players[5];
        let player_data = &save.players[5];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 0);
        assert_eq!(player.ore(), 20);
        assert_eq!(player.sulfur(), 5);
        assert_eq!(player.crystal(), 6);
        assert_eq!(player.gems(), 10);
        assert_eq!(player.gold(), 6900);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 0);
        assert_eq!(player_data.ore, 20);
        assert_eq!(player_data.sulfur, 5);
        assert_eq!(player_data.crystal, 6);
        assert_eq!(player_data.gems, 10);
        assert_eq!(player_data.gold, 6900);
    }

    {
        // Teal
        let player = &players[6];
        let player_data = &save.players[6];

        assert_eq!(player.wood(), 6);
        assert_eq!(player.mercury(), 6);
        assert_eq!(player.ore(), 11);
        assert_eq!(player.sulfur(), 4);
        assert_eq!(player.crystal(), 6);
        assert_eq!(player.gems(), 10);
        assert_eq!(player.gold(), 5950);

        assert_eq!(player_data.wood, 6);
        assert_eq!(player_data.mercury, 6);
        assert_eq!(player_data.ore, 11);
        assert_eq!(player_data.sulfur, 4);
        assert_eq!(player_data.crystal, 6);
        assert_eq!(player_data.gems, 10);
        assert_eq!(player_data.gold, 5950);
    }

    {
        // Pink
        let player = &players[7];
        let player_data = &save.players[7];

        assert_eq!(player.wood(), 6);
        assert_eq!(player.mercury(), 6);
        assert_eq!(player.ore(), 11);
        assert_eq!(player.sulfur(), 17);
        assert_eq!(player.crystal(), 6);
        assert_eq!(player.gems(), 10);
        assert_eq!(player.gold(), 5000);

        assert_eq!(player_data.wood, 6);
        assert_eq!(player_data.mercury, 6);
        assert_eq!(player_data.ore, 11);
        assert_eq!(player_data.sulfur, 17);
        assert_eq!(player_data.crystal, 6);
        assert_eq!(player_data.gems, 10);
        assert_eq!(player_data.gold, 5000);
    }
}

#[test]
fn resources_day3() {
    let save = open(&data_dir("resources").join("[hotseat] 113.GM5"));
    assert_date(&save, 1, 1, 3);
    let players = players(&save);

    {
        // Red
        let player = &players[0];
        let player_data = &save.players[0];

        assert_eq!(player.wood(), 19);
        assert_eq!(player.mercury(), 32);
        assert_eq!(player.ore(), 21);
        assert_eq!(player.sulfur(), 5);
        assert_eq!(player.crystal(), 21);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 10700);

        assert_eq!(player_data.wood, 19);
        assert_eq!(player_data.mercury, 32);
        assert_eq!(player_data.ore, 21);
        assert_eq!(player_data.sulfur, 5);
        assert_eq!(player_data.crystal, 21);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 10700);
    }

    {
        // Blue is vanquished, resource data remains fixed in the save file.
        let player = &players[1];
        let player_data = &save.players[1];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 12);
        assert_eq!(player.ore(), 23);
        assert_eq!(player.sulfur(), 0);
        assert_eq!(player.crystal(), 8);
        assert_eq!(player.gems(), 11);
        assert_eq!(player.gold(), 6700);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 12);
        assert_eq!(player_data.ore, 23);
        assert_eq!(player_data.sulfur, 0);
        assert_eq!(player_data.crystal, 8);
        assert_eq!(player_data.gems, 11);
        assert_eq!(player_data.gold, 6700);
    }

    {
        // Tan is vanquished, resource data remains fixed in the save file.
        let player = &players[2];
        let player_data = &save.players[2];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 5);
        assert_eq!(player.ore(), 21); // Picked up some ore before vanquished.
        assert_eq!(player.sulfur(), 18);
        assert_eq!(player.crystal(), 6);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 3700); // Picked up some gold before vanquished.

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 5);
        assert_eq!(player_data.ore, 21); // Picked up some ore before vanquished.
        assert_eq!(player_data.sulfur, 18);
        assert_eq!(player_data.crystal, 6);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 3700); // Picked up some gold before vanquished.
    }

    {
        // Green
        let player = &players[3];
        let player_data = &save.players[3];

        assert_eq!(player.wood(), 5);
        assert_eq!(player.mercury(), 10);
        assert_eq!(player.ore(), 10);
        assert_eq!(player.sulfur(), 14);
        assert_eq!(player.crystal(), 0);
        assert_eq!(player.gems(), 0);
        assert_eq!(player.gold(), 6800);

        assert_eq!(player_data.wood, 5);
        assert_eq!(player_data.mercury, 10);
        assert_eq!(player_data.ore, 10);
        assert_eq!(player_data.sulfur, 14);
        assert_eq!(player_data.crystal, 0);
        assert_eq!(player_data.gems, 0);
        assert_eq!(player_data.gold, 6800);
    }

    {
        // Orange is vanquished, resource data remains fixed in the save file.
        let player = &players[4];
        let player_data = &save.players[4];

        assert_eq!(player.wood(), 5);
        assert_eq!(player.mercury(), 3);
        assert_eq!(player.ore(), 5);
        assert_eq!(player.sulfur(), 0);
        assert_eq!(player.crystal(), 6);
        assert_eq!(player.gems(), 12);
        assert_eq!(player.gold(), 7500); // Basic estates (Erdamon) increased gold by 250.

        assert_eq!(player_data.wood, 5);
        assert_eq!(player_data.mercury, 3);
        assert_eq!(player_data.ore, 5);
        assert_eq!(player_data.sulfur, 0);
        assert_eq!(player_data.crystal, 6);
        assert_eq!(player_data.gems, 12);
        assert_eq!(player_data.gold, 7500); // Basic estates (Erdamon) increased gold by 250.
    }

    {
        // Purple
        let player = &players[5];
        let player_data = &save.players[5];

        assert_eq!(player.wood(), 0);
        assert_eq!(player.mercury(), 0);
        assert_eq!(player.ore(), 20);
        assert_eq!(player.sulfur(), 5);
        assert_eq!(player.crystal(), 6);
        assert_eq!(player.gems(), 10);
        assert_eq!(player.gold(), 6900);

        assert_eq!(player_data.wood, 0);
        assert_eq!(player_data.mercury, 0);
        assert_eq!(player_data.ore, 20);
        assert_eq!(player_data.sulfur, 5);
        assert_eq!(player_data.crystal, 6);
        assert_eq!(player_data.gems, 10);
        assert_eq!(player_data.gold, 6900);
    }

    {
        // Teal
        let player = &players[6];
        let player_data = &save.players[6];

        assert_eq!(player.wood(), 6);
        assert_eq!(player.mercury(), 6);
        assert_eq!(player.ore(), 11);
        assert_eq!(player.sulfur(), 4);
        assert_eq!(player.crystal(), 6);
        assert_eq!(player.gems(), 10);
        assert_eq!(player.gold(), 6200); // Basic estates (Thunar) increased gold by 250.

        assert_eq!(player_data.wood, 6);
        assert_eq!(player_data.mercury, 6);
        assert_eq!(player_data.ore, 11);
        assert_eq!(player_data.sulfur, 4);
        assert_eq!(player_data.crystal, 6);
        assert_eq!(player_data.gems, 10);
        assert_eq!(player_data.gold, 6200); // Basic estates (Thunar) increased gold by 250.
    }

    {
        // Pink
        let player = &players[7];
        let player_data = &save.players[7];

        assert_eq!(player.wood(), 6);
        assert_eq!(player.mercury(), 6);
        assert_eq!(player.ore(), 11);
        assert_eq!(player.sulfur(), 17);
        assert_eq!(player.crystal(), 6);
        assert_eq!(player.gems(), 10);
        assert_eq!(player.gold(), 5000);

        assert_eq!(player_data.wood, 6);
        assert_eq!(player_data.mercury, 6);
        assert_eq!(player_data.ore, 11);
        assert_eq!(player_data.sulfur, 17);
        assert_eq!(player_data.crystal, 6);
        assert_eq!(player_data.gems, 10);
        assert_eq!(player_data.gold, 5000);
    }
}

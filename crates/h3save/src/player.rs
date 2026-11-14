//! Players and their resources.

use crate::hero::Hero;
use crate::reader::{ReadResult, Reader};
use crate::savefile::SaveFile;
use crate::town::Town;

/// Number of player slots. A save always holds all eight.
pub const MAX_PLAYERS: usize = 8;

/// Size of one player record.
pub const PLAYER_RECORD_SIZE: usize = 149;

/// A player colour. The discriminant is the player's index in the save.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Color {
    Red = 0,
    Blue = 1,
    Tan = 2,
    Green = 3,
    Orange = 4,
    Purple = 5,
    Teal = 6,
    Pink = 7,
}

impl Color {
    pub const ALL: [Color; MAX_PLAYERS] = [
        Color::Red,
        Color::Blue,
        Color::Tan,
        Color::Green,
        Color::Orange,
        Color::Purple,
        Color::Teal,
        Color::Pink,
    ];

    #[must_use]
    pub fn index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub fn name(self) -> &'static str {
        ["Red", "Blue", "Tan", "Green", "Orange", "Purple", "Teal", "Pink"][self.index()]
    }

    /// The colour used in game.
    #[must_use]
    pub fn rgb(self) -> ColorRgb {
        PLAYER_COLORS[self.index()]
    }
}

/// An RGB colour.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// In-game player colours, indexed by player.
pub const PLAYER_COLORS: [ColorRgb; MAX_PLAYERS] = [
    ColorRgb { r: 248, g: 0, b: 0 },
    ColorRgb { r: 48, g: 80, b: 248 },
    ColorRgb { r: 160, g: 128, b: 80 },
    ColorRgb { r: 32, g: 148, b: 0 },
    ColorRgb { r: 248, g: 132, b: 0 },
    ColorRgb { r: 136, g: 40, b: 160 },
    ColorRgb { r: 8, g: 156, b: 160 },
    ColorRgb { r: 192, g: 120, b: 136 },
];

/// One player record: the player's resources.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct PlayerData {
    pub wood: u32,
    pub mercury: u32,
    pub ore: u32,
    pub sulfur: u32,
    pub crystal: u32,
    pub gems: u32,
    pub gold: u32,
}

impl PlayerData {
    pub(crate) fn read(data: &[u8], pos: usize) -> ReadResult<PlayerData> {
        let mut r = Reader::at(data, pos);
        r.skip(96)?;
        let player = PlayerData {
            wood: r.u32()?,
            mercury: r.u32()?,
            ore: r.u32()?,
            sulfur: r.u32()?,
            crystal: r.u32()?,
            gems: r.u32()?,
            gold: r.u32()?,
        };
        r.skip(25)?;
        Ok(player)
    }
}

/// Read the eight player records starting at `pos`.
pub(crate) fn read_all(data: &[u8], pos: usize) -> ReadResult<[PlayerData; MAX_PLAYERS]> {
    let mut players = [PlayerData::default(); MAX_PLAYERS];
    for (i, player) in players.iter_mut().enumerate() {
        *player = PlayerData::read(data, pos + i * PLAYER_RECORD_SIZE)?;
    }
    Ok(players)
}

/// A player with its resources, heroes and towns gathered from a save.
#[derive(Clone, Debug)]
pub struct Player {
    color: Color,
    data: PlayerData,
    heroes: Vec<Hero>,
    towns: Vec<Town>,
}

impl Player {
    #[must_use]
    pub fn color(&self) -> Color {
        self.color
    }

    #[must_use]
    pub fn data(&self) -> &PlayerData {
        &self.data
    }

    #[must_use]
    pub fn heroes(&self) -> &[Hero] {
        &self.heroes
    }

    #[must_use]
    pub fn towns(&self) -> &[Town] {
        &self.towns
    }

    /// A player is alive while it owns a hero or a town.
    #[must_use]
    pub fn alive(&self) -> bool {
        !self.heroes.is_empty() || !self.towns.is_empty()
    }

    /// Sum of the army strength of all heroes and town garrisons.
    pub fn kingdom_army_strength(&self) -> u64 {
        self.heroes.iter().map(Hero::army_strength).sum::<u64>()
            + self.towns.iter().map(Town::army_strength).sum::<u64>()
    }

    /// Highest AI value among the best creatures of every hero and town.
    pub fn best_creature_ai_value(&self) -> u32 {
        let heroes = self.heroes.iter().map(Hero::best_creature);
        let towns = self.towns.iter().map(Town::best_creature);
        heroes
            .chain(towns)
            .map(|c| c.ai_value().unwrap_or(0))
            .max()
            .unwrap_or(0)
    }

    #[must_use]
    pub fn wood(&self) -> u32 {
        self.data.wood
    }
    #[must_use]
    pub fn mercury(&self) -> u32 {
        self.data.mercury
    }
    #[must_use]
    pub fn ore(&self) -> u32 {
        self.data.ore
    }
    #[must_use]
    pub fn sulfur(&self) -> u32 {
        self.data.sulfur
    }
    #[must_use]
    pub fn crystal(&self) -> u32 {
        self.data.crystal
    }
    #[must_use]
    pub fn gems(&self) -> u32 {
        self.data.gems
    }
    #[must_use]
    pub fn gold(&self) -> u32 {
        self.data.gold
    }
}

/// All eight players of a save, with their heroes and towns, indexed by colour.
#[must_use]
pub fn players(save: &SaveFile) -> [Player; MAX_PLAYERS] {
    let mut players = Color::ALL.map(|color| Player {
        color,
        data: save.players[color.index()],
        heroes: Vec::new(),
        towns: Vec::new(),
    });
    for hero in &save.heroes {
        if let Some(p) = players.get_mut(usize::from(hero.player)) {
            p.heroes.push(hero.clone());
        }
    }
    for town in &save.towns {
        if let Some(p) = players.get_mut(usize::from(town.owner)) {
            p.towns.push(town.clone());
        }
    }
    players
}

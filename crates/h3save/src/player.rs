//! Players and their resources.

use crate::creatures::Creature;
use crate::hero::Hero;
use crate::reader::{ReadResult, Reader, record_pos};
use crate::savefile::SaveFile;
use crate::town::Town;

/// Number of player slots. A save always holds all eight.
pub const MAX_PLAYERS: usize = 8;

/// Size of one player record.
pub const PLAYER_RECORD_SIZE: usize = 149;

/// Most heroes a player can have on the map.
pub const MAX_HEROES_ON_MAP: usize = 8;

/// Stands for no hero or town in the player record's ID fields.
pub const NONE: u8 = 0xFF;

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
        match self {
            Color::Red => "Red",
            Color::Blue => "Blue",
            Color::Tan => "Tan",
            Color::Green => "Green",
            Color::Orange => "Orange",
            Color::Purple => "Purple",
            Color::Teal => "Teal",
            Color::Pink => "Pink",
        }
    }

    /// The colour used in game.
    #[must_use]
    pub fn rgb(self) -> ColorRgb {
        PLAYER_COLORS.get(self.index()).copied().unwrap_or_default()
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

/// One player record: the player's heroes on the map, towns and resources.
///
/// Byte offsets from the start of the record:
///
/// | Offset | Size | Field |
/// | --- | --- | --- |
/// | 0 | 1 | Player index, 0–7 (the colour) |
/// | 1 | 1 | Number of heroes on the map |
/// | 2 | 1 | Selected hero |
/// | 3 | 8 | Hero IDs of the heroes on the map, in the player's order |
/// | 24 | 1 | Number of towns |
/// | 25 | 1 | Selected town |
/// | 26 | 48 | Town IDs: indices into the save's towns |
/// | 98 | 28 | Resources: wood, mercury, ore, sulfur, crystal, gems, gold as `u32` |
///
/// Unused ID slots and no selection hold [`NONE`]. Heroes in a town garrison are not on the
/// map, so they are owned but not listed here.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayerData {
    /// Heroes on the map; the first `hero_count` entries of `hero_ids` are used.
    pub hero_count: u8,
    /// The selected hero's ID, or [`NONE`].
    pub selected_hero: u8,
    pub hero_ids: [u8; MAX_HEROES_ON_MAP],
    /// Towns owned; the first `town_count` entries of `town_ids` are used.
    pub town_count: u8,
    /// The selected town's index, or [`NONE`].
    pub selected_town: u8,
    pub town_ids: [u8; crate::town::MAX_TOWNS],
    pub wood: u32,
    pub mercury: u32,
    pub ore: u32,
    pub sulfur: u32,
    pub crystal: u32,
    pub gems: u32,
    pub gold: u32,
}

impl Default for PlayerData {
    /// A player with no heroes, towns or resources.
    fn default() -> Self {
        PlayerData {
            hero_count: 0,
            selected_hero: NONE,
            hero_ids: [NONE; MAX_HEROES_ON_MAP],
            town_count: 0,
            selected_town: NONE,
            town_ids: [NONE; crate::town::MAX_TOWNS],
            wood: 0,
            mercury: 0,
            ore: 0,
            sulfur: 0,
            crystal: 0,
            gems: 0,
            gold: 0,
        }
    }
}

impl PlayerData {
    /// Read a record, returning its player index too.
    pub(crate) fn read(data: &[u8], pos: usize) -> ReadResult<(u8, PlayerData)> {
        let mut r = Reader::at(data, pos);
        let index = r.u8()?;
        let hero_count = r.u8()?;
        let selected_hero = r.u8()?;
        let hero_ids = r.array()?;
        r.skip(13)?;
        let town_count = r.u8()?;
        let selected_town = r.u8()?;
        let town_ids = r.array()?;
        r.skip(24)?;
        let player = PlayerData {
            hero_count,
            selected_hero,
            hero_ids,
            town_count,
            selected_town,
            town_ids,
            wood: r.u32()?,
            mercury: r.u32()?,
            ore: r.u32()?,
            sulfur: r.u32()?,
            crystal: r.u32()?,
            gems: r.u32()?,
            gold: r.u32()?,
        };
        r.skip(23)?;
        debug_assert_eq!(r.pos().wrapping_sub(pos), PLAYER_RECORD_SIZE);
        Ok((index, player))
    }

    /// IDs of the heroes on the map, in the player's order.
    #[must_use]
    pub fn hero_ids_on_map(&self) -> &[u8] {
        self.hero_ids
            .get(..usize::from(self.hero_count))
            .unwrap_or(&self.hero_ids)
    }

    /// Indices of the player's towns in the save, in the player's order.
    #[must_use]
    pub fn town_indices(&self) -> &[u8] {
        self.town_ids
            .get(..usize::from(self.town_count))
            .unwrap_or(&self.town_ids)
    }

    /// Whether the record holds together: counts within range, used IDs below `heroes` and
    /// `towns`, unused ID slots empty, and any selection one of the player's own.
    #[must_use]
    pub fn is_plausible(&self, heroes: usize, towns: usize) -> bool {
        fn ids_ok(ids: &[u8], count: u8, limit: usize, selected: u8) -> bool {
            let Some((used, unused)) = ids.split_at_checked(usize::from(count)) else {
                return false;
            };
            used.iter().all(|&id| usize::from(id) < limit && id != NONE)
                && unused.iter().all(|&id| id == NONE)
                && (selected == NONE || used.contains(&selected))
        }
        ids_ok(&self.hero_ids, self.hero_count, heroes, self.selected_hero)
            && ids_ok(&self.town_ids, self.town_count, towns, self.selected_town)
    }
}

/// Read the eight player records starting at `pos`, or `None` unless every record carries its
/// own index and is [plausible](PlayerData::is_plausible).
pub(crate) fn read_all(data: &[u8], pos: usize, heroes: usize, towns: usize) -> Option<[PlayerData; MAX_PLAYERS]> {
    let mut players = [PlayerData::default(); MAX_PLAYERS];
    for (i, (player, index)) in players.iter_mut().zip(0u8..).enumerate() {
        let (found_index, record) = PlayerData::read(data, record_pos(pos, i, PLAYER_RECORD_SIZE).ok()?).ok()?;
        if found_index != index || !record.is_plausible(heroes, towns) {
            return None;
        }
        *player = record;
    }
    Some(players)
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
        let heroes = self.heroes.iter().map(Hero::army_strength);
        let towns = self.towns.iter().map(Town::army_strength);
        heroes.chain(towns).fold(0, u64::saturating_add)
    }

    /// The creature with the highest AI value in the armies of all heroes and town garrisons,
    /// or `None` if there is none with a positive value. Of creatures with the same value, the
    /// first is chosen: heroes before towns.
    #[must_use]
    pub fn best_creature(&self) -> Option<Creature> {
        let heroes = self.heroes.iter().map(Hero::best_creature);
        let towns = self.towns.iter().map(Town::best_creature);
        let mut best = None;
        let mut best_value = 0;
        for creature in heroes.chain(towns) {
            let value = creature.ai_value().unwrap_or(0);
            if value > best_value {
                best_value = value;
                best = Some(creature);
            }
        }
        best
    }

    /// Highest AI value among the creatures of every hero and town, 0 if there are none.
    pub fn best_creature_ai_value(&self) -> u32 {
        self.best_creature().and_then(Creature::ai_value).unwrap_or(0)
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
        data: save.players.get(color.index()).copied().unwrap_or_default(),
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

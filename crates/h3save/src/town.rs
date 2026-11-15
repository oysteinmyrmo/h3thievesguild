//! Town records.

mod names;

use crate::creatures::{Creature, MAX_STACKS};
use crate::id_type;
use crate::reader::{ReadResult, Reader};
use crate::stats;

pub use names::TOWN_NAMES;

/// Size of a town record without its name, in HotA 1.7 and 1.8 saves: the index byte plus the
/// town's data from its owner byte on.
pub const TOWN_SIZE_WITHOUT_NAME: usize = 567;

/// Longest town name accepted when looking for the town block. The game's own limit is lower.
pub const MAX_NAME_SIZE: usize = 32;

/// Largest possible town record.
pub const MAX_RECORD_SIZE: usize = TOWN_SIZE_WITHOUT_NAME + MAX_NAME_SIZE;

/// Offset from the start of a town record (the owner byte) to the name.
pub const NAME_OFFSET: usize = 71;

/// Owner value of a town nobody owns.
pub const NO_OWNER: u8 = 0xFF;

/// Length of the longest default town name, "Fleogan Mills".
pub const LONGEST_TOWN_NAME: usize = 13;

/// Most towns a map can have.
pub const MAX_TOWNS: usize = 48;

id_type! {
    /// A town's faction.
    TownType(u8) {
        CASTLE = 0 => "Castle",
        RAMPART = 1 => "Rampart",
        TOWER = 2 => "Tower",
        INFERNO = 3 => "Inferno",
        NECROPOLIS = 4 => "Necropolis",
        DUNGEON = 5 => "Dungeon",
        STRONGHOLD = 6 => "Stronghold",
        FORTRESS = 7 => "Fortress",
        CONFLUX = 8 => "Conflux",
        COVE = 9 => "Cove",
        FACTORY = 10 => "Factory",
        BULWARK = 11 => "Bulwark",
    }
}

/// One town record. The record is `TOWN_SIZE_WITHOUT_NAME + name.len()` bytes.
/// Field comments give the offset relative to the name, as in the C++ code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Town {
    /// -71. Owning player 0-7, or `NO_OWNER`.
    pub owner: u8,
    /// -68
    pub town_type: TownType,
    /// -67
    pub x: u8,
    /// -66
    pub y: u8,
    /// -65
    pub z: u8,
    /// -62. Garrison.
    pub creatures: [Creature; MAX_STACKS],
    /// -34
    pub creature_count: [u32; MAX_STACKS],
    /// -2. Length of the name in bytes, as stored.
    pub name_size: u16,
    /// 0
    pub name: String,
}

impl Default for Town {
    fn default() -> Self {
        Town {
            owner: NO_OWNER,
            town_type: TownType::CASTLE,
            x: 0,
            y: 0,
            z: 0,
            creatures: [Creature::EMPTY; MAX_STACKS],
            creature_count: [0; MAX_STACKS],
            name_size: 0,
            name: String::new(),
        }
    }
}

impl Town {
    /// Read one town record starting at its owner byte.
    pub(crate) fn read(data: &[u8], pos: usize) -> ReadResult<Town> {
        let mut r = Reader::at(data, pos);
        let owner = r.u8()?;
        r.skip(2)?;
        let town_type = TownType(r.u8()?);
        let x = r.u8()?;
        let y = r.u8()?;
        let z = r.u8()?;
        r.skip(2)?;
        let creatures = r.each(Reader::u32)?.map(Creature);
        let creature_count = r.each(Reader::u32)?;
        r.skip(4)?;
        let name_size = r.u16()?;
        let name = r.string(usize::from(name_size))?;
        Ok(Town {
            owner,
            town_type,
            x,
            y,
            z,
            creatures,
            creature_count,
            name_size,
            name,
        })
    }

    /// Size of this town's record in the save file, including its index byte.
    #[must_use]
    pub fn record_size(&self) -> usize {
        TOWN_SIZE_WITHOUT_NAME.saturating_add(usize::from(self.name_size))
    }

    /// AI value of the garrison.
    #[must_use]
    pub fn army_strength(&self) -> u64 {
        stats::army_ai_value(&self.creatures, &self.creature_count)
    }

    /// The garrison creature with the highest AI value, or `Creature::EMPTY`.
    #[must_use]
    pub fn best_creature(&self) -> Creature {
        stats::best_creature(&self.creatures)
    }
}

impl Town {
    /// Whether the fields look like a real town. Used to tell the town block from other data.
    fn is_plausible(&self) -> bool {
        let owner_ok = usize::from(self.owner) < crate::player::MAX_PLAYERS || self.owner == NO_OWNER;
        let name_size = usize::from(self.name_size);
        owner_ok
            && self.town_type.is_known()
            && self.z <= 1
            && (1..=MAX_NAME_SIZE).contains(&name_size)
            && self.name.chars().all(|c| !c.is_control())
    }
}

/// Read the town block starting at its count byte.
///
/// The block is a count byte, then for each town its index (0, 1, 2, …) and its data. Returns
/// the towns and the position right after the block, or `None` if the data at `pos` is not a
/// town block: an index out of order, an implausible town, or the data ending early.
#[must_use]
pub fn read_block(data: &[u8], pos: usize) -> Option<(Vec<Town>, usize)> {
    let count = *data.get(pos)?;
    if usize::from(count) > MAX_TOWNS {
        return None;
    }
    let mut towns = Vec::with_capacity(usize::from(count));
    let mut index_pos = pos.checked_add(1)?;
    for index in 0..count {
        if *data.get(index_pos)? != index {
            return None;
        }
        let town = Town::read(data, index_pos.checked_add(1)?).ok()?;
        if !town.is_plausible() {
            return None;
        }
        index_pos = index_pos.checked_add(town.record_size())?;
        towns.push(town);
    }
    // `index_pos` now points one past the last town's data.
    Some((towns, index_pos))
}

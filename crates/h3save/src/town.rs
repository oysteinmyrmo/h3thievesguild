//! Town records.

mod names;

use crate::creatures::{Creature, MAX_STACKS};
use crate::id_type;
use crate::reader::{ReadResult, Reader};
use crate::stats;

pub use names::TOWN_NAMES;

/// Size of a town record without its name, in a HotA 1.8 save.
pub const TOWN_SIZE_WITHOUT_NAME: usize = 567;

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
        let creatures = r.each(|r| r.u32().map(Creature))?;
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

    /// Size of this town's record in the save file.
    #[must_use]
    pub fn record_size(&self) -> usize {
        TOWN_SIZE_WITHOUT_NAME + usize::from(self.name_size)
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

/// Read `count` consecutive town records starting at `pos`.
pub(crate) fn read_all(data: &[u8], mut pos: usize, count: usize) -> ReadResult<Vec<Town>> {
    let mut towns = Vec::with_capacity(count);
    for _ in 0..count {
        let town = Town::read(data, pos)?;
        pos += town.record_size();
        towns.push(town);
    }
    Ok(towns)
}

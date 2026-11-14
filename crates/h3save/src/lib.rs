//! Parser for Heroes of Might and Magic III: Horn of the Abyss save files.
//!
//! Supports HotA 1.8 saves. The player, town and hero records are found by the shape of
//! their data, so maps with renamed heroes and towns read the same as any other; see
//! [`savefile`] for how.
//!
//! ```no_run
//! use h3save::{SaveFile, player::players};
//!
//! let save = SaveFile::read("[hotseat] 111.GM2")?;
//! for player in players(&save).iter().filter(|p| p.alive()) {
//!     println!("{:?}: {} gold, army strength {}", player.color(), player.gold(), player.kingdom_army_strength());
//! }
//! # Ok::<(), h3save::Error>(())
//! ```

// Save files come from users, so the library must not panic on any input: no indexing or
// slicing that can go out of bounds, no unwrap, no unchecked arithmetic. Tests may.
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented,
        clippy::arithmetic_side_effects
    )
)]

mod id;
mod reader;

pub mod artifacts;
pub mod creatures;
pub mod date;
pub mod hero;
pub mod player;
pub mod savefile;
pub mod skills;
pub mod spells;
pub mod stats;
pub mod town;

pub use artifacts::{Artifact, ArtifactClass, ArtifactDetails, ArtifactEntry, ArtifactSlot, ArtifactSlotType};
pub use creatures::Creature;
pub use date::Date;
pub use hero::{Hero, Orientation};
pub use player::{Color, ColorRgb, Player, PlayerData};
pub use savefile::{Error, Layout, MapSize, SaveFile};
pub use skills::{Skill, SkillLevel, SkillSlot, SkillSlots};
pub use spells::Spell;
pub use town::{Town, TownType};

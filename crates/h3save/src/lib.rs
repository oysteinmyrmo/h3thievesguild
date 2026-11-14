//! Parser for Heroes of Might and Magic III: Horn of the Abyss save files.
//!
//! The first iteration is a port of the `homm3tools` C++ parser. It supports
//! HotA 1.8 saves and finds heroes and towns the same way the C++ code does:
//! by searching for the first hero's name and for town names.
//!
//! ```no_run
//! use h3save::{Input, SaveFile, player::players};
//!
//! let save = SaveFile::read(&Input::new("[hotseat] 111.GM2"))?;
//! for player in players(&save).iter().filter(|p| p.alive()) {
//!     println!("{:?}: {} gold, army strength {}", player.color(), player.gold(), player.kingdom_army_strength());
//! }
//! # Ok::<(), h3save::Error>(())
//! ```

mod id;
mod reader;

pub mod artifacts;
pub mod creatures;
pub mod hero;
pub mod player;
pub mod savefile;
pub mod skills;
pub mod spells;
pub mod stats;
pub mod town;

pub use artifacts::{Artifact, ArtifactClass, ArtifactDetails, ArtifactEntry, ArtifactSlot, ArtifactSlotType};
pub use creatures::Creature;
pub use hero::{Hero, Orientation};
pub use player::{Color, ColorRgb, Player, PlayerData};
pub use savefile::{Error, Input, MapSize, SaveFile};
pub use skills::{Skill, SkillLevel, SkillSlot, SkillSlots};
pub use spells::Spell;
pub use town::{Town, TownType};

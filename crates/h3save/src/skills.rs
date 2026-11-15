// Generated from the homm3tools C++ sources. Edit by hand from here on.

use crate::id_type;

id_type! {
    /// A secondary skill ID.
    Skill(u8) {
        PATHFINDING = 0 => "Pathfinding",
        ARCHERY = 1 => "Archery",
        LOGISTICS = 2 => "Logistics",
        SCOUTING = 3 => "Scouting",
        DIPLOMACY = 4 => "Diplomacy",
        NAVIGATION = 5 => "Navigation",
        LEADERSHIP = 6 => "Leadership",
        WISDOM = 7 => "Wisdom",
        MYSTICISM = 8 => "Mysticism",
        LUCK = 9 => "Luck",
        BALLISTICS = 10 => "Ballistics",
        EAGLE_EYE = 11 => "EagleEye",
        NECROMANCY = 12 => "Necromancy",
        ESTATES = 13 => "Estates",
        FIRE_MAGIC = 14 => "FireMagic",
        AIR_MAGIC = 15 => "AirMagic",
        WATER_MAGIC = 16 => "WaterMagic",
        EARTH_MAGIC = 17 => "EarthMagic",
        SCHOLAR = 18 => "Scholar",
        TACTICS = 19 => "Tactics",
        ARTILLERY = 20 => "Artillery",
        LEARNING = 21 => "Learning",
        OFFENSE = 22 => "Offense",
        ARMORER = 23 => "Armorer",
        INTELLIGENCE = 24 => "Intelligence",
        SORCERY = 25 => "Sorcery",
        /// Not used in HotA. (Not tested.)
        RESISTANCE = 26 => "Resistance",
        FIRST_AID = 27 => "FirstAid",
        INTERFERENCE = 28 => "Interference",
        /// New in HotA 1.8, with the Bulwark town.
        RUNES = 29 => "Runes",
        /// Helper for Hero class.
        NONE = 31 => "NONE",
    }
}

id_type! {
    /// Level of a secondary skill.
    SkillLevel(u8) {
        NONE = 0 => "None",
        BASIC = 1 => "Basic",
        ADVANCED = 2 => "Advanced",
        EXPERT = 3 => "Expert",
    }
}

/// Number of secondary skills stored per hero in the newest supported format.
pub const SKILL_COUNT: usize = 30;

/// A skill learned in one of a hero's eight skill slots.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SkillSlot {
    pub skill: Skill,
    pub level: SkillLevel,
}

impl Default for SkillSlot {
    fn default() -> Self {
        SkillSlot {
            skill: Skill::NONE,
            level: SkillLevel::NONE,
        }
    }
}

impl SkillSlot {
    #[must_use]
    pub fn new(skill: Skill, level: SkillLevel) -> Self {
        SkillSlot { skill, level }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.level == SkillLevel::NONE || self.skill == Skill::NONE
    }
}

/// Number of secondary skill slots a hero has.
pub const SKILL_SLOT_COUNT: usize = 8;

/// A hero's skills in slot order.
pub type SkillSlots = [SkillSlot; SKILL_SLOT_COUNT];

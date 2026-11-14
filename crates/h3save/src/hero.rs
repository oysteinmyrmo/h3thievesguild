//! Hero records.

use crate::artifacts::{ARTIFACT_SLOT_COUNT, Artifact, ArtifactDetails, ArtifactEntry, ArtifactSlot};
use crate::creatures::{Creature, MAX_STACKS};
use crate::id_type;
use crate::reader::{ReadResult, Reader, latin1, record_pos};
use crate::skills::{SKILL_COUNT, SKILL_SLOT_COUNT, Skill, SkillLevel, SkillSlot, SkillSlots};
use crate::spells::{SPELL_COUNT, Spell};
use crate::stats;

/// Number of hero records in a HotA 1.8 save. A save always holds every hero, hired or not.
/// See <https://heroes.thelazy.net/index.php/List_of_heroes_(HotA)>.
pub const HERO_COUNT: usize = 215;

/// Size of one hero record in a HotA 1.8 save.
pub const HERO_RECORD_SIZE: usize = 1160;

/// Offset from the start of a hero record to the hero's name.
pub const NAME_OFFSET: usize = 195;

/// In-game primary skill values are capped at this. The save files can hold more.
pub const MAX_PRIMARY_SKILL_VALUE: u8 = 99;

id_type! {
    /// Direction a hero faces on the map.
    Orientation(u8) {
        N = 0 => "N",
        NE = 1 => "NE",
        E = 2 => "E",
        SE = 3 => "SE",
        S = 4 => "S",
        SW = 5 => "SW",
        W = 6 => "W",
        NW = 7 => "NW",
    }
}

/// One hero record. Field comments give the offset relative to the name, as in the C++ code.
/// See also <http://heroescommunity.com/viewthread.php3?TID=18817&PID=1366926#focus>.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hero {
    /// -195. `0xFFFF` when unset.
    pub x: u16,
    /// -193
    pub y: u16,
    /// -191
    pub z: u16,
    /// -189. 1 on the map, 0 in a town garrison or a prison. The position is set in a prison.
    pub on_map: u8,
    /// -169. Owning player 0-7, `0xFF` for none.
    pub player: u8,
    /// -154
    pub orientation: Orientation,
    /// -150
    pub dest_x: u32,
    /// -146
    pub dest_y: u32,
    /// -142
    pub dest_z: u32,
    /// -138. Movement points at the start of the day.
    pub movement_total: u32,
    /// -134
    pub movement_remaining: u32,
    /// -130
    pub experience: u32,
    /// -126. Number of secondary skills.
    pub skill_count: u32,
    /// -122
    pub spell_points: u16,
    /// -120
    pub level: u8,
    /// -56
    pub creatures: [Creature; MAX_STACKS],
    /// -28
    pub creature_count: [u32; MAX_STACKS],
    /// 0. Null-padded.
    pub name: [u8; 13],
    /// +13. One per skill, Runes last.
    pub skill_levels: [SkillLevel; SKILL_COUNT],
    /// +69
    pub attack: u8,
    /// +70
    pub defense: u8,
    /// +71
    pub power: u8,
    /// +72
    pub knowledge: u8,
    /// +73. Spells written in the spell book: 1 if written, otherwise 0.
    pub spellbook: [u8; SPELL_COUNT],
    /// +143. All available spells, from the spell book and artifacts: 1 if available, otherwise 0.
    pub spells: [u8; SPELL_COUNT],
    /// +213. Worn artifacts, then the backpack.
    pub artifacts: [ArtifactEntry; ARTIFACT_SLOT_COUNT],
    /// +923. Slot 1-8 of each skill, 0 if not learned.
    pub skill_slots: [u8; SKILL_COUNT],
}

impl Default for Hero {
    fn default() -> Self {
        Hero {
            x: 0xFFFF,
            y: 0xFFFF,
            z: 0xFFFF,
            on_map: 0,
            player: 0xFF,
            orientation: Orientation::N,
            dest_x: 0xFFFF_FFFF,
            dest_y: 0xFFFF_FFFF,
            dest_z: 0xFFFF_FFFF,
            movement_total: 0,
            movement_remaining: 0,
            experience: 0,
            skill_count: 0,
            spell_points: 0,
            level: 0,
            creatures: [Creature::EMPTY; MAX_STACKS],
            creature_count: [0; MAX_STACKS],
            name: [0; 13],
            skill_levels: [SkillLevel::NONE; SKILL_COUNT],
            attack: 0,
            defense: 0,
            power: 0,
            knowledge: 0,
            spellbook: [0; SPELL_COUNT],
            spells: [0; SPELL_COUNT],
            artifacts: [ArtifactEntry::default(); ARTIFACT_SLOT_COUNT],
            skill_slots: [0; SKILL_COUNT],
        }
    }
}

impl Hero {
    /// Read one hero record starting at `pos`.
    pub(crate) fn read(data: &[u8], pos: usize) -> ReadResult<Hero> {
        let mut r = Reader::at(data, pos);
        let x = r.u16()?;
        let y = r.u16()?;
        let z = r.u16()?;
        let on_map = r.u8()?;
        r.skip(19)?;
        let player = r.u8()?;
        r.skip(14)?;
        let orientation = Orientation(r.u8()?);
        r.skip(3)?;
        let dest_x = r.u32()?;
        let dest_y = r.u32()?;
        let dest_z = r.u32()?;
        let movement_total = r.u32()?;
        let movement_remaining = r.u32()?;
        let experience = r.u32()?;
        let skill_count = r.u32()?;
        let spell_points = r.u16()?;
        let level = r.u8()?;
        r.skip(63)?;
        let creatures = r.each(Reader::u32)?.map(Creature);
        let creature_count = r.each(Reader::u32)?;
        debug_assert_eq!(r.pos().wrapping_sub(pos), NAME_OFFSET);
        let name = r.array()?;
        let skill_levels = r.each(Reader::u8)?.map(SkillLevel);
        r.skip(26)?; // Legacy skill slots, unused.
        let attack = r.u8()?;
        let defense = r.u8()?;
        let power = r.u8()?;
        let knowledge = r.u8()?;
        let spellbook = r.array()?;
        let spells = r.array()?;
        let artifacts = r
            .each(|r| Ok((r.u32()?, r.u32()?)))?
            .map(|(id, scroll_spell_id)| ArtifactEntry {
                artifact: Artifact(id),
                scroll_spell_id,
            });
        r.skip(46)?;
        let skill_slots = r.array()?;
        r.skip(12)?; // New in HotA 1.8.
        debug_assert_eq!(r.pos().wrapping_sub(pos), HERO_RECORD_SIZE);
        Ok(Hero {
            x,
            y,
            z,
            on_map,
            player,
            orientation,
            dest_x,
            dest_y,
            dest_z,
            movement_total,
            movement_remaining,
            experience,
            skill_count,
            spell_points,
            level,
            creatures,
            creature_count,
            name,
            skill_levels,
            attack,
            defense,
            power,
            knowledge,
            spellbook,
            spells,
            artifacts,
            skill_slots,
        })
    }

    /// The hero's name, up to the first null byte.
    #[must_use]
    pub fn name(&self) -> String {
        latin1(self.name.split(|&b| b == 0).next().unwrap_or_default())
    }

    /// In a town garrison: owned, but not on the map.
    #[must_use]
    pub fn garrison(&self) -> bool {
        self.on_map == 0 && usize::from(self.player) < crate::player::MAX_PLAYERS
    }

    /// In a prison: not on the map and not owned.
    #[must_use]
    pub fn prison(&self) -> bool {
        self.on_map == 0 && self.player == 0xFF
    }

    #[must_use]
    pub fn attack_in_game(&self) -> u8 {
        self.attack.min(MAX_PRIMARY_SKILL_VALUE)
    }

    #[must_use]
    pub fn defense_in_game(&self) -> u8 {
        self.defense.min(MAX_PRIMARY_SKILL_VALUE)
    }

    #[must_use]
    pub fn power_in_game(&self) -> u8 {
        self.power.min(MAX_PRIMARY_SKILL_VALUE)
    }

    #[must_use]
    pub fn knowledge_in_game(&self) -> u8 {
        self.knowledge.min(MAX_PRIMARY_SKILL_VALUE)
    }

    /// Whether the spell is available, from the spell book or an artifact.
    #[must_use]
    pub fn has_spell(&self, spell: Spell) -> bool {
        self.spells.get(usize::from(spell.0)).is_some_and(|&v| v != 0)
    }

    /// Whether the spell is written in the spell book.
    #[must_use]
    pub fn has_spell_in_spellbook(&self, spell: Spell) -> bool {
        self.spellbook.get(usize::from(spell.0)).is_some_and(|&v| v != 0)
    }

    /// The hero's secondary skills in slot order. Empty slots are default.
    #[must_use]
    pub fn skills(&self) -> SkillSlots {
        let mut skills = [SkillSlot::default(); SKILL_SLOT_COUNT];
        let levels_and_slots = self.skill_levels.iter().zip(&self.skill_slots);
        // Skill IDs are the array index; `0u8..` counts them without a cast.
        for (id, (&level, &slot)) in (0u8..).zip(levels_and_slots) {
            if level == SkillLevel::NONE {
                continue;
            }
            // Slots count from 1. A learned skill without a valid slot is left out.
            let slot = usize::from(slot);
            if let Some(entry) = slot.checked_sub(1).and_then(|s| skills.get_mut(s)) {
                *entry = SkillSlot::new(Skill(id), level);
            }
        }
        skills
    }

    /// Army strength: AI value of the army, scaled by attack and defense when the hero is on the map.
    #[must_use]
    pub fn army_strength(&self) -> u64 {
        stats::hero_army_strength(self)
    }

    /// The creature with the highest AI value, or `Creature::EMPTY` for an empty army.
    #[must_use]
    pub fn best_creature(&self) -> Creature {
        stats::best_creature(&self.creatures)
    }

    /// The artifact in a slot, or `Artifact::NONE`.
    #[must_use]
    pub fn artifact(&self, slot: ArtifactSlot) -> Artifact {
        self.artifact_at(usize::from(slot.0))
    }

    /// The artifact at a slot index, or `Artifact::NONE` for an empty or out-of-range slot.
    #[must_use]
    pub fn artifact_at(&self, index: usize) -> Artifact {
        self.artifacts.get(index).map_or(Artifact::NONE, |e| e.artifact)
    }

    /// The spell on a spell scroll in a slot.
    #[must_use]
    pub fn spell_from_scroll(&self, slot: ArtifactSlot) -> Spell {
        let id = self
            .artifacts
            .get(usize::from(slot.0))
            .map_or(u32::MAX, |e| e.scroll_spell_id);
        // Spell IDs fit in a u8. An empty slot (u32::MAX) gives 255, as the C++ truncation does.
        Spell(u8::try_from(id).unwrap_or(u8::MAX))
    }

    #[must_use]
    pub fn helm(&self) -> Artifact {
        self.artifact(ArtifactSlot::HELM)
    }
    #[must_use]
    pub fn cloak(&self) -> Artifact {
        self.artifact(ArtifactSlot::CLOAK)
    }
    #[must_use]
    pub fn neck(&self) -> Artifact {
        self.artifact(ArtifactSlot::NECK)
    }
    #[must_use]
    pub fn weapon(&self) -> Artifact {
        self.artifact(ArtifactSlot::WEAPON)
    }
    #[must_use]
    pub fn shield(&self) -> Artifact {
        self.artifact(ArtifactSlot::SHIELD)
    }
    #[must_use]
    pub fn armor(&self) -> Artifact {
        self.artifact(ArtifactSlot::ARMOR)
    }
    #[must_use]
    pub fn ring_left(&self) -> Artifact {
        self.artifact(ArtifactSlot::RING_LEFT)
    }
    #[must_use]
    pub fn ring_right(&self) -> Artifact {
        self.artifact(ArtifactSlot::RING_RIGHT)
    }
    #[must_use]
    pub fn boots(&self) -> Artifact {
        self.artifact(ArtifactSlot::BOOTS)
    }
    #[must_use]
    pub fn misc01(&self) -> Artifact {
        self.artifact(ArtifactSlot::MISC01)
    }
    #[must_use]
    pub fn misc02(&self) -> Artifact {
        self.artifact(ArtifactSlot::MISC02)
    }
    #[must_use]
    pub fn misc03(&self) -> Artifact {
        self.artifact(ArtifactSlot::MISC03)
    }
    #[must_use]
    pub fn misc04(&self) -> Artifact {
        self.artifact(ArtifactSlot::MISC04)
    }
    #[must_use]
    pub fn misc05(&self) -> Artifact {
        self.artifact(ArtifactSlot::MISC05)
    }
    #[must_use]
    pub fn ballista(&self) -> Artifact {
        self.artifact(ArtifactSlot::BALLISTA)
    }
    #[must_use]
    pub fn ammo_cart(&self) -> Artifact {
        self.artifact(ArtifactSlot::AMMO_CART)
    }
    #[must_use]
    pub fn first_aid_tent(&self) -> Artifact {
        self.artifact(ArtifactSlot::FIRST_AID_TENT)
    }
    #[must_use]
    pub fn catapult(&self) -> Artifact {
        self.artifact(ArtifactSlot::CATAPULT)
    }
    #[must_use]
    pub fn spell_book(&self) -> Artifact {
        self.artifact(ArtifactSlot::SPELL_BOOK)
    }

    /// The artifact at an index in the artifact list. Index 19 is the first backpack slot.
    #[must_use]
    pub fn inventory(&self, index: usize) -> Artifact {
        self.artifact_at(index)
    }

    /// Every artifact the hero carries, worn or in the backpack, with its details.
    /// Artifacts without known details are skipped.
    pub fn artifacts_with_details(&self) -> impl Iterator<Item = (Artifact, ArtifactDetails)> + '_ {
        self.artifacts
            .iter()
            .filter(|e| !e.is_empty())
            .filter_map(|e| e.artifact.details().map(|d| (e.artifact, d)))
    }
}

/// Whether the bytes at `pos` look like a hero record. Used to find the hero array.
///
/// Every check holds for every hero record in the test saves. The army check rejects runs of
/// zero bytes, where a creature would have a count of 0.
#[must_use]
pub fn is_plausible_record(data: &[u8], pos: usize) -> bool {
    pos.checked_add(HERO_RECORD_SIZE)
        .and_then(|end| data.get(pos..end))
        .and_then(plausible_record)
        .unwrap_or(false)
}

/// The checks of [`is_plausible_record`] on one record. `None` only if `r` is too short.
fn plausible_record(r: &[u8]) -> Option<bool> {
    let byte = |i: usize| r.get(i).copied();
    let u16_at = |i: usize| Some(u16::from_le_bytes(r.get(i..i.checked_add(2)?)?.try_into().ok()?));
    let u32_le = |b: &[u8]| b.try_into().ok().map(u32::from_le_bytes);

    let owner = byte(26)?;
    let unset_position = u16_at(0)? == 0xFFFF && u16_at(2)? == 0xFFFF && u16_at(4)? == 0xFFFF;
    let basics = byte(6)? <= 1
        && (usize::from(owner) < crate::player::MAX_PLAYERS || owner == 0xFF)
        && byte(41)? <= 7
        && (unset_position || u16_at(4)? <= 1);
    if !basics {
        return Some(false);
    }
    let army = r.get(139..195)?;
    let (ids, counts) = army.split_at_checked(4 * MAX_STACKS)?;
    let well_formed_army =
        ids.chunks_exact(4)
            .zip(counts.chunks_exact(4))
            .all(|(id, count)| match (u32_le(id), u32_le(count)) {
                (Some(id), Some(count)) => (id == Creature::EMPTY.0 && count == 0) || (id < 1024 && count > 0),
                _ => false,
            });
    // Heroes added in HotA 1.7 and 1.8 that are not in play have an all-zero army, no owner
    // and no position. Zero-filled data has owner 0, so it still fails.
    let unused_record = owner == 0xFF && unset_position && army.iter().all(|&b| b == 0);
    let army_ok = well_formed_army || unused_record;
    // A name is printable characters followed by null bytes only.
    let name = r.get(NAME_OFFSET..NAME_OFFSET + 13)?;
    let name_len = name.iter().position(|&b| b == 0).unwrap_or(name.len());
    let (text, padding) = name.split_at_checked(name_len)?;
    let name_ok = text.iter().all(|&b| b >= 0x20 && b != 0x7F) && padding.iter().all(|&b| b == 0);
    Some(
        army_ok
            && name_ok
            && r.get(208..208 + SKILL_COUNT)?.iter().all(|&b| b <= 3)
            && r.get(268..268 + 2 * SPELL_COUNT)?.iter().all(|&b| b <= 1)
            && r.get(1118..1118 + SKILL_COUNT)?
                .iter()
                .all(|&b| usize::from(b) <= SKILL_SLOT_COUNT),
    )
}

/// Read `count` consecutive hero records starting at `pos`.
pub(crate) fn read_all(data: &[u8], pos: usize, count: usize) -> ReadResult<Vec<Hero>> {
    (0..count)
        .map(|i| Hero::read(data, record_pos(pos, i, HERO_RECORD_SIZE)?))
        .collect()
}

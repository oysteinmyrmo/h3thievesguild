//! Army strength and AI values.

use crate::creatures::{Creature, MAX_STACKS};
use crate::hero::Hero;

/// AI value of one stack. Unknown creatures count as 0.
#[must_use]
pub fn ai_value(creature: Creature, count: u32) -> u64 {
    u64::from(creature.ai_value().unwrap_or(0)).saturating_mul(u64::from(count))
}

/// Sum of the AI values of an army's stacks.
#[must_use]
pub fn army_ai_value(creatures: &[Creature; MAX_STACKS], counts: &[u32; MAX_STACKS]) -> u64 {
    creatures
        .iter()
        .zip(counts)
        .map(|(&c, &n)| ai_value(c, n))
        .fold(0, u64::saturating_add)
}

/// Army strength of a hero, from <https://heroes.thelazy.net/index.php/Army_strength>.
/// On the map, the AI value is scaled by the hero's attack and defense.
#[must_use]
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the factor is positive and AI values are far below 2^52; truncation matches the C++ code"
)]
pub fn hero_army_strength(hero: &Hero) -> u64 {
    let ai_value = army_ai_value(&hero.creatures, &hero.creature_count);
    if hero.on_map != 0 {
        let attack = f64::from(hero.attack_in_game());
        let defense = f64::from(hero.defense_in_game());
        let factor = ((attack * 0.05 + 1.0) * (defense * 0.05 + 1.0)).sqrt();
        // Truncation matches the C++ code: uint64_t(aiValue * heroFactor).
        return (ai_value as f64 * factor) as u64;
    }
    ai_value
}

/// The creature with the highest AI value, or `Creature::EMPTY` if none has a positive value.
#[must_use]
pub fn best_creature(creatures: &[Creature; MAX_STACKS]) -> Creature {
    let mut best = Creature::EMPTY;
    let mut best_value = 0;
    for &creature in creatures {
        let value = creature.ai_value().unwrap_or(0);
        if value > best_value {
            best_value = value;
            best = creature;
        }
    }
    best
}

/// AI value of a hero's army, without the attack and defense scaling.
#[must_use]
pub fn hero_ai_value(hero: &Hero) -> u64 {
    army_ai_value(&hero.creatures, &hero.creature_count)
}

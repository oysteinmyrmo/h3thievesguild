//! The statistics in a data set: what each is called and how it is read from a save.

use h3save::player::Player;
use h3save::{Creature, Hero, SaveFile};
use serde::Serialize;

/// Describes one statistic, so the frontend can list and label it.
#[cfg_attr(
    feature = "typescript",
    derive(tsify::Tsify),
    tsify(missing_as_null, hashmap_as_object)
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatInfo {
    /// Key in the `values` of a player or hero series.
    pub id: &'static str,
    /// Name to show.
    pub name: &'static str,
    /// Category to show it under, such as "Economy". The page lists the categories in the
    /// order they first appear.
    pub group: &'static str,
    /// What the values count.
    pub unit: Unit,
}

/// What a statistic's values count.
#[cfg_attr(feature = "typescript", derive(tsify::Tsify))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Unit {
    /// Whole things: gold, heroes, levels and so on.
    Count,
    /// A percentage, from 0 to 100.
    Percent,
}

/// A player statistic and how to read it.
pub(crate) struct PlayerStat {
    pub info: StatInfo,
    pub read: fn(&Player, &SaveFile) -> f64,
    /// Text to show with the value, such as the name of the creature it is the AI value of.
    /// `None` for statistics that are only a number.
    pub label: Option<fn(&Player, &SaveFile) -> Option<String>>,
}

/// A hero statistic and how to read it.
pub(crate) struct HeroStat {
    pub info: StatInfo,
    pub read: fn(&Hero) -> f64,
}

const fn info(id: &'static str, name: &'static str, group: &'static str) -> StatInfo {
    StatInfo {
        id,
        name,
        group,
        unit: Unit::Count,
    }
}

const fn percent(id: &'static str, name: &'static str, group: &'static str) -> StatInfo {
    StatInfo {
        id,
        name,
        group,
        unit: Unit::Percent,
    }
}

/// The share of the map the player has seen, in percent.
fn map_explored(player: &Player, save: &SaveFile) -> f64 {
    save.visibility.share_seen(player.color()) * 100.0
}

#[expect(
    clippy::cast_precision_loss,
    reason = "values below 2^53 are exact; larger ones only lose precision"
)]
fn big(value: u64) -> f64 {
    value as f64
}

fn on_map(player: &Player, save: &SaveFile) -> f64 {
    save.players
        .get(player.color().index())
        .map_or(0.0, |data| f64::from(data.hero_count))
}

fn total_levels(player: &Player, _: &SaveFile) -> f64 {
    player.heroes().iter().map(|h| f64::from(h.level)).sum()
}

fn highest_level(player: &Player, _: &SaveFile) -> f64 {
    player.heroes().iter().map(|h| f64::from(h.level)).fold(0.0, f64::max)
}

fn total_experience(player: &Player, _: &SaveFile) -> f64 {
    player.heroes().iter().map(|h| f64::from(h.experience)).sum()
}

fn count(n: usize) -> f64 {
    u32::try_from(n).map_or(f64::from(u32::MAX), f64::from)
}

/// Statistics of each player, in the order the frontend should list them.
pub(crate) const PLAYER_STATS: &[PlayerStat] = &[
    PlayerStat {
        info: info("gold", "Gold", "Economy"),
        read: |p, _| f64::from(p.gold()),
        label: None,
    },
    PlayerStat {
        info: info("wood", "Wood", "Economy"),
        read: |p, _| f64::from(p.wood()),
        label: None,
    },
    PlayerStat {
        info: info("ore", "Ore", "Economy"),
        read: |p, _| f64::from(p.ore()),
        label: None,
    },
    PlayerStat {
        info: info("mercury", "Mercury", "Economy"),
        read: |p, _| f64::from(p.mercury()),
        label: None,
    },
    PlayerStat {
        info: info("sulfur", "Sulfur", "Economy"),
        read: |p, _| f64::from(p.sulfur()),
        label: None,
    },
    PlayerStat {
        info: info("crystal", "Crystal", "Economy"),
        read: |p, _| f64::from(p.crystal()),
        label: None,
    },
    PlayerStat {
        info: info("gems", "Gems", "Economy"),
        read: |p, _| f64::from(p.gems()),
        label: None,
    },
    PlayerStat {
        info: info("armyStrength", "Kingdom army strength", "Army"),
        read: |p, _| big(p.kingdom_army_strength()),
        label: None,
    },
    PlayerStat {
        info: info("bestCreature", "Best creature", "Army"),
        read: |p, _| f64::from(p.best_creature_ai_value()),
        label: Some(|p, _| p.best_creature().and_then(Creature::display_name)),
    },
    PlayerStat {
        info: info("heroes", "Heroes", "Kingdom"),
        read: |p, _| count(p.heroes().len()),
        label: None,
    },
    PlayerStat {
        info: info("heroesOnMap", "Heroes on the map", "Kingdom"),
        read: on_map,
        label: None,
    },
    PlayerStat {
        info: info("totalHeroLevels", "Total hero levels", "Kingdom"),
        read: total_levels,
        label: None,
    },
    PlayerStat {
        info: info("highestHeroLevel", "Highest hero level", "Kingdom"),
        read: highest_level,
        label: None,
    },
    PlayerStat {
        info: info("totalExperience", "Total experience", "Kingdom"),
        read: total_experience,
        label: None,
    },
    PlayerStat {
        info: info("towns", "Towns", "Kingdom"),
        read: |p, _| count(p.towns().len()),
        label: None,
    },
    PlayerStat {
        info: percent("mapExplored", "Map explored", "Map"),
        read: map_explored,
        label: None,
    },
];

/// Statistics of each hero.
pub(crate) const HERO_STATS: &[HeroStat] = &[
    HeroStat {
        info: info("level", "Level", "Hero"),
        read: |h| f64::from(h.level),
    },
    HeroStat {
        info: info("experience", "Experience", "Hero"),
        read: |h| f64::from(h.experience),
    },
    HeroStat {
        info: info("attack", "Attack", "Primary skills"),
        read: |h| f64::from(h.attack),
    },
    HeroStat {
        info: info("defense", "Defense", "Primary skills"),
        read: |h| f64::from(h.defense),
    },
    HeroStat {
        info: info("power", "Spell power", "Primary skills"),
        read: |h| f64::from(h.power),
    },
    HeroStat {
        info: info("knowledge", "Knowledge", "Primary skills"),
        read: |h| f64::from(h.knowledge),
    },
    HeroStat {
        info: info("armyStrength", "Army strength", "Army"),
        read: |h| big(h.army_strength()),
    },
];

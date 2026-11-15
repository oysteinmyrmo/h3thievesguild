//! The statistics in a data set: what each is called and how it is read from a save.

use h3save::player::Player;
use h3save::{Hero, SaveFile};
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
    /// Group to show it under, such as "Resources".
    pub group: &'static str,
}

/// A player statistic and how to read it.
pub(crate) struct PlayerStat {
    pub info: StatInfo,
    pub read: fn(&Player, &SaveFile) -> f64,
}

/// A hero statistic and how to read it.
pub(crate) struct HeroStat {
    pub info: StatInfo,
    pub read: fn(&Hero) -> f64,
}

const fn info(id: &'static str, name: &'static str, group: &'static str) -> StatInfo {
    StatInfo { id, name, group }
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
        info: info("gold", "Gold", "Resources"),
        read: |p, _| f64::from(p.gold()),
    },
    PlayerStat {
        info: info("wood", "Wood", "Resources"),
        read: |p, _| f64::from(p.wood()),
    },
    PlayerStat {
        info: info("ore", "Ore", "Resources"),
        read: |p, _| f64::from(p.ore()),
    },
    PlayerStat {
        info: info("mercury", "Mercury", "Resources"),
        read: |p, _| f64::from(p.mercury()),
    },
    PlayerStat {
        info: info("sulfur", "Sulfur", "Resources"),
        read: |p, _| f64::from(p.sulfur()),
    },
    PlayerStat {
        info: info("crystal", "Crystal", "Resources"),
        read: |p, _| f64::from(p.crystal()),
    },
    PlayerStat {
        info: info("gems", "Gems", "Resources"),
        read: |p, _| f64::from(p.gems()),
    },
    PlayerStat {
        info: info("armyStrength", "Kingdom army strength", "Army"),
        read: |p, _| big(p.kingdom_army_strength()),
    },
    PlayerStat {
        info: info("heroes", "Heroes", "Heroes"),
        read: |p, _| count(p.heroes().len()),
    },
    PlayerStat {
        info: info("heroesOnMap", "Heroes on the map", "Heroes"),
        read: on_map,
    },
    PlayerStat {
        info: info("totalHeroLevels", "Total hero levels", "Heroes"),
        read: total_levels,
    },
    PlayerStat {
        info: info("highestHeroLevel", "Highest hero level", "Heroes"),
        read: highest_level,
    },
    PlayerStat {
        info: info("totalExperience", "Total experience", "Heroes"),
        read: total_experience,
    },
    PlayerStat {
        info: info("towns", "Towns", "Towns"),
        read: |p, _| count(p.towns().len()),
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

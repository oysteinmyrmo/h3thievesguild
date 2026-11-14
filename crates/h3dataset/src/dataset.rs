//! Building a [`Dataset`] from saves.

use std::collections::{BTreeMap, BTreeSet};

use h3save::player::{MAX_PLAYERS, players};
use h3save::{Color, Date, SaveFile};
use serde::Serialize;

use crate::error::{DatasetError, DayFiles, GameFiles, UnreadableFile};
use crate::stats::{HERO_STATS, PLAYER_STATS, StatInfo};

/// One game's statistics over time, ready for plotting.
///
/// Every `values` row has one entry per day in [`Dataset::days`], `null` where the player or
/// hero was not in the game that day.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    pub map: MapInfo,
    /// The days with a save, in order. This is the x-axis.
    pub days: Vec<Day>,
    /// The player statistics, in the order to list them.
    pub player_stats: Vec<StatInfo>,
    /// The hero statistics, in the order to list them.
    pub hero_stats: Vec<StatInfo>,
    /// Players that were in the game on at least one of the days, in colour order.
    pub players: Vec<PlayerSeries>,
    /// Heroes owned by a player on at least one of the days, by hero ID.
    pub heroes: Vec<HeroSeries>,
}

/// The map the game is played on.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MapInfo {
    pub name: String,
    pub description: String,
    /// Size name: S, M, L, XL, H, XH or G.
    pub size: String,
    /// Tiles per side.
    pub tiles: u32,
    pub underground: bool,
}

/// One day with a save.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    /// Day number in the game, counting from 1. Use it for a value x-axis, where days without
    /// a save leave a gap.
    pub day: u32,
    pub month: u16,
    pub week: u16,
    /// Day of the week, 1–7.
    pub day_of_week: u16,
    /// Short label for an axis, such as "1-2-3" for month 1, week 2, day 3.
    pub label: String,
    /// The save file this day comes from.
    pub file: String,
}

/// A player's statistics over the game.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSeries {
    /// Colour ID, such as "red". Heroes refer to their owner by it.
    pub id: String,
    /// Name to show, such as "Red".
    pub name: String,
    /// The player's colour as a CSS colour, such as "#f80000".
    pub color: String,
    /// The first day the player was out of the game after having been in it, if any.
    pub eliminated_day: Option<u32>,
    /// One row per player statistic, keyed by [`StatInfo::id`].
    pub values: BTreeMap<String, Vec<Option<f64>>>,
}

/// A hero's statistics over the game.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeroSeries {
    /// Index of the hero record in the save.
    pub id: u32,
    pub name: String,
    /// The owner's colour ID on each day, `null` while the hero has no owner.
    pub owner: Vec<Option<String>>,
    /// One row per hero statistic, keyed by [`StatInfo::id`]. `null` while unowned.
    pub values: BTreeMap<String, Vec<Option<f64>>>,
}

/// What a data set needs from one save.
#[derive(Clone, Debug)]
struct Summary {
    file: String,
    date: Date,
    map: MapInfo,
    /// Each player's statistics, `None` when the player is not in the game.
    players: [Option<Vec<f64>>; MAX_PLAYERS],
    heroes: Vec<OwnedHero>,
}

/// A hero owned by a player in one save.
#[derive(Clone, Debug)]
struct OwnedHero {
    id: u32,
    name: String,
    owner: Color,
    values: Vec<f64>,
}

impl Summary {
    fn new(file: String, save: &SaveFile) -> Summary {
        let map = MapInfo {
            name: save.map_name.clone(),
            description: save.description.clone(),
            size: save
                .map_size
                .name()
                .map_or_else(|| save.map_size.0.to_string(), str::to_string),
            tiles: u32::from(save.map_size.0),
            underground: save.has_underground != 0,
        };
        let players = players(save).map(|player| {
            player
                .alive()
                .then(|| PLAYER_STATS.iter().map(|stat| (stat.read)(&player, save)).collect())
        });
        let heroes = (0u32..)
            .zip(&save.heroes)
            .filter_map(|(id, hero)| {
                let owner = *Color::ALL.get(usize::from(hero.player))?;
                let values = HERO_STATS.iter().map(|stat| (stat.read)(hero)).collect();
                Some(OwnedHero {
                    id,
                    name: hero.name(),
                    owner,
                    values,
                })
            })
            .collect();
        Summary {
            file,
            date: save.date,
            map,
            players,
            heroes,
        }
    }

    /// Saves of the same game are on the same map.
    fn game(&self) -> (&str, &str, bool) {
        (&self.map.name, &self.map.size, self.map.underground)
    }
}

/// Collects saves and builds a [`Dataset`] from them.
#[derive(Debug, Default)]
pub struct Builder {
    saves: Vec<Summary>,
    unreadable: Vec<UnreadableFile>,
}

impl Builder {
    #[must_use]
    pub fn new() -> Builder {
        Builder::default()
    }

    /// Add a save file as read from disk. A file that cannot be read is reported by
    /// [`Builder::finish`], together with any others.
    pub fn add_file(&mut self, name: impl Into<String>, contents: &[u8]) {
        let name = name.into();
        match h3save::savefile::decompress(contents).and_then(|data| SaveFile::parse(&data)) {
            Ok(save) => self.add_save(name, &save),
            Err(e) => self.unreadable.push(UnreadableFile {
                file: name,
                reason: e.to_string(),
            }),
        }
    }

    /// Add a parsed save.
    pub fn add_save(&mut self, name: impl Into<String>, save: &SaveFile) {
        self.saves.push(Summary::new(name.into(), save));
    }

    /// Number of files added so far, readable or not.
    #[must_use]
    pub fn len(&self) -> usize {
        self.saves.len().saturating_add(self.unreadable.len())
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Build the data set.
    ///
    /// # Errors
    ///
    /// Checked in this order, each listing every file involved:
    /// [`DatasetError::NoSaves`] if nothing was added, [`DatasetError::Unreadable`] if any
    /// file is not a readable save, [`DatasetError::MixedGames`] if the saves are from more than
    /// one map, and [`DatasetError::DuplicateDays`] if two saves are from the same day.
    pub fn finish(self) -> Result<Dataset, DatasetError> {
        let Builder {
            mut saves,
            mut unreadable,
        } = self;
        if saves.is_empty() && unreadable.is_empty() {
            return Err(DatasetError::NoSaves);
        }
        if !unreadable.is_empty() {
            unreadable.sort_by(|a, b| a.file.cmp(&b.file));
            return Err(DatasetError::Unreadable(unreadable));
        }
        check_one_game(&saves)?;
        check_one_save_per_day(&saves)?;
        saves.sort_by_key(|s| s.date);
        Ok(assemble(&saves))
    }
}

/// Build a data set from save files given as `(name, contents)`.
///
/// # Errors
///
/// As [`Builder::finish`].
pub fn build<'a>(files: impl IntoIterator<Item = (&'a str, &'a [u8])>) -> Result<Dataset, DatasetError> {
    let mut builder = Builder::new();
    for (name, contents) in files {
        builder.add_file(name, contents);
    }
    builder.finish()
}

fn sorted_files<'a>(saves: impl Iterator<Item = &'a Summary>) -> Vec<String> {
    let mut files: Vec<String> = saves.map(|s| s.file.clone()).collect();
    files.sort();
    files
}

fn check_one_game(saves: &[Summary]) -> Result<(), DatasetError> {
    let games: BTreeSet<_> = saves.iter().map(Summary::game).collect();
    if games.len() <= 1 {
        return Ok(());
    }
    let games = games
        .into_iter()
        .map(|game| {
            let (map_name, map_size, underground) = game;
            GameFiles {
                map_name: map_name.to_string(),
                map_size: map_size.to_string(),
                underground,
                files: sorted_files(saves.iter().filter(|s| s.game() == game)),
            }
        })
        .collect();
    Err(DatasetError::MixedGames(games))
}

fn check_one_save_per_day(saves: &[Summary]) -> Result<(), DatasetError> {
    let mut by_date: BTreeMap<Date, Vec<&Summary>> = BTreeMap::new();
    for save in saves {
        by_date.entry(save.date).or_default().push(save);
    }
    let duplicates: Vec<DayFiles> = by_date
        .into_iter()
        .filter(|(_, saves)| saves.len() > 1)
        .map(|(date, saves)| DayFiles {
            date: date.to_string(),
            files: sorted_files(saves.into_iter()),
        })
        .collect();
    if duplicates.is_empty() {
        Ok(())
    } else {
        Err(DatasetError::DuplicateDays(duplicates))
    }
}

fn day_number(date: Date) -> u32 {
    date.days_elapsed().saturating_add(1)
}

/// Build from saves of one game, one per day, sorted by date.
fn assemble(saves: &[Summary]) -> Dataset {
    let days: Vec<Day> = saves
        .iter()
        .map(|s| Day {
            day: day_number(s.date),
            month: s.date.month(),
            week: s.date.week(),
            day_of_week: s.date.day(),
            label: format!("{}-{}-{}", s.date.month(), s.date.week(), s.date.day()),
            file: s.file.clone(),
        })
        .collect();
    let players = Color::ALL
        .into_iter()
        .filter_map(|color| player_series(saves, color))
        .collect();
    let heroes = hero_series(saves);
    let map = saves.first().map(|s| s.map.clone()).unwrap_or_default();
    Dataset {
        map,
        days,
        player_stats: PLAYER_STATS.iter().map(|s| s.info).collect(),
        hero_stats: HERO_STATS.iter().map(|s| s.info).collect(),
        players,
        heroes,
    }
}

fn color_id(color: Color) -> String {
    color.name().to_ascii_lowercase()
}

/// The series of one player, or `None` if the player is never in the game.
fn player_series(saves: &[Summary], color: Color) -> Option<PlayerSeries> {
    let rows: Vec<Option<&Vec<f64>>> = saves
        .iter()
        .map(|s| s.players.get(color.index()).and_then(Option::as_ref))
        .collect();
    let first_alive = rows.iter().position(Option::is_some)?;
    let eliminated_day = rows
        .iter()
        .zip(saves)
        .skip(first_alive)
        .find(|(row, _)| row.is_none())
        .map(|(_, s)| day_number(s.date));
    let values = PLAYER_STATS
        .iter()
        .enumerate()
        .map(|(i, stat)| {
            let row = rows
                .iter()
                .map(|r| r.and_then(|values| values.get(i).copied()))
                .collect();
            (stat.info.id.to_string(), row)
        })
        .collect();
    let rgb = color.rgb();
    Some(PlayerSeries {
        id: color_id(color),
        name: color.name().to_string(),
        color: format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b),
        eliminated_day,
        values,
    })
}

fn hero_series(saves: &[Summary]) -> Vec<HeroSeries> {
    let ids: BTreeSet<u32> = saves.iter().flat_map(|s| s.heroes.iter().map(|h| h.id)).collect();
    ids.into_iter()
        .map(|id| {
            let rows: Vec<Option<&OwnedHero>> = saves.iter().map(|s| s.heroes.iter().find(|h| h.id == id)).collect();
            // The latest name, in case a map renames the hero.
            let name = rows
                .iter()
                .rev()
                .flatten()
                .map(|h| h.name.clone())
                .next()
                .unwrap_or_default();
            let owner = rows.iter().map(|r| r.map(|h| color_id(h.owner))).collect();
            let values = HERO_STATS
                .iter()
                .enumerate()
                .map(|(i, stat)| {
                    let row = rows.iter().map(|r| r.and_then(|h| h.values.get(i).copied())).collect();
                    (stat.info.id.to_string(), row)
                })
                .collect();
            HeroSeries {
                id,
                name,
                owner,
                values,
            }
        })
        .collect()
}

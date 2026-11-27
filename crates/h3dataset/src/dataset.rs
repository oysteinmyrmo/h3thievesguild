//! Building a [`Dataset`] from saves.

use std::collections::{BTreeMap, BTreeSet};

use h3save::player::{MAX_PLAYERS, players};
use h3save::{Color, Date, SaveFile};
use serde::Serialize;

use crate::error::{DatasetError, DayFiles, GameFiles, UnreadableFile};

/// Most days a data set covers, from the first save to the last: ten years of game time. A
/// longer series means a damaged date, and would take a lot of memory to chart.
pub const MAX_DAYS: u32 = 10 * 12 * 28;
use crate::stats::{HERO_STATS, Label, PLAYER_STATS, StatInfo};

/// One game's statistics over time, ready for plotting.
///
/// Every `values` row has one entry per day in [`Dataset::days`], `null` where the player or
/// hero was not in the game that day.
#[cfg_attr(
    feature = "typescript",
    derive(tsify::Tsify),
    tsify(missing_as_null, hashmap_as_object)
)]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    pub map: MapInfo,
    /// Every day from the first save to the last, in order: the x-axis. Days without a save
    /// are included, with no file and `null` values, so the charts show the gap.
    pub days: Vec<Day>,
    /// Runs of days without a save between the first and the last save. Empty for a
    /// complete series.
    pub missing_days: Vec<DayRange>,
    /// A message for the user when days are missing, asking whether to go on. `None` for a
    /// complete series.
    pub missing_days_warning: Option<String>,
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
#[cfg_attr(
    feature = "typescript",
    derive(tsify::Tsify),
    tsify(missing_as_null, hashmap_as_object)
)]
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
#[cfg_attr(
    feature = "typescript",
    derive(tsify::Tsify),
    tsify(missing_as_null, hashmap_as_object)
)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    /// Day number in the game, counting from 1.
    pub day: u32,
    pub month: u16,
    pub week: u16,
    /// Day of the week, 1–7.
    pub day_of_week: u16,
    /// Short label for an axis, such as "1-2-3" for month 1, week 2, day 3.
    pub label: String,
    /// The save file this day comes from, or `None` if there is no save for the day.
    pub file: Option<String>,
}

/// A run of consecutive days, such as days without a save.
#[cfg_attr(
    feature = "typescript",
    derive(tsify::Tsify),
    tsify(missing_as_null, hashmap_as_object)
)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DayRange {
    /// First day number of the run.
    pub first: u32,
    /// Last day number of the run, the same as `first` for a single day.
    pub last: u32,
    /// The run as text, such as "Month 1, Week 1, Day 2" or
    /// "Month 1, Week 1, Day 4 to Month 1, Week 2, Day 1".
    pub label: String,
}

/// A player's statistics over the game.
#[cfg_attr(
    feature = "typescript",
    derive(tsify::Tsify),
    tsify(missing_as_null, hashmap_as_object)
)]
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
    /// One row per player statistic, keyed by [`StatInfo::id`]. Every statistic is 0 on the
    /// day the player is eliminated, so the lines drop to 0 there, and missing after it.
    pub values: BTreeMap<String, Vec<Option<f64>>>,
    /// Text to show with the values, for the statistics that have it, keyed by
    /// [`StatInfo::id`]: for "bestCreature", the creature's name. One entry per day, `null` on
    /// days without a value or without anything to name, such as a player with no creatures.
    pub labels: BTreeMap<String, Vec<Option<String>>>,
    /// Pictures to show with the labels, keyed and laid out like `labels`, `null` where there
    /// is none. A picture is named by its path in the site's `assets` folder, without the
    /// extension: "creatures/BlackDragon" is `assets/creatures/BlackDragon.png`.
    pub icons: BTreeMap<String, Vec<Option<String>>>,
}

/// A hero's statistics over the game.
#[cfg_attr(
    feature = "typescript",
    derive(tsify::Tsify),
    tsify(missing_as_null, hashmap_as_object)
)]
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
    players: [Option<PlayerDay>; MAX_PLAYERS],
    heroes: Vec<OwnedHero>,
}

/// A player's statistics in one save.
#[derive(Clone, Debug)]
struct PlayerDay {
    /// One value per entry of [`PLAYER_STATS`].
    values: Vec<f64>,
    /// One label per entry of [`PLAYER_STATS`], `None` for those without.
    labels: Vec<Option<Label>>,
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
            player.alive().then(|| PlayerDay {
                values: PLAYER_STATS.iter().map(|stat| (stat.read)(&player, save)).collect(),
                labels: PLAYER_STATS
                    .iter()
                    .map(|stat| stat.label.and_then(|label| label(&player, save)))
                    .collect(),
            })
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
    /// one map, [`DatasetError::DuplicateDays`] if two saves are from the same day, and
    /// [`DatasetError::TooManyDays`] if the saves span more than [`MAX_DAYS`] days.
    ///
    /// Days without a save are not an error: they are in [`Dataset::missing_days`], and the
    /// frontend should ask the user whether to go on.
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
        check_span(&saves)?;
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

/// Saves sorted by date must not span more than [`MAX_DAYS`].
fn check_span(saves: &[Summary]) -> Result<(), DatasetError> {
    let (Some(first), Some(last)) = (saves.first(), saves.last()) else {
        return Ok(());
    };
    let days = day_number(last.date)
        .saturating_sub(day_number(first.date))
        .saturating_add(1);
    if days <= MAX_DAYS {
        return Ok(());
    }
    Err(DatasetError::TooManyDays {
        days,
        first: DayFiles {
            date: first.date.to_string(),
            files: vec![first.file.clone()],
        },
        last: DayFiles {
            date: last.date.to_string(),
            files: vec![last.file.clone()],
        },
    })
}

fn day_number(date: Date) -> u32 {
    date.days_elapsed().saturating_add(1)
}

/// The date of a day number, which counts from 1.
fn date_of(day: u32) -> Option<Date> {
    Date::from_days_elapsed(day.checked_sub(1)?)
}

/// For each day from the first save to the last, the index of its save, if any.
fn slots(saves: &[Summary]) -> Vec<Option<usize>> {
    let (Some(first), Some(last)) = (saves.first(), saves.last()) else {
        return Vec::new();
    };
    let (first, last) = (day_number(first.date), day_number(last.date));
    let mut slots = Vec::new();
    let mut next = saves.iter().map(|s| day_number(s.date)).enumerate().peekable();
    for day in first..=last {
        slots.push(next.next_if(|&(_, d)| d == day).map(|(i, _)| i));
    }
    slots
}

/// Spread values given per save over every day, with `None` for days without a save.
fn spread<T: Clone>(per_save: &[Option<T>], slots: &[Option<usize>]) -> Vec<Option<T>> {
    slots
        .iter()
        .map(|slot| slot.and_then(|i| per_save.get(i).cloned().flatten()))
        .collect()
}

fn day_label(date: Date) -> String {
    format!("{}-{}-{}", date.month(), date.week(), date.day())
}

/// The runs of days without a save.
fn missing_days(days: &[Day]) -> Vec<DayRange> {
    let mut runs: Vec<(u32, u32)> = Vec::new();
    for day in days.iter().filter(|d| d.file.is_none()).map(|d| d.day) {
        match runs.last_mut() {
            Some((_, last)) if last.checked_add(1) == Some(day) => *last = day,
            _ => runs.push((day, day)),
        }
    }
    let text = |day: u32| date_of(day).map_or_else(|| format!("day {day}"), |d| d.to_string());
    runs.into_iter()
        .map(|(first, last)| DayRange {
            first,
            last,
            label: if first == last {
                text(first)
            } else {
                format!("{} to {}", text(first), text(last))
            },
        })
        .collect()
}

fn missing_days_warning(missing: &[DayRange]) -> Option<String> {
    if missing.is_empty() {
        return None;
    }
    let count: u32 = missing
        .iter()
        .map(|r| r.last.saturating_sub(r.first).saturating_add(1))
        .sum();
    let days = if count == 1 { "1 day" } else { "" };
    let days = if days.is_empty() {
        format!("{count} days")
    } else {
        days.to_string()
    };
    let mut message = format!("There are no saves for {days} between the first and the last save:");
    for run in missing {
        message.push_str("\n- ");
        message.push_str(&run.label);
    }
    message.push_str("\nThese days are left empty in the charts. Continue?");
    Some(message)
}

/// Build from saves of one game, one per day, sorted by date.
fn assemble(saves: &[Summary]) -> Dataset {
    let slots = slots(saves);
    let first_day = saves.first().map_or(1, |s| day_number(s.date));
    let days: Vec<Day> = (first_day..)
        .zip(&slots)
        .filter_map(|(day, slot)| {
            let date = date_of(day)?;
            Some(Day {
                day,
                month: date.month(),
                week: date.week(),
                day_of_week: date.day(),
                label: day_label(date),
                file: slot.and_then(|i| saves.get(i)).map(|s| s.file.clone()),
            })
        })
        .collect();
    let missing_days = missing_days(&days);
    let missing_days_warning = missing_days_warning(&missing_days);
    let players = Color::ALL
        .into_iter()
        .filter_map(|color| player_series(saves, &slots, color))
        .collect();
    let heroes = hero_series(saves, &slots);
    let map = saves.first().map(|s| s.map.clone()).unwrap_or_default();
    Dataset {
        map,
        days,
        missing_days,
        missing_days_warning,
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
fn player_series(saves: &[Summary], slots: &[Option<usize>], color: Color) -> Option<PlayerSeries> {
    let rows: Vec<Option<&PlayerDay>> = saves
        .iter()
        .map(|s| s.players.get(color.index()).and_then(Option::as_ref))
        .collect();
    // Elimination is judged on the days with a save: a missing day says nothing.
    let first_alive = rows.iter().position(Option::is_some)?;
    let eliminated = rows
        .iter()
        .enumerate()
        .skip(first_alive)
        .find_map(|(save, row)| row.is_none().then_some(save));
    let eliminated_day = eliminated.and_then(|save| saves.get(save)).map(|s| day_number(s.date));
    let values = PLAYER_STATS
        .iter()
        .enumerate()
        .map(|(i, stat)| {
            let per_save: Vec<Option<f64>> = rows
                .iter()
                .enumerate()
                .map(|(save, r)| match r {
                    Some(day) => day.values.get(i).copied(),
                    None if Some(save) == eliminated => Some(0.0),
                    None => None,
                })
                .collect();
            (stat.info.id.to_string(), spread(&per_save, slots))
        })
        .collect();
    // The label of each labelled statistic on each saved day, by statistic ID.
    let labelled: Vec<(&str, Vec<Option<&Label>>)> = PLAYER_STATS
        .iter()
        .enumerate()
        .filter(|(_, stat)| stat.label.is_some())
        .map(|(i, stat)| {
            let per_save = rows
                .iter()
                .map(|r| r.and_then(|day| day.labels.get(i)).and_then(Option::as_ref))
                .collect();
            (stat.info.id, per_save)
        })
        .collect();
    let by_day = |part: fn(&Label) -> Option<String>| -> BTreeMap<String, Vec<Option<String>>> {
        labelled
            .iter()
            .map(|(id, per_save)| {
                let per_save: Vec<Option<String>> = per_save.iter().map(|l| l.and_then(part)).collect();
                ((*id).to_string(), spread(&per_save, slots))
            })
            .collect()
    };
    let labels = by_day(|l| Some(l.text.clone()));
    let icons = by_day(|l| l.icon.clone());
    let rgb = color.rgb();
    Some(PlayerSeries {
        id: color_id(color),
        name: color.name().to_string(),
        color: format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b),
        eliminated_day,
        values,
        labels,
        icons,
    })
}

fn hero_series(saves: &[Summary], slots: &[Option<usize>]) -> Vec<HeroSeries> {
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
            let per_save: Vec<Option<String>> = rows.iter().map(|r| r.map(|h| color_id(h.owner))).collect();
            let owner = spread(&per_save, slots);
            let values = HERO_STATS
                .iter()
                .enumerate()
                .map(|(i, stat)| {
                    let per_save: Vec<Option<f64>> =
                        rows.iter().map(|r| r.and_then(|h| h.values.get(i).copied())).collect();
                    (stat.info.id.to_string(), spread(&per_save, slots))
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

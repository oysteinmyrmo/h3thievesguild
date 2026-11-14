//! Why a set of saves cannot be turned into a data set.

use std::fmt;

use serde::Serialize;

/// A file that could not be read as a save.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct UnreadableFile {
    pub file: String,
    pub reason: String,
}

/// The files of one game, when saves from several games were mixed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameFiles {
    pub map_name: String,
    pub map_size: String,
    pub underground: bool,
    pub files: Vec<String>,
}

/// Files saved on the same in-game day.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DayFiles {
    /// The day, such as "Month 1, Week 2, Day 3".
    pub date: String,
    pub files: Vec<String>,
}

/// Why the saves do not make a data set. Each variant lists every file involved, sorted, so the
/// user can see what to fix.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", content = "details", rename_all = "camelCase")]
pub enum DatasetError {
    /// No saves were given.
    NoSaves,
    /// Some files are not saves, or are damaged.
    Unreadable(Vec<UnreadableFile>),
    /// The saves come from more than one game.
    MixedGames(Vec<GameFiles>),
    /// More than one save for the same day.
    DuplicateDays(Vec<DayFiles>),
    /// The saves span more days than a data set can hold; a date is likely damaged.
    #[serde(rename_all = "camelCase")]
    TooManyDays { days: u32, first: DayFiles, last: DayFiles },
}

fn list(files: &[String]) -> String {
    files.join(", ")
}

impl fmt::Display for DatasetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatasetError::NoSaves => write!(f, "no save files were given"),
            DatasetError::Unreadable(files) => {
                write!(f, "these files could not be read as saves:")?;
                for file in files {
                    write!(f, "\n- {}: {}", file.file, file.reason)?;
                }
                Ok(())
            }
            DatasetError::MixedGames(games) => {
                write!(
                    f,
                    "the saves come from {} different games; select the saves of one game:",
                    games.len()
                )?;
                for game in games {
                    let underground = if game.underground { " with underground" } else { "" };
                    let (name, size) = (&game.map_name, &game.map_size);
                    write!(f, "\n- map \"{name}\" ({size}{underground}): {}", list(&game.files))?;
                }
                Ok(())
            }
            DatasetError::TooManyDays { days, first, last } => write!(
                f,
                "the saves span {days} days, more than a data set can hold; check the dates of the first save, {} ({}), and the last, {} ({})",
                list(&first.files),
                first.date,
                list(&last.files),
                last.date
            ),
            DatasetError::DuplicateDays(days) => {
                write!(f, "more than one save for the same day; keep one save per day:")?;
                for day in days {
                    write!(f, "\n- {}: {}", day.date, list(&day.files))?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for DatasetError {}

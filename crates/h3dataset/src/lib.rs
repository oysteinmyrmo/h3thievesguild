//! Turns the save files of one game into a data set for plotting.
//!
//! Each save is one point in time. [`Builder`] reads the saves, checks that they belong to one
//! game with one save per day, and lines them up by date into a [`Dataset`]: one row of values
//! per player and per hero for each statistic, aligned with the list of days. Days without a
//! save between the first and the last are kept, with no values, and listed in
//! [`Dataset::missing_days`] so the user can be asked whether to go on.
//!
//! The data set is shaped for [ECharts](https://echarts.apache.org): `days` gives the x-axis,
//! and each player's `values[stat]` is a ready-made series with `null` where the player was not
//! in the game.
//!
//! ```no_run
//! let mut builder = h3dataset::Builder::new();
//! for name in ["[hotseat] 111.GM2", "[hotseat] 112.GM2"] {
//!     builder.add_file(name, &std::fs::read(name)?);
//! }
//! let dataset = builder.finish()?;
//! for player in &dataset.players {
//!     println!("{}: gold {:?}", player.name, player.values["gold"]);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

// Save files come from users: the library must not panic on any input. Tests may.
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

mod dataset;
mod error;
pub mod stats;

pub use dataset::{Builder, Dataset, Day, DayRange, HeroSeries, MAX_DAYS, MapInfo, PlayerSeries, build};
pub use error::{DatasetError, DayFiles, GameFiles, UnreadableFile};
pub use stats::{StatInfo, Unit};

//! Reading a save file: decompression, the file header, and locating players, towns and heroes.

use std::fmt;
use std::io::Read;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;

use crate::hero::{self, HERO_COUNT, Hero};
use crate::id_type;
use crate::player::{self, MAX_PLAYERS, PLAYER_RECORD_SIZE, PlayerData};
use crate::reader::{OutOfBounds, Reader, find};
use crate::town::{self, LONGEST_TOWN_NAME, TOWN_NAMES, TOWN_SIZE_WITHOUT_NAME, Town};

/// Magic bytes of a scenario save, including the trailing null byte.
pub const HEADER_NORMAL: &[u8; 6] = b"H3SVG\0";
/// Magic bytes of a campaign save, including the trailing null byte.
pub const HEADER_CAMPAIGN: &[u8; 6] = b"H3SVC\0";
/// Name of the first hero in the hero array, unless the map renames him.
pub const DEFAULT_FIRST_HERO_NAME: &str = "Orrin";

id_type! {
    /// Map size in tiles per side. All maps are square and may have an underground level.
    MapSize(u8) {
        NA = 0 => "NA",
        S = 36 => "S",
        M = 72 => "M",
        L = 108 => "L",
        XL = 144 => "XL",
        H = 180 => "H",
        XH = 216 => "XH",
        G = 252 => "G",
    }
}

/// What to read, and how to find the towns and heroes.
#[derive(Clone, Debug)]
pub struct Input {
    /// The HotA save file to read.
    pub path: PathBuf,
    /// Name of red player's start town, which is the first town in the file. If empty, towns
    /// are found by searching backwards from the first hero for default town names.
    pub first_town_name: String,
    /// Number of towns. Must be set when any town has a non-default name; 0 means find them
    /// automatically.
    pub number_of_towns: usize,
    /// Name of the first hero, used to find the heroes. Orrin unless the map renames him.
    pub first_hero_name: String,
}

impl Input {
    /// Read a save whose towns all have default names.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Input {
            path: path.into(),
            first_town_name: String::new(),
            number_of_towns: 0,
            first_hero_name: DEFAULT_FIRST_HERO_NAME.to_owned(),
        }
    }

    /// Read a save with renamed towns: give red player's start town and the number of towns.
    pub fn with_towns(path: impl Into<PathBuf>, first_town_name: impl Into<String>, number_of_towns: usize) -> Self {
        Input {
            first_town_name: first_town_name.into(),
            number_of_towns,
            ..Input::new(path)
        }
    }

    /// Whether towns are found by searching for default town names.
    #[must_use]
    pub fn read_all_towns_automatically(&self) -> bool {
        self.first_town_name.is_empty() || self.number_of_towns == 0
    }
}

/// Why a save could not be read.
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    /// The file does not decompress, or decompresses to something that is not a save.
    NotASaveFile,
    /// A record ran past the end of the decompressed data.
    Truncated {
        pos: usize,
        len: usize,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "could not read the save file: {e}"),
            Error::NotASaveFile => write!(f, "not a Heroes 3 save file"),
            Error::Truncated { pos, len } => write!(f, "save data ends before a {len}-byte read at offset {pos}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<OutOfBounds> for Error {
    fn from(e: OutOfBounds) -> Self {
        Error::Truncated { pos: e.pos, len: e.len }
    }
}

/// Decompress a save file read from disk.
///
/// # Errors
///
/// [`Error::Io`] if the file cannot be read, and the errors of [`decompress`].
pub fn decompress_file(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    decompress(&std::fs::read(path)?)
}

/// Decompress the gzip data of a save file.
///
/// Every save ends with a gzip trailer that does not match the data, so the integrity check
/// fails at the very end. As in the C++ code, that error is ignored once all data is out.
///
/// # Errors
///
/// [`Error::NotASaveFile`] if the data is not gzip or does not start with a save header.
pub fn decompress(compressed: &[u8]) -> Result<Vec<u8>, Error> {
    let mut out = Vec::with_capacity(compressed.len() * 16);
    // On error, `read_to_end` keeps the bytes read so far. Only the trailer check can fail
    // after data has come out, so an error with data is the expected end of a save.
    if GzDecoder::new(compressed).read_to_end(&mut out).is_err() && out.is_empty() {
        return Err(Error::NotASaveFile);
    }
    if out.len() < HEADER_NORMAL.len() || !is_save_header(&out) {
        return Err(Error::NotASaveFile);
    }
    Ok(out)
}

fn is_save_header(data: &[u8]) -> bool {
    data.starts_with(HEADER_NORMAL) || data.starts_with(HEADER_CAMPAIGN)
}

/// A parsed save file.
#[derive(Clone, Debug)]
pub struct SaveFile {
    pub header: [u8; 8],
    /// Byte 80. Number of hero records (HotA 1.8).
    pub hero_count: u32,
    /// Byte 90.
    pub map_size: MapSize,
    /// Byte 94. 1 if the map has an underground level.
    pub has_underground: u8,
    /// Byte 95.
    pub map_name_size: u16,
    /// Byte 97.
    pub map_name: String,
    /// Byte 97 + map name size.
    pub description_size: u16,
    /// Byte 99 + map name size.
    pub description: String,

    pub players: [PlayerData; MAX_PLAYERS],
    pub towns: Vec<Town>,
    pub heroes: Vec<Hero>,
}

impl SaveFile {
    /// Read and parse a save file.
    ///
    /// # Errors
    ///
    /// The errors of [`decompress_file`] and [`SaveFile::parse`].
    pub fn read(input: &Input) -> Result<SaveFile, Error> {
        let data = decompress_file(&input.path)?;
        SaveFile::parse(&data, input)
    }

    /// Parse decompressed save data. `input.path` is not used.
    ///
    /// # Errors
    ///
    /// [`Error::Truncated`] if a record runs past the end of the data.
    pub fn parse(data: &[u8], input: &Input) -> Result<SaveFile, Error> {
        let mut r = Reader::at(data, 0);
        let header = r.array()?;
        r.skip(72)?;
        let hero_count = r.u32()?;
        r.skip(6)?;
        let map_size = MapSize(r.u8()?);
        r.skip(3)?;
        let has_underground = r.u8()?;
        let map_name_size = r.u16()?;
        let map_name = r.string(usize::from(map_name_size))?;
        let description_size = r.u16()?;
        let description = r.string(usize::from(description_size))?;

        let first_hero = first_hero_index(data, &input.first_hero_name);
        let (first_town, town_count) = if input.read_all_towns_automatically() {
            match first_hero {
                Some(hero_idx) => first_town_index_and_count(data, hero_idx),
                None => (None, 0),
            }
        } else {
            (first_town_index(data, &input.first_town_name), input.number_of_towns)
        };
        let first_player = first_town.and_then(first_player_index);

        let players = match first_player {
            Some(idx) => player::read_all(data, idx)?,
            None => [PlayerData::default(); MAX_PLAYERS],
        };
        let towns = match first_town {
            Some(idx) => town::read_all(data, idx, town_count)?,
            None => Vec::new(),
        };
        let heroes = match first_hero {
            Some(idx) => hero::read_all(data, idx, HERO_COUNT)?,
            None => vec![Hero::default(); HERO_COUNT],
        };

        Ok(SaveFile {
            header,
            hero_count,
            map_size,
            has_underground,
            map_name_size,
            map_name,
            description_size,
            description,
            players,
            towns,
            heroes,
        })
    }

    /// Whether the header is a scenario or campaign save.
    #[must_use]
    pub fn valid(&self) -> bool {
        self.normal_save_file() || self.campaign_save_file()
    }

    #[must_use]
    pub fn normal_save_file(&self) -> bool {
        self.header.starts_with(HEADER_NORMAL)
    }

    #[must_use]
    pub fn campaign_save_file(&self) -> bool {
        self.header.starts_with(HEADER_CAMPAIGN)
    }

    /// The first town with this name.
    #[must_use]
    pub fn find_town(&self, name: &str) -> Option<&Town> {
        self.towns.iter().find(|t| t.name == name)
    }

    /// The first hero with this name.
    #[must_use]
    pub fn find_hero(&self, name: &str) -> Option<&Hero> {
        self.heroes.iter().find(|h| h.name() == name)
    }
}

/// Encode a name the way the game stores it (Latin-1). Characters outside Latin-1 cannot occur.
fn latin1_bytes(name: &str) -> Vec<u8> {
    name.chars()
        .map(|c| u8::try_from(u32::from(c)).unwrap_or(b'?'))
        .collect()
}

/// The players come right before the first town (or the first hero when there are no towns).
#[must_use]
pub fn first_player_index(first_town_idx: usize) -> Option<usize> {
    first_town_idx.checked_sub(MAX_PLAYERS * PLAYER_RECORD_SIZE)
}

/// Index of the first town and the number of towns, found by searching backwards from the first
/// hero for default town names.
///
/// Returns `(None, 0)` when the search runs off the start of the data.
#[must_use]
pub fn first_town_index_and_count(data: &[u8], first_hero_idx: usize) -> (Option<usize>, usize) {
    let Some(mut town_end) = first_hero_idx.checked_sub(1) else {
        return (None, 0);
    };
    let mut town_count = 0;
    // The window start is an approximation, since the length of the town name is unknown.
    while let Some(town_start) = town_end.checked_sub(TOWN_SIZE_WITHOUT_NAME + LONGEST_TOWN_NAME) {
        let Some(window) = data.get(town_start..town_end) else {
            break;
        };
        let Some(name_idx_in_window) = TOWN_NAMES.iter().find_map(|name| find(window, &latin1_bytes(name))) else {
            break;
        };
        town_count += 1;
        let name_idx = town_start + name_idx_in_window;
        match name_idx.checked_sub(town::NAME_OFFSET + 1) {
            Some(end) => town_end = end,
            None => return (None, 0),
        }
    }
    // The file layout differs slightly when there are no towns. The C++ code has the same
    // "+ 2" with a note that the reason is not known; it makes saves without towns work.
    let first_town = if town_count == 0 { town_end + 2 } else { town_end + 1 };
    (Some(first_town), town_count)
}

/// Index of the first town, found by its name. The first town is always red player's start town.
#[must_use]
pub fn first_town_index(data: &[u8], first_town_name: &str) -> Option<usize> {
    if first_town_name.is_empty() {
        return None;
    }
    find(data, &latin1_bytes(first_town_name))?.checked_sub(town::NAME_OFFSET)
}

/// Index of the first hero, found by searching for the first hero's name (Orrin by default).
#[must_use]
pub fn first_hero_index(data: &[u8], first_hero_name: &str) -> Option<usize> {
    if first_hero_name.is_empty() {
        return None;
    }
    find(data, &latin1_bytes(first_hero_name))?.checked_sub(hero::NAME_OFFSET)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::Compression;
    use flate2::write::GzEncoder;
    use std::io::Write;

    fn gzip(bytes: &[u8]) -> Vec<u8> {
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(bytes).unwrap();
        e.finish().unwrap()
    }

    #[test]
    fn rejects_data_that_is_not_gzip() {
        assert!(matches!(decompress(b"not a save file"), Err(Error::NotASaveFile)));
    }

    #[test]
    fn rejects_gzip_that_is_not_a_save() {
        assert!(matches!(decompress(&gzip(b"hello world")), Err(Error::NotASaveFile)));
    }

    #[test]
    fn accepts_a_save_header_and_ignores_a_bad_trailer() {
        let mut compressed = gzip(b"H3SVG\0 rest of the save");
        let len = compressed.len();
        compressed[len - 8..len - 4].copy_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]); // Corrupt the CRC.
        assert_eq!(decompress(&compressed).unwrap(), b"H3SVG\0 rest of the save");
    }

    #[test]
    fn truncated_header_is_an_error_not_a_panic() {
        let result = SaveFile::parse(b"H3SVG\0", &Input::new("unused"));
        assert!(matches!(result, Err(Error::Truncated { .. })));
    }

    #[test]
    fn input_finds_towns_automatically_unless_both_are_given() {
        assert!(Input::new("a").read_all_towns_automatically());
        assert!(Input::with_towns("a", "", 3).read_all_towns_automatically());
        assert!(Input::with_towns("a", "RedsTown", 0).read_all_towns_automatically());
        assert!(!Input::with_towns("a", "RedsTown", 3).read_all_towns_automatically());
    }
}

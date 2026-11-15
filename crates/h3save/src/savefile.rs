//! Reading a save file: decompression, the file header, and locating players, towns and heroes.
//!
//! The players, towns and heroes are found by the shape of their data, not by names:
//!
//! The few sizes that change between HotA versions are in [`Format`], chosen by the version
//! in the header.
//!
//! 1. The hero array holds one fixed-size record per hero, and the header says how many.
//!    It is found by looking for the one position where every record passes
//!    [`hero::is_plausible_record`].
//! 2. The town block ends exactly where the heroes begin. It starts with a town count byte,
//!    followed by each town's index (0, 1, 2, …) and record. It is found by looking backwards
//!    from the heroes for the count byte from which the towns read through to the heroes.
//! 3. The eight player records end right where the town block starts. Each record starts with
//!    its player index, and its hero and town IDs must be in range.
//! 4. The date is a fixed distance after the end of the hero array, [`Format::date_offset`].

use std::fmt;
use std::io::Read;
use std::path::Path;

use flate2::read::GzDecoder;

use crate::date::{self, Date};
use crate::hero::{self, Hero};
use crate::id_type;
use crate::player::{self, MAX_PLAYERS, PLAYER_RECORD_SIZE, PlayerData};
use crate::reader::{OutOfBounds, Reader, record_pos};
use crate::town::{self, MAX_TOWNS, Town};

/// Magic bytes of a scenario save, including the trailing null byte.
pub const HEADER_NORMAL: &[u8; 6] = b"H3SVG\0";
/// Magic bytes of a campaign save, including the trailing null byte.
pub const HEADER_CAMPAIGN: &[u8; 6] = b"H3SVC\0";
/// Largest decompressed save accepted, 64 MiB. Saves of the largest maps are well below this;
/// the limit keeps a malicious file from decompressing into gigabytes.
pub const MAX_DECOMPRESSED_SIZE: usize = 64 << 20;

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

/// The parts of the save layout that differ between HotA versions. Everything else is found
/// by the shape of the data; see the [module documentation](self).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Format {
    /// HotA major and minor version, from header bytes 16 and 20.
    pub version: (u32, u32),
    /// Size of one hero record.
    pub hero_record_size: usize,
    /// Number of secondary skills, which sets the size of the skill slot table in each hero
    /// record.
    pub skill_count: usize,
    /// Bytes from the end of the hero array to the date. Between them are two bytes per
    /// hero, so this grows with the number of heroes.
    pub date_offset: usize,
}

impl Format {
    pub const HOTA_1_7: Format = Format {
        version: (1, 7),
        hero_record_size: 1159,
        skill_count: 29,
        date_offset: 407,
    };
    /// Adds the Runes skill with the Bulwark town, one more skill slot per hero.
    pub const HOTA_1_8: Format = Format {
        version: (1, 8),
        hero_record_size: 1160,
        skill_count: 30,
        date_offset: 441,
    };
    /// Every supported format, oldest first.
    pub const ALL: [Format; 2] = [Format::HOTA_1_7, Format::HOTA_1_8];

    /// The format of decompressed save data, by the HotA version in its header.
    ///
    /// # Errors
    ///
    /// [`Error::Truncated`] if the data ends inside the version, and
    /// [`Error::UnsupportedVersion`] if no format has that version.
    pub fn of(data: &[u8]) -> Result<Format, Error> {
        let mut r = Reader::at(data, 16);
        let version = (r.u32()?, r.u32()?);
        Format::ALL
            .into_iter()
            .find(|f| f.version == version)
            .ok_or(Error::UnsupportedVersion(version))
    }
}

/// Why a save could not be read.
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    /// The file does not decompress, or decompresses to something that is not a save.
    NotASaveFile,
    /// The file decompresses to more than [`MAX_DECOMPRESSED_SIZE`] bytes.
    TooLarge,
    /// The save was written by a HotA version with no [`Format`]. Holds the major and minor
    /// version.
    UnsupportedVersion((u32, u32)),
    /// A record ran past the end of the decompressed data.
    Truncated {
        pos: usize,
        len: usize,
    },
    /// No position in the data holds a hero array of the length the header gives.
    HeroesNotFound,
    /// More than one position holds a plausible hero array.
    AmbiguousHeroes {
        first: usize,
        second: usize,
    },
    /// No town block ends where the hero array starts.
    TownsNotFound,
    /// The eight player records before the town block do not hold together.
    PlayersNotFound,
    /// The date after the hero array is not a valid date. Holds the stored day, week and
    /// month bytes.
    InvalidDate([u8; date::SIZE]),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "could not read the save file: {e}"),
            Error::NotASaveFile => write!(f, "not a Heroes 3 save file"),
            Error::TooLarge => write!(f, "save data is larger than {MAX_DECOMPRESSED_SIZE} bytes"),
            Error::UnsupportedVersion((major, minor)) => write!(f, "saves from HotA {major}.{minor} are not supported"),
            Error::Truncated { pos, len } => write!(f, "save data ends before a {len}-byte read at offset {pos}"),
            Error::HeroesNotFound => write!(f, "could not find the hero records"),
            Error::AmbiguousHeroes { first, second } => {
                write!(f, "found hero records at both offset {first} and offset {second}")
            }
            Error::TownsNotFound => write!(f, "could not find the town records"),
            Error::PlayersNotFound => write!(f, "could not find the player records"),
            Error::InvalidDate(bytes) => write!(f, "invalid in-game date bytes {bytes:02x?}"),
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
/// [`Error::NotASaveFile`] if the data is not gzip or does not start with a save header, and
/// [`Error::TooLarge`] if it decompresses to more than [`MAX_DECOMPRESSED_SIZE`] bytes.
pub fn decompress(compressed: &[u8]) -> Result<Vec<u8>, Error> {
    let mut out = Vec::with_capacity(compressed.len().saturating_mul(16).min(MAX_DECOMPRESSED_SIZE));
    // Read one byte past the limit to tell a save of exactly the limit from a larger one.
    let limit = u64::try_from(MAX_DECOMPRESSED_SIZE).map_or(u64::MAX, |n| n.saturating_add(1));
    // On error, `read_to_end` keeps the bytes read so far. Only the trailer check can fail
    // after data has come out, so an error with data is the expected end of a save.
    if GzDecoder::new(compressed).take(limit).read_to_end(&mut out).is_err() && out.is_empty() {
        return Err(Error::NotASaveFile);
    }
    if out.len() > MAX_DECOMPRESSED_SIZE {
        return Err(Error::TooLarge);
    }
    if out.len() < HEADER_NORMAL.len() || !is_save_header(&out) {
        return Err(Error::NotASaveFile);
    }
    Ok(out)
}

fn is_save_header(data: &[u8]) -> bool {
    data.starts_with(HEADER_NORMAL) || data.starts_with(HEADER_CAMPAIGN)
}

/// Where the player, town and hero blocks start in the decompressed data.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Layout {
    /// First of the eight player records.
    pub players: usize,
    /// The town count byte. Town 0's index byte follows it.
    pub town_count: usize,
    /// First hero record.
    pub heroes: usize,
}

impl Layout {
    /// Find the player, town and hero blocks in decompressed save data of the given format.
    ///
    /// # Errors
    ///
    /// [`Error::HeroesNotFound`], [`Error::AmbiguousHeroes`], [`Error::TownsNotFound`] or
    /// [`Error::PlayersNotFound`] when a block cannot be found.
    pub fn find(data: &[u8], hero_count: usize, format: Format) -> Result<Layout, Error> {
        let heroes = find_heroes(data, hero_count, format)?;
        let town_count = find_town_count(data, heroes).ok_or(Error::TownsNotFound)?;
        let towns = data.get(town_count).map_or(0, |&n| usize::from(n));
        let players = town_count
            .checked_sub(MAX_PLAYERS * PLAYER_RECORD_SIZE)
            .ok_or(Error::PlayersNotFound)?;
        player::read_all(data, players, hero_count, towns).ok_or(Error::PlayersNotFound)?;
        Ok(Layout {
            players,
            town_count,
            heroes,
        })
    }
}

/// Start of the hero array: the one position where `hero_count` consecutive records of the
/// given format all look like hero records.
///
/// # Errors
///
/// [`Error::HeroesNotFound`] if there is no such position, [`Error::AmbiguousHeroes`] if there
/// is more than one.
pub fn find_heroes(data: &[u8], hero_count: usize, format: Format) -> Result<usize, Error> {
    let record_size = format.hero_record_size;
    let block_size = hero_count.checked_mul(record_size).ok_or(Error::HeroesNotFound)?;
    let last_start = data.len().checked_sub(block_size).ok_or(Error::HeroesNotFound)?;
    let mut found = None;
    for pos in 0..=last_start {
        // Checking the first record rejects almost every position at once.
        if !hero::is_plausible_record(data, pos, format) {
            continue;
        }
        let all_plausible = (1..hero_count).all(|i| {
            record_pos(pos, i, record_size).is_ok_and(|record| hero::is_plausible_record(data, record, format))
        });
        if all_plausible {
            match found {
                None => found = Some(pos),
                Some(first) => return Err(Error::AmbiguousHeroes { first, second: pos }),
            }
        }
    }
    found.ok_or(Error::HeroesNotFound)
}

/// Position of the town count byte: the town block must read through to exactly `heroes`.
///
/// Looking backwards from the heroes, a count byte of 0 right before them always parses, so
/// among the positions that work, the one with the most towns is taken. A block with towns has
/// to number them 0, 1, 2, … in order, which only the real block does.
#[must_use]
pub fn find_town_count(data: &[u8], heroes: usize) -> Option<usize> {
    let max_block = 1 + MAX_TOWNS * town::MAX_RECORD_SIZE;
    let lowest = heroes.saturating_sub(max_block);
    let mut best: Option<(usize, usize)> = None;
    for pos in (lowest..heroes).rev() {
        let Some(&count) = data.get(pos) else { continue };
        let count = usize::from(count);
        if count > MAX_TOWNS || best.is_some_and(|(_, n)| n >= count) {
            continue;
        }
        if let Some((towns, end)) = town::read_block(data, pos) {
            if end == heroes {
                best = Some((pos, towns.len()));
            }
        }
    }
    best.map(|(pos, _)| pos)
}

/// The date stored after the hero array that ends at `heroes_end`.
fn read_date(data: &[u8], heroes_end: usize, format: Format) -> Result<Date, Error> {
    let pos = heroes_end.checked_add(format.date_offset).ok_or(Error::Truncated {
        pos: heroes_end,
        len: format.date_offset,
    })?;
    let bytes = Reader::at(data, pos).array()?;
    Date::from_bytes(bytes).ok_or(Error::InvalidDate(bytes))
}

/// A parsed save file.
#[derive(Clone, Debug)]
pub struct SaveFile {
    pub header: [u8; 8],
    /// From the HotA version at byte 16.
    pub format: Format,
    /// Byte 80. Number of hero records: 198 in HotA 1.7, 215 in HotA 1.8.
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
    /// The in-game date when the game was saved.
    pub date: Date,

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
    pub fn read(path: impl AsRef<Path>) -> Result<SaveFile, Error> {
        SaveFile::parse(&decompress_file(path)?)
    }

    /// Parse decompressed save data.
    ///
    /// # Errors
    ///
    /// [`Error::Truncated`] if a record runs past the end of the data, [`Error::InvalidDate`]
    /// if the date is out of range, and the errors of [`Format::of`] and [`Layout::find`].
    pub fn parse(data: &[u8]) -> Result<SaveFile, Error> {
        let mut r = Reader::at(data, 0);
        let header = r.array()?;
        let format = Format::of(data)?;
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

        let hero_count_usize = usize::try_from(hero_count).map_err(|_| Error::HeroesNotFound)?;
        let layout = Layout::find(data, hero_count_usize, format)?;
        let towns_in_block = data.get(layout.town_count).map_or(0, |&n| usize::from(n));
        let players =
            player::read_all(data, layout.players, hero_count_usize, towns_in_block).ok_or(Error::PlayersNotFound)?;
        let (towns, _) = town::read_block(data, layout.town_count).ok_or(Error::TownsNotFound)?;
        let heroes = hero::read_all(data, layout.heroes, hero_count_usize, format)?;

        Ok(SaveFile {
            header,
            format,
            hero_count,
            map_size,
            has_underground,
            map_name_size,
            map_name,
            description_size,
            description,
            date: read_date(
                data,
                record_pos(layout.heroes, hero_count_usize, format.hero_record_size)?,
                format,
            )?,
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
        assert!(matches!(SaveFile::parse(b"H3SVG\0"), Err(Error::Truncated { .. })));
    }

    /// Save data of `len` zero bytes with a save header for HotA `major`.`minor`.
    fn empty_save(len: usize, major: u8, minor: u8) -> Vec<u8> {
        let mut data = vec![0u8; len];
        data[..6].copy_from_slice(HEADER_NORMAL);
        data[16] = major;
        data[20] = minor;
        data
    }

    #[test]
    fn data_without_heroes_is_an_error() {
        let mut data = empty_save(4096, 1, 8);
        data[80] = 215;
        assert!(matches!(SaveFile::parse(&data), Err(Error::HeroesNotFound)));
    }

    #[test]
    fn the_format_is_chosen_by_the_version_in_the_header() {
        assert_eq!(Format::of(&empty_save(100, 1, 7)).unwrap(), Format::HOTA_1_7);
        assert_eq!(Format::of(&empty_save(100, 1, 8)).unwrap(), Format::HOTA_1_8);
    }

    #[test]
    fn an_unknown_version_is_an_error() {
        for (major, minor) in [(1, 6), (1, 9), (2, 8), (0, 0)] {
            let mut data = empty_save(4096, major, minor);
            data[80] = 215;
            let result = SaveFile::parse(&data);
            assert!(
                matches!(result, Err(Error::UnsupportedVersion(v)) if v == (u32::from(major), u32::from(minor))),
                "{major}.{minor}: {result:?}"
            );
        }
        assert_eq!(
            Error::UnsupportedVersion((1, 9)).to_string(),
            "saves from HotA 1.9 are not supported"
        );
    }

    #[test]
    fn data_ending_inside_the_version_is_truncated() {
        let data = empty_save(100, 1, 8);
        for len in [6, 16, 20, 23] {
            assert!(
                matches!(Format::of(&data[..len]), Err(Error::Truncated { .. })),
                "length {len}"
            );
        }
    }

    #[test]
    fn formats_differ_by_one_skill_slot_per_hero() {
        let [v17, v18] = Format::ALL;
        assert_eq!(v18.skill_count, v17.skill_count + 1);
        assert_eq!(v18.hero_record_size, v17.hero_record_size + 1);
        assert_eq!(v18.skill_count, crate::skills::SKILL_COUNT);
        // 17 heroes came with Bulwark, two bytes each between the heroes and the date.
        assert_eq!(v18.date_offset, v17.date_offset + 2 * 17);
    }
}

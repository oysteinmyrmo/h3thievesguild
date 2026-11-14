// Tests for the in-game date, beyond the date checks in each test file.

use std::path::{Path, PathBuf};

use h3save::hero::HERO_RECORD_SIZE;
use h3save::savefile::{Error, Layout, decompress_file};
use h3save::{Date, SaveFile, date};

/// Every save file under `test_data`, sorted.
fn all_saves() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e.to_string_lossy().starts_with("GM")) {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test_data"), &mut out);
    out.sort();
    out
}

/// The date an autosave's name gives: `[hotseat] MWD.GMn`, or `[hotseat] MWD.xx.GMn`.
/// The `start` saves and the Bulwark save were saved on the first day.
fn date_from_name(path: &Path) -> Date {
    let name = path.file_name().unwrap().to_str().unwrap();
    if name.starts_with("start.") || name == "test-bulwark.GM2" {
        return Date::FIRST;
    }
    let digits: Vec<u16> = name
        .strip_prefix("[hotseat] ")
        .unwrap_or_else(|| panic!("unexpected save name {name}"))
        .chars()
        .take(3)
        .map(|c| u16::try_from(c.to_digit(10).unwrap()).unwrap())
        .collect();
    Date::new(digits[0], digits[1], digits[2]).unwrap()
}

/// Decompressed save data and the position of its date.
fn data_and_date_pos(path: &Path) -> (Vec<u8>, usize) {
    let data = decompress_file(path).unwrap();
    let hero_count = usize::try_from(u32::from_le_bytes(data[80..84].try_into().unwrap())).unwrap();
    let layout = Layout::find(&data, hero_count).unwrap();
    (
        data,
        layout.heroes + hero_count * HERO_RECORD_SIZE + date::OFFSET_AFTER_HEROES,
    )
}

fn sample() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test_data/experience/[hotseat] 121.GM2")
}

#[test]
fn every_test_save_has_the_date_its_autosave_name_gives() {
    let saves = all_saves();
    assert_eq!(saves.len(), 69);
    for path in saves {
        let save = SaveFile::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        assert_eq!(save.date, date_from_name(&path), "{}", path.display());
    }
}

#[test]
fn consecutive_autosaves_are_one_day_apart() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test_data/experience");
    let names = ["111", "112", "113", "114", "115", "116", "117", "121"];
    let days: Vec<u32> = names
        .iter()
        .map(|n| {
            SaveFile::read(dir.join(format!("[hotseat] {n}.GM2")))
                .unwrap()
                .date
                .days_elapsed()
        })
        .collect();
    assert_eq!(days, (0..8).collect::<Vec<_>>());
}

#[test]
fn the_date_is_read_from_its_bytes_not_the_file_name() {
    let (mut data, pos) = data_and_date_pos(&sample());
    assert_eq!(&data[pos..pos + date::SIZE], &[1, 0, 2, 0, 1, 0]);
    for (month, week, day) in [
        (1, 1, 1),
        (1, 4, 7),
        (2, 3, 5),
        (12, 1, 1),
        (300, 2, 6),
        (u16::MAX, 4, 7),
    ] {
        let [m0, m1] = u16::to_le_bytes(month);
        data[pos..pos + date::SIZE].copy_from_slice(&[day, 0, week, 0, m0, m1]);
        let save = SaveFile::parse(&data).unwrap();
        assert_eq!(save.date, Date::new(month, u16::from(week), u16::from(day)).unwrap());
    }
}

#[test]
fn an_out_of_range_date_is_an_error() {
    let (mut data, pos) = data_and_date_pos(&sample());
    let invalid: [[u8; 6]; 7] = [
        [0, 0, 1, 0, 1, 0], // Day 0.
        [8, 0, 1, 0, 1, 0], // Day 8.
        [1, 1, 1, 0, 1, 0], // Day 257.
        [1, 0, 0, 0, 1, 0], // Week 0.
        [1, 0, 5, 0, 1, 0], // Week 5.
        [1, 0, 1, 0, 0, 0], // Month 0.
        [0xFF; 6],
    ];
    for bytes in invalid {
        data[pos..pos + date::SIZE].copy_from_slice(&bytes);
        match SaveFile::parse(&data) {
            Err(Error::InvalidDate(found)) => assert_eq!(found, bytes),
            other => panic!("{bytes:?}: expected InvalidDate, got {other:?}"),
        }
    }
}

#[test]
fn data_ending_inside_the_date_is_truncated() {
    let (data, pos) = data_and_date_pos(&sample());
    for end in [pos, pos + 1, pos + date::SIZE - 1] {
        match SaveFile::parse(&data[..end]) {
            Err(Error::Truncated { pos: at, len }) => assert_eq!((at, len), (pos, date::SIZE)),
            other => panic!("ending at {end}: expected Truncated, got {other:?}"),
        }
    }
    assert!(SaveFile::parse(&data[..pos + date::SIZE]).is_ok());
}

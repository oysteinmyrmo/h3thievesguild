//! Helpers shared by the integration tests.

// Each test file uses only some of the helpers.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use h3save::creatures::MAX_STACKS;
use h3save::savefile::{Format, Layout, decompress_file};
use h3save::{Creature, Hero, SaveFile, Town};

/// A folder, or a file, under `test_data`.
pub fn data_dir(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_data")
        .join(path)
}

/// Every save file under `test_data`, sorted.
pub fn all_saves() -> Vec<PathBuf> {
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
    walk(&data_dir(""), &mut out);
    out.sort();
    out
}

/// Read a save, failing the test with the path and error if it cannot be read.
pub fn open(path: &Path) -> SaveFile {
    SaveFile::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// A save's decompressed data and where its blocks are.
pub struct Decompressed {
    pub data: Vec<u8>,
    pub format: Format,
    pub layout: Layout,
    pub hero_count: usize,
}

impl Decompressed {
    pub fn read(path: &Path) -> Decompressed {
        let data = decompress_file(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let hero_count = usize::try_from(u32::from_le_bytes(data[80..84].try_into().unwrap())).unwrap();
        let format = Format::of(&data).unwrap();
        let layout = Layout::find(&data, hero_count, format).unwrap();
        Decompressed {
            data,
            format,
            layout,
            hero_count,
        }
    }

    /// One past the hero array.
    pub fn heroes_end(&self) -> usize {
        self.layout.heroes + self.hero_count * self.format.hero_record_size
    }
}

/// The hero with this name, failing the test if there is none.
pub fn find_hero<'a>(save: &'a SaveFile, name: &str) -> &'a Hero {
    save.find_hero(name).unwrap_or_else(|| panic!("hero {name} not found"))
}

/// The town with this name, failing the test if there is none.
pub fn find_town<'a>(save: &'a SaveFile, name: &str) -> &'a Town {
    save.find_town(name).unwrap_or_else(|| panic!("town {name} not found"))
}

/// Anything with an army: heroes and town garrisons.
pub trait Army {
    fn creatures(&self) -> &[Creature; MAX_STACKS];
    fn creature_count(&self) -> &[u32; MAX_STACKS];
}

impl Army for Hero {
    fn creatures(&self) -> &[Creature; MAX_STACKS] {
        &self.creatures
    }
    fn creature_count(&self) -> &[u32; MAX_STACKS] {
        &self.creature_count
    }
}

impl Army for Town {
    fn creatures(&self) -> &[Creature; MAX_STACKS] {
        &self.creatures
    }
    fn creature_count(&self) -> &[u32; MAX_STACKS] {
        &self.creature_count
    }
}

/// Check one stack of an army.
#[track_caller]
pub fn assert_creature(item: &impl Army, idx: usize, creature: Creature, count: u32) {
    assert_eq!(item.creatures()[idx], creature, "creature in slot {idx}");
    assert_eq!(item.creature_count()[idx], count, "count in slot {idx}");
}

/// Check the in-game date of a save.
#[track_caller]
pub fn assert_date(save: &SaveFile, month: u16, week: u16, day: u16) {
    assert_eq!(
        (save.date.month(), save.date.week(), save.date.day()),
        (month, week, day),
        "date"
    );
}

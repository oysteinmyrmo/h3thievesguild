//! Shared helpers for the integration tests.
//!
//! The tests are a one-to-one port of the homm3tools C++ tests. `require!` and
//! `require_eq!` stand in for Catch2's `REQUIRE` and count every check, so the
//! number of checks run can be compared with the C++ suite (16,125).

#![allow(dead_code)]

use std::cell::Cell;
use std::path::{Path, PathBuf};

use h3save::creatures::MAX_STACKS;
use h3save::{Creature, Hero, Input, SaveFile, Town};

thread_local! {
    static CHECKS: Cell<usize> = const { Cell::new(0) };
}

/// Count one check. Used by the `require!` macros.
pub fn count_check() {
    CHECKS.with(|c| c.set(c.get() + 1));
}

/// Prints the number of checks a test ran when it ends, for comparison with the C++ suite.
/// Run with `cargo test -- --nocapture --test-threads=1` to see the counts.
pub struct Guard {
    name: &'static str,
}

impl Guard {
    pub fn new(name: &'static str) -> Self {
        CHECKS.with(|c| c.set(0));
        Guard { name }
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let n = CHECKS.with(Cell::get);
            println!("checks {} {n}", self.name);
        }
    }
}

/// Catch2's `REQUIRE(expr)`.
#[macro_export]
macro_rules! require {
    ($cond:expr) => {{
        $crate::common::count_check();
        assert!($cond, "REQUIRE({}) failed", stringify!($cond));
    }};
}

/// Catch2's `REQUIRE(left == right)`, printing both values on failure.
#[macro_export]
macro_rules! require_eq {
    ($left:expr, $right:expr) => {{
        $crate::common::count_check();
        assert_eq!(
            $left,
            $right,
            "REQUIRE({} == {}) failed",
            stringify!($left),
            stringify!($right)
        );
    }};
}

/// The test data folder for one group of tests.
pub fn data_dir(group: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_data")
        .join(group)
}

/// Read a save whose towns all have default names.
pub fn open(path: &Path) -> SaveFile {
    SaveFile::read(&Input::new(path)).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Read a save with renamed towns.
pub fn open_with_towns(path: &Path, first_town_name: &str, number_of_towns: usize) -> SaveFile {
    SaveFile::read(&Input::with_towns(path, first_town_name, number_of_towns))
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// `SaveFile::findHero`. The C++ version returns a default hero when not found; every test
/// looks up a hero that exists, so here a missing hero fails the test.
pub fn find_hero<'a>(save: &'a SaveFile, name: &str) -> &'a Hero {
    save.find_hero(name).unwrap_or_else(|| panic!("hero {name} not found"))
}

/// `SaveFile::findTown`, failing the test when the town is missing.
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

/// `test_creature` from the C++ tests: check one stack of an army.
pub fn test_creature(item: &impl Army, idx: usize, creature: Creature, count: u32) {
    require_eq!(item.creatures()[idx], creature);
    require_eq!(item.creature_count()[idx], count);
}

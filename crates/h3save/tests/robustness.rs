// The parser must never panic, whatever bytes it is given: save files come from users, and in
// the browser a panic would take down the page. Every test here feeds broken input and checks
// that the result is an error or a save that still holds together, never a panic.
//
// Random cases use a fixed-seed generator, so every run tests the same inputs.

mod common;

use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};

use common::{Decompressed, data_dir};
use flate2::Compression;
use flate2::write::GzEncoder;
use h3save::hero::HERO_RECORD_SIZE;
use h3save::savefile::{Error, HEADER_NORMAL, Layout, MAX_DECOMPRESSED_SIZE, decompress};
use h3save::town::MAX_TOWNS;
use h3save::{Date, SaveFile, date};

/// xorshift64*: a small, fixed-seed generator, so failures reproduce.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A number in `0..n`.
    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(n).unwrap()).unwrap()
    }

    fn byte(&mut self) -> u8 {
        self.next().to_le_bytes()[0]
    }
}

/// Saves covering 2 and 8 players, small and gigantic maps, with and without underground.
const SAMPLES: [&str; 4] = [
    "towns/[hotseat] 113.GM2",
    "savefile/S_U/[hotseat] 111.GM2",
    "resources/[hotseat] 111.GM8",
    "movement_gigantic/[hotseat] 111.GM2",
];

/// A decompressed sample save and where its blocks are.
struct Sample {
    name: &'static str,
    data: Vec<u8>,
    layout: Layout,
    hero_count: usize,
    /// One past the last byte the parser reads: the end of the date.
    end: usize,
    towns: Vec<usize>,
}

impl Sample {
    fn load(name: &'static str) -> Sample {
        let decompressed = Decompressed::read(&data_dir(name));
        let heroes_end = decompressed.heroes_end();
        let Decompressed {
            data,
            layout,
            hero_count,
        } = decompressed;
        let save = SaveFile::parse(&data).unwrap();
        let mut towns = Vec::new();
        let mut pos = layout.town_count + 1;
        for town in &save.towns {
            towns.push(pos);
            pos += town.record_size();
        }
        assert_eq!(pos, layout.heroes);
        Sample {
            name,
            data,
            layout,
            hero_count,
            end: heroes_end + date::OFFSET_AFTER_HEROES + date::SIZE,
            towns,
        }
    }

    /// Positions where a block or record starts or ends, where off-by-one errors would show.
    fn boundaries(&self) -> Vec<usize> {
        let heroes = self.layout.heroes;
        let mut out = vec![
            0,
            6,
            80,
            84,
            90,
            95,
            97,
            self.layout.players,
            self.layout.town_count,
            heroes,
            heroes + HERO_RECORD_SIZE,
            heroes + (self.hero_count - 1) * HERO_RECORD_SIZE,
            heroes + self.hero_count * HERO_RECORD_SIZE,
            self.end - date::SIZE,
            self.end,
        ];
        out.extend(&self.towns);
        out
    }
}

/// Parse, turning a panic into a test failure that names the input.
fn parse_without_panic(data: &[u8], what: &str) -> Result<SaveFile, Error> {
    catch_unwind(AssertUnwindSafe(|| SaveFile::parse(data))).unwrap_or_else(|_| panic!("parser panicked on {what}"))
}

/// What a successfully parsed save must satisfy, however damaged its input was.
fn check_consistent(save: &SaveFile, what: &str) {
    assert_eq!(save.heroes.len(), usize::try_from(save.hero_count).unwrap(), "{what}");
    assert!(save.towns.len() <= MAX_TOWNS, "{what}");
    let date = save.date;
    assert_eq!(Date::new(date.month(), date.week(), date.day()), Some(date), "{what}");
    for town in &save.towns {
        assert!(town.owner < 8 || town.owner == 0xFF, "{what}");
        assert!(town.z <= 1, "{what}");
        assert!((1..=32).contains(&town.name.len()), "{what}");
    }
    // Everything derived from a save must work too.
    for data in &save.players {
        assert!(data.is_plausible(save.heroes.len(), save.towns.len()), "{what}");
        let _ = (data.hero_ids_on_map(), data.town_indices());
    }
    for player in h3save::player::players(save) {
        let _ = (
            player.kingdom_army_strength(),
            player.best_creature_ai_value(),
            player.color().rgb(),
        );
    }
    for hero in &save.heroes {
        let _ = (hero.name(), hero.skills(), hero.army_strength(), hero.best_creature());
        let _ = hero.artifacts_with_details().count();
        for slot in 0..=u8::MAX {
            let _ = hero.spell_from_scroll(h3save::ArtifactSlot(slot));
            let _ = hero.artifact(h3save::ArtifactSlot(slot));
        }
    }
}

#[test]
fn every_prefix_ending_at_or_next_to_a_boundary_is_an_error() {
    for name in SAMPLES {
        let sample = Sample::load(name);
        for boundary in sample.boundaries() {
            for len in boundary.saturating_sub(3)..=boundary + 3 {
                if len >= sample.end {
                    continue;
                }
                let what = format!("{name} cut to {len} bytes");
                assert!(parse_without_panic(&sample.data[..len], &what).is_err(), "{what}");
            }
        }
        assert!(
            SaveFile::parse(&sample.data[..sample.end]).is_ok(),
            "{name} ending right after the date"
        );
    }
}

#[test]
fn every_short_prefix_and_random_prefixes_are_errors() {
    let sample = Sample::load(SAMPLES[0]);
    let mut lengths: Vec<usize> = (0..=256).collect();
    let mut rng = Rng(0x5EED_0001);
    lengths.extend((0..200).map(|_| rng.below(sample.end)));
    for len in lengths {
        let what = format!("{} cut to {len} bytes", sample.name);
        assert!(parse_without_panic(&sample.data[..len], &what).is_err(), "{what}");
    }
}

#[test]
fn random_byte_changes_never_panic() {
    let mut rng = Rng(0x5EED_0002);
    for name in SAMPLES {
        let sample = Sample::load(name);
        let boundaries = sample.boundaries();
        let mut parsed = 0;
        for case in 0..150 {
            let mut data = sample.data.clone();
            // Change 1 to 16 bytes, mostly near block boundaries where the parser decides
            // where things are, sometimes anywhere in the data the parser reads.
            for _ in 0..=rng.below(16) {
                let pos = if rng.below(4) == 0 {
                    rng.below(sample.end)
                } else {
                    let at = boundaries[rng.below(boundaries.len())];
                    (at + rng.below(64)).saturating_sub(32).min(sample.end - 1)
                };
                data[pos] = rng.byte();
            }
            let what = format!("{name}, random change {case}");
            if let Ok(save) = parse_without_panic(&data, &what) {
                check_consistent(&save, &what);
                parsed += 1;
            }
        }
        // Most changes hit bytes that are not checked, so most saves still parse.
        assert!(parsed > 0, "{name}: no changed save parsed");
    }
}

#[test]
fn extreme_values_in_header_fields_never_panic() {
    let sample = Sample::load(SAMPLES[0]);
    // Hero count (u32 at 80), map name size and description size (u16 at 95 and after the name).
    let description_size = 97 + usize::from(u16::from_le_bytes([sample.data[95], sample.data[96]]));
    let mut fields: Vec<(usize, Vec<u8>)> = Vec::new();
    for value in [0u32, 1, 214, 216, 0xFFFF, 0x0100_0000, u32::MAX] {
        fields.push((80, value.to_le_bytes().to_vec()));
    }
    for at in [95, description_size] {
        for value in [0u16, 1, 0x7FFF, u16::MAX] {
            fields.push((at, value.to_le_bytes().to_vec()));
        }
    }
    for (at, bytes) in fields {
        let mut data = sample.data.clone();
        data[at..at + bytes.len()].copy_from_slice(&bytes);
        let what = format!("bytes {bytes:02x?} at {at}");
        if let Ok(save) = parse_without_panic(&data, &what) {
            check_consistent(&save, &what);
        }
    }
}

#[test]
fn hero_count_must_match_the_hero_array() {
    let sample = Sample::load(SAMPLES[0]);
    for value in [0u32, 1, 214, 216, u32::MAX] {
        let mut data = sample.data.clone();
        data[80..84].copy_from_slice(&value.to_le_bytes());
        let what = format!("hero count {value}");
        let result = parse_without_panic(&data, &what);
        assert!(
            matches!(
                result,
                Err(Error::HeroesNotFound | Error::AmbiguousHeroes { .. } | Error::TownsNotFound)
            ),
            "{what}: {result:?}"
        );
    }
}

#[test]
fn damaged_town_and_hero_records_are_errors() {
    for name in SAMPLES {
        let sample = Sample::load(name);
        // Each change breaks a check the layout search relies on, so the save cannot be read.
        let first_hero = sample.layout.heroes;
        let last_hero = first_hero + (sample.hero_count - 1) * HERO_RECORD_SIZE;
        let mut changes = vec![
            (first_hero + 26, 9),   // First hero's owner.
            (last_hero + 26, 9),    // Last hero's owner.
            (first_hero + 41, 8),   // Orientation.
            (last_hero + 208, 4),   // Skill level.
            (first_hero + 1118, 9), // Skill slot.
            (last_hero + 195, 1),   // Control character in the name.
        ];
        if let Some(&town) = sample.towns.first() {
            changes.push((town, 1)); // Town 0 numbered 1.
        }
        for (pos, value) in changes {
            let mut data = sample.data.clone();
            data[pos] = value;
            let what = format!("{name}: byte {pos} set to {value}");
            let result = parse_without_panic(&data, &what);
            assert!(result.is_err(), "{what}: parsed");
        }
    }
}

#[test]
fn random_data_with_a_save_header_is_an_error() {
    let mut rng = Rng(0x5EED_0003);
    for case in 0..40 {
        let mut data: Vec<u8> = (0..rng.below(300_000)).map(|_| rng.byte()).collect();
        if data.len() >= 84 {
            data[..6].copy_from_slice(HEADER_NORMAL);
            let hero_count = if case % 2 == 0 { 215 } else { rng.below(300) };
            data[80..84].copy_from_slice(&u32::try_from(hero_count).unwrap().to_le_bytes());
        }
        let what = format!("random data {case} of {} bytes", data.len());
        assert!(parse_without_panic(&data, &what).is_err(), "{what}");
    }
}

fn gzip(bytes: &[u8]) -> Vec<u8> {
    let mut e = GzEncoder::new(Vec::new(), Compression::fast());
    e.write_all(bytes).unwrap();
    e.finish().unwrap()
}

#[test]
fn truncated_and_damaged_gzip_never_panics() {
    let compressed = std::fs::read(data_dir(SAMPLES[0])).unwrap();
    let mut rng = Rng(0x5EED_0004);
    let mut inputs: Vec<(String, Vec<u8>)> = Vec::new();
    for len in (0..compressed.len())
        .step_by(997)
        .chain([1, 2, 9, 10, 11, compressed.len() - 1])
    {
        inputs.push((format!("gzip cut to {len} bytes"), compressed[..len].to_vec()));
    }
    for case in 0..60 {
        let mut damaged = compressed.clone();
        for _ in 0..=rng.below(8) {
            let pos = rng.below(damaged.len());
            damaged[pos] = rng.byte();
        }
        inputs.push((format!("damaged gzip {case}"), damaged));
    }
    for (what, input) in inputs {
        let result = catch_unwind(|| decompress(&input).map(|data| SaveFile::parse(&data)))
            .unwrap_or_else(|_| panic!("panicked on {what}"));
        if let Ok(Ok(save)) = result {
            check_consistent(&save, &what);
        }
    }
}

#[test]
fn decompressed_size_is_limited() {
    let mut data = vec![0u8; MAX_DECOMPRESSED_SIZE];
    data[..6].copy_from_slice(HEADER_NORMAL);
    assert_eq!(decompress(&gzip(&data)).unwrap().len(), MAX_DECOMPRESSED_SIZE);
    data.push(0);
    assert!(matches!(decompress(&gzip(&data)), Err(Error::TooLarge)));
}

#[test]
fn a_learned_skill_without_a_slot_is_left_out() {
    let sample = Sample::load(SAMPLES[0]);
    let hero = sample.layout.heroes;
    let level_of = |id: usize| hero + 208 + id;
    let slot_of = |id: usize| hero + 1118 + id;
    let unlearned = (0..h3save::skills::SKILL_COUNT)
        .find(|&id| sample.data[level_of(id)] == 0 && sample.data[slot_of(id)] == 0)
        .unwrap();
    let mut data = sample.data.clone();
    data[level_of(unlearned)] = 3;
    let save = parse_without_panic(&data, "a learned skill in slot 0").unwrap();
    let before = SaveFile::parse(&sample.data).unwrap().heroes[0].skills();
    assert_eq!(save.heroes[0].skills(), before);
    check_consistent(&save, "a learned skill in slot 0");
}

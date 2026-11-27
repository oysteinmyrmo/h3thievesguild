//! Which map tiles each player has seen: the fog of war.
//!
//! A tile stays seen once a player's hero or town has had it in view, so this is how much of
//! the map each player has explored, not what they see right now.
//!
//! The save holds two bytes per tile, [`OFFSET_AFTER_DATE`] bytes after the start of the date,
//! level by level (surface first), row by row, so the tile at `(x, y, z)` is number
//! `(z * size + y) * size + x`. In the first byte, bit `p` is set when player `p` (0 red, 1 blue,
//! …) has seen the tile. The second byte is 0 or 1 in every test save; what it means is not
//! known, and it is not read.

use crate::player::Color;
use crate::reader::{ReadResult, Reader};

/// Bytes from the start of the date to the first tile. The same in HotA 1.7 and 1.8 saves.
pub const OFFSET_AFTER_DATE: usize = 119;

/// Bytes per tile.
pub const TILE_SIZE: usize = 2;

/// The tiles each player has seen, on every level of the map.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Visibility {
    size: usize,
    levels: usize,
    /// One byte per tile, a bit per player.
    seen: Vec<u8>,
}

impl Visibility {
    /// Read the tiles of a map `size` tiles across with `levels` levels, starting at `pos`.
    pub(crate) fn read(data: &[u8], pos: usize, size: usize, levels: usize) -> ReadResult<Visibility> {
        let tiles = size.saturating_mul(size).saturating_mul(levels);
        let bytes = Reader::at(data, pos).bytes(tiles.saturating_mul(TILE_SIZE))?;
        let seen = bytes
            .chunks_exact(TILE_SIZE)
            .filter_map(|tile| tile.first().copied())
            .collect();
        Ok(Visibility { size, levels, seen })
    }

    /// Tiles across the map, and down it.
    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }

    /// 2 with an underground level, otherwise 1.
    #[must_use]
    pub fn levels(&self) -> usize {
        self.levels
    }

    /// Tiles on the whole map, all levels.
    #[must_use]
    pub fn tile_count(&self) -> usize {
        self.seen.len()
    }

    /// Whether the player has seen the tile. False for a tile off the map.
    #[must_use]
    pub fn is_seen(&self, color: Color, x: usize, y: usize, z: usize) -> bool {
        if x >= self.size || y >= self.size || z >= self.levels {
            return false;
        }
        let index = z
            .checked_mul(self.size)
            .and_then(|i| i.checked_add(y))
            .and_then(|i| i.checked_mul(self.size))
            .and_then(|i| i.checked_add(x));
        index
            .and_then(|i| self.seen.get(i))
            .is_some_and(|&tile| seen_by(tile, color))
    }

    /// Tiles the player has seen, all levels.
    #[must_use]
    pub fn tiles_seen(&self, color: Color) -> usize {
        self.seen.iter().filter(|&&tile| seen_by(tile, color)).count()
    }

    /// Tiles the player has seen on one level: 0 for the surface, 1 for the underground.
    #[must_use]
    pub fn tiles_seen_on_level(&self, color: Color, z: usize) -> usize {
        let per_level = self.size.saturating_mul(self.size);
        self.seen
            .chunks(per_level.max(1))
            .nth(z)
            .map_or(0, |level| level.iter().filter(|&&tile| seen_by(tile, color)).count())
    }

    /// The share of the whole map the player has seen, from 0 to 1. 0 for a map without tiles.
    #[must_use]
    #[expect(clippy::cast_precision_loss, reason = "tile counts are far below 2^52")]
    pub fn share_seen(&self, color: Color) -> f64 {
        if self.seen.is_empty() {
            return 0.0;
        }
        self.tiles_seen(color) as f64 / self.tile_count() as f64
    }
}

fn seen_by(tile: u8, color: Color) -> bool {
    tile.checked_shr(u32::try_from(color.index()).unwrap_or(u32::MAX))
        .is_some_and(|bits| bits & 1 == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_bit_of_a_tile_is_one_player() {
        let v = Visibility {
            size: 2,
            levels: 1,
            seen: vec![0b0000_0001, 0b1000_0010, 0, 0xFF],
        };
        let counts: Vec<usize> = Color::ALL.iter().map(|&c| v.tiles_seen(c)).collect();
        assert_eq!(counts, [2, 2, 1, 1, 1, 1, 1, 2]);
        assert!(v.is_seen(Color::Pink, 1, 0, 0));
        assert!(!v.is_seen(Color::Pink, 0, 1, 0));
        assert!((v.share_seen(Color::Red) - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn reads_the_first_byte_of_each_tile_and_stops_at_the_end_of_the_data() {
        let data = [9, 9, 1, 7, 2, 0, 4, 1, 8, 0];
        let v = Visibility::read(&data, 2, 2, 1).unwrap();
        assert_eq!(v.seen, [1, 2, 4, 8]);
        assert!(Visibility::read(&data, 3, 2, 1).is_err());
        let empty = Visibility::read(&data, 10, 0, 1).unwrap();
        assert_eq!((empty.tile_count(), empty.share_seen(Color::Red)), (0, 0.0));
        assert_eq!(empty.tiles_seen_on_level(Color::Red, 0), 0);
    }
}

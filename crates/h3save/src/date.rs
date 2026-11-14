//! The in-game date: month, week and day, all counted from 1.
//!
//! The date is stored as three little-endian `u16`s (day, week, month), 441 bytes after the
//! end of the hero array. That distance is the same in every test save, across map sizes,
//! player counts and with or without an underground level.

use std::fmt;

/// Days in a week.
pub const DAYS_PER_WEEK: u16 = 7;
/// Weeks in a month.
pub const WEEKS_PER_MONTH: u16 = 4;
/// Days in a month.
pub const DAYS_PER_MONTH: u16 = DAYS_PER_WEEK * WEEKS_PER_MONTH;
/// Bytes from the end of the hero array to the date.
pub const OFFSET_AFTER_HEROES: usize = 441;
/// Size of the stored date: day, week and month as `u16`.
pub const SIZE: usize = 6;

/// An in-game date. Orders chronologically.
///
/// ```
/// use h3save::Date;
///
/// let date = Date::new(1, 2, 3).unwrap();
/// assert_eq!(date.to_string(), "Month 1, Week 2, Day 3");
/// assert_eq!(date.days_elapsed(), 9);
/// assert_eq!(Date::from_days_elapsed(9), Some(date));
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    // Field order gives the chronological order derived above.
    month: u16,
    week: u16,
    day: u16,
}

impl Date {
    /// The first day of the game.
    pub const FIRST: Date = Date {
        month: 1,
        week: 1,
        day: 1,
    };

    /// A date, or `None` unless `month >= 1`, `week` is 1–4 and `day` is 1–7.
    #[must_use]
    pub const fn new(month: u16, week: u16, day: u16) -> Option<Date> {
        if month >= 1 && week >= 1 && week <= WEEKS_PER_MONTH && day >= 1 && day <= DAYS_PER_WEEK {
            Some(Date { month, week, day })
        } else {
            None
        }
    }

    /// The date `days` days after the first day, or `None` if the month would not fit in a `u16`.
    #[must_use]
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "divisors are non-zero constants; results stay small"
    )]
    pub fn from_days_elapsed(days: u32) -> Option<Date> {
        let month = u16::try_from(days / u32::from(DAYS_PER_MONTH) + 1).ok()?;
        // Both remainders are below 28, so the conversions cannot fail.
        let in_month = u16::try_from(days % u32::from(DAYS_PER_MONTH)).ok()?;
        Some(Date {
            month,
            week: in_month / DAYS_PER_WEEK + 1,
            day: in_month % DAYS_PER_WEEK + 1,
        })
    }

    #[must_use]
    pub const fn month(self) -> u16 {
        self.month
    }

    #[must_use]
    pub const fn week(self) -> u16 {
        self.week
    }

    /// Day of the week, 1–7.
    #[must_use]
    pub const fn day(self) -> u16 {
        self.day
    }

    /// Days since the first day of the game: 0 on month 1, week 1, day 1.
    #[must_use]
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "`new` keeps every part at least 1, and 65535 months of days fit in a u32"
    )]
    pub fn days_elapsed(self) -> u32 {
        (u32::from(self.month) - 1) * u32::from(DAYS_PER_MONTH)
            + u32::from(self.week - 1) * u32::from(DAYS_PER_WEEK)
            + u32::from(self.day - 1)
    }

    /// Decode the stored date: day, week and month as little-endian `u16`s.
    #[must_use]
    pub fn from_bytes(bytes: [u8; SIZE]) -> Option<Date> {
        let [d0, d1, w0, w1, m0, m1] = bytes;
        Date::new(
            u16::from_le_bytes([m0, m1]),
            u16::from_le_bytes([w0, w1]),
            u16::from_le_bytes([d0, d1]),
        )
    }
}

impl Default for Date {
    fn default() -> Self {
        Date::FIRST
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Month {}, Week {}, Day {}", self.month, self.week, self.day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_accepts_exactly_the_valid_ranges() {
        for month in 0..=3 {
            for week in 0..=6 {
                for day in 0..=9 {
                    let valid = month >= 1 && (1..=4).contains(&week) && (1..=7).contains(&day);
                    assert_eq!(Date::new(month, week, day).is_some(), valid, "{month} {week} {day}");
                }
            }
        }
        assert!(Date::new(u16::MAX, 4, 7).is_some());
    }

    #[test]
    fn accessors_return_the_parts() {
        let date = Date::new(3, 4, 5).unwrap();
        assert_eq!((date.month(), date.week(), date.day()), (3, 4, 5));
    }

    #[test]
    fn days_elapsed_counts_from_zero() {
        assert_eq!(Date::FIRST.days_elapsed(), 0);
        assert_eq!(Date::new(1, 1, 7).unwrap().days_elapsed(), 6);
        assert_eq!(Date::new(1, 2, 1).unwrap().days_elapsed(), 7);
        assert_eq!(Date::new(1, 4, 7).unwrap().days_elapsed(), 27);
        assert_eq!(Date::new(2, 1, 1).unwrap().days_elapsed(), 28);
        assert_eq!(
            Date::new(u16::MAX, 4, 7).unwrap().days_elapsed(),
            u32::from(u16::MAX) * 28 - 1
        );
    }

    #[test]
    fn from_days_elapsed_inverts_days_elapsed() {
        let mut previous = None;
        for days in 0..(28 * 13) {
            let date = Date::from_days_elapsed(days).unwrap();
            assert_eq!(date.days_elapsed(), days);
            assert_eq!(Date::new(date.month(), date.week(), date.day()), Some(date));
            if let Some(previous) = previous {
                assert!(previous < date, "{previous} is not before {date}");
            }
            previous = Some(date);
        }
        let last = u32::from(u16::MAX) * 28 - 1;
        assert_eq!(Date::from_days_elapsed(last), Date::new(u16::MAX, 4, 7));
        assert_eq!(Date::from_days_elapsed(last + 1), None);
        assert_eq!(Date::from_days_elapsed(u32::MAX), None);
    }

    #[test]
    fn orders_by_month_then_week_then_day() {
        let a = Date::new(1, 4, 7).unwrap();
        let b = Date::new(2, 1, 1).unwrap();
        let c = Date::new(2, 1, 2).unwrap();
        let d = Date::new(2, 2, 1).unwrap();
        assert!(a < b && b < c && c < d);
        assert_eq!(Date::default(), Date::FIRST);
    }

    #[test]
    fn from_bytes_reads_day_week_month_little_endian() {
        assert_eq!(Date::from_bytes([1, 0, 1, 0, 1, 0]), Some(Date::FIRST));
        assert_eq!(Date::from_bytes([7, 0, 2, 0, 1, 0]), Date::new(1, 2, 7));
        assert_eq!(Date::from_bytes([3, 0, 4, 0, 0x2C, 0x01]), Date::new(300, 4, 3));
        // High bytes count: day 0x0101 is not day 1.
        assert_eq!(Date::from_bytes([1, 1, 1, 0, 1, 0]), None);
        assert_eq!(Date::from_bytes([1, 0, 1, 1, 1, 0]), None);
        assert_eq!(Date::from_bytes([0, 0, 1, 0, 1, 0]), None);
        assert_eq!(Date::from_bytes([8, 0, 1, 0, 1, 0]), None);
        assert_eq!(Date::from_bytes([1, 0, 5, 0, 1, 0]), None);
        assert_eq!(Date::from_bytes([1, 0, 1, 0, 0, 0]), None);
    }

    #[test]
    fn displays_all_three_parts() {
        assert_eq!(Date::new(12, 3, 6).unwrap().to_string(), "Month 12, Week 3, Day 6");
    }
}

/// Trait for interoperation with different datetime crates.
///
/// At the moment, [chrono](https://crates.io/crates/chrono) and [time](https://crates.io/crates/time).
pub trait DateTimeLike {
    /// Returns the year number
    fn full_year(&self) -> i32;

    /// Returns the month number starting from 1.
    ///
    /// The return value ranges from 1 to 12.
    fn month(&self) -> u32;

    /// Returns the day of month starting from 1.
    ///
    /// The return value ranges from 1 to 31. (The last day of month differs by months.)
    fn day(&self) -> u32;

    /// Returns the hour number from 0 to 23.
    fn hour(&self) -> u32;

    /// Returns the minute number from 0 to 59.
    fn minute(&self) -> u32;

    /// Returns the second number from 0 to 59.
    fn second(&self) -> u32;

    /// Returns the number of microseconds since the whole non-leap second.
    /// The range from 1,000,000 to 1,999,999 represents
    /// the leap second
    fn microsecond(&self) -> u32;
}

use chrono::{Datelike, Timelike};

impl DateTimeLike for chrono::NaiveDateTime {
    fn full_year(&self) -> i32 {
        self.year()
    }
    fn month(&self) -> u32 {
        Datelike::month(self)
    }
    fn day(&self) -> u32 {
        Datelike::day(self)
    }
    fn hour(&self) -> u32 {
        Timelike::hour(self)
    }
    fn minute(&self) -> u32 {
        Timelike::minute(self)
    }
    fn second(&self) -> u32 {
        Timelike::second(self)
    }
    fn microsecond(&self) -> u32 {
        Timelike::nanosecond(self) / 1_000
    }
}

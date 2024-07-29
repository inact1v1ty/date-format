use chrono::Datelike;

/// Trait for interoperation with different datetime crates.
///
/// At the moment, [chrono](https://crates.io/crates/chrono) and [time](https://crates.io/crates/time).
pub trait DateTimeLike {
    fn full_year(&self) -> i32;
}

impl DateTimeLike for chrono::NaiveDateTime {
    fn full_year(&self) -> i32 {
        self.year()
    }
}

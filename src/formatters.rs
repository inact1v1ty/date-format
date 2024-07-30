//! |     | Unit                           |     | Unit                           |
//! |-----|--------------------------------|-----|--------------------------------|
//! |  a  | AM, PM                         |  A* | Milliseconds in day            |
//! |  b  | AM, PM, noon, midnight         |  B  | Flexible day period            |
//! |  c  | Stand-alone local day of week  |  C* | Localized hour w/ day period   |
//! |  d  | Day of month                   |  D  | Day of year                    |
//! |  e  | Local day of week              |  E  | Day of week                    |
//! |  f  |                                |  F* | Day of week in month           |
//! |  g* | Modified Julian day            |  G  | Era                            |
//! |  h  | Hour [1-12]                    |  H  | Hour [0-23]                    |
//! |  i! | ISO day of week                |  I! | ISO week of year               |
//! |  j* | Localized hour w/ day period   |  J* | Localized hour w/o day period  |
//! |  k  | Hour [1-24]                    |  K  | Hour [0-11]                    |
//! |  l* | (deprecated)                   |  L  | Stand-alone month              |
//! |  m  | Minute                         |  M  | Month                          |
//! |  n  |                                |  N  |                                |
//! |  o! | Ordinal number modifier        |  O  | Timezone (GMT)                 |
//! |  p! | Long localized time            |  P! | Long localized date            |
//! |  q  | Stand-alone quarter            |  Q  | Quarter                        |
//! |  r* | Related Gregorian year         |  R! | ISO week-numbering year        |
//! |  s  | Second                         |  S  | Fraction of second             |
//! |  t! | Seconds timestamp              |  T! | Milliseconds timestamp         |
//! |  u  | Extended year                  |  U* | Cyclic year                    |
//! |  v* | Timezone (generic non-locat.)  |  V* | Timezone (location)            |
//! |  w  | Local week of year             |  W* | Week of month                  |
//! |  x  | Timezone (ISO-8601 w/o Z)      |  X  | Timezone (ISO-8601)            |
//! |  y  | Year (abs)                     |  Y  | Local week-numbering year      |
//! |  z  | Timezone (specific non-locat.) |  Z* | Timezone (aliases)             |
//!
//! Letters marked by * are not implemented but reserved by Unicode standard.
//!
//! Letters marked by ! are non-standard, but implemented by date-fns:
//! - `o` modifies the previous token to turn it into an ordinal (see `format` docs)
//! - `i` is ISO day of week. For `i` and `ii` is returns numeric ISO week days,
//!   i.e. 7 for Sunday, 1 for Monday, etc.
//! - `I` is ISO week of year, as opposed to `w` which is local week of year.
//! - `R` is ISO week-numbering year, as opposed to `Y` which is local week-numbering year.
//!   `R` is supposed to be used in conjunction with `I` and `i`
//!   for universal ISO week-numbering date, whereas
//!   `Y` is supposed to be used in conjunction with `w` and `e`
//!   for week-numbering date specific to the locale.
//! - `P` is long localized date format
//! - `p` is long localized time format
//!

use std::borrow::Cow;

use crate::{
    light_formatters,
    locale::{Era, EraWidth, Locale},
    types::DateTimeLike,
};

pub(crate) trait Formatter<'a, D: DateTimeLike + 'a>:
    Fn(&'a D, Cow<'a, str>, &Box<dyn Locale>) -> Cow<'a, str>
{
}

impl<'a, T, D: DateTimeLike + 'a> Formatter<'a, D> for T where
    T: Fn(&'a D, Cow<'a, str>, &Box<dyn Locale>) -> Cow<'a, str>
{
}

pub(crate) fn is_formatter(first_char: char) -> bool {
    match first_char {
        _ => false,
    }
}

pub(crate) fn get_formatter<'a, D: DateTimeLike + 'a>(first_char: char) -> impl Formatter<'a, D> {
    match first_char {
        'G' => era_formatter::<D>,
        _ => unreachable!(),
    }
}

// Era
fn era_formatter<'a, D: DateTimeLike + 'a>(
    datetime: &'a D,
    token: Cow<'a, str>,
    locale: &Box<dyn Locale>,
) -> Cow<'a, str> {
    let era = if datetime.full_year() > 0 {
        Era::AD
    } else {
        Era::BC
    };
    let era_width = match token.as_ref() {
        "G" => EraWidth::Abbreviated,
        "GG" => EraWidth::Abbreviated,
        "GGG" => EraWidth::Abbreviated,
        "GGGGG" => EraWidth::Narrow,
        "GGGG" => EraWidth::Wide,
        _ => EraWidth::Wide,
    };
    locale.era(era, era_width)
}

// Year
fn year_formatter<'a, D: DateTimeLike + 'a>(
    datetime: &'a D,
    token: Cow<'a, str>,
    locale: &Box<dyn Locale>,
) -> Cow<'a, str> {
    if token == "yo" {
        let signed_year = datetime.full_year();
        let year = if signed_year > 0 {
            signed_year
        } else {
            1 - signed_year
        };
        locale.ordinal_number(year.try_into().unwrap())
    } else {
        light_formatters::y(datetime, token)
    }
}

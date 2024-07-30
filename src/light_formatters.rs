//! |     | Unit                           |     | Unit                           |
//! |-----|--------------------------------|-----|--------------------------------|
//! |  a  | AM, PM                         |  A* |                                |
//! |  d  | Day of month                   |  D  |                                |
//! |  h  | Hour [1-12]                    |  H  | Hour [0-23]                    |
//! |  m  | Minute                         |  M  | Month                          |
//! |  s  | Second                         |  S  | Fraction of second             |
//! |  y  | Year (abs)                     |  Y  |                                |
//!
//! Letters marked by * are not implemented but reserved by Unicode standard.

use std::borrow::Cow;

use crate::{types::DateTimeLike, utils::add_leading_zeroes};

/// Year
///
/// From http://www.unicode.org/reports/tr35/tr35-31/tr35-dates.html#Date_Format_tokens
///
/// | Year     |     y | yy |   yyy |  yyyy | yyyyy |
/// |----------|-------|----|-------|-------|-------|
/// | AD 1     |     1 | 01 |   001 |  0001 | 00001 |
/// | AD 12    |    12 | 12 |   012 |  0012 | 00012 |
/// | AD 123   |   123 | 23 |   123 |  0123 | 00123 |
/// | AD 1234  |  1234 | 34 |  1234 |  1234 | 01234 |
/// | AD 12345 | 12345 | 45 | 12345 | 12345 | 12345 |
pub(crate) fn y<'a, D: DateTimeLike + 'a>(datetime: &'a D, token: Cow<'a, str>) -> Cow<'a, str> {
    let signed_year = datetime.full_year();
    let year = if signed_year > 0 {
        signed_year
    } else {
        1 - signed_year
    };
    return add_leading_zeroes(if token == "yy" { year % 100 } else { year }, token.len()).into();
}

/// Month
pub(crate) fn M<'a, D: DateTimeLike + 'a>(datetime: &'a D, token: Cow<'a, str>) -> Cow<'a, str> {
    let month = datetime.month();
    if token == "M" {
        month.to_string().into()
    } else {
        add_leading_zeroes(month.try_into().unwrap(), 2).into()
    }
}

/// Day
pub(crate) fn d<'a, D: DateTimeLike + 'a>(datetime: &'a D, token: Cow<'a, str>) -> Cow<'a, str> {
    add_leading_zeroes(datetime.day().try_into().unwrap(), token.len()).into()
}

enum AmPm {
    Am,
    Pm,
}

/// AM or PM
pub(crate) fn a<'a, D: DateTimeLike + 'a>(datetime: &'a D, token: Cow<'a, str>) -> Cow<'a, str> {
    let ampm = if datetime.hour() >= 12 {
        AmPm::Pm
    } else {
        AmPm::Am
    };

    match token.as_ref() {
        "a" | "aa" => match ampm {
            AmPm::Am => "AM".into(),
            AmPm::Pm => "PM".into(),
        },
        "aaa" => match ampm {
            AmPm::Am => "am".into(),
            AmPm::Pm => "pm".into(),
        },
        "aaaaa" => match ampm {
            AmPm::Am => "a".into(),
            AmPm::Pm => "p".into(),
        },
        "aaaa" | _ => match ampm {
            AmPm::Am => "a.m.".into(),
            AmPm::Pm => "p.m.".into(),
        },
    }
}

/// Hour [1-12]
pub(crate) fn h<'a, D: DateTimeLike + 'a>(datetime: &'a D, token: Cow<'a, str>) -> Cow<'a, str> {
    let mut hour = datetime.hour() % 12;
    if hour == 0 {
        hour = 12;
    }
    add_leading_zeroes(hour.try_into().unwrap(), token.len()).into()
}

/// Hour [0-23]
pub(crate) fn H<'a, D: DateTimeLike + 'a>(datetime: &'a D, token: Cow<'a, str>) -> Cow<'a, str> {
    add_leading_zeroes(datetime.hour().try_into().unwrap(), token.len()).into()
}

/// Minute
pub(crate) fn m<'a, D: DateTimeLike + 'a>(datetime: &'a D, token: Cow<'a, str>) -> Cow<'a, str> {
    add_leading_zeroes(datetime.minute().try_into().unwrap(), token.len()).into()
}

/// Second
pub(crate) fn s<'a, D: DateTimeLike + 'a>(datetime: &'a D, token: Cow<'a, str>) -> Cow<'a, str> {
    add_leading_zeroes(datetime.second().try_into().unwrap(), token.len()).into()
}

/// Fraction of second
pub(crate) fn S<'a, D: DateTimeLike + 'a>(datetime: &'a D, token: Cow<'a, str>) -> Cow<'a, str> {
    let len = token.len().min(6);
    let mut fraction = datetime.microsecond();

    for _ in len..6 {
        fraction /= 10;
    }

    add_leading_zeroes(fraction.try_into().unwrap(), len).into()
}

use std::borrow::Cow;

use crate::{
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

pub(crate) fn get_formatter<'a, D: DateTimeLike + 'a>(first_char: char) -> impl Formatter<'a, D> {
    match first_char {
        'G' => era_formatter::<D>,
        _ => unreachable!(),
    }
}

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

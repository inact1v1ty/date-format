use std::borrow::Cow;

use crate::locale::{Era, EraWidth, Locale};

pub struct En;

impl Locale for En {
    fn era<'a>(&self, era: Era, era_width: EraWidth) -> Cow<'a, str> {
        match era_width {
            EraWidth::Narrow => match era {
                Era::BC => "B".into(),
                Era::AD => "A".into(),
            },
            EraWidth::Abbreviated => match era {
                Era::BC => "BC".into(),
                Era::AD => "AD".into(),
            },
            EraWidth::Wide => match era {
                Era::BC => "Before Christ".into(),
                Era::AD => "Anno Domini".into(),
            },
        }
    }

    fn ordinal_number<'a>(&self, num: u32) -> Cow<'a, str> {
        let rem100 = num % 100;
        let ending = if rem100 > 20 || rem100 < 10 {
            match rem100 % 10 {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            }
        } else {
            "th"
        };
        (num.to_string() + ending).into()
    }
}

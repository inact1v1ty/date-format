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
}

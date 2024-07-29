use crate::locale::Locale;

mod en;

pub enum Locales {
    En,
    Custom(Box<dyn Locale>),
}

impl Locales {
    pub(crate) fn to_locale(self) -> Box<dyn Locale> {
        match self {
            Locales::En => Box::new(en::En),
            Locales::Custom(boxed) => boxed,
        }
    }
}

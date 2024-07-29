use std::borrow::Cow;

pub enum Era {
    BC,
    AD,
}

pub enum EraWidth {
    Narrow,
    Abbreviated,
    Wide,
}

pub trait Locale {
    fn era<'a>(&self, era: Era, era_width: EraWidth) -> Cow<'a, str>;
}

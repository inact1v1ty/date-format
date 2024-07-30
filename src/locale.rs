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
    fn ordinal_number<'a>(&self, num: u32) -> Cow<'a, str>;
}

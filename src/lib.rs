#![warn(missing_docs)]

//! Date formating functions with localization built in.

use std::borrow::Cow;

use anyhow::Result;
use fancy_regex::Regex as FancyRegex;
use itertools::Itertools;
use locale::Locale;
use locales::Locales;
use once_cell::sync::Lazy;
use regex::Regex;
use types::DateTimeLike;

mod formatters;
mod light_formatters;
pub mod locale;
pub mod locales;
pub(crate) mod types;
pub(crate) mod utils;

/// This RegExp consists of three parts separated by `|`:
/// - \[yYQqMLwIdDecihHKkms\]o matches any available ordinal number token
///   (one of the certain letters followed by `o`)
/// - (\w)\1* matches any sequences of the same letter
/// - '' matches two quote characters in a row
/// - '(''|[^'])+('|$) matches anything surrounded by two quote characters ('),
///   except a single quote symbol, which ends the sequence.
///   Two quote characters do not end the sequence.
///   If there is no matching single quote
///   then the sequence will continue until the end of the string.
/// - . matches any single character unmatched by previous parts of the RegExps
///
/// Uses fancy_regex for backtracking \1 in second part
static FORMATTING_TOKENS_RE: Lazy<FancyRegex> =
    Lazy::new(|| FancyRegex::new(r"[yYQqMLwIdDecihHKkms]o|(\w)\1*|''|'(''|[^'])+('|$)|.").unwrap());

/// This RegExp catches symbols escaped by quotes, and also
/// sequences of symbols P, p, and the combinations like `PPPPPPPppppp`
static LONG_FORMATTING_TOKENS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"P+p+|P+|p+|''|'(''|[^'])+('|$)|.").unwrap());

static ESCAPED_STRING_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^'([^]*?)'?$").unwrap());

static DOUBLE_QUOTE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"''").unwrap());

struct FormatPart<'a> {
    is_token: bool,
    value: Cow<'a, str>,
}

/// Return the formatted date string in the given format. The result may vary by locale.
///
/// See: <https://github.com/date-fns/date-fns/blob/master/docs/unicodeTokens.md>
///
/// The characters wrapped between two single quotes characters (') are escaped.
/// Two single quotes in a row, whether inside or outside a quoted sequence, represent a 'real' single quote.
pub fn datetime_format<D: DateTimeLike>(
    datetime: D,
    format_str: &str,
    locale: Locales,
) -> Result<String> {
    let locale = locale.to_locale();
    let long_processed = LONG_FORMATTING_TOKENS_RE
        .find_iter(format_str)
        .map(|m| {
            // TODO: long formatters
            m.as_str()
        })
        .join("");

    let parts = FORMATTING_TOKENS_RE
        .find_iter(&long_processed)
        .map(|r| {
            r.map_err(anyhow::Error::new).map(|m| {
                let substring = m.as_str();
                if substring == "''" {
                    return FormatPart {
                        is_token: false,
                        value: "'".into(),
                    };
                }
                let first_char = substring.chars().next().unwrap();
                if first_char == '\'' {
                    return FormatPart {
                        is_token: false,
                        value: clean_escaped_string(substring),
                    };
                }
                if true
                /* formatters[first_chat] */
                {
                    return FormatPart {
                        is_token: true,
                        value: substring.into(),
                    };
                }

                if first_char.is_ascii_alphabetic() {
                    panic!(
                        "Format string contains an unescaped latin alphabet character `{}`",
                        first_char
                    );
                }

                FormatPart {
                    is_token: false,
                    value: substring.into(),
                }
            })
        })
        .map(|r| r.unwrap());

    // TODO: localize preprocessors

    Ok(parts
        .map(|p| {
            if !p.is_token {
                return p.value;
            }

            let first_char = p.value.chars().next().unwrap();

            let formatter = formatters::get_formatter::<D>(first_char);

            formatter(&datetime, p.value, &locale)
        })
        .join(""))
}

fn clean_escaped_string<'a>(value: &'a str) -> Cow<'a, str> {
    if let Some(caps) = ESCAPED_STRING_RE.captures(value) {
        DOUBLE_QUOTE_RE.replace(caps.get(1).unwrap().as_str(), "'")
    } else {
        value.into()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn era_works() {
        let datetime = chrono::NaiveDate::from_ymd_opt(2020, 4, 12)
            .unwrap()
            .and_hms_opt(9, 10, 11)
            .unwrap();

        assert_eq!(datetime_format(datetime, "GG", Locales::En).unwrap(), "AD");
    }
}

//! This crate implements the case-folding rules used by Dropbox for file paths.
//!
//! It's a recreation of what Python 2.5's unicode.lower() did (which was the current version of
//! Python at the time of Dropbox's founding).
//!
//! For every character in the Unicode 4.1 character database which has a "simple lowercase mapping"
//! property, it replaces it with the corresponding character.
//!
//! The mapping is hardcoded, but the code can be regenerated manually from the Unicode database
//! using an included program in the codebase.

mod generated;

/// The mapping from upper-case characters to lower-case characters.
pub use generated::MAP;

/// Case-fold a character to lower-case, using the same rules as Dropbox uses for file paths.
/// If the character is not an upper-case character according to the mapping, returns the original
/// character unchanged.
pub fn dbx_lowercase(c: char) -> char {
    MAP.binary_search_by(|(upper, _)| upper.cmp(&c))
        .map(|i| MAP[i].1)
        .unwrap_or(c)
}

/// Case-fold a string to lower-case. See [`dbx_lowercase`].
pub fn dbx_str_lowercase(s: &str) -> String {
    s.chars().map(dbx_lowercase).collect()
}

/// Check whether two strings are equal, ignoring case. See [`dbx_lowercase`].
pub fn dbx_eq_ignore_case(a: &str, b: &str) -> bool {
    a.chars().map(dbx_lowercase).eq(b.chars().map(dbx_lowercase))
}

/// Returns a string slice with the prefix removed, ignoring case. See [`dbx_lowercase`] and
/// [`str::strip_prefix`].
pub fn dbx_strip_prefix_ignore_case<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let mut pfx_it = prefix.chars().map(dbx_lowercase);
    let mut last = None;
    let trimmed = s.trim_start_matches(|c: char| {
        last = pfx_it.next();
        last == Some(dbx_lowercase(c))
    });
    if last.is_some() {
        None
    } else {
        Some(trimmed)
    }
}

#[cfg(test)]
mod test {
    use crate::{dbx_str_lowercase, dbx_lowercase, MAP};

    #[test]
    fn examples() {
        assert_eq!(MAP.len(), 893);
        assert_eq!('a', dbx_lowercase('A'));
        assert_eq!('a', dbx_lowercase('a'));
        assert_eq!('i', dbx_lowercase('İ')); // capital dotted I goes to plain ascii i
        assert_eq!('ⓜ', dbx_lowercase('Ⓜ'));
        assert_eq!('\u{B5}', dbx_lowercase('\u{B5}')); // MICRO SIGN
        assert_eq!('\u{1F80}', dbx_lowercase('\u{1F88}')); // GREEK CAPITAL LETTER ALPHA WITH PSILI AND PROSGEGRAMMENI

        assert_eq!("hi thére", dbx_str_lowercase("Hİ THÉRE"));
    }

    #[test]
    fn test_mapping() {
        let mut upper_str = String::new();
        let mut lower_str = String::new();
        for (upper, lower) in MAP {
            assert_eq!(dbx_lowercase(*upper), *lower);
            assert_eq!(dbx_lowercase(*lower), *lower);
            upper_str.push(*upper);
            lower_str.push(*lower);
        }
        assert_eq!(dbx_str_lowercase(&upper_str), lower_str);
    }
}

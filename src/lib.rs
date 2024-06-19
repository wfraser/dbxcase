//! This crate implements the case-folding rules used by Dropbox for file paths.
//!
//! It's a recreation of what Python 2.5's unicode.lower() did (which was the current version of
//! Python at the time of Dropbox's founding).
//!
//! For every character in the Unicode 4.1 character database which has a "simple lowercase mapping"
//! property, it replaces it with the corresponding character.
//!
//! This is different from a proper lowercasing, where at least one upper-case codepoint (U+0130,
//! "Latin Capital Letter I with Dot Above") maps to two lower-case codepoints. It also uses a very
//! old version of Unicode which lacks many characters added since 2003.
//!
//! The mapping is hardcoded, but the code can be regenerated manually from the Unicode database
//! using an included program in the codebase.

mod generated;

/// The mapping from upper-case characters to lower-case characters, excluding ASCII A-Z (check for
/// those separately, since they are so common).
pub use generated::MAP;

/// Case-fold a character to lower-case, using the same rules as Dropbox uses for file paths.
/// If the character is not an upper-case character according to the mapping, returns the original
/// character unchanged.
pub fn dbx_lowercase(c: char) -> char {
    if c.is_ascii() {
        c.to_ascii_lowercase()
    } else {
        MAP.binary_search_by(|(upper, _)| upper.cmp(&c))
            .map(|i| MAP[i].1)
            .unwrap_or(c)
    }
}

/// Case-fold a string to lower-case. See [`dbx_lowercase`].
pub fn dbx_str_lowercase(s: &str) -> String {
    s.chars().map(dbx_lowercase).collect()
}

/// Check whether two strings are equal, ignoring case. See [`dbx_lowercase`].
pub fn dbx_eq_ignore_case(a: &str, b: &str) -> bool {
    a.chars()
        .map(dbx_lowercase)
        .eq(b.chars().map(dbx_lowercase))
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
    use super::*;

    #[test]
    fn test_lowercase() {
        assert_eq!(MAP.len(), 893 - 26); // A-Z (26 letters) omitted
        assert_eq!('a', dbx_lowercase('A'));
        assert_eq!('a', dbx_lowercase('a'));
        assert_eq!('ⓜ', dbx_lowercase('Ⓜ'));

        // Capital dotted I should be lowercased to two codepoints: plain ascii 'i' and combining
        // dot above (which is visually redundant).
        // Python 2.5 just lowercases it to ascii 'i'.
        assert_eq!('i', dbx_lowercase('İ'));

        // U+1F88 Greek Capital Letter Alpha With Psili And Prosgegrammeni
        // A modern implementation would lowercase this to two codepoints: U+1F62 U+03B9
        // Python 2.5 lowercases it to U+1F80 Greek Small Letter Alpha with Psili and Ypogegrammeni
        assert_eq!('\u{1F80}', dbx_lowercase('\u{1F88}'));

        // U+A64A Cyrillic Capital Letter Monograph Uk
        // Added in Unicode 5.1; Python 2.5 knows nothing about it and leaves it unchanged.
        assert_eq!('Ꙋ', dbx_lowercase('Ꙋ'));

        // Should properly be lowercased to "σσς" but Python 2.5 is not context-sensitive.
        assert_eq!("σσσ", dbx_str_lowercase("ΣΣΣ"));

        assert_eq!("hi thére", dbx_str_lowercase("Hİ THÉRE"));
    }

    #[test]
    fn test_helpers() {
        assert_eq!(
            Some("_SUFFIX"),
            dbx_strip_prefix_ignore_case("SİX_SUFFIX", "six")
        );
        assert_eq!(None, dbx_strip_prefix_ignore_case("ABC", "abcd"));
        assert_eq!(Some("ABC"), dbx_strip_prefix_ignore_case("ABC", ""));
        assert!(dbx_eq_ignore_case("Ⓗİ THÉRE", "ⓗi thére"));
        assert!(!dbx_eq_ignore_case("ABCD", "abcde"));
        assert!(dbx_eq_ignore_case("", ""));
        assert!(!dbx_eq_ignore_case("x", ""));
        assert!(!dbx_eq_ignore_case("", "x"));
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

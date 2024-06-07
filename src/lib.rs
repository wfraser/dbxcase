//! This crate implements the case-folding rules used by Dropbox for file paths.
//!
//! It's a recreation of what Python 2.5's unicode.lower() did (which was the current version of
//! Python at the time of Dropbox's founding).
//!
//! For every character in the Unicode 4.1 character database which has a "simple lowercase mapping"
//! property, it replaces it with the corresponding character.
//!
//! The mapping is pre-generated for speed and size, but the code can be regenerated manually from
//! the Unicode database using an included program in the codebase.

mod precompiled;

/// The mapping from upper-case characters to lower-case characters.
pub use precompiled::MAP;

/// Case-fold a character to lower-case, using the same rules as Dropbox uses for file paths.
/// If the character is not an upper-case character according to the mapping, returns the original
/// character unchanged.
pub fn dbx_tolower(c: char) -> char {
    MAP.get(&c).cloned().unwrap_or(c)
}

/// Case-fold a string to lower-case. See [`dbx_tolower`].
pub fn dbx_str_tolower(s: &str) -> String {
    s.chars().map(dbx_tolower).collect()
}

#[cfg(test)]
mod test {
    use crate::{dbx_str_tolower, dbx_tolower};

    #[test]
    fn test() {
        assert_eq!(crate::precompiled::MAP.len(), 893);
        assert_eq!('a', dbx_tolower('A'));
        assert_eq!('a', dbx_tolower('a'));
        assert_eq!('i', dbx_tolower('İ')); // capital dotted I goes to plain ascii i
        assert_eq!('ⓜ', dbx_tolower('Ⓜ'));
        assert_eq!('\u{B5}', dbx_tolower('\u{B5}')); // MICRO SIGN
        assert_eq!('\u{1F80}', dbx_tolower('\u{1F88}')); // GREEK CAPITAL LETTER ALPHA WITH PSILI AND PROSGEGRAMMENI

        assert_eq!("hi thére", dbx_str_tolower("Hİ THÉRE"));
    }
}

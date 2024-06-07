//! Use this to generate src/precompiled.rs from scratch using the Unicode character data.
//! It requires additional dependencies; run it using:
//!     cargo run --bin generate --features generate

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let ucd = ureq::get("https://unicode.org/Public/4.1.0/ucd/UnicodeData.txt")
        .call()
        .expect("failed to download Unicode data file")
        .into_reader();

    let mut phf = phf_codegen::Map::new();

    for line in BufReader::new(ucd).lines() {
        let line = line.expect("read error in unicode data file");
        let mut fields = line.split(';');
        let code = fields.next().expect("missing codepoint field");
        let lowercode = fields.nth(12).expect("missing simple lowercase mapping field (#13)");

        if lowercode.is_empty() {
            // No simple lowercase mapping.
            continue;
        }

        let upper = char::from_u32(u32::from_str_radix(code, 16).expect("invalid codepoint")).expect("invalid character");
        let lower = char::from_u32(u32::from_str_radix(lowercode, 16).expect("invalid codepoint")).expect("invalid character");

        phf.entry(upper, &format!("'{lower}'"));
    }

    let mut out = File::create(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("precompiled.rs")
    ).expect("failed to create src/precompiled.rs");

    out.write_all(b"pub const MAP: ::phf::Map<char, char> = ").expect("src/precompiled.rs write error");
    out.write_all(phf.build().to_string().as_bytes()).expect("src/precompiled.rs write error");
    out.write_all(b";\n").expect("src/precompiled.rs write error");
}
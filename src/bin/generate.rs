//! Use this to generate src/generated.rs from scratch using the Unicode character data.
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

    let mut out = File::create(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("generated.rs"),
    )
    .expect("failed to create src/generated.rs");

    out.write_all(
        b"//! This file was generated by src/bin/generate.rs\n\
            pub const MAP: &[(char, char)] = &[\n",
    )
    .expect("src/generated.rs write error");

    for line in BufReader::new(ucd).lines() {
        let line = line.expect("read error in unicode data file");
        let mut fields = line.split(';');
        let code = fields.next().expect("missing codepoint field");
        let lowercode = fields
            .nth(12)
            .expect("missing simple lowercase mapping field (#13)");

        if lowercode.is_empty() {
            // No simple lowercase mapping.
            continue;
        }

        let upper = char::from_u32(u32::from_str_radix(code, 16).expect("invalid codepoint"))
            .expect("invalid character");
        let lower = char::from_u32(u32::from_str_radix(lowercode, 16).expect("invalid codepoint"))
            .expect("invalid character");

        writeln!(out, "    ({upper:?}, {lower:?}),").expect("src/generated.rs write error");
    }

    out.write_all(b"];\n")
        .expect("src/generated.rs write error");
}

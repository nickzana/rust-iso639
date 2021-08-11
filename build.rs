use csv::{ReaderBuilder, StringRecord};
use inflector::Inflector;
use std::collections::HashSet;
use std::env;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const TAG3: usize = 0;
const TAG1: usize = 1;
const NAME: usize = 2;
const AUTONYM: usize = 3;

macro_rules! template {
    () => {
        r#"
#[derive(Debug, PartialEq, Eq)]
pub enum Iso639 {{
    {variants}
}}

impl IsoCompat for Iso639 {{
    fn name(&self) -> &str {{
        match self {{
            {name_arms}
        }}
    }}

    fn iso639_3(&self) -> &str {{
        match self {{
            {iso639_3_arms}
        }}
    }}

    fn iso639_1(&self) -> Option<&str> {{
        match self {{
            {iso639_1_arms}
            _ => None,
        }}
    }}

    fn autonym(&self) -> Option<&str> {{
        match self {{
            {autonym_arms}
            _ => None,
        }}
    }}

    fn from_name(name: &str) -> Result<Self, Err> {{
        match name {{
            {from_name_arms}
            _ => Err(Err::UnknownName(name.to_string())),
        }}
    }}

    fn from_iso639_3(code: &str) -> Result<Self, Err> {{
        match code {{
            {from_iso639_3_arms}
            _ => Err(Err::UnknownLanguage(code.to_string())),
        }}
    }}

    fn from_iso639_1(code: &str) -> Result<Self, Err> {{
        match code {{
            {from_iso639_1_arms}
            _ => Err(Err::UnknownIso639_1(code.to_string())),
        }}
    }}

    fn from_autonym(autonym: &str) -> Result<Self, Err> {{
        match autonym {{
            {from_autonym_arms}
            _ => Err(Err::UnknownAutonym(autonym.to_string())),
        }}
    }}
}}
"#
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("./iso639-databases/iso639-autonyms.tsv");
    let f = std::fs::File::open(&path)?;

    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_reader(f);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("iso_639.rs");
    let mut f = BufWriter::new(File::create(&dest_path)?);

    let mut variants = String::new();

    let mut name_arms = String::new();
    let mut iso639_3_arms = String::new();
    let mut iso639_1_arms = String::new();
    let mut autonym_arms = String::new();

    let mut from_name_arms = String::new();
    let mut from_iso639_3_arms = String::new();
    let mut from_iso639_1_arms = String::new();
    let mut from_autonym_arms = String::new();

    // Some iso639 codes share the same name, leaving the first (in this case, alphabetically) arm
    // in the match statement with any given name to be the only reachable path
    // This eliminates the warnings about unreachable code paths
    let mut used_names = HashSet::new();
    let mut used_autonyms = HashSet::new();

    reader.records().filter_map(|r| r.ok()).try_for_each(
        |r: StringRecord| -> Result<(), Box<dyn std::error::Error>> {
            let name = &r[NAME];
            let iso639_3 = &r[TAG3].to_title_case();
            let iso639_1 = match r[TAG1].trim() {
                "" => None,
                s => Some(s),
            };
            let autonym = match r[AUTONYM].trim() {
                "" => None,
                s => Some(s),
            };

            // TODO: Is it possible to eliminate the extra memory allocation of format!?
            writeln!(variants, "{},", iso639_3)?;

            writeln!(name_arms, "Self::{} => \"{}\",", iso639_3, name)?;
            if !used_names.contains(name) {
                used_names.insert(name.to_string());
                writeln!(from_name_arms, "\"{}\" => Ok(Self::{}),", name, iso639_3)?;
            }

            writeln!(
                iso639_3_arms,
                "Self::{} => \"{}\",",
                iso639_3,
                iso639_3.to_ascii_lowercase()
            )?;
            writeln!(
                from_iso639_3_arms,
                "\"{}\" => Ok(Self::{}),",
                iso639_3.to_ascii_lowercase(),
                iso639_3
            )?;

            if let Some(s) = iso639_1 {
                writeln!(
                    iso639_1_arms,
                    "Self::{} => Some(\"{}\"),",
                    iso639_3,
                    s.to_ascii_lowercase()
                )?;
                writeln!(
                    from_iso639_1_arms,
                    "\"{}\" => Ok(Self::{}),",
                    s.to_ascii_lowercase(),
                    iso639_3
                )?;
            }
            if let Some(s) = autonym {
                writeln!(autonym_arms, "Self::{} => Some(\"{}\"),", iso639_3, s)?;
                if !used_autonyms.contains(s) {
                    used_autonyms.insert(s.to_string());
                    writeln!(from_autonym_arms, "\"{}\" => Ok(Self::{}),", s, iso639_3)?;
                }
            }

            Ok(())
        },
    )?;

    writeln!(
        f,
        template!(),
        variants = variants,
        name_arms = name_arms,
        iso639_3_arms = iso639_3_arms,
        iso639_1_arms = iso639_1_arms,
        autonym_arms = autonym_arms,
        from_name_arms = from_name_arms,
        from_iso639_3_arms = from_iso639_3_arms,
        from_iso639_1_arms = from_iso639_1_arms,
        from_autonym_arms = from_autonym_arms
    )?;

    Ok(())
}

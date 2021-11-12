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
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Iso639 {{
    {variants}
}}

static TO_NAME: phf::Map<i32, &'static str> = {to_name};
static TO_ISO639_3: phf::Map<i32, &'static str> = {to_iso639_3};
static TO_ISO639_1: phf::Map<i32, &'static str> = {to_iso639_1};
static TO_AUTONYM: phf::Map<i32, &'static str> = {to_autonym};
static FROM_NAMES: phf::Map<&'static str, Iso639> = {from_name};
static FROM_ISO639_3: phf::Map<&'static str, Iso639> = {from_iso639_3};
static FROM_ISO639_1: phf::Map<&'static str, Iso639> = {from_iso639_1};
static FROM_AUTONYM: phf::Map<&'static str, Iso639> = {from_autonym};
"#
    };
}

struct Language {
    enum_val: String,
    enum_key: String,
    name: String,
    iso639_3: String,
    iso639_1: Option<String>,
    autonym: Option<String>,
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
    let mut to_name = phf_codegen::Map::new();
    let mut to_iso639_3 = phf_codegen::Map::new();
    let mut to_iso639_1 = phf_codegen::Map::new();
    let mut to_autonym = phf_codegen::Map::new();

    let mut from_name = phf_codegen::Map::new();
    let mut from_iso639_3 = phf_codegen::Map::new();
    let mut from_iso639_1 = phf_codegen::Map::new();
    let mut from_autonym = phf_codegen::Map::new();

    // Some iso639 codes share the same name, leaving the first (in this case, alphabetically) arm
    // in the match statement with any given name to be the only reachable path
    // This eliminates the warnings about unreachable code paths
    let mut used_names: HashSet<String> = HashSet::new();
    let mut used_autonyms: HashSet<String> = HashSet::new();

    let langs: Vec<Language> = reader
        .records()
        .filter_map(std::result::Result::ok)
        .map(|r: StringRecord| Language {
            enum_val: r[TAG3].to_title_case(),
            enum_key: format!("Iso639::{}", r[TAG3].to_title_case()),
            name: r[NAME].to_string(),
            iso639_3: r[TAG3].to_ascii_lowercase(),
            iso639_1: match r[TAG1].trim() {
                "" => None,
                s => Some(s.to_ascii_lowercase()),
            },
            autonym: match r[AUTONYM].trim() {
                "" => None,
                s => Some(s.to_string()),
            },
        })
        .collect();
    for (i, l) in langs.iter().enumerate() {
        let i = i as i32;
        writeln!(variants, "{} = {},", l.enum_val, i)?;

        to_name.entry(i, &format!("\"{}\"", &l.name));
        if !used_names.contains(&l.name) {
            used_names.insert(l.name.clone());
            from_name.entry(&l.name, &l.enum_key);
        }

        to_iso639_3.entry(i, &format!("\"{}\"", &l.iso639_3));
        from_iso639_3.entry(&l.iso639_3, &l.enum_key);

        if let Some(s) = &l.iso639_1 {
            to_iso639_1.entry(i, &format!("\"{}\"", s));
            from_iso639_1.entry(s, &l.enum_key);
        }

        if let Some(s) = &l.autonym {
            to_autonym.entry(i, &format!("\"{}\"", s));

            if !used_autonyms.contains(s) {
                used_autonyms.insert(s.clone());
                from_autonym.entry(s, &l.enum_key);
            }
        }
    }

    writeln!(
        f,
        template!(),
        variants = variants,
        to_name = to_name.build(),
        to_iso639_3 = to_iso639_3.build(),
        to_iso639_1 = to_iso639_1.build(),
        to_autonym = to_autonym.build(),
        from_name = from_name.build(),
        from_iso639_3 = from_iso639_3.build(),
        from_iso639_1 = from_iso639_1.build(),
        from_autonym = from_autonym.build(),
    )?;

    Ok(())
}

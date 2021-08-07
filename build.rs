extern crate inflector;

use csv::ReaderBuilder;
use inflector::Inflector;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const TAG3: usize = 0;
const TAG1: usize = 1;
const NAME: usize = 2;
const AUTONYM: usize = 3;

#[derive(Debug)]
struct Language {
    tag1: Option<String>,
    tag3: String,
    name: String,
    autonym: Option<String>,
}

impl Language {
    fn id(&self) -> String {
        self.tag3.trim().to_title_case()
    }
}

fn main() {
    // Update iso639-databases submodule
    std::process::Command::new("git")
        .args(&["submodule", "update", "--init"])
        .spawn()
        .unwrap();

    let path = Path::new("./iso639-databases/iso639-autonyms.tsv");
    let f = std::fs::File::open(&path).unwrap();

    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_reader(f);

    let mut langs = vec![];
    let mut ids: Vec<String> = vec![];

    let mut names: Vec<String> = vec![];
    let mut tag3s: Vec<String> = vec![];
    let mut tag1s: Vec<Option<String>> = vec![];
    let mut autonyms: Vec<Option<String>> = vec![];

    for lang in reader.records() {
        let lang = lang.unwrap();
        let lang = Language {
            tag1: match lang[TAG1].trim() {
                "" => None,
                s => Some(s.to_string()),
            },
            tag3: lang[TAG3].to_string(),
            name: lang[NAME].to_string(),
            autonym: match lang[AUTONYM].trim() {
                "" => None,
                s => Some(s.to_string()),
            },
        };
        langs.push(lang);
    }

    for lang in langs {
        ids.push(lang.id());
        names.push(lang.name);
        tag3s.push(lang.tag3);
        tag1s.push(lang.tag1);
        autonyms.push(lang.autonym);
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("iso_639.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    // Enum
    writeln!(&mut f, "#[derive(Debug, PartialEq, Eq)]").unwrap();
    writeln!(&mut f, "pub enum Iso639 {{").unwrap();

    for id in &ids {
        writeln!(&mut f, "{},", id).unwrap();
    }

    writeln!(&mut f, "}}").unwrap();

    writeln!(&mut f, "impl IsoCompat for Iso639 {{").unwrap();

    writeln!(
        &mut f,
        "fn name(&self) -> &str {{ {} }}",
        name_match_statement(&ids, &names)
    )
    .unwrap();
    writeln!(
        &mut f,
        "fn iso639_3(&self) -> &str {{ {} }}",
        iso639_3_match_statement(&ids, &tag3s)
    )
    .unwrap();
    writeln!(
        &mut f,
        "fn iso639_1(&self) -> Option<&str> {{ {} }}",
        iso639_1_match_statement(&ids, &tag1s)
    )
    .unwrap();
    writeln!(
        &mut f,
        "fn autonym(&self) -> Option<&str> {{ {} }}",
        autonym_match_statement(&ids, &autonyms)
    )
    .unwrap();

    writeln!(
        &mut f,
        "fn from_name(name: &str) -> Result<Self, Err> {{ {} }}",
        from_name_match_statement(&ids, &names)
    )
    .unwrap();
    writeln!(
        &mut f,
        "fn from_iso639_3(code: &str) -> Result<Self, Err> {{ {} }}",
        from_iso639_3_match_statement(&ids, &tag3s)
    )
    .unwrap();
    writeln!(
        &mut f,
        "fn from_iso639_1(code: &str) -> Result<Self, Err> {{ {} }}",
        from_iso639_1_match_statement(&ids, &tag1s)
    )
    .unwrap();
    writeln!(
        &mut f,
        "fn from_autonym(autonym: &str) -> Result<Self, Err>{{ {} }}",
        from_autonym_match_statement(&ids, &autonyms)
    )
    .unwrap();

    writeln!(&mut f, "}}").unwrap();
}

fn match_statement(
    matching: &str,
    keys: &[String],
    values: &[String],
    default: Option<&str>,
) -> String {
    let mut s = String::new();

    s.push_str(&format!("match {} {{", matching));

    // This map avoid duplicate keys in match statements
    // As a result, for patterns with collisions, the first (in this case, alphabetically) value will be the one that gets used
    let mut used = HashMap::new();

    for (i, key) in keys.iter().enumerate() {
        if !used.contains_key(key) {
            used.insert(key, true);
            let m = format!("{} => {}, \n", key, values[i],);
            s.push_str(&m);
        }
    }

    if let Some(v) = default {
        s.push_str(&format!("_ => {}", v))
    }

    s.push('}');
    s
}

fn formatted_ids(ids: &[String]) -> Vec<String> {
    ids.iter()
        .map(|i| format!("Iso639::{}", i))
        .collect::<Vec<String>>()
        .to_vec()
}

fn with_surrounding_quotes(s: &str) -> String {
    format!("\"{}\"", s)
}

fn surrounded_in_quotes(arr: &[String]) -> Vec<String> {
    arr.iter()
        .map(|v| match &**v {
            "None" => v.clone(),
            _ => with_surrounding_quotes(v),
        })
        .collect::<Vec<String>>()
        .to_vec()
}

fn none_mapped_to_literal(arr: &[Option<String>]) -> Vec<String> {
    arr.iter()
        .map(|v| match v {
            Some(s) => s.clone(),
            None => "None".to_string(),
        })
        .collect::<Vec<String>>()
        .to_vec()
}

fn wrapped_in_literals(arr: &[String], literal: &str) -> Vec<String> {
    arr.iter()
        .map(|v| match &**v {
            "None" => v.clone(),
            _ => format!("{}({})", literal, v),
        })
        .collect::<Vec<String>>()
        .to_vec()
}

fn name_match_statement(ids: &[String], names: &[String]) -> String {
    match_statement(
        "self",
        &formatted_ids(ids),
        &surrounded_in_quotes(names),
        None,
    )
}

fn iso639_3_match_statement(ids: &[String], tag3s: &[String]) -> String {
    match_statement(
        "self",
        &formatted_ids(ids),
        &surrounded_in_quotes(tag3s),
        None,
    )
}

fn iso639_1_match_statement(ids: &[String], tag1s: &[Option<String>]) -> String {
    match_statement(
        "self",
        &formatted_ids(ids),
        &wrapped_in_literals(
            &surrounded_in_quotes(&none_mapped_to_literal(tag1s)),
            "Some",
        ),
        None,
    )
}

fn autonym_match_statement(ids: &[String], autonyms: &[Option<String>]) -> String {
    match_statement(
        "self",
        &formatted_ids(ids),
        &wrapped_in_literals(
            &surrounded_in_quotes(&none_mapped_to_literal(autonyms)),
            "Some",
        ),
        None,
    )
}

fn from_name_match_statement(ids: &[String], names: &[String]) -> String {
    match_statement(
        "name",
        &surrounded_in_quotes(names),
        &wrapped_in_literals(&formatted_ids(ids), "Ok"),
        Some("Err(Err::UnknownName)"),
    )
}

fn from_iso639_3_match_statement(ids: &[String], tag3s: &[String]) -> String {
    match_statement(
        "code",
        &surrounded_in_quotes(tag3s),
        &wrapped_in_literals(&formatted_ids(ids), "Ok"),
        Some("Err(Err::UnknownLanguage)"),
    )
}

fn from_iso639_1_match_statement(ids: &[String], tag1s: &[Option<String>]) -> String {
    let mut filtered_tag1s: Vec<String> = vec![];
    let mut filtered_ids: Vec<String> = vec![];

    for (i, tag) in tag1s.iter().enumerate() {
        match tag {
            Some(s) => {
                filtered_tag1s.push(s.to_string());
                filtered_ids.push(ids[i].clone())
            }
            None => (),
        }
    }

    match_statement(
        "code",
        &surrounded_in_quotes(&filtered_tag1s),
        &wrapped_in_literals(&formatted_ids(&filtered_ids), "Ok"),
        Some("Err(Err::UnknownIso639_1)"),
    )
}

fn from_autonym_match_statement(ids: &[String], autonyms: &[Option<String>]) -> String {
    let mut filtered_autonyms: Vec<String> = vec![];
    let mut filtered_ids: Vec<String> = vec![];

    for (i, autonym) in autonyms.iter().enumerate() {
        match autonym {
            Some(s) => {
                filtered_autonyms.push(s.to_string());
                filtered_ids.push(ids[i].clone())
            }
            None => (),
        }
    }

    match_statement(
        "autonym",
        &surrounded_in_quotes(&filtered_autonyms),
        &wrapped_in_literals(&formatted_ids(&filtered_ids), "Ok"),
        Some("Err(Err::UnknownAutonym)"),
    )
}

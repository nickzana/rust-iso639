use std::fmt::Display;

#[derive(Debug)]
pub enum Err {
    // invalid iso639_3 code provided
    UnknownLanguage(String),
    // invalid language name provided
    UnknownName(String),
    // invalid iso639_1 code provided
    UnknownIso639_1(String),
    // invalid autonym provided
    UnknownAutonym(String),
}

impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            match self {
                Err::UnknownLanguage(_) => "Unknown ISO639_3 Language code provided",
                Err::UnknownName(_) => "Unknown Language name provided",
                Err::UnknownIso639_1(_) => "Unknown ISO639_1 Language code provided",
                Err::UnknownAutonym(_) => "Unknown Autonym provided",
            },
            *self
        )
    }
}

pub trait IsoCompat
where
    Self: std::marker::Sized,
{
    fn name(&self) -> &str;
    fn iso639_3(&self) -> &str;
    fn iso639_1(&self) -> Option<&str>;
    fn autonym(&self) -> Option<&str>;

    fn from_name(name: &str) -> Result<Self, Err>;
    fn from_iso639_3(code: &str) -> Result<Self, Err>;
    fn from_iso639_1(code: &str) -> Result<Self, Err>;
    fn from_autonym(autonym: &str) -> Result<Self, Err>;
}

pub enum Err {
    UnknownLanguage,
    UnknownName,
    UnknownIso639_1,
    UnknownAutonym,
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

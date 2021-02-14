pub trait IsoCompat where Self: std::marker::Sized {
    fn name(&self) -> &str;
    fn iso639_3(&self) -> &str;
    fn iso639_1(&self) -> Option<&str>;
    fn autonym(&self) -> Option<&str>;

    fn from_name(name: &str) -> Option<Self>;
    fn from_iso639_3(code: &str) -> Option<Self>;
    fn from_iso639_1(code: &str) -> Option<Self>;
    fn from_autonym(autonym: &str) -> Option<Self>;
}

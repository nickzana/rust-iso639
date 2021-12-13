extern crate anyhow;

mod iso_639;
mod iso_compat;
mod localized_string;

pub use iso_639::Iso639 as Language;
pub use iso_compat::{Err, IsoCompat};
pub use localized_string::*;

#[cfg(test)]
mod tests {
    use super::{IsoCompat, Language};

    #[test]
    fn test_get_properties() {
        assert_eq!(Language::Eng.name(), "English");
        assert_eq!(Language::Eng.iso639_3(), "eng");
        assert_eq!(Language::Eng.iso639_1(), Some("en"));
        assert_eq!(Language::Eng.autonym(), Some("English"));
    }

    #[test]
    fn test_get_from_properties() {
        assert_eq!(Language::from_name("English").ok(), Some(Language::Eng));
        assert_eq!(Language::from_iso639_3("eng").ok(), Some(Language::Eng));
        assert_eq!(Language::from_iso639_1("en").ok(), Some(Language::Eng));
        assert_eq!(Language::from_autonym("English").ok(), Some(Language::Eng));
    }
}

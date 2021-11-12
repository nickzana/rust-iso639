extern crate anyhow;

mod iso_639;
mod iso_compat;
mod localized_string;

pub use iso_639::Iso639 as Language;
pub use localized_string::*;

#[cfg(test)]
mod tests {
    use crate::iso_639::Iso639;
    use crate::iso_compat::IsoCompat;

    #[test]
    fn test_get_properties() {
        assert_eq!(Iso639::Eng.name(), "English");
        assert_eq!(Iso639::Eng.iso639_3(), "eng");
        assert_eq!(Iso639::Eng.iso639_1(), Some("en"));
        assert_eq!(Iso639::Eng.autonym(), Some("English"));
    }

    #[test]
    fn test_get_from_properties() {
        assert_eq!(Iso639::from_name("English").ok(), Some(Iso639::Eng));
        assert_eq!(Iso639::from_iso639_3("eng").ok(), Some(Iso639::Eng));
        assert_eq!(Iso639::from_iso639_1("en").ok(), Some(Iso639::Eng));
        assert_eq!(Iso639::from_autonym("English").ok(), Some(Iso639::Eng));
    }
}

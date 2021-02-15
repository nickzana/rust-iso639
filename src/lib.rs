extern crate anyhow;

mod iso_639;
mod iso_compat;

pub use iso_639::Iso639;
pub use iso_639::Iso639::*;
pub use iso_compat::IsoCompat;

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
        assert_eq!(Iso639::from_name("English"), Some(Iso639::Eng));
        assert_eq!(Iso639::from_iso639_3("eng"), Some(Iso639::Eng));
        assert_eq!(Iso639::from_iso639_1("en"), Some(Iso639::Eng));
        assert_eq!(Iso639::from_autonym("English"), Some(Iso639::Eng));
    }
}

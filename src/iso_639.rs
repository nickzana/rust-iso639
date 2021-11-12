use crate::iso_compat::{Err, IsoCompat};

include!(concat!(env!("OUT_DIR"), "/iso_639.rs"));

impl IsoCompat for Iso639 {
    fn name(&self) -> &str {
        TO_NAME[&(*self as i32)]
    }

    fn iso639_3(&self) -> &str {
        TO_ISO639_3[&(*self as i32)]
    }

    fn iso639_1(&self) -> Option<&str> {
        Some(*(TO_ISO639_1.get(&(*self as i32))?))
    }

    fn autonym(&self) -> Option<&str> {
        Some(*(TO_AUTONYM.get(&(*self as i32))?))
    }

    fn from_name(name: &str) -> Result<Self, Err> {
        FROM_NAMES
            .get(name)
            .map_or(Err(Err::UnknownName(name.to_string())), |s| Ok(*s))
    }

    fn from_iso639_3(code: &str) -> Result<Self, Err> {
        FROM_ISO639_3
            .get(code)
            .map_or(Err(Err::UnknownLanguage(code.to_string())), |s| Ok(*s))
    }

    fn from_iso639_1(code: &str) -> Result<Self, Err> {
        FROM_ISO639_1
            .get(code)
            .map_or(Err(Err::UnknownIso639_1(code.to_string())), |s| Ok(*s))
    }

    fn from_autonym(autonym: &str) -> Result<Self, Err> {
        FROM_AUTONYM
            .get(autonym)
            .map_or(Err(Err::UnknownAutonym(autonym.to_string())), |s| Ok(*s))
    }
}

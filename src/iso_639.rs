use crate::iso_compat::{Err as IsoErr, IsoCompat};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

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

    fn from_name(name: &str) -> Result<Self, IsoErr> {
        FROM_NAMES
            .get(name)
            .map_or(Err(IsoErr::UnknownName(name.to_string())), |s| Ok(*s))
    }

    fn from_iso639_3(code: &str) -> Result<Self, IsoErr> {
        FROM_ISO639_3.get(code).map_or(
            Err(IsoErr::UnknownLanguage(code.to_ascii_lowercase())),
            |s| Ok(*s),
        )
    }

    fn from_iso639_1(code: &str) -> Result<Self, IsoErr> {
        FROM_ISO639_1.get(code).map_or(
            Err(IsoErr::UnknownIso639_1(code.to_ascii_lowercase())),
            |s| Ok(*s),
        )
    }

    fn from_autonym(autonym: &str) -> Result<Self, IsoErr> {
        FROM_AUTONYM
            .get(autonym)
            .map_or(Err(IsoErr::UnknownAutonym(autonym.to_string())), |s| Ok(*s))
    }
}

impl FromStr for Iso639 {
    type Err = IsoErr;

    fn from_str(s: &str) -> Result<Self, IsoErr> {
        Iso639::from_iso639_3(s)
    }
}

impl Serialize for Iso639 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.iso639_3())
    }
}

impl<'de> Deserialize<'de> for Iso639 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Iso639::from_str(&s).map_err(serde::de::Error::custom)
    }
}

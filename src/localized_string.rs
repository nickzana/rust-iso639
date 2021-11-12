use crate::Language;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct String {
    // None indicates that the language is unknown
    pub language: Option<Language>,
    pub title: std::string::String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct LocalizedString {
    // The title in the language in which the media was produced
    pub local: Option<String>,
    pub other: Vec<String>,
}

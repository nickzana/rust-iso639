## iso639_enum

An ISO639 Language Code implementation developed for the rms-metadata library.

# Usage:

``` rust
assert_eq!(Language::Eng.name(), "English");
assert_eq!(Language::Eng.iso639_3(), "eng");
assert_eq!(Language::Eng.iso639_1(), Some("en"));
assert_eq!(Language::Eng.autonym(), Some("English"));


assert_eq!(Language::from_name("English").ok(), Some(Language::Eng));
assert_eq!(Language::from_iso639_3("eng").ok(), Some(Language::Eng));
assert_eq!(Language::from_iso639_1("en").ok(), Some(Language::Eng));
assert_eq!(Language::from_autonym("English").ok(), Some(Language::Eng));
```

# Data

The data is sourced from [https://github.com/bbqsrc/iso639-databases].

use inflector::cases::camelcase::*;
use inflector::cases::kebabcase::*;
use inflector::cases::pascalcase::*;
use inflector::cases::screamingsnakecase::*;
use inflector::cases::snakecase::*;
use inflector::cases::traincase::*;

/// A replacement Query
pub enum Query {
    /// Substitute `old` with `new`
    Substring(String, String),
    /// Replace the parts matching the regex with `replacement`
    Regex(regex::Regex, String),
    /// Replace all instances of `pattern` with `replacement`, by
    /// using case conversion methods.
    /// This allows replacing FooBar with SpamEggs and foo_bar with spam_eggs
    /// using only one query
    Subvert(Vec<(String, String)>),
}

impl Query {
    /// Constructor for the Substring variant
    pub fn substring(old: &str, new: &str) -> Self {
        Self::Substring(old.to_string(), new.to_string())
    }

    /// Constructor for the Regex variant
    pub fn regex(re: regex::Regex, replacement: &str) -> Self {
        Self::Regex(re, replacement.to_string())
    }

    /// Constructor for the Subvert variant
    pub fn subvert(pattern: &str, replacement: &str) -> Self {
        fn to_ugly_case(input: &str) -> String {
            to_train_case(input).replace("-", "_")
        }

        let mut items = vec![];
        for func in &[
            to_camel_case,
            to_kebab_case,
            to_pascal_case,
            to_screaming_snake_case,
            to_snake_case,
            to_train_case,
            to_ugly_case,
        ] {
            items.push((func(pattern), func(replacement)));
        }
        Self::Subvert(items)
    }
}

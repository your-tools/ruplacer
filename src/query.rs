/// A replacement Query
pub enum Query {
    /// Substitute `old` with `new`
    Simple(String, String),
    /// Replace the parts matching the regex with `replacement`
    Regex(regex::Regex, String),
    /// Replace all instances of `pattern` with `replacement`, by
    /// using case conversion methods.
    /// This allows replacing FooBar with SpamEggs and foo_bar with spam_eggs
    /// using only one query
    PreserveCase(String, String),
}

impl Query {
    /// Constructor for the Substring variant
    pub fn simple(old: &str, new: &str) -> Self {
        Self::Simple(old.to_string(), new.to_string())
    }

    /// Constructor for the Regex variant
    pub fn regex(re: regex::Regex, replacement: &str) -> Self {
        Self::Regex(re, replacement.to_string())
    }

    /// Constructor for the PreserveCase variant
    pub fn preserve_case(pattern: &str, replacement: &str) -> Self {
        Self::PreserveCase(pattern.to_string(), replacement.to_string())
    }
}

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
    Subvert(String, String),
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
        Self::Subvert(pattern.to_string(), replacement.to_string())
    }
}

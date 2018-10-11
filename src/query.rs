use regex;

pub enum Query {
    Substring(String, String),
    Regex(regex::Regex, String),
}

pub fn substring(old: &str, new: &str) -> Query {
    Query::Substring(old.to_string(), new.to_string())
}

pub fn from_regex(re: regex::Regex, replacement: &str) -> Query {
    Query::Regex(re, replacement.to_string())
}

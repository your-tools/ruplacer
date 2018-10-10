pub enum Query {
    Substring(String, String),
    Regex(String, String),
}

pub fn substring(old: &str, new: &str) -> Query {
    Query::Substring(old.to_string(), new.to_string())
}

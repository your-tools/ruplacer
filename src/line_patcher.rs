use query::Query;

pub struct LinePatcher {
    input: String,
}

impl LinePatcher {
    pub fn new(input: &str) -> LinePatcher {
        LinePatcher {
            input: input.to_string(),
        }
    }

    pub fn replace(&self, query: &Query) -> String {
        match query {
            Query::Substring(old, new) => self.input.replace(old, new),
            _ => self.input.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use query;

    #[test]
    fn test_substring_replace() {
        let input = "this is old, everything is old!";
        let actual = LinePatcher::new(input).replace(&query::substring("old", "new"));
        assert_eq!(actual, "this is new, everything is new!");
    }
}

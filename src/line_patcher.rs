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
            Query::Regex(re, replacement) => {
                let res = re.replace_all(&self.input, replacement as &str);
                res.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use query;
    use regex;

    #[test]
    fn test_substring() {
        let input = "this is old, everything is old!";
        let actual = LinePatcher::new(input).replace(&query::substring("old", "new"));
        assert_eq!(actual, "this is new, everything is new!");
    }

    #[test]
    fn test_regex() {
        let re = regex::Regex::new(r"(\w+) (\w+)").unwrap();
        let re_query = query::from_regex(re, r"$2 $1");
        let input = "first second";
        let actual = LinePatcher::new(input).replace(&re_query);
        assert_eq!(actual, "second first");
    }
}

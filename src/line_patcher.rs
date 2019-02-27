use inflector::cases::camelcase::*;
use inflector::cases::kebabcase::*;
use inflector::cases::pascalcase::*;
use inflector::cases::screamingsnakecase::*;
use inflector::cases::snakecase::*;
use crate::query::Query;

pub struct LinePatcher {
    input: String,
}

fn subvert_line(input: &str, pattern: &str, replacement: &str) -> String {
    let res = input.replace(&to_camel_case(pattern), &to_camel_case(replacement));
    let res = res.replace(&to_pascal_case(pattern), &to_pascal_case(replacement));
    let res = res.replace(&to_snake_case(pattern), &to_snake_case(replacement));
    let res = res.replace(&to_kebab_case(pattern), &to_kebab_case(replacement));
    let res = res.replace(
        &to_screaming_snake_case(pattern),
        &to_screaming_snake_case(replacement),
    );
    res.to_string()
}

impl LinePatcher {
    pub fn new(input: &str) -> LinePatcher {
        LinePatcher {
            input: input.to_string(),
        }
    }

    pub fn replace(&self, query: &Query) -> String {
        match query {
            Query::Substring(old, new) => {
                self.input.replace(old, new)
            }
            Query::Regex(re, replacement) => {
                re.replace_all(&self.input, replacement as &str).to_string()
            }
            Query::Subvert(pattern, replacement) => {
                subvert_line(&self.input, pattern, replacement)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query;
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

    #[test]
    fn test_subvert_happy() {
        let input = "foo_bar, FooBar, FOO_BAR and foo-bar";
        let query = query::subvert("foo_bar", "spam_eggs");
        let actual = LinePatcher::new(input).replace(&query);
        assert_eq!(actual, "spam_eggs, SpamEggs, SPAM_EGGS and spam-eggs");
    }

    #[test]
    fn test_subvert_inconsistent() {
        let input = "foo_bar, FooBar, FOO_BAR and foo-bar";
        let query = query::subvert("foo_bar", "SpamEggs");
        let actual = LinePatcher::new(input).replace(&query);
        assert_eq!(actual, "spam_eggs, SpamEggs, SPAM_EGGS and spam-eggs");
    }
}

use std::collections::HashMap;

use colored::*;
use inflector::cases::camelcase::*;
use inflector::cases::kebabcase::*;
use inflector::cases::pascalcase::*;
use inflector::cases::screamingsnakecase::*;
use inflector::cases::snakecase::*;
use regex::Regex;
use regex::Replacer;

use crate::query::Query;

/// Main entry point: take a line and a query, return a replacements map and the
/// new line
///
/// You can use replacements.print_self() later on to display the patch (for
/// instance)
pub fn patch_line(input: &str, query: &Query) -> (Replacements, String) {
    let replacements = match query {
        Query::Substring(old, new) => get_replacements_substring(&input, old, new),
        Query::Regex(regex, new) => get_replacements_regex(&input, regex, new),
        Query::Subvert(old, new) => get_replacements_subvert(&input, old, new),
    };
    let output = replacements.apply(input);
    (replacements, output)
}

#[derive(Debug)]
pub struct ReplaceValue {
    // Note: start_pos is stored inside a HashMap
    end_pos: usize,
    new: String,
}

#[derive(Debug)]
pub struct Replacements {
    values: HashMap<usize, ReplaceValue>,
}

impl Replacements {
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn apply(&self, input: &str) -> String {
        let chars: Vec<_> = input.chars().collect();
        let mut i = 0;
        let mut res = String::new();
        while i < input.len() {
            if let Some(ReplaceValue { end_pos, new, .. }) = self.values.get(&i) {
                res.push_str(new);
                i = *end_pos;
            } else {
                let c = chars[i];
                res.push(c);
                i += 1;
            }
        }
        res
    }

    pub fn print_self(&self, input: &str) {
        print!("{}", "-- ".red());
        let chars: Vec<_> = input.chars().collect();
        let mut i = 0;
        while i < input.len() {
            if let Some(ReplaceValue { end_pos, .. }) = self.values.get(&i) {
                let old = &input[i..*end_pos];
                print!("{}", &old.red().underline());
                i = *end_pos;
            } else {
                let c = chars[i];
                print!("{}", c);
                i += 1;
            }
        }
        println!();

        print!("{}", "++ ".green());
        let chars: Vec<_> = input.chars().collect();
        let mut i = 0;
        while i < input.len() {
            if let Some(ReplaceValue { end_pos, new, .. }) = self.values.get(&i) {
                print!("{}", new.green().underline());
                i = *end_pos;
            } else {
                let c = chars[i];
                print!("{}", c);
                i += 1;
            }
        }
        println!();
    }
}

fn get_replacements_substring(input: &str, old: &str, new: &str) -> Replacements {
    let mut values = HashMap::new();
    for m in input.match_indices(&old) {
        let (start, _) = m;
        values.insert(
            start,
            ReplaceValue {
                end_pos: start + old.len(),
                new: new.to_string(),
            },
        );
    }

    Replacements { values }
}

fn get_replacements_regex(input: &str, regex: &Regex, replacement: &str) -> Replacements {
    // Replacer trait uses &mut, so we need to build a mutable ref from the
    // query
    let mut s = String::from(replacement);
    let mut replacement: &str = &mut s;
    let mut values = HashMap::new();

    // Implementation taken from Regex::replacen source code:
    if let Some(rep) = replacement.no_expansion() {
        let matches = regex.find_iter(input);
        for m in matches {
            let start = m.start();
            let old = &m.as_str();
            values.insert(
                start,
                ReplaceValue {
                    end_pos: start + old.len(),
                    new: rep.to_string(),
                },
            );
        }
    } else {
        let captures = regex.captures_iter(input);
        for capture in captures {
            let m = capture.get(0).unwrap();
            let start = m.start();
            let end = m.end();
            // unwrap on 0 is OK because captures only reports matches
            let mut new = String::new();
            replacement.replace_append(&capture, &mut new);
            values.insert(start, ReplaceValue { end_pos: end, new });
        }
    }

    Replacements { values }
}

fn get_replacements_subvert(input: &str, old: &str, new: &str) -> Replacements {
    let mut values = HashMap::new();
    let camel = get_replacements_substring(input, &to_camel_case(old), &to_camel_case(new));
    let kebab = get_replacements_substring(input, &to_kebab_case(old), &to_kebab_case(new));
    let pascal = get_replacements_substring(input, &to_pascal_case(old), &to_pascal_case(new));
    let snake = get_replacements_substring(input, &to_snake_case(old), &to_snake_case(new));
    let scream = get_replacements_substring(
        input,
        &to_screaming_snake_case(old),
        &to_screaming_snake_case(new),
    );
    values.extend(camel.values);
    values.extend(kebab.values);
    values.extend(pascal.values);
    values.extend(snake.values);
    values.extend(scream.values);
    Replacements { values }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_replacements_substring() {
        let input = "this is an old string";
        let actual = get_replacements_substring(input, "old", "new");
        let actual_values = actual.values;
        assert_eq!(actual_values.len(), 1);
        let actual_replacement = &actual_values[&11];
        assert_eq!(actual_replacement.end_pos, 14);
        assert_eq!(actual_replacement.new, "new");
    }

    #[test]
    fn test_get_replacements_basic_regex() {
        let re = Regex::new("ba(r|z)").unwrap();
        let input = "bar and baz";
        let actual = get_replacements_regex(input, &re, "bof");
        let actual_values = actual.values;
        assert_eq!(actual_values.len(), 2);
        let first = &actual_values[&0];
        assert_eq!(first.end_pos, 3);
        assert_eq!(first.new, "bof");

        let second = &actual_values[&8];
        assert_eq!(second.end_pos, 11);
        assert_eq!(second.new, "bof");
    }

    #[test]
    fn test_get_replacements_regex_with_patterns_one_match() {
        let re = Regex::new(r"(\w+), (\w+)").unwrap();
        let input = "Last, First";

        let actual_values = get_replacements_regex(input, &re, "$2 $1").values;
        let actual_new = &actual_values[&0].new;
        assert_eq!(actual_new, "First Last");
    }

    #[test]
    fn test_get_replacements_regex_with_patterns_several_matches() {
        let re = Regex::new(r"ba(r|z)").unwrap();
        let input = "bar, baz and bat";
        let actual_replacements = get_replacements_regex(input, &re, "BABA$1");
        let actual_output = actual_replacements.apply(input);
        assert_eq!(actual_output, "BABAr, BABAz and bat");
    }

    #[test]
    fn test_get_replacements_subvert() {
        let input = "foo_bar and FooBar";
        let actual_replacements = get_replacements_subvert(input, "foo_bar", "spam_eggs");
        let actual_output = actual_replacements.apply(input);
        assert_eq!(actual_output, "spam_eggs and SpamEggs");
    }
}

use term;
use regex::Regex;
use difference::{Changeset, Difference};

mod file_replacer;

pub use self::file_replacer::FileReplacer;

pub struct Replacer {
    re: Regex,
    replacement: Option<String>,
}

#[allow(unused_must_use)]
fn term_show_diff(before: &str, after: &str) {
   // Compare both texts, the third parameter defines the split level.
    let Changeset { diffs, .. } = Changeset::new(before, after, "\n");

    let mut t = term::stdout().unwrap();

    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref _x) => {
                t.reset().unwrap();
                writeln!(t, "");
            }
            Difference::Add(ref x) => {
                t.fg(term::color::GREEN).unwrap();
                writeln!(t, "+{}", x);
            }
            Difference::Rem(ref x) => {
                t.fg(term::color::RED).unwrap();
                writeln!(t, "-{}", x);
            }
        }
    }
    t.reset().unwrap();
    t.flush().unwrap();
}

impl Replacer {
    pub fn new(pattern: &str, replacement: Option<String>) -> Replacer {
        let re = Regex::new(pattern).expect("Invalid regular expression");
        Replacer {
            re: re,
            replacement: replacement,
        }
    }

    pub fn grep_or_replace(&self, buf: &str) {
        match self.replacement {
            None => self.grep(buf),
            Some(_) => {self.replace(buf);},
        };
    }

    pub fn grep(&self, _buf: &str) {
        println!("Grep not implemented yet");
        // for m in greper.iter(data.as_bytes()) {
        //     println!("match: {} - {}", m.start(), m.end())
        // }
    }

    pub fn replace(&self, buf: &str) -> String {
        let replacement = self.replacement.as_ref().unwrap();
        let after = self.re.replace_all(buf, replacement.as_str());

        if buf != after {
            term_show_diff(buf, &after)
        }
        after.to_string()
    }
}


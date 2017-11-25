use term;
use regex::Regex;
use difference::{Changeset, Difference};
use std::io::{Result as IoResult};

mod file_replacer;

pub use self::file_replacer::FileReplacer;

pub struct Replacer {
    re: Regex,
    replacement: Option<String>,
}

fn term_show_diff(before: &str, after: &str) -> IoResult<()> {
   // Compare both texts, the third parameter defines the split level.
    let Changeset { diffs, .. } = Changeset::new(before, after, "\n");

    let mut t = term::stdout().unwrap();

    for diff in diffs {
        match diff {
            Difference::Same(ref _x) => {
                t.reset().unwrap();
                writeln!(t, "")?;
            }
            Difference::Add(ref x) => {
                t.fg(term::color::GREEN).unwrap();
                writeln!(t, "+{}", x.trim())?;
            }
            Difference::Rem(ref x) => {
                t.fg(term::color::RED).unwrap();
                writeln!(t, "-{}", x.trim())?;
            }
        }
    }
    t.reset().unwrap();
    t.flush().unwrap();
    Ok(())
}

impl Replacer {
    pub fn new(pattern: &str, replacement: Option<String>) -> Replacer {
        let re = Regex::new(pattern).expect("Invalid regular expression");
        Replacer {
            re: re,
            replacement: replacement,
        }
    }

    pub fn grep(&self, _buf: &str) -> IoResult<()> {
        println!("Grep not implemented yet");
        Ok(())
        // for m in greper.iter(data.as_bytes()) {
        //     println!("match: {} - {}", m.start(), m.end())
        // }
    }

    pub fn replace(&self, buf: &str) -> IoResult<String> {
        let replacement = self.replacement.as_ref().unwrap();
        let after = self.re.replace_all(buf, replacement.as_str());

        if buf != after {
            term_show_diff(buf, &after)?;
        }
        Ok(after.to_string())
    }
}


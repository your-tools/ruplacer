use term;
use regex::Regex;
use difference::{Changeset, Difference};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Result as IoResult, Error as IoError, ErrorKind};

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

pub trait FileReplacer {
    fn replace_in_file(&self, file_name: &Path, dry_run: bool) -> IoResult<()>;
}

impl FileReplacer for Replacer {
    fn replace_in_file(&self, file_path: &Path, dry_run: bool) -> IoResult<()> {
        if !file_path.is_file() {
            return Err(IoError::from(ErrorKind::InvalidInput))
        }
        println!("{}", file_path.display());

        let mut data = String::new();
        {
            let mut f = File::open(file_path).expect("file not found");
            f.read_to_string(&mut data).expect("error reading file");
        }
        let new_data = self.replace(data.as_str());
        if !dry_run {
            println!("TODO: implement replace");
            // Recreate the file and dump the processed contents to it
            let mut dst = File::create(&file_path).expect("error opening the file for writing");
            dst.write(new_data.as_bytes()).expect("error writing to file");
        }
        Ok(())
    }
}

impl Replacer {
    pub fn new(pattern: &str, replacement: Option<String>) -> Replacer {
        let re = Regex::new(pattern).expect("invalid regular expression");
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


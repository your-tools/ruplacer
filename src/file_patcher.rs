use anyhow::{Context, Result};
use colored::*;
use difference::{Changeset, Difference};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::line_patcher::LinePatcher;
use crate::query::Query;

pub struct FilePatcher {
    replacements: Vec<Replacement>,
    path: PathBuf,
    new_contents: String,
}

impl FilePatcher {
    pub fn new(path: &Path, query: &Query) -> Result<Option<FilePatcher>> {
        let mut replacements = vec![];
        let file =
            File::open(&path).with_context(|| format!("Could not open {}", path.display()))?;
        let reader = BufReader::new(file);
        let mut new_contents = String::new();
        // Note: not using lines() because we need to preserve the line endings
        // when writing the file later on
        for (num, chunk) in reader.split(b'\n').enumerate() {
            let chunk = chunk.with_context(|| format!("Error while reading {}", path.display()))?;
            let line = String::from_utf8(chunk);
            if line.is_err() {
                return Ok(None);
            }
            let line = line.unwrap();
            let line_patcher = LinePatcher::new(&line);
            let new_line = line_patcher.replace(&query);
            if new_line != line {
                let replacement = Replacement {
                    line_no: num + 1,
                    old: line,
                    new: new_line.clone(),
                };
                replacements.push(replacement);
                new_contents.push_str(&new_line);
            } else {
                new_contents.push_str(&line);
            }
            new_contents.push('\n');
        }
        Ok(Some(FilePatcher {
            replacements,
            path: path.to_path_buf(),
            new_contents,
        }))
    }

    pub fn replacements(&self) -> &Vec<Replacement> {
        &self.replacements
    }

    pub fn run(&self) -> Result<()> {
        std::fs::write(&self.path, &self.new_contents)
            .with_context(|| format!("Could not write {}", self.path.display()))?;
        Ok(())
    }

    pub fn print_patch(&self) {
        println!(
            "{} {}",
            "Patching".blue(),
            self.path.to_string_lossy().bold()
        );
        for replacement in &self.replacements {
            replacement.print_self();
            println!();
        }
        println!();
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Replacement {
    line_no: usize,
    old: String,
    new: String,
}

impl Replacement {
    fn print_self(&self) {
        let changeset = Changeset::new(&self.old, &self.new, "");
        print!("{} ", "--".red());
        for diff in &changeset.diffs {
            match diff {
                Difference::Same(s) => print!("{}", s),
                Difference::Rem(s) => print!("{}", s.red().underline()),
                _ => (),
            }
        }
        println!();
        print!("{} ", "++".green());
        for diff in &changeset.diffs {
            match diff {
                Difference::Same(s) => print!("{}", s),
                Difference::Add(s) => print!("{}", s.green().underline()),
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query;
    use std::fs;

    #[test]
    fn test_compute_replacements() {
        let top_path = std::path::Path::new("tests/data/top.txt");
        let file_patcher = FilePatcher::new(&top_path, &query::substring("old", "new"))
            .unwrap()
            .unwrap();
        let replacements = file_patcher.replacements();
        assert_eq!(replacements.len(), 1);
        let actual_replacement = &replacements[0];
        assert_eq!(actual_replacement.line_no, 2);
        // ruplacer preserves line endings: on Windows, there is a
        // possibility the actual lines contain \r, depending
        // of the git configuration.
        // So strip the \r before comparing them to the expected result.
        let actual_new = actual_replacement.new.replace("\r", "");
        let actual_old = actual_replacement.old.replace("\r", "");
        assert_eq!(actual_new, "Top: new is nice");
        assert_eq!(actual_old, "Top: old is nice");
    }

    #[test]
    fn test_patch_file() {
        let temp_dir = tempdir::TempDir::new("test-ruplacer").unwrap();
        let file_path = temp_dir.path().join("foo.txt");
        fs::write(&file_path, "first line\nI say: old is nice\nlast line\n").unwrap();
        let file_patcher = FilePatcher::new(&file_path, &query::substring("old", "new")).unwrap();
        file_patcher.unwrap().run().unwrap();
        let actual = fs::read_to_string(&file_path).unwrap();
        let expected = "first line\nI say: new is nice\nlast line\n";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_replacement_display() {
        // This test cannot fail. It's just here so you can tweak the look and feel
        // of ruplacer easily.
        let replacement = Replacement {
            line_no: 1,
            old: "trustchain_creation: 0".to_owned(),
            new: "blockchain_creation: 0".to_owned(),
        };
        replacement.print_self();
    }
}

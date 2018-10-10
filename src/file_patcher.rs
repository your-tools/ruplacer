use colored::*;
use difference::{Changeset, Difference};
use std;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use line_patcher;

pub struct FilePatcher {
    replacements: Vec<Replacement>,
    path: PathBuf,
    new_contents: String,
}

impl FilePatcher {
    pub fn new(
        path: PathBuf,
        pattern: &str,
        replacement: &str,
    ) -> Result<FilePatcher, std::io::Error> {
        let mut replacements = vec![];
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let mut new_contents = String::new();
        for (num, line) in reader.lines().enumerate() {
            let line = line?;
            let new_line = line_patcher::patch(&line, pattern, replacement);
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
        }
        Ok(FilePatcher {
            replacements,
            path,
            new_contents,
        })
    }

    pub fn replacements(&self) -> &Vec<Replacement> {
        &self.replacements
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        std::fs::write(&self.path, &self.new_contents)?;
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
            print!("\n");
        }
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
        print!("\n");
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
    use std::fs;

    #[test]
    fn test_compute_replacements() {
        let top_path = std::path::Path::new("tests/data/top.txt");
        let file_patcher = FilePatcher::new(top_path.to_path_buf(), "old", "new").unwrap();
        let replacements = file_patcher.replacements();
        assert_eq!(replacements.len(), 1);
        let actual_replacement = &replacements[0];
        assert_eq!(actual_replacement.line_no, 2);
        assert_eq!(actual_replacement.new, "Top: new is nice");
        assert_eq!(actual_replacement.old, "Top: old is nice");
    }

    #[test]
    fn test_patch_file() {
        let top_path = std::path::Path::new("tests/data/top.txt");
        let file_patcher = FilePatcher::new(top_path.to_path_buf(), "old", "new").unwrap();
        file_patcher.run().unwrap();
        let actual = fs::read_to_string(&top_path).unwrap();
        let expected = "first line\nTop: new is nice\nsecond line\n";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_replacement_display() {
        let replacement = Replacement {
            line_no: 1,
            old: "trustchain_creation: 0".to_owned(),
            new: "blockchain_creation: 0".to_owned(),
        };
        replacement.print_self();
    }
}

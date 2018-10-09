extern crate difference;
use errors::Error;
use ignore;
use std;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub struct Replacer {
    path: PathBuf,
    dry_run: bool,
}

impl Replacer {
    pub fn new(path: PathBuf) -> Replacer {
        Replacer {
            path,
            dry_run: false,
        }
    }

    pub fn replace(&self, pattern: &str, replacement: &str) -> Result<(), Error> {
        self.walk(pattern, replacement)?;
        Ok(())
    }

    pub fn dry_run(&mut self, dry_run: bool) {
        self.dry_run = dry_run
    }

    pub fn process_file(
        &self,
        entry: &Path,
        pattern: &str,
        replacement: &str,
    ) -> Result<(), Error> {
        let file_patcher = FilePatcher::new(entry.to_path_buf(), pattern, replacement)?;
        let replacements = file_patcher.replacements();
        if replacements.is_empty() {
            return Ok(());
        }
        file_patcher.print_patch();
        if !self.dry_run {
            file_patcher.run()?;
        }
        Ok(())
    }

    fn walk(&self, pattern: &str, replacement: &str) -> Result<(), Error> {
        for result in ignore::Walk::new(&self.path) {
            match result {
                Ok(entry) => {
                    if let Some(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            self.process_file(&entry.path(), pattern, replacement)?;
                        }
                    }
                }
                Err(err) => return Err(err.into()),
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Replacement {
    line_no: usize,
    old: String,
    new: String,
}

struct FilePatcher {
    replacements: Vec<Replacement>,
    path: PathBuf,
    new_contents: String,
}

impl FilePatcher {
    pub fn new(path: PathBuf, pattern: &str, replacement: &str) -> Result<FilePatcher, Error> {
        let mut replacements = vec![];
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let mut new_contents = String::new();
        for (num, line) in reader.lines().enumerate() {
            let line = line?;
            if line.contains(pattern) {
                let new_line = line.replace(pattern, replacement);
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

    pub fn run(&self) -> Result<(), Error> {
        std::fs::write(&self.path, &self.new_contents)?;
        Ok(())
    }

    pub fn print_patch(&self) {
        println!("Patching: {}", self.path.to_string_lossy());
        for replacement in &self.replacements {
            let Replacement { line_no, old, new } = replacement;
            let changeset = difference::Changeset::new(old, new, " ");
            println!("{} {}", line_no, changeset);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std;

    #[test]
    fn test_replacements() {
        let top_path = std::path::Path::new("tests/data/top.txt");
        let file_patcher = FilePatcher::new(top_path.to_path_buf(), "old", "new").unwrap();
        let replacements = file_patcher.replacements();
        assert_eq!(replacements.len(), 1);
        let actual_replacement = &replacements[0];
        assert_eq!(actual_replacement.line_no, 2);
        assert_eq!(actual_replacement.new, "Top: new is nice");
        assert_eq!(actual_replacement.old, "Top: old is nice");
    }

}

use anyhow::{Context, Result};
use colored::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::line_patcher::{patch_line, Replacements};
use crate::query::Query;

// Associate line number with the replacments
// for the given line
type Patch = HashMap<usize, Replacements>;

pub struct FilePatcher {
    patch: Patch,
    path: PathBuf,
    // Since computing the new contents from the patch is expensive, we do it
    // once and store the results here
    // This also means we read the file only once
    new_contents: String,
}

impl FilePatcher {
    pub fn new(path: &Path, query: &Query) -> Result<Option<FilePatcher>> {
        let mut patch = HashMap::new();
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
            let (replacements, new_line) = patch_line(&line, &query);
            if replacements.is_empty() {
                new_contents.push_str(&line);
            } else {
                new_contents.push_str(&new_line);
                patch.insert(num + 1, replacements);
            }
            new_contents.push('\n');
        }
        Ok(Some(FilePatcher {
            patch,
            path: path.to_path_buf(),
            new_contents,
        }))
    }

    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn run(&self) -> Result<()> {
        std::fs::write(&self.path, &self.new_contents)
            .with_context(|| format!("Could not write {}", self.path.display()))
    }

    pub fn print_patch(&self) {
        println!(
            "{} {}",
            "Patching".blue(),
            self.path.to_string_lossy().bold()
        );
        for (i, line) in self.new_contents.lines().enumerate() {
            let line_no = i + 1;
            if let Some(replacements) = self.patch.get(&line_no) {
                replacements.print_self(&line);
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query;
    use std::fs;

    #[test]
    fn test_patch_file() {
        let temp_dir = tempdir::TempDir::new("test-ruplacer").unwrap();
        let file_path = temp_dir.path().join("foo.txt");
        fs::write(&file_path, "first line\nI say: old is nice\nlast line\n").unwrap();
        let file_patcher = FilePatcher::new(&file_path, &query::substring("old", "new")).unwrap();
        file_patcher.unwrap().run().unwrap();

        let file_path = temp_dir.path().join("foo.txt");
        let actual = fs::read_to_string(file_path).unwrap();
        let expected = "first line\nI say: new is nice\nlast line\n";
        assert_eq!(actual, expected);
    }
}

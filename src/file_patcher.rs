use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::query::Query;
use crate::replace;
use crate::Console;

/// Run replacement query on a given file
///
/// Example, assuming the `data.txt` file contains 'This is my old car'
/// ```rust
/// use ruplacer::{Console, FilePatcher, Query};
/// use std::path::PathBuf;
///
/// # std::fs::write("data.txt", "This is my old car.").unwrap();
/// let file = PathBuf::from("data.txt");
/// let query = Query::substring("old", "new");
/// let console = Console::new();
/// let file_patcher = FilePatcher::new(&console, &file, &query).unwrap();
/// file_patcher.unwrap().run().unwrap();
///
/// let new_contents = std::fs::read_to_string("data.txt").unwrap();
/// assert_eq!(new_contents, "This is my new car.");
/// ```
pub struct FilePatcher {
    path: PathBuf,
    new_contents: String,
    num_replacements: usize,
    num_lines: usize,
}

impl FilePatcher {
    pub fn new(console: &Console, path: &Path, query: &Query) -> Result<Option<FilePatcher>> {
        let mut num_replacements = 0;
        let mut num_lines = 0;
        let file =
            File::open(&path).with_context(|| format!("Could not open {}", path.display()))?;
        let reader = BufReader::new(file);
        let mut new_contents = String::new();
        // Note: not using lines() because we need to preserve the line endings
        // when writing the file later on
        for (num, chunk) in LineIterator::new(b'\n', reader).enumerate() {
            let chunk = chunk.with_context(|| format!("Error while reading {}", path.display()))?;
            let line = std::str::from_utf8(&chunk);
            if line.is_err() {
                return Ok(None);
            }
            let line = line.unwrap();
            let replacement = replace(line, query);
            match replacement {
                None => new_contents.push_str(line),
                Some(replacement) => {
                    num_lines += 1;
                    num_replacements += replacement.num_fragments();
                    let lineno = num + 1;
                    let prefix = format!("{}:{} ", path.display(), lineno);
                    console.print_replacement(&prefix, &replacement);
                    let new_line = replacement.output();
                    new_contents.push_str(new_line);
                }
            }
        }
        Ok(Some(FilePatcher {
            path: path.to_path_buf(),
            new_contents,
            num_lines,
            num_replacements,
        }))
    }

    pub(crate) fn num_replacements(&self) -> usize {
        self.num_replacements
    }

    pub(crate) fn num_lines(&self) -> usize {
        self.num_lines
    }

    /// Write new contents to the file.
    pub fn run(&self) -> Result<()> {
        std::fs::write(&self.path, &self.new_contents)
            .with_context(|| format!("Could not write {}", self.path.display()))?;
        Ok(())
    }
}

/// `LineIterator` wraps `BufRead`'s `read_until` method in an iterator, thereby
/// preserving the delimiter in the yielded values.
struct LineIterator<T: BufRead> {
    delimiter: u8,
    reader: T,
}

impl<T: BufRead> LineIterator<T> {
    fn new(delimiter: u8, reader: T) -> Self {
        Self { delimiter, reader }
    }
}

impl<T: BufRead> Iterator for LineIterator<T> {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = Vec::new();
        match self.reader.read_until(self.delimiter, &mut buf) {
            Ok(0) => None,
            Ok(_) => Some(Ok(buf)),
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Query;
    use std::fs;
    use tempfile::TempDir;

    fn temp_dir() -> TempDir {
        tempfile::Builder::new()
            .prefix("test-ruplacer")
            .tempdir()
            .unwrap()
    }

    #[test]
    fn test_patch_file() {
        let temp_dir = temp_dir();

        let file_path = temp_dir.path().join("without-trailing-newline.txt");
        fs::write(&file_path, "first line\nI say: old is nice\nlast line").unwrap();
        let query = Query::substring("old", "new");
        let console = Console::new();
        let file_patcher = FilePatcher::new(&console, &file_path, &query).unwrap();
        file_patcher.unwrap().run().unwrap();
        let actual = fs::read_to_string(&file_path).unwrap();
        let expected = "first line\nI say: new is nice\nlast line";
        assert_eq!(actual, expected);

        let file_path = temp_dir.path().join("with-trailing-newline.txt");
        fs::write(&file_path, "first line\nI say: old is nice\nlast line\n").unwrap();
        let query = Query::substring("old", "new");
        let file_patcher = FilePatcher::new(&console, &file_path, &query).unwrap();
        file_patcher.unwrap().run().unwrap();
        let actual = fs::read_to_string(&file_path).unwrap();
        let expected = "first line\nI say: new is nice\nlast line\n";
        assert_eq!(actual, expected);
    }
}

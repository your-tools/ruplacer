use errors::Error;
use ignore;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Replacer {
    path: PathBuf,
}

impl Replacer {
    pub fn new(path: PathBuf) -> Replacer {
        Replacer { path }
    }

    pub fn replace(&self, pattern: &str, replacement: &str) -> Result<(), Error> {
        self.walk(pattern, replacement)?;
        Ok(())
    }

    pub fn process_file(
        &self,
        entry: &Path,
        pattern: &str,
        replacement: &str,
    ) -> Result<(), Error> {
        let contents = fs::read_to_string(entry)?;
        let contents = contents.replace(pattern, replacement);
        println!("Processing: {:?}", entry);
        fs::write(entry, &contents)?;

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

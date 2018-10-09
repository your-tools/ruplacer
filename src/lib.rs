extern crate ignore;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Replacer {
    path: PathBuf,
}

#[derive(Debug)]
pub struct Error {
    description: String,
}

impl Error {
    pub fn new(description: &str) -> Error {
        Error {
            description: String::from(description),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.description)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error {
            description: format!("I/O error: {}", error),
        }
    }
}

impl From<ignore::Error> for Error {
    fn from(error: ignore::Error) -> Error {
        Error {
            description: format!("Error when parsing .ignore files: {}", error),
        }
    }
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

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Replacer {
    path: PathBuf,
}

impl Replacer {
    pub fn new(path: PathBuf) -> Replacer {
        Replacer { path }
    }

    pub fn replace(&self, pattern: &str, replacement: &str) -> io::Result<()> {
        self.walk(pattern, replacement)?;
        Ok(())
    }

    pub fn process_file(&self, entry: &Path, pattern: &str, replacement: &str) -> io::Result<()> {
        let contents = fs::read_to_string(entry)?;
        let contents = contents.replace(pattern, replacement);
        println!("Processing: {:?}", entry);
        fs::write(entry, &contents)?;

        Ok(())
    }

    fn walk(&self, pattern: &str, replacement: &str) -> io::Result<()> {
        let mut subdirs: Vec<PathBuf> = vec![self.path.to_path_buf()];
        while !subdirs.is_empty() {
            let subdir = subdirs.pop().unwrap();
            for entry in fs::read_dir(subdir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    subdirs.push(path);
                } else {
                    self.process_file(&entry.path(), pattern, replacement)?;
                }
            }
        }
        Ok(())
    }
}

use ignore;

use errors::Error;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use file_patcher::FilePatcher;
use query::Query;

#[derive(Default)]
pub struct Stats {
    pub matching_files: usize,
    pub num_replacements: usize,
}

impl Stats {
    pub fn update(&mut self, num_replacements: usize) {
        self.matching_files +=1;
        self.num_replacements += num_replacements;
    }
}

pub struct DirectoryPatcher {
    path: PathBuf,
    dry_run: bool,
    stats: Stats,
}

impl DirectoryPatcher {
    pub fn new(path: PathBuf) -> DirectoryPatcher {
        let stats = Stats::default();
        DirectoryPatcher {
            path,
            dry_run: false,
            stats,
        }
    }

    pub fn patch(&mut self, query: Query) -> Result<(), Error> {
        self.walk(query)?;
        Ok(())
    }

    pub fn stats(self) -> Stats {
        self.stats
    }

    pub fn dry_run(&mut self, dry_run: bool) {
        self.dry_run = dry_run
    }

    pub fn patch_file(&mut self, entry: &Path, query: &Query) -> Result<(), Error> {
        let file_patcher = FilePatcher::new(entry.to_path_buf(), &query);
        if let Err(err) = &file_patcher {
            match err.kind() {
                // Just ignore binary or non-utf8 files
                ErrorKind::InvalidData => return Ok(()),
                _ => return Error::from_read_error(entry, err),
            }
        }
        let file_patcher = file_patcher.unwrap();
        let replacements = file_patcher.replacements();
        if replacements.is_empty() {
            return Ok(());
        }
        self.stats.update(replacements.len());
        file_patcher.print_patch();
        if self.dry_run {
            return Ok(());
        }
        if let Err(err) = file_patcher.run() {
            return Error::from_write_error(&entry, &err);
        }
        Ok(())
    }

    fn walk(&mut self, query: Query) -> Result<(), Error> {
        for result in ignore::Walk::new(&self.path) {
            match result {
                Ok(entry) => {
                    if let Some(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            self.patch_file(&entry.path(), &query)?;
                        }
                    }
                }
                Err(err) => return Err(err.into()),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
}

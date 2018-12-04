use ignore;

use errors::Error;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use file_patcher::FilePatcher;
use query::Query;
use stats::Stats;

pub struct DirectoryPatcher {
    path: PathBuf,
    dry_run: bool,
    stats: Stats,
    file_type: Option<String>,
}

impl DirectoryPatcher {
    pub fn new(path: PathBuf) -> DirectoryPatcher {
        let stats = Stats::default();
        DirectoryPatcher {
            path,
            dry_run: false,
            stats,
            file_type: None,
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

    pub fn file_type(&mut self, file_type: Option<String>) {
        self.file_type = file_type;
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

    fn build_walker(&self) -> Result<ignore::Walk, Error> {
        let mut types_builder = ignore::types::TypesBuilder::new();
        types_builder.add_defaults();
        if let Some(selected_type) = &self.file_type {
            types_builder.select(&selected_type);
        }
        let types_matcher = types_builder.build()?;
        let mut walk_builder = ignore::WalkBuilder::new(&self.path);
        walk_builder.types(types_matcher);
        Ok(walk_builder.build())
    }

    fn walk(&mut self, query: Query) -> Result<(), Error> {
        let walker = self.build_walker()?;
        for result in walker {
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
mod tests {}

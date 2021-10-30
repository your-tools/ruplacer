use anyhow::{Context, Result};
use std::path::Path;

use crate::file_patcher::FilePatcher;
use crate::query::Query;
use crate::settings::Settings;
use crate::stats::Stats;

#[derive(Debug)]
/// Used to run replacement query on every text file present in a given path
/// ```rust
/// use ruplacer::{DirectoryPatcher, Query, Settings, Stats};
/// use std::path::PathBuf;
///
/// let settings = Settings{
///     dry_run: true,
///     .. Default::default()
/// };
/// let path = PathBuf::from("tests/data");
/// let mut directory_patcher = DirectoryPatcher::new(&path, &settings);
///
/// let query = Query::substring("old", "new");
/// directory_patcher.run(&query).unwrap();
/// let stats = directory_patcher.stats();
/// println!("Found {} matching lines", stats.matching_lines());
/// ```
// Note: keep the dry_run: true in the doc test above or the integration test
// will fail ...
pub struct DirectoryPatcher<'a> {
    path: &'a Path,
    settings: &'a Settings,
    stats: Stats,
}

impl<'a> DirectoryPatcher<'a> {
    pub fn new(path: &'a Path, settings: &'a Settings) -> DirectoryPatcher<'a> {
        let stats = Stats::default();
        DirectoryPatcher {
            path,
            settings,
            stats,
        }
    }

    /// Run the given query on the selected files in self.path
    pub fn run(&mut self, query: &Query) -> Result<()> {
        let walker = self.build_walker()?;
        for entry in walker {
            let entry = entry.with_context(|| "Could not read directory entry")?;
            if let Some(file_type) = entry.file_type() {
                if file_type.is_file() {
                    self.patch_file(entry.path(), query)?;
                }
            }
        }
        Ok(())
    }

    pub fn stats(self) -> Stats {
        self.stats
    }

    pub(crate) fn patch_file(&mut self, entry: &Path, query: &Query) -> Result<()> {
        let file_patcher = FilePatcher::new(entry, query)?;
        let file_patcher = match file_patcher {
            None => return Ok(()),
            Some(f) => f,
        };
        let num_replacements = file_patcher.num_replacements();
        if num_replacements != 0 {
            println!();
        }
        let num_lines = file_patcher.num_lines();
        self.stats.update(num_lines, num_replacements);
        if self.settings.dry_run {
            return Ok(());
        }
        file_patcher.run()?;
        Ok(())
    }

    fn build_walker(&self) -> Result<ignore::Walk> {
        let mut types_builder = ignore::types::TypesBuilder::new();
        types_builder.add_defaults();
        let mut count: u32 = 0;
        for t in &self.settings.selected_file_types {
            // Check if filter is file type or glob pattern
            if t.contains('*') {
                let new_type = format!("type{}", count);
                // Note: .add(name, glob) only returns error with wrong name, hence unwrap()
                types_builder.add(&new_type, t).unwrap();
                types_builder.select(&new_type);
                count += 1;
            } else {
                types_builder.select(t);
            }
        }
        for t in &self.settings.ignored_file_types {
            // Check if filter is file type or glob pattern
            if t.contains('*') {
                let new_type = format!("type{}", count);
                // Note: .add(name, glob) only returns error with wrong name, hence unwrap()
                types_builder.add(&new_type, t).unwrap();
                types_builder.negate(&new_type);
                count += 1;
            } else {
                types_builder.negate(t);
            }
        }
        let types_matcher = types_builder.build()?;
        let mut walk_builder = ignore::WalkBuilder::new(&self.path);
        walk_builder.types(types_matcher);
        // Note: the walk_builder configures the "ignore" settings of the Walker,
        // hence the negations
        if self.settings.ignored {
            walk_builder.ignore(false);
        }
        if self.settings.hidden {
            walk_builder.hidden(false);
        }
        Ok(walk_builder.build())
    }
}

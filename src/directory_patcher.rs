use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::file_patcher::FilePatcher;
use crate::query::Query;
use crate::settings::Settings;
use crate::stats::Stats;

pub struct DirectoryPatcher {
    path: PathBuf,
    settings: Settings,
    stats: Stats,
}

impl DirectoryPatcher {
    pub fn new(path: PathBuf, settings: Settings) -> DirectoryPatcher {
        let stats = Stats::default();
        DirectoryPatcher {
            path,
            settings,
            stats,
        }
    }

    pub fn run(&mut self, query: &Query) -> Result<()> {
        let walker = self.build_walker();
        for entry in walker {
            let entry = entry.with_context(|| "Could not read directory entry")?;
            if let Some(file_type) = entry.file_type() {
                if file_type.is_file() {
                    self.patch_file(&entry.path(), &query)?;
                }
            }
        }
        Ok(())
    }

    pub fn stats(self) -> Stats {
        self.stats
    }

    pub fn patch_file(&mut self, entry: &Path, query: &Query) -> Result<()> {
        let file_patcher = FilePatcher::new(entry, &query)?;
        let file_patcher = match file_patcher {
            None => return Ok(()),
            Some(f) => f,
        };
        let replacements = file_patcher.replacements();
        if replacements.is_empty() {
            return Ok(());
        }
        self.stats.update(replacements.len());
        file_patcher.print_patch();
        if self.settings.dry_run {
            return Ok(());
        }
        file_patcher.run()
    }

    fn build_walker(&self) -> ignore::Walk {
        let mut types_builder = ignore::types::TypesBuilder::new();
        types_builder.add_defaults();
        for t in &self.settings.selected_file_types {
            types_builder.select(t);
        }
        for t in &self.settings.ignored_file_types {
            types_builder.negate(t);
        }
        let types_matcher = types_builder
            .build()
            .expect("Error when building file types");
        let mut walk_builder = ignore::WalkBuilder::new(&self.path);
        walk_builder.types(types_matcher);
        // Note: the walk_builder configures the "ignore" settings of the Walker,
        // hence the negations
        if !self.settings.ignored {
            walk_builder.ignore(true);
        }
        if self.settings.hidden {
            walk_builder.hidden(false);
        }
        walk_builder.build()
    }
}

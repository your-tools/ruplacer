use anyhow::{bail, Context, Result};
use dyn_clone::DynClone;
use ignore::WalkState;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::console::Console;
use crate::file_patcher::FilePatcher;
use crate::query::Query;
use crate::settings::Settings;
use crate::stats::Stats;

use self::path_deduplicator::PathDeduplicator;

mod path_deduplicator;

#[derive(Debug)]
/// Used to run replacement query on every text file present in a given path
/// ```rust
/// use ruplacer::{Console, DirectoryPatcher, Query, Settings, Stats};
/// use std::path::PathBuf;
///
/// let settings = Settings{
///     dry_run: true,
///     .. Default::default()
/// };
/// let path = PathBuf::from("tests/data");
/// let console = Console::new();
/// let mut directory_patcher = DirectoryPatcher::new(&console, &path, &settings);
///
/// let query = Query::substring("old", "new");
/// directory_patcher.run(&query).unwrap();
/// let stats = directory_patcher.stats();
/// println!("Found {} matching lines", stats.matching_lines());
/// ```
// Note: keep the dry_run: true in the doc test above or the integration test
// will fail ...
pub struct DirectoryPatcher<'a> {
    paths: Box<dyn PathsIter<'a> + 'a>,
    settings: &'a Settings,
    console: &'a Console,
    stats: Stats,
}

pub trait PathsIter<'a>
where
    Self: Debug + DynClone + Iterator<Item = &'a Path> + Send,
{
}

dyn_clone::clone_trait_object!(<'a> PathsIter<'a>);

impl<'a, T> PathsIter<'a> for T where Self: Debug + DynClone + Iterator<Item = &'a Path> + Send + 'a {}

impl<'a> DirectoryPatcher<'a> {
    pub fn new(
        console: &'a Console,
        paths: Box<dyn PathsIter<'a> + 'a>,
        settings: &'a Settings,
    ) -> DirectoryPatcher<'a> {
        let stats = Stats::default();
        DirectoryPatcher {
            console,
            paths,
            settings,
            stats,
        }
    }

    /// Run the given query on the selected files in self.path
    pub fn run(&mut self, query: &Query) -> Result<()> {
        let walker = self.build_walker()?;
        let mut error_happened = Arc::new(Mutex::new(false));
        walker.run(|| {
            let error_happened = error_happened.clone();
            let console = self.console;
            let stats = &self.stats;
            let settings = &self.settings;
            Box::new(move |entry| -> WalkState {
                let res = (|| -> Result<()> {
                    let entry = entry.with_context(|| "Could not read directory entry")?;
                    if let Some(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            Self::patch_file(console, stats, settings, entry.path(), query)?;
                        }
                    }
                    Ok(())
                })();

                match res {
                    Ok(()) => WalkState::Continue,
                    Err(e) => {
                        *error_happened.lock().unwrap() = true;
                        console.print_error(&format!("{:?}", e));
                        WalkState::Quit
                    }
                }
            })
        });
        let error_happened = *Arc::get_mut(&mut error_happened)
            .expect("no references to error flag expected after dir walking")
            .get_mut()
            .unwrap();
        if error_happened {
            bail!("one or more directory walking operations failed, see above for more details")
        } else {
            Ok(())
        }
    }

    pub fn stats(self) -> Stats {
        self.stats
    }

    pub(crate) fn patch_file(
        console: &Console,
        stats: &Stats,
        settings: &Settings,
        entry: &Path,
        query: &Query,
    ) -> Result<()> {
        let file_patcher = FilePatcher::new(console, entry, query)?;
        let file_patcher = match file_patcher {
            None => return Ok(()),
            Some(f) => f,
        };
        let num_replacements = file_patcher.num_replacements();
        if num_replacements != 0 {
            console.print_message("\n");
        }
        let num_lines = file_patcher.num_lines();
        stats.update(num_lines, num_replacements);
        if settings.dry_run {
            return Ok(());
        }
        file_patcher.run()?;
        Ok(())
    }

    fn build_walker(&self) -> Result<ignore::WalkParallel> {
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

        let mut paths = self.paths.clone();

        let mut walk_builder = ignore::WalkBuilder::new(
            paths
                .next()
                .expect("internal error: expected at least one path"),
        );

        for path in paths {
            walk_builder.add(path);
        }

        walk_builder.types(types_matcher);
        // Note: the walk_builder configures the "ignore" settings of the Walker,
        // hence the negations
        if self.settings.ignored {
            walk_builder.ignore(false);
        }
        if self.settings.hidden {
            walk_builder.hidden(false);
        }

        let path_deduplicator = Arc::new(Mutex::new(PathDeduplicator::new()));
        walk_builder.filter_entry(move |dir_entry| {
            fs::canonicalize(dir_entry.path()).map_or(false, |abs_path_buf| {
                let was_not_seen_before =
                    path_deduplicator.lock().unwrap().insert_path(&abs_path_buf);
                was_not_seen_before
            })
        });

        // NOTE(erichdongubler): `walkdir` parallel API lets us do fancy things like skipping
        // duplicate canonicalized entries, which we absolutely need with specification of multiple
        // paths for walking. However, we only use a single thread here for now because there's
        // a few issues to tackle with enabling multiple threads:
        //
        // - Blocker: Console printing is only synchronized for individual write operations --
        //   which is not good enough when we have entire blocks of output that need to stay
        //   together.
        // - Minor: Design thinking should be given to the following issues:
        //   - Errors for any reason in old logic halted the entire search-and-replace operation.
        //     However, we can no [longer guarantee that there won't be a few straggler
        //     operations][stragglers] before `walkdir` quits its iteration flow when we encounter
        //     an error.
        //
        //     [stragglers]: https://docs.rs/ignore/latest/ignore/enum.WalkState.html#variant.Quit
        //
        //     This is probably fine to just note as expected behavior. However, we also have
        //     an opportunity for user-defined behavior for this!
        //
        //     Idea: expose a `--on-replace-error=(report-and-continue|ignore|stop)` flag.
        //   - How to expose the number of threads used to a user?
        //     - Idea: Stick to 1 by default for now, to keep behavior until a new breaking version.
        //     - Idea: allow specifying non-zero number, or "max", ex. `--num-threads=(<1..n>|max)`,
        //       `-j` for short like with many other *nix tools.
        //   - How to print results -- the first alternatives that come to mind are:
        //     - Approach 1: Print replacement reports per file one-at-a-time, queueing them once
        //       the entire file is processed.
        //     - Approach 2: Print replacement reports per file one-at-a-time, queueing individual
        //       line replacements as they are processed.
        //       - Pro: Slightly more responsive UI?
        //       - Con: Once a file is picked for reporting, even finished reports can't print
        //         until the picked one is done. This could have some pathological UX compared to
        //         approach 1.
        //     - Approach 3: Print replacements reports after all of them have been received.
        //       - Almost certainly not what we want -- the most memory and waiting to show the
        //         user something.
        walk_builder.threads(1);

        Ok(walk_builder.build_parallel())
    }
}

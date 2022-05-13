use crate::console::Verbosity;

#[derive(Debug, Default)]
/// Settings applied for a DirectoryPatcher run
pub struct Settings {
    /// Control verbosity of ruplacer's console output
    pub verbosity: Verbosity,
    /// If true, do not write changes to the file system (default: false)
    pub dry_run: bool,
    /// If true, also patch hidden files (default: false)
    pub hidden: bool,
    /// If true, also patch ignored files (default: false)
    pub ignored: bool,
    /// List of file types to select (default: empty)
    pub selected_file_types: Vec<String>,
    /// List of file types to ignore (default: empty)
    pub ignored_file_types: Vec<String>,
}

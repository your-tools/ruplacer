#[derive(Debug)]
/// Settings applied for a DirectoryPatcher run
pub struct Settings {
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

impl Default for Settings {
    fn default() -> Self {
        Self {
            dry_run: false,
            hidden: false,
            ignored: false,
            selected_file_types: vec![],
            ignored_file_types: vec![],
        }
    }
}

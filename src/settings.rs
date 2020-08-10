#[derive(Debug)]
pub struct Settings {
    pub dry_run: bool,
    pub hidden: bool,
    pub ignored: bool,
    pub selected_file_types: Vec<String>,
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

#[derive(Debug)]
pub struct Settings {
    pub dry_run: bool,
    pub selected_file_types: Vec<String>,
    pub ignored_file_types: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            dry_run: false,
            selected_file_types: vec![],
            ignored_file_types: vec![],
        }
    }
}

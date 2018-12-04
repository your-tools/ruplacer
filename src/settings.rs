#[derive(Default)]
pub struct Settings {
    pub dry_run: bool,
    pub selected_file_types: Vec<String>,
    pub ignored_file_types: Vec<String>,
}

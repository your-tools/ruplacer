#[derive(Default)]
pub struct Settings {
    pub dry_run: bool,
    pub selected_file_type: Option<String>,
    pub ignored_file_type: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,
    Normal,
}

impl Default for Verbosity {
    fn default() -> Self {
        Verbosity::Normal
    }
}

#[derive(Debug, Default)]
pub struct Console {
    verbosity: Verbosity,
}

impl Console {
    pub fn with_verbosity(verbosity: Verbosity) -> Self {
        Self { verbosity }
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn print_message(&self, message: &str) {
        if matches!(self.verbosity, Verbosity::Quiet) {
            return;
        }
        print!("{message}");
    }

    pub fn print_error(&self, error: &str) {
        eprintln!("{error}");
    }
}

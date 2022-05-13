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
/// Used to print messages to the console according to a Verbosity
/// level
pub struct Console {
    verbosity: Verbosity,
}

impl Console {
    /// Create a new console with the given verbosity
    pub fn with_verbosity(verbosity: Verbosity) -> Self {
        Self { verbosity }
    }

    /// Create a new console with default verbosity
    pub fn new() -> Self {
        Default::default()
    }

    /// Print a message to the console
    /// (using standard output)
    pub fn print_message(&self, message: &str) {
        if matches!(self.verbosity, Verbosity::Quiet) {
            return;
        }
        print!("{message}");
    }

    /// Print an error message to the console
    /// (using standard error)
    pub fn print_error(&self, error: &str) {
        eprintln!("{error}");
    }
}

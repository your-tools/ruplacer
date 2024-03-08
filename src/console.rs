use colored::*;

use crate::{replacer::Fragment, Replacement};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Control how much information ruplacer prints to standard output
pub enum Verbosity {
    Quiet,
    #[default]
    Normal,
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

    /// Print the replacement as two lines (red then green)
    /// ```
    /// use ruplacer::{Console, Query, replace};
    /// let input = "let foo_bar = FooBar::new();";
    /// let query = Query::subvert("foo_bar", "spam_eggs");
    /// let replacement = replace(input, &query).unwrap();
    /// let console = Console::new();
    /// console.print_replacement("foo.rs:3", &replacement);
    /// // outputs:
    /// // foo.rs:3 let foo_bar = FooBar::new()
    /// // foo.rs:3 let spam_eggs = SpamEggs::new()
    /// ```
    pub fn print_replacement(&self, prefix: &str, replacement: &Replacement) {
        let red_underline = { |x: &str| x.red().underline() };
        let fragments = replacement.fragments();
        let input_fragments = fragments.into_iter().map(|x| &x.0);
        let red_prefix = format!("{}{}", prefix, "- ".red());
        self.print_fragments(
            &red_prefix,
            red_underline,
            replacement.input(),
            input_fragments,
        );

        let green_underline = { |x: &str| x.green().underline() };
        let green_prefix = format!("{}{}", prefix, "+ ".green());
        let output_fragments = fragments.into_iter().map(|x| &x.1);
        self.print_fragments(
            &green_prefix,
            green_underline,
            replacement.output(),
            output_fragments,
        );
    }

    fn print_fragments<'f, C>(
        &self,
        prefix: &str,
        color: C,
        line: &str,
        fragments: impl Iterator<Item = &'f Fragment>,
    ) where
        C: Fn(&str) -> ColoredString,
    {
        self.print_message(prefix);
        let mut current_index = 0;
        for (i, fragment) in fragments.enumerate() {
            let Fragment { index, text } = fragment;
            // Whitespace between prefix and the first fragment does not matter
            if i == 0 {
                self.print_message((&line[current_index..*index].trim_start()).as_ref());
            } else {
                self.print_message(&line[current_index..*index]);
            }
            self.print_message(&format!("{}", color(text)));
            current_index = index + text.len();
        }
        self.print_message(&line[current_index..]);
        if !line.ends_with('\n') {
            self.print_message("\n");
        }
    }
}

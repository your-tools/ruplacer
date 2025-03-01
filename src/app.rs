use anyhow::{anyhow, Error, Result};
use clap::Parser;
use colored::*;
use std::io::{prelude::*, IsTerminal};
use std::path::{Path, PathBuf};
use std::process;
use std::str::FromStr;

use crate::{console::Verbosity, replace, Console, DirectoryPatcher, Query, Settings};

#[derive(Debug, Clone, Copy)]
enum ColorWhen {
    Always,
    Never,
    Auto,
}

impl std::str::FromStr for ColorWhen {
    type Err = Error;

    fn from_str(s: &str) -> Result<ColorWhen, Error> {
        match s {
            "always" => Ok(ColorWhen::Always),
            "auto" => Ok(ColorWhen::Auto),
            "never" => Ok(ColorWhen::Never),
            _ => Err(anyhow!("Choose between 'always', 'auto', or 'never'")),
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "ruplacer",
    version,
    after_help = "
Examples:
    Replace 'foo' with 'bar'
    $ ruplacer foo bar

    Replace 'LastName, FirstName' with 'FirstName LastName'
    $ ruplacer '(\\w+), (\\w+)' '$2 $1'

    Replace '--foo-bar' with '--spam-eggs':
    Note the use of '--' because the pattern and the replacement
    start with two dashes:
    $ ruplacer -- --foo-bar --spam-eggs

    Replace 'FooBar' with 'SpamEggs', 'foo_bar' with 'spam_eggs', ...
    $ ruplacer --preserve-case FooBar SpamEggs
"
)]
struct Options {
    #[arg(long = "go", help = "Write the changes to the filesystem")]
    go: bool,

    #[arg(
        long = "quiet",
        help = "Don't show any output (except in case of errors)"
    )]
    quiet: bool,

    #[arg(help = "The pattern to search for")]
    pattern: String,

    #[arg(help = "The replacement")]
    replacement: String,

    #[arg(
        value_parser = PathBuf::from_str,
        help = "The source path. Defaults to the working directory"
    )]
    path: Option<PathBuf>,

    #[arg(
        long = "no-regex",
        help = "Interpret pattern as a raw string. Default is: regex"
    )]
    no_regex: bool,

    #[arg(long = "hidden", help = "Also patch hidden files")]
    hidden: bool,

    #[arg(long = "ignored", help = "Also patch ignored files")]
    ignored: bool,

    #[arg(
        long = "word-regex",
        short = 'w',
        help = "Interpret pattern as a 'word' regex"
    )]
    word_regex: bool,

    #[arg(
        long = "preserve-case",
        help = "Replace all case variants of the pattern (snake_case, CamelCase and so on)"
    )]
    preserve_case: bool,

    #[arg(
        short = 't',
        long = "type",
        help = "Only search files matching <file_type> or glob pattern.",
        num_args= 0..,
    )]
    selected_file_types: Vec<String>,

    #[arg(
        short = 'T',
        long = "type-not",
        help = "Ignore files matching <file_type> or glob pattern.",
        num_args = 0..,
    )]
    ignored_file_types: Vec<String>,

    #[arg(long = "type-list", help = "List the known file types")]
    file_type_list: bool,

    #[arg(
        long = "color",
        help = "Whether to enable colorful output. Choose between 'always', 'auto', or 'never'. Default is 'auto'"
    )]
    color_when: Option<ColorWhen>,

    #[arg(
        short = 'e',
        long = "allow-empty",
        help = "Exits without error when no matches are found."
    )]
    allow_empty: bool,
}

fn regex_query_or_die(pattern: &str, replacement: &str, word: bool) -> Query {
    let actual_pattern = if word {
        format!(r"\b({})\b", pattern)
    } else {
        pattern.to_string()
    };
    let re = regex::Regex::new(&actual_pattern);
    if let Err(e) = re {
        eprintln!("{}: {}", "Invalid regex".bold().red(), e);
        process::exit(1);
    }
    let re = re.unwrap();
    Query::regex(re, replacement)
}

// Set proper env variable so that the colored crate behaves properly.
// See: https://bixense.com/clicolors/
fn configure_color(when: &ColorWhen) {
    // Safety: this is a private function that is only called once in main()
    // So calling set_env_var is safe here
    match when {
        ColorWhen::Always => unsafe { std::env::set_var("CLICOLOR_FORCE", "1") },
        ColorWhen::Never => unsafe { std::env::set_var("CLICOLOR", "0") },
        ColorWhen::Auto => {
            let is_a_tty = std::io::stdout().is_terminal();
            if is_a_tty {
                unsafe { std::env::set_var("CLICOLOR", "1") }
            } else {
                unsafe { std::env::set_var("CLICOLOR", "0") }
            }
        }
    }
}

fn on_type_list() {
    println!("Known file types:");
    let mut types_builder = ignore::types::TypesBuilder::new();
    types_builder.add_defaults();
    for def in types_builder.definitions() {
        let name = def.name();
        let globs = def.globs();
        println!("{}: {}", name.bold(), globs.join(", "));
    }
}

/// Main entry point
pub fn run() -> Result<()> {
    // PATTERN and REPLACEMENT are always required, except
    // when --type-list is used
    //
    // So we cache the ErrorKind::MissingRequiredArgument error
    // to handle the exception, rather than having the usage
    // looking like `ruplacer [OPTIONS]`
    let mut args = std::env::args();
    let parsed = Options::try_parse();
    let opt = match parsed {
        Ok(o) => o,
        Err(e) => {
            let used_typed_list = args.any(|x| &x == "--type-list");
            if used_typed_list {
                on_type_list();
                return Ok(());
            } else {
                e.exit();
            }
        }
    };
    let Options {
        color_when,
        file_type_list: _,
        go,
        quiet,
        hidden,
        ignored,
        ignored_file_types,
        no_regex,
        path,
        pattern,
        replacement,
        selected_file_types,
        preserve_case,
        word_regex,
        allow_empty,
    } = opt;

    let dry_run = !go;
    let verbosity = if quiet {
        Verbosity::Quiet
    } else {
        Verbosity::Normal
    };
    let console = Console::with_verbosity(verbosity);

    let color_when = &color_when.unwrap_or(ColorWhen::Auto);
    configure_color(color_when);

    let query = if no_regex {
        Query::simple(&pattern, &replacement)
    } else if preserve_case {
        Query::preserve_case(&pattern, &replacement)
    } else {
        regex_query_or_die(&pattern, &replacement, word_regex)
    };

    let settings = Settings {
        verbosity,
        dry_run,
        hidden,
        ignored,
        selected_file_types,
        ignored_file_types,
        allow_empty,
    };

    let path = path.unwrap_or_else(|| Path::new(".").to_path_buf());
    if path == PathBuf::from("-") {
        run_on_stdin(query)
    } else {
        run_on_directory(console, path, settings, query)
    }
}

fn run_on_stdin(query: Query) -> Result<()> {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let replacement = replace(&line, &query);
        if let Some(replacement) = replacement {
            println!("{}", replacement.output());
        } else {
            println!("{}", line);
        }
    }
    Ok(())
}

fn run_on_directory(
    console: Console,
    path: PathBuf,
    settings: Settings,
    query: Query,
) -> Result<()> {
    let dry_run = settings.dry_run;
    let mut directory_patcher = DirectoryPatcher::new(&console, &path, &settings);
    directory_patcher.run(&query)?;
    let stats = directory_patcher.stats();
    if stats.total_replacements() == 0 {
        match settings.allow_empty {
            true => {
                console.print_message("nothing found to replace\n");
                process::exit(0);
            }
            false => {
                console.print_error(&format!(
                    "{}: {}",
                    "Error".bold().red(),
                    "nothing found to replace"
                ));
                process::exit(2);
            }
        }
    }

    let stats = &stats;
    let message = if dry_run {
        "Would perform "
    } else {
        "Performed "
    };
    console.print_message(message);
    console.print_message(&format!("{stats}\n"));

    if dry_run {
        console
            .print_message("Re-run ruplacer with --go to write these changes to the filesystem\n");
    }
    Ok(())
}

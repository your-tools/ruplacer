use anyhow::{anyhow, Error, Result};
use clap::Parser;
use colored::*;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process;

use crate::{replace, DirectoryPatcher, Query, Settings, Stats};

#[derive(Debug)]
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
#[clap(
    name = "ruplacer",
    version,
    after_help = "
EXAMPLES:
    Replace 'foo' with 'bar'
    $ ruplacer foo bar

    Replace 'LastName, FirstName' with 'FirstName LastName'
    $ ruplacer '(\\w+), (\\w+)' '$2 $1'

    Replace '--foo-bar' with '--spam-eggs':
    Note the use of '--' because the pattern and the replacement
    start with two dashes:
    $ ruplacer -- --foo-bar --spam-eggs

    Replace 'FooBar' with 'SpamEggs', 'foo_bar' with 'spam_eggs', ...
    $ ruplacer --subvert FooBar SpamEggs
"
)]
struct Options {
    #[clap(long = "go", help = "Write the changes to the filesystem")]
    go: bool,

    #[clap(help = "The pattern to search for")]
    pattern: String,

    #[clap(help = "The replacement")]
    replacement: String,

    #[clap(
        parse(from_os_str),
        help = "The source path. Defaults to the working directory"
    )]
    path: Option<PathBuf>,

    #[clap(
        long = "--no-regex",
        help = "Interpret pattern as a raw string. Default is: regex"
    )]
    no_regex: bool,

    #[clap(long = "--hidden", help = "Also patch hidden files")]
    hidden: bool,

    #[clap(long = "--ignored", help = "Also patch ignored files")]
    ignored: bool,

    #[clap(
        long = "--word-regex",
        short = 'w',
        help = "Interpret pattern as a 'word' regex"
    )]
    word_regex: bool,

    #[clap(
        long = "--subvert",
        help = "Replace all variants of the pattern (snake_case, CamelCase and so on)"
    )]
    subvert: bool,

    #[clap(
        short = 't',
        long = "type",
        help = "Only search files matching <file_type> or glob pattern.",
        multiple_occurrences = true,
        number_of_values = 1
    )]
    selected_file_types: Vec<String>,

    #[clap(
        short = 'T',
        long = "type-not",
        help = "Ignore files matching <file_type> or glob pattern.",
        multiple_occurrences = true,
        number_of_values = 1
    )]
    ignored_file_types: Vec<String>,

    #[clap(long = "type-list", help = "List the known file types")]
    file_type_list: bool,

    #[clap(
        long = "--color",
        help = "Whether to enable colorful output. Choose between 'always', 'auto', or 'never'. Default is 'auto'"
    )]
    color_when: Option<ColorWhen>,
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
    match when {
        ColorWhen::Always => std::env::set_var("CLICOLOR_FORCE", "1"),
        ColorWhen::Never => std::env::set_var("CLICOLOR", "0"),
        ColorWhen::Auto => {
            if atty::is(atty::Stream::Stdout) {
                std::env::set_var("CLICOLOR", "1")
            } else {
                std::env::set_var("CLICOLOR", "0")
            }
        }
    }
}

fn print_stats(stats: &Stats, dry_run: bool) {
    if dry_run {
        print!("Would perform ")
    } else {
        print!("Performed ")
    }
    println!("{}", stats)
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

pub fn run() -> Result<()> {
    let opt = Options::parse();
    let Options {
        color_when,
        file_type_list,
        go,
        hidden,
        ignored,
        ignored_file_types,
        no_regex,
        path,
        pattern,
        replacement,
        selected_file_types,
        subvert,
        word_regex,
    } = opt;

    if file_type_list {
        on_type_list();
        return Ok(());
    }

    let dry_run = !go;

    let color_when = &color_when.unwrap_or(ColorWhen::Auto);
    configure_color(color_when);

    let query = if no_regex {
        Query::substring(&pattern, &replacement)
    } else if subvert {
        Query::subvert(&pattern, &replacement)
    } else {
        regex_query_or_die(&pattern, &replacement, word_regex)
    };

    let settings = Settings {
        dry_run,
        hidden,
        ignored,
        selected_file_types,
        ignored_file_types,
    };

    let path = path.unwrap_or_else(|| Path::new(".").to_path_buf());
    if path == PathBuf::from("-") {
        run_on_stdin(query)
    } else {
        run_on_directory(path, settings, query)
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

fn run_on_directory(path: PathBuf, settings: Settings, query: Query) -> Result<()> {
    let dry_run = settings.dry_run;
    let mut directory_patcher = DirectoryPatcher::new(&path, &settings);
    directory_patcher.run(&query)?;
    let stats = directory_patcher.stats();
    if stats.total_replacements() == 0 {
        #[allow(clippy::print_literal)]
        {
            eprintln!("{}: {}", "Error".bold().red(), "nothing found to replace");
        }
        process::exit(2);
    }
    print_stats(&stats, dry_run);
    if dry_run {
        println!("Re-run ruplacer with --go to write these changes to the filesystem");
    }
    Ok(())
}

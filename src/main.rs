extern crate colored;
extern crate isatty;
extern crate regex;
extern crate structopt;

use colored::*;
use isatty::stdout_isatty;
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;

extern crate ruplacer;

#[derive(Debug)]
enum ColorWhen {
    Always,
    Never,
    Auto,
}

impl std::str::FromStr for ColorWhen {
    type Err = ruplacer::Error;

    fn from_str(s: &str) -> Result<ColorWhen, ruplacer::Error> {
        match s {
            "always" => Ok(ColorWhen::Always),
            "auto" => Ok(ColorWhen::Auto),
            "never" => Ok(ColorWhen::Never),
            _ => Err(ruplacer::Error::new(
                "Choose between 'always', 'auto', or 'never'",
            )),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ruplacer",
    after_help = "
EXAMPLES:
    Replace 'foo' with 'bar'
    $ ruplacer foo bar

    Replace 'LastName, FirstName' with 'FirstName LastName'
    $ ruplacer '(\\w)+, (\\w)+' '$2 $1'

    Replace 'FooBar' with 'SpamEggs', 'foo_bar' with 'spam_eggs', ...
    $ ruplacer --subvert FooBar SpamEggs
"
)]
struct Opt {
    #[structopt(long = "go")]
    go: bool,

    #[structopt(help = "The pattern to search for")]
    pattern: String,

    #[structopt(help = "The replacement")]
    replacement: String,

    #[structopt(parse(from_os_str), help = "The source path. Defaults to the working directory")]
    path: Option<PathBuf>,

    #[structopt(
        long = "--no-regex", help = "Interpret pattern as a a raw string. Default is: regex"
    )]
    no_regex: bool,

    #[structopt(
        long = "--subvert",
        help = "Replace all variants of the pattern (snake_case, CamelCase and so on)"
    )]
    subvert: bool,

    #[structopt(short = "t", long = "type", help = "Only search files matching <file_type>")]
    file_type: Option<String>,

    #[structopt(
        long = "--color",
        help = "Wether to enable colorful output. Choose between 'always', 'auto', or 'never'. Default is 'auto'"
    )]
    color_when: Option<ColorWhen>,
}

fn regex_query_or_die(pattern: &str, replacement: &str) -> ruplacer::query::Query {
    let re = regex::Regex::new(pattern);
    if let Err(e) = re {
        eprintln!("{}: {}", "Invalid regex".bold().red(), e);
        process::exit(1);
    }
    let re = re.unwrap();
    ruplacer::query::from_regex(re, replacement)
}

// Set proper env variable so that the colored crate behaves properly.
// See: https://bixense.com/clicolors/
fn configure_color(when: &ColorWhen) {
    match when {
        ColorWhen::Always => std::env::set_var("CLICOLOR_FORCE", "1"),
        ColorWhen::Never => std::env::set_var("CLICOLOR", "0"),
        ColorWhen::Auto => {
            if stdout_isatty() {
                std::env::set_var("CLICOLOR", "1")
            } else {
                std::env::set_var("CLICOLOR", "0")
            }
        }
    }
}

fn print_stats(stats: ruplacer::Stats, dry_run: bool) {
    if dry_run {
        print!("Would perform ")
    } else {
        print!("Performed ")
    }
    println!("{}", stats)
}

fn main() {
    let opt = Opt::from_args();
    let dry_run = !&opt.go;

    let color_when = &opt.color_when.unwrap_or(ColorWhen::Auto);
    configure_color(&color_when);

    let path = opt.path;
    let path = path.unwrap_or(Path::new(".").to_path_buf());

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let Opt { pattern, replacement, .. } = opt;
    let query = if opt.no_regex {
        ruplacer::query::substring(&pattern, &replacement)
    } else if opt.subvert {
        ruplacer::query::subvert(&pattern, &replacement)
    } else {
        regex_query_or_die(&pattern, &replacement)
    };

    let settings = ruplacer::Settings {
        dry_run,
        file_type: opt.file_type,
    };
    let mut directory_patcher = ruplacer::DirectoryPatcher::new(path, settings);
    let outcome = directory_patcher.patch(query);
    if let Err(err) = outcome {
        eprintln!("{}: {}", "Error".bold().red(), err);
        process::exit(1);
    }

    let stats = directory_patcher.stats();
    if stats.num_replacements == 0 {
        eprintln!("{}: {}", "Error".bold().red(), "nothing found to replace");
        process::exit(2);
    }
    print_stats(stats, dry_run);
    if dry_run {
        println!("Re-run ruplacer with --go to write these changes to the filesystem");
    }
}

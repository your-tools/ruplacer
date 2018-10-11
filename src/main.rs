#[macro_use]
extern crate structopt;
extern crate isatty;
extern crate regex;
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
#[structopt(name = "ruplacer")]
struct Opt {
    #[structopt(long = "go")]
    go: bool,

    #[structopt(help = "The pattern to search for")]
    pattern: String,

    #[structopt(
        long = "--fixed-strings", help = "Interpret pattern as a a raw string. Default is: regex"
    )]
    fixed_string: bool,

    #[structopt(
        long = "--color",
        help = "Wether to enable colorful output. Choose between 'always', 'auto', or 'never'. Default is 'auto'"
    )]
    color_when: Option<ColorWhen>,

    #[structopt(help = "The replacement")]
    replacement: String,

    #[structopt(parse(from_os_str), help = "The source path. Defaults to the working directory")]
    path: Option<PathBuf>,
}

fn regex_query_or_die(pattern: &str, replacement: &str) -> ruplacer::query::Query {
    let re = regex::Regex::new(pattern);
    if let Err(e) = re {
        eprintln!("Invalid regex: {}: {}", pattern, e);
        process::exit(1);
    }
    let re = re.unwrap();
    ruplacer::query::from_regex(re, replacement)
}

// Just set proper env variable so that the colored crates
// behaves properly.
// See: https://bixense.com/clicolors/
fn configure_color(when: ColorWhen) {
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

fn main() {
    let opt = Opt::from_args();
    let dry_run = !opt.go;

    let color_when = opt.color_when.unwrap_or(ColorWhen::Auto);
    configure_color(color_when);

    let path = opt.path;
    let path = path.unwrap_or(Path::new(".").to_path_buf());
    let query = if opt.fixed_string {
        ruplacer::query::substring(&opt.pattern, &opt.replacement)
    } else {
        regex_query_or_die(&opt.pattern, &opt.replacement)
    };
    let mut directory_patcher = ruplacer::DirectoryPatcher::new(path);
    directory_patcher.dry_run(dry_run);
    let outcome = directory_patcher.patch(query);
    if let Err(err) = outcome {
        eprintln!("{}", err);
        process::exit(1);
    }
}

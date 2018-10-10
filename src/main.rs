#[macro_use]
extern crate structopt;
extern crate regex;
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;

extern crate ruplacer;

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

fn main() {
    let opt = Opt::from_args();
    let dry_run = !opt.go;

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

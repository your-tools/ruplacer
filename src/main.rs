extern crate ignore;
extern crate regex;
extern crate difference;
extern crate term;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod replacer;

use structopt::StructOpt;
use ignore::Walk;
use std::path::Path;
use replacer::{Replacer, FileReplacer};

#[derive(StructOpt, Debug)]
#[structopt(name = "fr",
            version = "0.1.0",
            author = "The ruplacer team",
            about = "Find and Ruplace patterns in files and filenames.")]
struct ReplacerOpts {
    #[structopt(short = "v", long = "verbose", help = "Activate verbose mode")]
    debug: bool,

    #[structopt(long = "go", help = "Perform changes rather than just printing them")]
    go: bool,

    #[structopt(help = "The pattern to search for")]
    pattern: String,

    #[structopt(help = "The replacement")]
    replacement: Option<String>,

    #[structopt(help = "paths (default to current working directory)", default_value="./")]
    paths: Vec<String>,
}

fn walk_file(replacer: &Replacer, opts: &ReplacerOpts, file_name: &Path) {
    if opts.replacement != None {
        if let Err(x) = replacer.replace_in_file(file_name, !opts.go) {
            println!("{}", x);
        }
    } else {
        if let Err(x) = replacer.grep_in_file(file_name) {
            println!("{}", x);
        }
    }
}

fn walk_paths(replacer: &Replacer, opts: &ReplacerOpts) {
    for dir in opts.paths.iter() {
        println!("Root directory: {}", dir);
        for result in Walk::new(dir) {
            // Each item yielded by the iterator is either a directory entry or an
            // error, so either print the path or the error.
            match result {
                Ok(entry) => walk_file(&replacer, opts, entry.path()),
                Err(err) => println!("ERROR: {}", err),
            }
        }
    }
}

fn main() {
    let opts = &ReplacerOpts::from_args();

    let replacer = Replacer::new(opts.pattern.as_str(), opts.replacement.clone());
    walk_paths(&replacer, &opts)
}

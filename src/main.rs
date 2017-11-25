extern crate clap;
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
use std::fs::File;
use std::io::prelude::*;
use replacer::Replacer;


fn replace_file(replacer: &Replacer, file_name: &Path) {
    if !file_name.is_file() {
        return
    }
    println!("{}", file_name.display());

    let mut f = File::open(file_name).expect("file not found");
    let mut data = String::new();
    f.read_to_string(&mut data).expect("error reading file");
    replacer.grep_or_replace(data.as_str());
}

#[derive(StructOpt, Debug)]
#[structopt(name = "fr",
            version = "0.1.0",
            author = "The ruplacer team",
            about = "Find and Ruplace patterns in files and filenames.")]
struct Opt {
    #[structopt(short = "v", long = "verbose", help = "Activate verbose mode")]
    debug: bool,

    #[structopt(help = "The pattern to grep for")]
    pattern: String,

    #[structopt(help = "The replacement")]
    replacement: Option<String>,

    #[structopt(help = "paths (default to current working directory)")]
    paths: Vec<String>,
}

fn main() {
    let opts = Opt::from_args();
    let dirs: Vec<_> = match opts.paths.len() {
        0 => vec!["./".to_string()],
        _ => opts.paths,
    };

    let replacer = Replacer::new(opts.pattern.as_str(), opts.replacement);

    for dir in dirs {
        println!("Root directory: {}", dir);
        for result in Walk::new(dir) {
            // Each item yielded by the iterator is either a directory entry or an
            // error, so either print the path or the error.
            match result {
                Ok(entry) => replace_file(&replacer, entry.path()),
                Err(err) => println!("ERROR: {}", err),
            }
        }
    }
}

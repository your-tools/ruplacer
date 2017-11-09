extern crate clap;
extern crate ignore;

use clap::{App, Arg};
use ignore::Walk;

fn main() {
    let matches = App::new("fr")
        .version("0.1.0")
        .author("The ruplacer team")
        .about("Find and replace")
        .arg(Arg::with_name("PATTERN")
            .help("the pattern to grep for")
            .required(true)
            .index(1))
        .arg(Arg::with_name("DIR")
            .short("d")
            .long("dir")
            .value_name("DIR")
            .index(2)
            .help("the directory to find into")
            .takes_value(true))
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();
    //println!("{}", matches);
    let dir = matches.value_of("DIR").unwrap_or("./");
    println!("Root directory: {}", dir);

    for result in Walk::new(dir) {
        // Each item yielded by the iterator is either a directory entry or an
        // error, so either print the path or the error.
        match result {
            Ok(entry) => println!("{}", entry.path().display()),
            Err(err) => println!("ERROR: {}", err),
        }
    }
}

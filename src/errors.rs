use ignore;
use std;

#[derive(Debug)]
pub struct Error {
    description: String,
}

impl Error {
    fn new(description: &str) -> Error {
        Error {
            description: String::from(description),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.description)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::new(&format!("I/O error: {}", error))
    }
}

impl From<ignore::Error> for Error {
    fn from(error: ignore::Error) -> Error {
        Error::new(&format!("Error when parsing .ignore files: {}", error))
    }
}

use ignore;
use std;

#[derive(Debug)]
pub struct Error {
    description: String,
}

impl Error {
    pub fn new(description: &str) -> Error {
        Error {
            description: String::from(description),
        }
    }

    pub fn from_read_error(path: &std::path::Path, io_error: &std::io::Error) -> Result<(), Error> {
        let path = path.to_string_lossy();
        let message = format!("Error when reading {}: {}", path, io_error);
        Err(Error::new(&message))
    }

    pub fn from_write_error(
        path: &std::path::Path,
        io_error: &std::io::Error,
    ) -> Result<(), Error> {
        let path = path.to_string_lossy();
        let message = format!("Error when writing {}: {}", path, io_error);
        Err(Error::new(&message))
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

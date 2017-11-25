use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Result as IoResult, Error as IoError, ErrorKind};

use replacer::Replacer;

pub trait FileReplacer {
    fn replace_in_file(&self, file_name: &Path, dry_run: bool) -> IoResult<()>;
}

impl FileReplacer for Replacer {
    fn replace_in_file(&self, file_path: &Path, dry_run: bool) -> IoResult<()> {
        if !file_path.is_file() {
            return Err(IoError::from(ErrorKind::InvalidInput))
        }
        println!("{}", file_path.display());

        let mut data = String::new();
        {
            let mut f = File::open(file_path).expect("file not found");
            f.read_to_string(&mut data).expect("error reading file");
        }
        let new_data = self.replace(data.as_str());
        if !dry_run {
            // Recreate the file and dump the processed contents to it
            let mut dst = File::create(&file_path).expect("error opening the file for writing");
            dst.write(new_data.as_bytes()).expect("error writing to file");
        }
        Ok(())
    }
}


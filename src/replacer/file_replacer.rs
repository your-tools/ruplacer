use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Result as IoResult, Error as IoError, ErrorKind};

use replacer::Replacer;

pub trait FileReplacer {
    fn replace_in_file(&self, file_name: &Path, dry_run: bool) -> IoResult<()>;
    fn grep_in_file(&self, file_name: &Path) -> IoResult<()>;
}

fn read_file(file_path: &Path) -> IoResult<String> {
    if !file_path.is_file() {
        return Err(IoError::from(ErrorKind::InvalidInput));
    }

    let mut data = String::new();
    {
        let mut f = File::open(file_path).expect("file not found");
        f.read_to_string(&mut data).expect("error reading file");
    }
    Ok(data)
}

impl FileReplacer for Replacer {

    fn replace_in_file(&self, file_path: &Path, dry_run: bool) -> IoResult<()> {
        let data = read_file(file_path)?;
        println!("{}", file_path.display());

        let new_data = self.replace(data.as_str())?;
        if !dry_run {
            // Recreate the file and dump the processed contents to it
            let mut dst = File::create(&file_path)?;
            dst.write(new_data.as_bytes())?;
        }
        Ok(())
    }

    fn grep_in_file(&self, file_path: &Path) -> IoResult<()> {
        let data = read_file(file_path)?;
        println!("{}", file_path.display());
        self.grep(data.as_str())
    }
}


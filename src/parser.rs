use std::{path::{PathBuf, Path}, io, ffi::OsStr, fs};

#[derive(Debug)]
pub struct Parser {
    pub script_path: PathBuf
}

impl Parser {
    pub fn new(script_path: impl AsRef<Path>) -> io::Result<Self> {
        let this = Self {
            script_path: script_path.as_ref().to_owned(),
        };

        if !this.script_path.is_file() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "The given input is not a file."));
        }

        if let Some(ext) = this.script_path.extension() {
            if ext != OsStr::new("cpra") {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "File does not have a .crpa extension."));
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "File does not have a .crpa extension."));
        }

        Ok(this)
    }

    pub fn read(&self) -> io::Result<String> {
        fs::read_to_string(&self.script_path)
    }
}


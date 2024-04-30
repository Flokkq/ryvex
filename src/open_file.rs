use std::path::PathBuf;

use crate::{cursor::Cursor, error::Error, file_access::FileAccess};

pub struct OpenFile {
    pub path: PathBuf,
    pub buffer: String,
    pub cursor: Cursor,
}

impl OpenFile {
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        let mut buffer = String::new();
        FileAccess::read_from_file_if_exists(&path, &mut buffer)?;

        let cursor = Cursor::place();
        Ok(OpenFile {
            path,
            buffer,
            cursor,
        })
    }
}

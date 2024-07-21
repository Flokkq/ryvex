use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use crate::core::error;

pub struct FileAccess;

impl FileAccess {
    pub fn read_from_file_if_exists(
        path: &PathBuf,
        buffer: &mut String,
    ) -> Result<(), error::Error> {
        if let Ok(_metadata) = fs::metadata(path) {
            let mut file =
                File::open(path).map_err(error::Error::Io)?;
            file.read_to_string(buffer)
                .map_err(error::Error::Io)?;
        }

        Ok(())
    }

    pub fn write_to_file(
        path: &PathBuf,
        buffer: &String,
    ) -> Result<(), error::Error> {
        let mut file =
            File::create(path).map_err(error::Error::Io)?;

        file.write_all(buffer.as_bytes())
            .map_err(error::Error::Io)?;

        Ok(())
    }
}

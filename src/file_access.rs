use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use crate::error;

pub struct FileAccess;

impl FileAccess {
    pub fn read_from_file_if_exists(
        path: &PathBuf,
        buffer: &mut String,
    ) -> Result<(), error::Error> {
        match fs::metadata(path) {
            Ok(_metadata) => {
                let mut file =
                    File::open(path).map_err(|err| error::Error::Io(err))?;
                file.read_to_string(buffer)
                    .map_err(|err| error::Error::Io(err))?;
            }
            Err(_) => {}
        }

        Ok(())
    }

    pub fn write_to_file(
        path: &PathBuf,
        buffer: &String,
    ) -> Result<(), error::Error> {
        let mut file =
            File::create(path).map_err(|err| error::Error::Io(err))?;

        file.write_all(buffer.as_bytes())
            .map_err(|err| error::Error::Io(err))?;

        Ok(())
    }
}

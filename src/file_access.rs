use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

pub struct FileAccess;

impl FileAccess {
    pub fn read_from_file_if_exists(
        path: &PathBuf,
        buffer: &mut String,
    ) -> Result<(), std::io::Error> {
        match fs::metadata(path) {
            Ok(_metadata) => {
                let mut file = File::open(path)?;
                file.read_to_string(buffer)?;
            }
            Err(_) => {}
        }

        Ok(())
    }

    pub fn write_to_file(
        path: &PathBuf,
        buffer: &mut String,
    ) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write_all(buffer.as_bytes())?;

        Ok(())
    }
}

use super::error::Error;

pub fn save_file() -> Result<(), Error> {
    Ok(())
}

pub fn exit_application() -> Result<(), Error> {
    std::process::exit(1);
}

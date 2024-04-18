use super::error::ActionError;

pub fn save_file() -> Result<(), ActionError> {
    Ok(())
}

pub fn exit_application() -> Result<(), ActionError> {
    std::process::exit(1);
}

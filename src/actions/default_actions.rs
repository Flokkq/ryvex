use std::os::unix::process;

use crate::{
    file_access::FileAccess, keys::keybind::ActionResult,
    state::get_global_state, telemetry::SingletonLogger,
};

use super::error::ActionError;

pub fn save_file() -> Result<ActionResult, ActionError> {
    let logger = SingletonLogger::get_instance();

    let global_state = get_global_state();
    let state = global_state
        .get_state()
        .map_err(|_| ActionError::Unexpected)?;

    if let Some(file) = &state.file {
        FileAccess::write_to_file(&file.path, &file.buffer)
            .map_err(|_| ActionError::Unexpected)?;
    }

    Ok(ActionResult::Continue)
}

pub fn exit_application() -> Result<ActionResult, ActionError> {
    Ok(ActionResult::Exit)
}

use crate::{
    core::{
        keys::keybind::ActionResult,
        state::get_global_state,
        ui::{overlay::Overlay, MessageLevel},
    },
    file_access::FileAccess,
};

use super::error::ActionError;

pub fn save_file() -> Result<ActionResult, ActionError> {
    let global_state = get_global_state();
    let state = global_state
        .get_state()
        .map_err(|_| ActionError::Unexpected)?;

    if let Some(file) = &state.file {
        FileAccess::write_to_file(&file.path, file.buffer.content()).map_err(
            |err| {
                Overlay::display_primitive_message(
                    String::from(&format!("Could not save buffer {}", err)),
                    MessageLevel::Error,
                );
                ActionError::Unexpected
            },
        )?;

        Overlay::display_primitive_message(
            String::from("Buffer saved"),
            MessageLevel::Info,
        );
    }

    Ok(ActionResult::Continue)
}

pub fn exit_application() -> Result<ActionResult, ActionError> {
    Ok(ActionResult::Exit)
}

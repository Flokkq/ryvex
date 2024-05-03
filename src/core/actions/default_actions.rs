use crate::{
    core::{
        keys::keybind::ActionResult,
        state::get_global_state,
        ui::{overlay::Overlay, MessageLevel, MessageOverlayPosition},
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
        FileAccess::write_to_file(&file.path, &file.buffer.get_content())
            .map_err(|_| {
                Overlay::render_decorative_message(
                    String::from("Could not save buffer"),
                    MessageOverlayPosition::TopRight,
                    MessageLevel::Error,
                );
                ActionError::Unexpected
            })?;
    }

    Overlay::render_decorative_message(
        String::from("Buffer saved"),
        MessageOverlayPosition::TopRight,
        MessageLevel::Warning,
    );

    Overlay::render_primitive_message(String::from("Buffer savedBuffer savedBuffer savedBuffer savedBuffer savedBuffer savedBuffer savedBuffer savedBuffer savedBuffer savedBuffer savedBuffer savedBuffer savedBuffer savedBuffer savedBuffer saved"), MessageLevel::Error);

    Ok(ActionResult::Continue)
}

pub fn exit_application() -> Result<ActionResult, ActionError> {
    Ok(ActionResult::Exit)
}
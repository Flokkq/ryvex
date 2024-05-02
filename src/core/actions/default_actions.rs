use crate::{
    core::{
        keys::keybind::ActionResult,
        state::get_global_state,
        ui::overlay::{Overlay, OverlayPosition},
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
            .map_err(|_| ActionError::Unexpected)?;

        Overlay::show(
            String::from("Buffer saved to left"),
            OverlayPosition::MidLeft,
        );
        Overlay::show(
            String::from("Buffer saved to right"),
            OverlayPosition::MidRight,
        );
    }

    Ok(ActionResult::Continue)
}

pub fn exit_application() -> Result<ActionResult, ActionError> {
    Ok(ActionResult::Exit)
}

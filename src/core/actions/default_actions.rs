use std::io::stdout;

use crate::{
    core::{
        buffer::Direction,
        state::get_global_state,
        ui::{overlay::Overlay, MessageLevel},
    },
    file_access::FileAccess,
};

use super::{action::ActionResult, error::ActionError};

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

pub fn save_and_exit() -> Result<ActionResult, ActionError> {
    save_file()?;
    exit_application()
}

fn r#move(direction: Direction) -> Result<ActionResult, ActionError> {
    let mut stdout = stdout().lock();
    let global_state = get_global_state();
    let mut state = global_state
        .get_state()
        .map_err(|_| ActionError::Unexpected)?;

    if let Some(file) = &mut state.file {
        file.buffer.move_cursor(direction);

        file.redraw(&mut stdout).map_err(|err| {
            Overlay::display_primitive_message(
                String::from(&format!("Could not save buffer {}", err)),
                MessageLevel::Error,
            );
            ActionError::Unexpected
        })?
    }

    Ok(ActionResult::Continue)
}

pub fn move_left() -> Result<ActionResult, ActionError> {
    r#move(Direction::Left)
}

pub fn move_right() -> Result<ActionResult, ActionError> {
    r#move(Direction::Right)
}

pub fn move_up() -> Result<ActionResult, ActionError> {
    r#move(Direction::Up)
}

pub fn move_down() -> Result<ActionResult, ActionError> {
    r#move(Direction::Down)
}

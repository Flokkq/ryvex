use crate::{
    actions,
    keys::{key::Key, keybind::KeyBind},
};

pub struct Settings {
    pub keybinds: Vec<KeyBind<fn() -> Result<(), actions::error::Error>>>,
}

pub fn get_configuration() -> Result<Settings, ()> {
    let ctrl_w = Key::bind(23); // ASCII for CTRL-W
    let ctrl_s = Key::bind(19); // ASCII for CTRL-S
    let ctrl_q = Key::bind(17); // ASCII for CTRL-Q

    // Define your callback functions
    fn save() -> Result<(), actions::error::Error> {
        // Implementation of save function
        Ok(())
    }

    fn exit() -> Result<(), actions::error::Error> {
        // Implementation of exit function
        Ok(())
    }

    // Create KeyBind instances with callbacks
    let keybind_w = KeyBind::new(
        vec![ctrl_w],
        save as fn() -> Result<(), actions::error::Error>,
    );
    let keybind_s = KeyBind::new(
        vec![ctrl_s],
        save as fn() -> Result<(), actions::error::Error>,
    );
    let keybind_q = KeyBind::new(
        vec![ctrl_q],
        exit as fn() -> Result<(), actions::error::Error>,
    );

    let keybinds: Vec<KeyBind<fn() -> Result<(), actions::error::Error>>> =
        vec![keybind_q, keybind_s, keybind_w];

    Ok(Settings { keybinds })
}

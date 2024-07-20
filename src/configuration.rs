use crate::core::{
    actions::default_actions::{exit_application, save_file},
    keys::{
        key::Key,
        keybind::{IOOperation, KeyBind, Operation},
        keycode::KeyCode,
    },
};

pub struct Settings {
    pub keybinds: Vec<KeyBind>,
}

pub fn get_configuration() -> Settings {
    let ctrl_c = Key::bind(KeyCode::Etx);
    let ctrl_s = Key::bind(KeyCode::Dc3);

    let keybind_c =
        KeyBind::new(ctrl_c, Operation::Count, Some(exit_application));
    let keybind_s = KeyBind::new(
        ctrl_s,
        Operation::IO(IOOperation::Write),
        Some(save_file),
    );

    let keybinds = vec![keybind_c, keybind_s];

    Settings { keybinds }
}

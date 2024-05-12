use crate::core::{
    actions::default_actions::{exit_application, save_file},
    keys::{key::Key, keybind::KeyBind, keycode::KeyCode},
};

pub struct Settings {
    pub keybinds: Vec<KeyBind>,
}

pub fn get_configuration() -> Settings {
    let ctrl_c = Key::bind(KeyCode::Etx);
    let ctrl_w = Key::bind(KeyCode::Etb);
    let ctrl_s = Key::bind(KeyCode::Dc3);
    let ctrl_q = Key::bind(KeyCode::Dc1);

    let keybind_c = KeyBind::new(vec![ctrl_c], exit_application);
    let keybind_w = KeyBind::new(vec![ctrl_w], save_file);
    let keybind_s = KeyBind::new(vec![ctrl_s], save_file);
    let keybind_q = KeyBind::new(vec![ctrl_q], exit_application);

    let keybinds = vec![keybind_c, keybind_q, keybind_s, keybind_w];

    Settings { keybinds }
}

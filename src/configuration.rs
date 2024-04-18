use crate::{
    actions::default_actions::{exit_application, save_file},
    keys::{key::Key, keybind::KeyBind},
};

pub struct Settings {
    pub keybinds: Vec<KeyBind>,
}

pub fn get_configuration() -> Settings {
    let ctrl_w = Key::bind(23);
    let ctrl_s = Key::bind(19);
    let ctrl_q = Key::bind(17);

    let keybind_w = KeyBind::new(vec![ctrl_w], save_file);
    let keybind_s = KeyBind::new(vec![ctrl_s], save_file);
    let keybind_q = KeyBind::new(vec![ctrl_q], exit_application);

    let keybinds = vec![keybind_q, keybind_s, keybind_w];

    Settings { keybinds }
}

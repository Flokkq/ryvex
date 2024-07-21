use crate::core::{
    self,
    actions::default_actions::{exit_application, save_and_exit, save_file},
    keys::keybind::KeyBind,
};

pub struct Settings {
    pub keybinds: Vec<KeyBind>,
    pub custom_commands: Vec<core::command::Command>,
}

pub fn get_configuration() -> Settings {
    Settings {
        keybinds: get_keybinds(),
        custom_commands: get_commands(),
    }
}

fn get_keybinds() -> Vec<KeyBind> {
    vec![]
}

fn get_commands() -> Vec<core::command::Command> {
    let save = core::command::Command::new("w", save_file);
    let exit = core::command::Command::new("q", exit_application);
    let save_and_exit = core::command::Command::new("wq", save_and_exit);

    vec![save, exit, save_and_exit]
}

use crate::core::{
    self,
    actions::default_actions::{
        exit_application, move_down, move_left, move_right, move_up,
        save_and_exit, save_file,
    },
    keys::{
        key::Key,
        keybind::{IOOperation, KeyBind, Operation},
        keycode::KeyCode,
    },
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
    let move_left = KeyBind::new(
        Key::bind(KeyCode::LowerH),
        Operation::IO(IOOperation::Write),
        Some(move_left),
    );

    let move_right = KeyBind::new(
        Key::bind(KeyCode::LowerL),
        Operation::IO(IOOperation::Write),
        Some(move_right),
    );

    let move_up = KeyBind::new(
        Key::bind(KeyCode::LowerK),
        Operation::IO(IOOperation::Write),
        Some(move_up),
    );

    let move_down = KeyBind::new(
        Key::bind(KeyCode::LowerJ),
        Operation::IO(IOOperation::Write),
        Some(move_down),
    );

    vec![move_left, move_right, move_up, move_down]
}

fn get_commands() -> Vec<core::command::Command> {
    let save = core::command::Command::new("w", save_file);
    let exit = core::command::Command::new("q", exit_application);
    let save_and_exit = core::command::Command::new("wq", save_and_exit);

    vec![save, exit, save_and_exit]
}

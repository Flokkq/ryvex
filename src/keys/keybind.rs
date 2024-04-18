use super::key::Key;
use super::key::KeyType;
use crate::actions;
use crate::actions::error::Error;
use crate::layers::layer::TerminalLayer;

pub type ActionFn = fn() -> Result<(), Error>;

pub struct KeyBind {
    pub keys: Vec<Key>,
    pub layer: TerminalLayer,
    pub on_activate: ActionFn, // Use the type alias here
}

impl KeyBind {
    pub fn new(keys: Vec<Key>, on_activate: ActionFn) -> Self {
        let first_key = keys
            .get(0)
            .expect("KeyBind must be initialized with at least one key");

        let layer = TerminalLayer::from(&first_key.r#type);

        Self {
            keys,
            layer,
            on_activate,
        }
    }
}

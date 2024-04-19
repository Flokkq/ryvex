use super::key::Key;
use crate::actions::error::ActionError;
use crate::layers::layer::TerminalLayer;

pub enum ActionResult {
    Continue,
    Exit,
}

pub type ActionFn = fn() -> Result<ActionResult, ActionError>;

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

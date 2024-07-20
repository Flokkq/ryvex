use super::key::Key;
use crate::core::actions::error::ActionError;

pub enum ActionResult {
    Continue,
    Exit,
}

pub type ActionFn = fn() -> Result<ActionResult, ActionError>;

pub struct KeyBind {
    pub key: Key,
    pub operation: Operation,
    pub callback: Option<ActionFn>, // Use the type alias here
}

pub enum Operation {
    IO(IOOperation),
    Find,
    Select,
    Modifier,
    ExactMatch,
    Count,
}

pub enum IOOperation {
    Write,
    Read,
}

impl KeyBind {
    pub fn new(
        key: Key,
        operation: Operation,
        callback: Option<ActionFn>,
    ) -> Self {
        Self {
            key,
            operation,
            callback,
        }
    }
}

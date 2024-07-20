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
    pub callback: Option<ActionFn>,
}

#[derive(PartialEq)]
pub enum Operation {
    IO(IOOperation),
    Find,
    Select,
    Modifier,
    ExactMatch,
    Count,
}

#[derive(PartialEq)]
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
        assert!(
            !(matches!(operation, Operation::IO(_)) && callback.is_none()),
            "KeyBind with IO operation must have a callback"
        );

        Self {
            key,
            operation,
            callback,
        }
    }
}

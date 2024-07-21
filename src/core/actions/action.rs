use super::error::ActionError;

pub enum ActionResult {
    Continue,
    Exit,
}

pub type ActionFn = fn() -> Result<ActionResult, ActionError>;

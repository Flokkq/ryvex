use super::actions::action::ActionFn;

pub struct Command {
    pub alias: String,
    pub callback: ActionFn,
}

impl Command {
    pub fn new(alias: &str, callback: ActionFn) -> Self {
        assert!(
            !matches!(alias.chars().nth(0).unwrap_or('!'), '!'),
            "Custom commands cannot start with `!`"
        );

        Self {
            alias: alias.to_owned(),
            callback,
        }
    }
}

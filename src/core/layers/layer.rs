use crate::core::keys::key::KeyType;

pub enum TerminalLayer {
    Insert,
    Normal,
    Visual,
}

impl From<&KeyType> for TerminalLayer {
    fn from(value: &KeyType) -> Self {
        match value {
            KeyType::Escape | KeyType::Leader => TerminalLayer::Normal,
            _ => TerminalLayer::Insert,
        }
    }
}

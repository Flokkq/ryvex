use super::key::Key;
use super::key::KeyType;
use crate::actions;
use crate::layers::layer::TerminalLayer;

pub struct KeyBind<C>
where
    C: KeyBindCallback,
{
    pub keys: Vec<Key>,
    pub layer: TerminalLayer,
    pub on_press: C,
}

impl<C> KeyBind<C>
where
    C: KeyBindCallback,
{
    pub fn new(keys: Vec<Key>, on_press: C) -> Self {
        let first_key = keys
            .get(0)
            .expect("KeyBind must be initialized with at least one key");

        let layer = match first_key.r#type {
            KeyType::Leader | KeyType::Control | KeyType::Escape => {
                TerminalLayer::from(&first_key.r#type)
            }
            _ => {
                panic!("Invalid key type for KeyBind");
            }
        };

        Self {
            keys,
            layer,
            on_press,
        }
    }
}

pub trait KeyBindCallback {
    fn call(&self) -> Result<(), actions::error::Error>;
}

impl<F> KeyBindCallback for F
where
    F: Fn() -> Result<(), actions::error::Error>,
{
    fn call(&self) -> Result<(), actions::error::Error> {
        (*self)()
    }
}

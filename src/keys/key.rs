use super::keycode::KeyCode;

pub struct Key {
    pub r#type: KeyType,
    pub key_code: KeyCode,
}

impl Key {
    pub fn bind(key: u8) -> Self {
        let r#type = KeyType::char_to_key_type(key);

        Self {
            r#type,
            key_code: KeyCode::from_ascii(key).unwrap_or(KeyCode::Nul),
        }
    }
}

#[derive(PartialEq)]
pub enum KeyType {
    Leader,      // Could be associated with a specific key if needed
    Control,     // Control characters (0-31 and 127 in ASCII)
    Escape,      // ASCII for ESC
    Digit,       // ASCII for '0'-'9'
    Letter,      // ASCII for 'A'-'Z' and 'a'-'z'
    Punctuation, // All other punctuation characters
    Unknown,     // For all other ASCII values not categorized
}

impl KeyType {
    pub fn char_to_key_type(c: u8) -> KeyType {
        match c {
            27 => KeyType::Escape,
            0..=31 | 127 => KeyType::Control,
            b'0'..=b'9' => KeyType::Digit,
            b'A'..=b'Z' | b'a'..=b'z' => KeyType::Letter,
            32..=47 | 58..=64 | 91..=96 | 123..=126 => KeyType::Punctuation,
            _ => KeyType::Unknown,
        }
    }
}

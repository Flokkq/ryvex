use std::hash::Hash;

use crate::key::AsciiKeyCode;

pub mod source;
pub mod stream;

#[derive(PartialEq, Debug, Clone, Copy, Hash)]
pub enum Event {
	Key(AsciiKeyCode),
	Resize(u16, u16),
}

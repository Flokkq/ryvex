use std::hash::Hash;

use crate::key::{
	AsciiKeyCode,
	KeyModifiers,
};

#[derive(PartialEq, Debug, Clone, Copy, Hash)]
pub enum Event {
	Key(KeyEvent),
	Resize(u16, u16),
}

#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
	pub code:      AsciiKeyCode,
	pub modifiers: KeyModifiers,
}

impl KeyEvent {
	pub fn new(code: AsciiKeyCode, modifiers: KeyModifiers) -> KeyEvent {
		KeyEvent { code, modifiers }
	}

	fn normalize_case(mut self) -> KeyEvent {
		if self.code.to_char().is_uppercase() {
			self.modifiers = self.modifiers | KeyModifiers::SHIFT;
		} else if self.modifiers.contains(KeyModifiers::SHIFT) {
			self.code = AsciiKeyCode::from_ascii(
				self.code.to_char().to_ascii_uppercase() as u8,
			);
		}

		self
	}
}

impl Eq for KeyEvent {}

impl Hash for KeyEvent {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		let KeyEvent { code, modifiers } = self.normalize_case();
		code.hash(state);
		modifiers.hash(state);
	}
}

impl PartialEq for KeyEvent {
	fn eq(&self, other: &Self) -> bool {
		let KeyEvent {
			code: lhs_code,
			modifiers: lhs_modifiers,
		} = self.normalize_case();
		let KeyEvent {
			code: rhs_code,
			modifiers: rhs_modifiers,
		} = other.normalize_case();

		(lhs_code == rhs_code) && (lhs_modifiers == rhs_modifiers)
	}
}

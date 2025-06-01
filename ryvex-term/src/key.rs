use std::fmt::Display;

/// Represents an ASCII key code.
///
/// This enum covers all standard ASCII characters, including control
/// characters, punctuation, digits, and letters. Any non-ASCII value
/// passed into a conversion is mapped to [`KeyCode::Nul`].
#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy, Hash)]
pub enum AsciiKeyCode {
	Nul,
	Soh,
	Stx,
	Etx,
	Eot,
	Enq,
	Ack,
	Bell,
	Backspace,
	Tab,
	LineFeed,
	Vt,
	Ff,
	CarriageReturn,
	So,
	Si,
	Dle,
	Dc1,
	Dc2,
	Dc3,
	Dc4,
	Nak,
	Syn,
	Etb,
	Can,
	Em,
	Sub,
	Esc,
	Fs,
	Gs,
	Rs,
	Us,
	Space,
	Exclamation,
	DoubleQuote,
	Hash,
	Dollar,
	Percent,
	Ampersand,
	Quote,
	LeftParenthesis,
	RightParenthesis,
	Asterisk,
	Plus,
	Comma,
	Minus,
	Period,
	Slash,
	Zero,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Colon,
	Semicolon,
	LessThan,
	Equal,
	GreaterThan,
	Question,
	At,
	UpperA,
	UpperB,
	UpperC,
	UpperD,
	UpperE,
	UpperF,
	UpperG,
	UpperH,
	UpperI,
	UpperJ,
	UpperK,
	UpperL,
	UpperM,
	UpperN,
	UpperO,
	UpperP,
	UpperQ,
	UpperR,
	UpperS,
	UpperT,
	UpperU,
	UpperV,
	UpperW,
	UpperX,
	UpperY,
	UpperZ,
	LeftBracket,
	Backslash,
	RightBracket,
	Caret,
	Underscore,
	Grave,
	LowerA,
	LowerB,
	LowerC,
	LowerD,
	LowerE,
	LowerF,
	LowerG,
	LowerH,
	LowerI,
	LowerJ,
	LowerK,
	LowerL,
	LowerM,
	LowerN,
	LowerO,
	LowerP,
	LowerQ,
	LowerR,
	LowerS,
	LowerT,
	LowerU,
	LowerV,
	LowerW,
	LowerX,
	LowerY,
	LowerZ,
	LeftBrace,
	Pipe,
	RightBrace,
	Tilde,
	Del,
}

impl AsciiKeyCode {
	/// Converts [`u8`] to a KeyCode. Any non-ASCII value  passed into a
	/// conversion is mapped to [`KeyCode::Nul`].
	pub fn from_ascii(value: u8) -> Self {
		value.into()
	}

	/// Converts the `KeyCode` to its corresponding ASCII character.
	///
	/// This conversion leverages the underlying ASCII value associated with
	/// each key.
	///
	/// # Examples
	/// ```rust
	/// use ryvex_term::key::AsciiKeyCode;
	/// let key = AsciiKeyCode::from_ascii(65);
	/// assert_eq!(key.to_char(), 'A');
	/// ```
	///
	/// ```rust
	/// use ryvex_term::key::AsciiKeyCode;
	/// let key = AsciiKeyCode::from_ascii(9);
	/// assert_eq!(key.to_char(), '\t');
	/// ```
	pub fn to_char(&self) -> char {
		(*self).into()
	}

	pub fn is_control_character(self) -> bool {
		let n = self as u8;

		0 < n && n < 28
	}

	pub fn is_seperator(self) -> bool {
		let n = self as u8;

		27 < n && n < 33
	}
}

impl From<u8> for AsciiKeyCode {
	fn from(ascii: u8) -> Self {
		if ascii < 128 {
			// Safety: Since `KeyCode` is #[repr(u8)] and we have exactly 128
			// variants declared in order (0 to 127), transmuting is safe for
			// values 0–127.
			unsafe { std::mem::transmute(ascii) }
		} else {
			// For any non-ASCII value, we default to Nul.
			AsciiKeyCode::Nul
		}
	}
}

impl Into<char> for AsciiKeyCode {
	fn into(self) -> char {
		self as u8 as char
	}
}

impl Display for AsciiKeyCode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AsciiKeyCode::Nul => write!(f, "<C-@>"),
			AsciiKeyCode::Soh => write!(f, "<C-A>"),
			AsciiKeyCode::Stx => write!(f, "<C-B>"),
			AsciiKeyCode::Etx => write!(f, "<C-C>"),
			AsciiKeyCode::Eot => write!(f, "<C-D>"),
			AsciiKeyCode::Enq => write!(f, "<C-E>"),
			AsciiKeyCode::Ack => write!(f, "<C-F>"),
			AsciiKeyCode::Bell => write!(f, "<C-G>"),
			AsciiKeyCode::Backspace => write!(f, "<C-H>"),
			AsciiKeyCode::Tab => write!(f, "<C-I>"),
			AsciiKeyCode::LineFeed => write!(f, "<C-J>"),
			AsciiKeyCode::Vt => write!(f, "<C-K>"),
			AsciiKeyCode::Ff => write!(f, "<C-L>"),
			AsciiKeyCode::CarriageReturn => write!(f, "<C-M>"),
			AsciiKeyCode::So => write!(f, "<C-N>"),
			AsciiKeyCode::Si => write!(f, "<C-O>"),
			AsciiKeyCode::Dle => write!(f, "<C-P>"),
			AsciiKeyCode::Dc1 => write!(f, "<C-Q>"),
			AsciiKeyCode::Dc2 => write!(f, "<C-R>"),
			AsciiKeyCode::Dc3 => write!(f, "<C-S>"),
			AsciiKeyCode::Dc4 => write!(f, "<C-T>"),
			AsciiKeyCode::Nak => write!(f, "<C-U>"),
			AsciiKeyCode::Syn => write!(f, "<C-V>"),
			AsciiKeyCode::Etb => write!(f, "<C-W>"),
			AsciiKeyCode::Can => write!(f, "<C-X>"),
			AsciiKeyCode::Em => write!(f, "<C-Y>"),
			AsciiKeyCode::Sub => write!(f, "<C-Z>"),
			AsciiKeyCode::Esc => write!(f, "<C-[>"),
			AsciiKeyCode::Fs => write!(f, "<C-\\>"),
			AsciiKeyCode::Gs => write!(f, "<C-]>"),
			AsciiKeyCode::Rs => write!(f, "<C-^>"),
			AsciiKeyCode::Us => write!(f, "<C-_>"),
			AsciiKeyCode::Del => write!(f, "<Del>"),
			_printable_key_code => write!(f, "{}", self.to_char()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn value_greater_than_127_is_converted_to_nul() {
		let key = AsciiKeyCode::from_ascii(128);
		assert_eq!(key, AsciiKeyCode::Nul);
	}

	#[test]
	fn control_character_gets_recognized() {
		let ctrl_a = AsciiKeyCode::Soh;
		let ctrl_z = AsciiKeyCode::Sub;

		assert!(ctrl_a.is_control_character());
		assert!(ctrl_z.is_control_character());

		let a = AsciiKeyCode::LowerA;
		assert!(!a.is_control_character());
	}

	#[test]
	fn seperator_character_gets_recognized() {
		let file_seperator = AsciiKeyCode::Fs;
		let unit_seperator = AsciiKeyCode::Us;

		assert!(file_seperator.is_seperator());
		assert!(unit_seperator.is_seperator());

		let a = AsciiKeyCode::LowerA;
		assert!(!a.is_seperator());
	}
}

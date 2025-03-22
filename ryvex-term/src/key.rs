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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn value_greater_than_127_is_converted_to_nul() {
		let key = AsciiKeyCode::from_ascii(128);
		assert_eq!(key, AsciiKeyCode::Nul);
	}
}

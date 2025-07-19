use core::{
	fmt::Display,
	str::FromStr,
};

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
	/// use ryvex_target::key::AsciiKeyCode;
	/// let key = AsciiKeyCode::from_ascii(65);
	/// assert_eq!(key.to_char(), 'A');
	/// ```
	///
	/// ```rust
	/// use ryvex_target::key::AsciiKeyCode;
	/// let key = AsciiKeyCode::from_ascii(9);
	/// assert_eq!(key.to_char(), '\t');
	/// ```
	pub fn to_char(&self) -> char {
		(*self).into()
	}

	pub fn is_control_character(self) -> bool {
		let n = self as u8;

		AsciiKeyCode::Nul as u8 <= n && n <= AsciiKeyCode::Esc as u8
	}

	pub fn is_seperator(self) -> bool {
		let n = self as u8;

		AsciiKeyCode::Fs as u8 <= n && n <= AsciiKeyCode::Space as u8
	}

	pub fn is_digit(self) -> bool {
		let n = self as u8;

		AsciiKeyCode::Zero as u8 <= n && n <= AsciiKeyCode::Nine as u8
	}

	pub fn parse_human_str(s: &str) -> Result<Vec<AsciiKeyCode>, String> {
		let mut iter = s.chars().peekable();
		let mut codes = Vec::new();

		while let Some(ch) = iter.next() {
			if ch == '<' {
				let mut token = String::from("<");

				while let Some(&next) = iter.peek() {
					token.push(next);
					iter.next();

					if next == '>' {
						break;
					}
				}

				if !token.ends_with('>') {
					return Err(format!("unterminated '<' in `{}`", s));
				}
				codes.push(AsciiKeyCode::from_str(&token)?);
				continue;
			}

			if !ch.is_ascii() {
				return Err(format!("non-ASCII char '{}' in `{}`", ch, s));
			}

			codes.push(AsciiKeyCode::from_ascii(ch as u8));
		}

		Ok(codes)
	}

	pub fn to_human_readable(self) -> String {
		let s = match self {
			AsciiKeyCode::Nul => "<C-@>",
			AsciiKeyCode::Soh => "<C-A>",
			AsciiKeyCode::Stx => "<C-B>",
			AsciiKeyCode::Etx => "<C-C>",
			AsciiKeyCode::Eot => "<C-D>",
			AsciiKeyCode::Enq => "<C-E>",
			AsciiKeyCode::Ack => "<C-F>",
			AsciiKeyCode::Bell => "<C-G>",
			AsciiKeyCode::Backspace => "<C-H>",
			AsciiKeyCode::Tab => "<C-I>",
			AsciiKeyCode::LineFeed => "<C-J>",
			AsciiKeyCode::Vt => "<C-K>",
			AsciiKeyCode::Ff => "<C-L>",
			AsciiKeyCode::CarriageReturn => "<C-M>",
			AsciiKeyCode::So => "<C-N>",
			AsciiKeyCode::Si => "<C-O>",
			AsciiKeyCode::Dle => "<C-P>",
			AsciiKeyCode::Dc1 => "<C-Q>",
			AsciiKeyCode::Dc2 => "<C-R>",
			AsciiKeyCode::Dc3 => "<C-S>",
			AsciiKeyCode::Dc4 => "<C-T>",
			AsciiKeyCode::Nak => "<C-U>",
			AsciiKeyCode::Syn => "<C-V>",
			AsciiKeyCode::Etb => "<C-W>",
			AsciiKeyCode::Can => "<C-X>",
			AsciiKeyCode::Em => "<C-Y>",
			AsciiKeyCode::Sub => "<C-Z>",
			AsciiKeyCode::Esc => "<C-[>",
			AsciiKeyCode::Fs => "<C-\\>",
			AsciiKeyCode::Gs => "<C-]>",
			AsciiKeyCode::Rs => "<C-^>",
			AsciiKeyCode::Us => "<C-_>",
			AsciiKeyCode::Del => "<Del>",
			_printable_key_code => &self.to_char().to_string(),
		};

		s.to_string()
	}
}

impl From<u8> for AsciiKeyCode {
	fn from(ascii: u8) -> Self {
		if ascii < 128 {
			// Safety: Since `KeyCode` is #[repr(u8)] and we have exactly 128
			// variants declared in order (0 to 127), transmuting is safe for
			// values 0–127.
			unsafe { core::mem::transmute::<u8, Self>(ascii) }
		} else {
			// For any non-ASCII value, we default to Nul.
			AsciiKeyCode::Nul
		}
	}
}

impl From<AsciiKeyCode> for char {
	fn from(val: AsciiKeyCode) -> Self {
		val as u8 as char
	}
}

impl FromStr for AsciiKeyCode {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() == 1 && s.is_ascii() {
			let byte = s.as_bytes()[0];
			return Ok(AsciiKeyCode::from_ascii(byte));
		}

		if !s.starts_with('<') || !s.ends_with('>') {
			return Err(format!(
				"invalid format: expected single char or <...>, got '{}'",
				s
			));
		}

		let inner = &s[1..s.len() - 1];

		if inner == "Del" {
			return Ok(AsciiKeyCode::Del);
		}

		if let Some(rest) = inner.strip_prefix("C-") {
			if rest.chars().count() == 1 {
				let byte = rest.as_bytes()[0];

				if (b'@'..=b'_').contains(&byte) {
					let ctrl = byte & 0x1F;
					return Ok(AsciiKeyCode::from_ascii(ctrl));
				} else {
					return Err(format!(
						"invalid control character '{}', must be between @ \
						 and _",
						byte as char
					));
				}
			}
		}

		Err(format!(
			"invalid AsciiKeyCode literal '{}'; expected either a single \
			 ASCII char, '<Del>', or '<C-?>'",
			s
		))
	}
}

impl Display for AsciiKeyCode {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}", self.to_human_readable())
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

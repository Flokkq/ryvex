use alloc::borrow::Cow;

use crate::piece_table::PieceTable;

pub trait AsKey {
	fn as_key(&self) -> Cow<'_, str>;
}

/// The one “master” enum that covers every valid combination.
///
/// - `NavigationOnly`: something like `3w` (no operator).
/// - `OperatedNavigation`: something like `d3w`, `cw`.
/// - `OperatedRange`: e.g. `di(`, `y%`, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Motion {
	NavigationOnly {
		nav:   NavigationMotion,
		count: u32,
	},
	OperatedNavigation {
		motion_type: MotionType,
		nav:         NavigationMotion,
		count:       u32,
	},
	OperatedRange {
		motion_type: MotionType,
		range:       Range,
		count:       u32,
	},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionType {
	Visual,
	Delete,
	Yank,
	Change,
	Meta(MetaOperator),
}

impl AsKey for MotionType {
	fn as_key(&self) -> Cow<'_, str> {
		match self {
			MotionType::Visual => Cow::Borrowed("v"),
			MotionType::Delete => Cow::Borrowed("d"),
			MotionType::Yank => Cow::Borrowed("y"),
			MotionType::Change => Cow::Borrowed("c"),
			MotionType::Meta(op) => Cow::Owned(format!("q{}", op.as_key())),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaOperator {
	Uppercase,
	Lowercase,
	Format,
	Rot13,
	ToggleCase,
}

impl AsKey for MetaOperator {
	fn as_key(&self) -> Cow<'_, str> {
		let s = match self {
			MetaOperator::Uppercase => "U",
			MetaOperator::Lowercase => "u",
			MetaOperator::Format => "q",
			MetaOperator::Rot13 => "?",
			MetaOperator::ToggleCase => "~",
		};

		Cow::Borrowed(s)
	}
}

/// Motions that purely move the cursor (like 'w', 'b', 'e', etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationMotion {
	CharForward,
	CharBackward,
	LineForward,
	LineBackward,

	WordForward,
	WordBackward,
	EndOfWordForward,
	EndOfWordBackward,

	LineEnd,
	LineStart,

	EmptyLineAbove,
	EmptyLineBelow,

	Bottom,
	Top,
}

impl AsKey for NavigationMotion {
	fn as_key(&self) -> Cow<'_, str> {
		let s = match self {
			NavigationMotion::CharForward => "l",
			NavigationMotion::CharBackward => "h",
			NavigationMotion::LineForward => "j",
			NavigationMotion::LineBackward => "k",

			NavigationMotion::WordForward => "w",
			NavigationMotion::WordBackward => "b",
			NavigationMotion::EndOfWordForward => "e",
			NavigationMotion::EndOfWordBackward => "ge",

			NavigationMotion::LineEnd => "$",
			NavigationMotion::LineStart => "0",

			NavigationMotion::EmptyLineAbove => "{",
			NavigationMotion::EmptyLineBelow => "}",

			NavigationMotion::Bottom => "G",
			NavigationMotion::Top => "gg",
		};

		Cow::Borrowed(s)
	}
}

/// Text-object or advanced motions (like 'i(', 'a(', '%', 'f<char>', etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Range {
	Inside(Scope),
	Around(Scope),
	ForwardTo(char),
	ForwardTill(char),
	BackwardsTo(char),
	BackwardsTill(char),
	Word,
	Line,
	ForwardSearch(String),
	BackwardSearch(String),
	Mark(char),

	Percent(Scope),
	GoToLine(GoToLineNumber),
	SentenceEnd,
	SentenceStart,
}

impl AsKey for Range {
	fn as_key(&self) -> Cow<'_, str> {
		match self {
			Range::Inside(scope) => Cow::Owned(format!("i{}", scope.as_key())),
			Range::Around(scope) => Cow::Owned(format!("a{}", scope.as_key())),
			Range::ForwardTo(c) => Cow::Owned(format!("f{}", c)),
			Range::ForwardTill(c) => Cow::Owned(format!("t{}", c)),
			Range::BackwardsTo(c) => Cow::Owned(format! {"F{}", c}),
			Range::BackwardsTill(c) => Cow::Owned(format!("T{}", c)),
			Range::Word => Cow::Borrowed("w"),
			Range::Line => Cow::Borrowed("l"),
			Range::ForwardSearch(s) => Cow::Owned(format!("/{}", s)),
			Range::BackwardSearch(s) => Cow::Owned(format!("?{}", s)),
			Range::Mark(c) => Cow::Owned(format!("m{}", c)),
			Range::Percent(scope) => Cow::Owned(format!("%{}", scope.as_key())),
			Range::GoToLine(num) => num.as_key(),
			Range::SentenceEnd => Cow::Borrowed(")"),
			Range::SentenceStart => Cow::Borrowed("("),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoToLineNumber {
	First,
	Last,
	Number(u32),
}

impl AsKey for GoToLineNumber {
	fn as_key(&self) -> Cow<'_, str> {
		match self {
			GoToLineNumber::First => Cow::Borrowed("gg"),
			GoToLineNumber::Last => Cow::Borrowed("G"),
			GoToLineNumber::Number(num) => Cow::Owned(num.to_string()),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
	Parentheses,
	Brackets,
	Braces,
	AngleBrackets,
	SingleQuote,
	DoubleQuote,
	Backtick,
	Word,
	Paragraph,
}

impl AsKey for Scope {
	fn as_key(&self) -> Cow<'_, str> {
		let s = match self {
			Scope::Parentheses => "(",
			Scope::Brackets => "[",
			Scope::Braces => "{",
			Scope::AngleBrackets => "<",
			Scope::SingleQuote => "'",
			Scope::DoubleQuote => "\"",
			Scope::Backtick => "`",
			Scope::Word => "w",
			Scope::Paragraph => "p",
		};

		Cow::Borrowed(s)
	}
}

pub fn apply(_buffer: &mut PieceTable, _motion: Motion) -> Option<String> {
	None
}

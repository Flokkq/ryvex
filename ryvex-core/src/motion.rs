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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaOperator {
	Uppercase,  // gU
	Lowercase,  // gu
	Format,     // gq
	Rot13,      // g?
	ToggleCase, // g~
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoToLineNumber {
	First,
	Last,
	Number(u32),
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

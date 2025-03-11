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
    GoToLine(GoToLine),
    SentenceEnd,
    SentenceStart,
}

pub enum GoToLine {
    First,
    Last,
    Number(u32),
}

pub enum MotionType {
    Delete,
    Yank,
    Change,
}

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

pub struct Motion {
    motion_type: MotionType,
    count: u32,
    range: Option<Range>,
}

enum Range {
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
}

enum GoToLine {
    First,
    Last,
    Number(u32),
}

enum MotionType {
    Delete,
    Yank,
    Change,
}

enum Scope {
    Parentheses,
    Brackets,
    Braces,
    AngleBrackets,
    SingleQuote,
    DoubleQuote,
    Backtick,
    Word,
    Sentence,
    Paragraph,
    Tag,
    Block,
    Line,
}

struct Motion {
    motion_type: MotionType,
    count: u32,
    range: Option<Range>,
}

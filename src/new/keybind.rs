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

enum ActionType {
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

struct Action {
    action_type: ActionType,
    count: u32,
    range: Option<Range>,
}

impl Action {
    pub fn inside(scope: Scope, action_type: ActionType) -> Self {
        Self {
            action_type,
            count: 1,
            range: Some(Range::Inside(scope)),
        }
    }

    pub fn around(scope: Scope, action_type: ActionType) -> Self {
        Self {
            action_type,
            count: 1,
            range: Some(Range::Around(scope)),
        }
    }

    pub fn word(action_type: ActionType) -> Self {
        Self {
            action_type,
            count: 1,
            range: Some(Range::Word),
        }
    }

    pub fn line(action_type: ActionType, count: u32) -> Self {
        Self {
            action_type,
            count,
            range: Some(Range::Line),
        }
    }

    pub fn forward_to(target: char, action_type: ActionType) -> Self {
        Self {
            action_type,
            count: 1,
            range: Some(Range::ForwardTo(target)),
        }
    }

    pub fn forward_till(target: char, action_type: ActionType) -> Self {
        Self {
            action_type,
            count: 1,
            range: Some(Range::ForwardTill(target)),
        }
    }

    pub fn backwards_to(target: char, action_type: ActionType) -> Self {
        Self {
            action_type,
            count: 1,
            range: Some(Range::BackwardsTo(target)),
        }
    }

    pub fn backwards_till(target: char, action_type: ActionType) -> Self {
        Self {
            action_type,
            count: 1,
            range: Some(Range::BackwardsTill(target)),
        }
    }

    pub fn forward_search(query: String, action_type: ActionType) -> Self {
        Self {
            action_type,
            count: 1,
            range: Some(Range::ForwardSearch(query)),
        }
    }

    pub fn backward_search(query: String, action_type: ActionType) -> Self {
        Self {
            action_type,
            count: 1,
            range: Some(Range::BackwardSearch(query)),
        }
    }

    pub fn mark(mark: char, action_type: ActionType) -> Self {
        Self {
            action_type,
            count: 1,
            range: Some(Range::Mark(mark)),
        }
    }

    pub fn percent(scope: Scope, action_type: ActionType, count: u32) -> Self {
        Self {
            action_type,
            count,
            range: Some(Range::Percent(scope)),
        }
    }

    pub fn goto_line(
        goto: GoToLine,
        action_type: ActionType,
        count: u32,
    ) -> Self {
        Self {
            action_type,
            count,
            range: Some(Range::GoToLine(goto)),
        }
    }
}

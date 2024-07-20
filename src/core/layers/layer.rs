#[derive(PartialEq)]
pub enum TerminalLayer {
    Insert,
    Normal,
    Replace,
    Visual(VisualLayer),
}

#[derive(PartialEq)]
pub enum VisualLayer {
    Line,
    Block,
}

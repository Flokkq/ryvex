pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub fn place() -> Cursor {
        Cursor { x: 0, y: 0 }
    }
}

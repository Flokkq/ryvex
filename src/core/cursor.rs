use std::usize;

pub struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    pub fn place(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn move_up(&mut self, buffer: &String) {
        if self.y > 0 {
            self.y -= 1;
            self.x = buffer.lines().nth(self.y).map_or(0, |line| line.len());
        }
    }

    pub fn move_down(&mut self, buffer: &String) {
        let line_count = buffer.lines().count();
        if self.y + 1 < line_count {
            self.y += 1;
            self.x = buffer.lines().nth(self.y).map_or(0, |line| line.len());
        }
    }

    pub fn move_right(&mut self, buffer: &String) {
        let line_length =
            buffer.lines().nth(self.y).map_or(0, |line| line.len());
        if self.x < line_length {
            self.x += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_left_n(&mut self, n: usize) {
        if self.x >= n {
            self.x -= n;
        } else {
            self.x = 0;
        }
    }

    pub fn move_right_n(&mut self, n: usize, buffer: &String) {
        let line_length =
            buffer.lines().nth(self.y).map_or(0, |line| line.len());
        if self.x + n <= line_length {
            self.x += n;
        } else {
            self.x = line_length;
        }
    }

    pub fn move_up_n(&mut self, n: usize, buffer: &String) {
        for _ in 0..n {
            self.move_up(buffer);
        }
    }

    pub fn move_down_n(&mut self, n: usize, buffer: &String) {
        for _ in 0..n {
            self.move_down(buffer);
        }
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.get_x(), self.get_y())
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}

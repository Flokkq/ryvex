use core::str;
use std::path::PathBuf;

use crate::{error::Error, file_access::FileAccess};

pub struct OpenFile {
    pub path: PathBuf,
    pub buffer: String,
    pub cursor: Cursor,
}

impl OpenFile {
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        let mut buffer = String::new();
        FileAccess::read_from_file_if_exists(&path, &mut buffer)?;

        let lines = buffer.lines().collect::<Vec<&str>>();
        let y = lines.len().saturating_sub(1);
        let x = lines.last().map_or(0, |line| line.len());

        let cursor = Cursor { x, y };

        Ok(OpenFile {
            path,
            buffer,
            cursor,
        })
    }
}

pub struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
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

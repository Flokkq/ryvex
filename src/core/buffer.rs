use std::{
    collections::VecDeque,
    io::{StdoutLock, Write},
    usize,
};

use super::{cursor::Cursor, error::Error, keys::keycode::EscapeSequence};

pub struct Buffer {
    content: String,
    cursor: Cursor,
    history: VecDeque<BufferState>,
    _selection: Option<(usize, usize)>,
}

impl Buffer {
    const MAX_HISTORY: usize = 100;

    pub fn new(content: String) -> Self {
        let lines = content.lines().collect::<Vec<&str>>();
        let y = lines.len().saturating_sub(1);
        let x = lines.last().map_or(0, |line| line.len());

        Buffer {
            content,
            cursor: Cursor::place(x, y),
            history: VecDeque::new(),
            _selection: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.content.is_empty();
    }

    pub fn get_content(&self) -> &String {
        return &self.content;
    }

    pub fn insert(&mut self, ch: char) {
        let insert_pos =
            self.cursor_pos_to_index(self.cursor.get_x(), self.cursor.get_y());

        self.content.insert(insert_pos, ch);
        self.cursor.move_right(&self.content);
        self.record_state();
    }

    pub fn delete(&mut self) {
        if !self.content.is_empty() {
            let delete_pos = self
                .cursor_pos_to_index(self.cursor.get_x(), self.cursor.get_y());

            if delete_pos > 0 {
                self.content.remove(delete_pos - 1);
                self.cursor.move_left();
                self.record_state();
            }
        }
    }

    fn cursor_pos_to_index(&self, x: usize, y: usize) -> usize {
        if y >= self.content.lines().count() {
            return self.content.len();
        }

        self.content
            .lines()
            .take(y)
            .map(|line| line.len() + 1)
            .sum::<usize>()
            + x.min(self.content.lines().nth(y).unwrap().len())
    }

    fn record_state(&mut self) {
        if self.history.len() > Self::MAX_HISTORY {
            self.history.pop_front();
        }
        self.history.push_back(BufferState {
            _content: self.content.clone(),
            _cursor_position: self.cursor.get_position(),
        });
    }

    pub fn move_cursor(&mut self, direction: EscapeSequence) {
        match direction {
            EscapeSequence::ArrowUp => self.cursor.move_up(&self.content),
            EscapeSequence::ArrowDown => self.cursor.move_down(&self.content),
            EscapeSequence::ArrowRight => self.cursor.move_right(&self.content),
            EscapeSequence::ArrowLeft => self.cursor.move_left(),
        }
    }

    pub fn display(&self, stdout: &mut StdoutLock) -> Result<(), Error> {
        stdout.write_all(self.content.as_bytes())?;

        let cursor_position = format!(
            "\x1B[{};{}H",
            self.cursor.get_y() + 1,
            self.cursor.get_x() + 1
        );

        stdout.write_all(cursor_position.as_bytes())?;
        Ok(())
    }
}

struct BufferState {
    _content: String,
    _cursor_position: (usize, usize),
}

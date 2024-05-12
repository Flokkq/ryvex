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

        if insert_pos < self.content.len() {
            let (before, after) = self.content.split_at(insert_pos);
            self.content = format!("{}{}{}", before, ch, after);
        } else {
            self.content.push(ch);
        }

        self.cursor.move_right(&self.content);
        self.record_state();
    }

    pub fn insert_newline(&mut self) {
        self.insert('\n');
        self.cursor.move_down(&self.content);
        self.cursor.move_to(0, self.cursor.get_y());
    }

    pub fn delete(&mut self) {
        if self.content.is_empty() {
            return;
        }

        let delete_pos =
            self.cursor_pos_to_index(self.cursor.get_x(), self.cursor.get_y());

        if delete_pos == 0 {
            return;
        }

        let prev_char_pos = delete_pos - 1;
        let mut deleted_len = 1;

        if self.content.chars().nth(prev_char_pos).unwrap() == '\n' {
            // If the character to delete is a newline, adjust the deleted length
            // and check if merging with the next line is needed
            if let Some(next_line_end) = self.content[delete_pos..].find('\n') {
                deleted_len += next_line_end + 1;
            }
        }

        self.content.drain(prev_char_pos..delete_pos);

        self.cursor.move_left_n(deleted_len);
        self.record_state();
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
            EscapeSequence::ArrowUp => {
                if self.cursor.get_y() > 0 {
                    self.cursor.move_up(&self.content);
                }
            }
            EscapeSequence::ArrowDown => {
                let num_lines = self.content.lines().count();
                if self.cursor.get_y() + 1 < num_lines {
                    self.cursor.move_down(&self.content);
                }
            }
            EscapeSequence::ArrowRight => {
                let current_line =
                    self.content.lines().nth(self.cursor.get_y()).unwrap_or("");
                if self.cursor.get_x() + 1 < current_line.len() {
                    self.cursor.move_right(&self.content);
                } else if self.cursor.get_y() + 1 < self.content.lines().count()
                {
                    self.cursor.move_to(0, self.cursor.get_y() + 1);
                }
            }
            EscapeSequence::ArrowLeft => {
                if self.cursor.get_x() > 0 {
                    self.cursor.move_left();
                } else if self.cursor.get_y() > 0 {
                    let prev_line_len = self
                        .content
                        .lines()
                        .nth(self.cursor.get_y() - 1)
                        .unwrap_or("")
                        .len();
                    self.cursor.move_to(prev_line_len, self.cursor.get_y() - 1);
                }
            }
        }
    }

    pub fn display(&self, stdout: &mut StdoutLock) -> Result<(), Error> {
        let mut lines = self.content.lines();

        if let Some(line) = lines.next() {
            stdout.write_all(line.as_bytes())?;
        }

        for line in lines {
            stdout.write_all(b"\r\n")?;
            stdout.write_all(line.as_bytes())?;
        }

        let cursor_position = format!(
            "\x1B[{};{}H",
            self.cursor.get_y() + 1,
            self.cursor.get_x() + 1
        );
        stdout.write_all(cursor_position.as_bytes())?;
        stdout.flush()?;
        Ok(())
    }
}

struct BufferState {
    _content: String,
    _cursor_position: (usize, usize),
}

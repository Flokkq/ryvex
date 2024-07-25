use std::{
    collections::VecDeque,
    io::{StdoutLock, Write},
    usize,
};

use super::{
    cursor::Cursor,
    error::Error,
    keys::keycode::EscapeSequence,
    layer::{TerminalLayer, VisualLayer},
};

pub struct Buffer {
    content: String,
    cursor: Cursor,
    layer: TerminalLayer,
    history: VecDeque<BufferState>,
    selection: Option<(Cursor, Cursor)>,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<EscapeSequence> for Direction {
    fn from(value: EscapeSequence) -> Self {
        match value {
            EscapeSequence::ArrowLeft => Direction::Left,
            EscapeSequence::ArrowRight => Direction::Right,
            EscapeSequence::ArrowUp => Direction::Up,
            EscapeSequence::ArrowDown => Direction::Down,
        }
    }
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
            layer: TerminalLayer::Normal,
            history: VecDeque::new(),
            selection: None,
        }
    }

    pub fn change_layer(&mut self, layer: TerminalLayer) {
        self.layer = layer;
    }

    pub fn layer(&self) -> &TerminalLayer {
        &self.layer
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn content(&self) -> &String {
        &self.content
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

    pub fn reset_selection(&mut self) {
        self.selection = None;
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

    pub fn move_cursor(&mut self, direction: Direction) {
        match self.layer {
            TerminalLayer::Visual(_) => self.move_cursor_visual(direction),
            _ => self.move_cursor_normal(direction),
        }
    }

    // in VISUAL mode
    // .0 cursor moves to the left and up
    // .1 cursor moves to the righ and down
    // cursors cannot surpass the default cursor in the opposite direction
    // if a cursor would surpass the default cursor it get the pos of the default cursor
    // and the overflow gets applied to the cursor that goes into the other direction
    fn move_cursor_visual(&mut self, direction: Direction) {
        if self.selection.is_none() {
            self.selection = Some((self.cursor.clone(), self.cursor.clone()));
        }

        if let Some((mut start, mut end)) = self.selection.to_owned() {
            match direction {
                Direction::Up => {
                    if end.get_y() > 0 {
                        end.move_up(&self.content);
                    }
                    if end.get_y() < start.get_y()
                        || (end.get_y() == start.get_y()
                            && end.get_x() < start.get_x())
                    {
                        start = end.clone();
                    }
                }
                Direction::Down => {
                    let num_lines = self.content.lines().count();
                    if end.get_y() + 1 < num_lines {
                        end.move_down(&self.content);
                    }
                    if end.get_y() > start.get_y()
                        || (end.get_y() == start.get_y()
                            && end.get_x() > start.get_x())
                    {
                        start = end.clone();
                    }
                }
                Direction::Right => {
                    let current_line =
                        self.content.lines().nth(end.get_y()).unwrap_or("");
                    if end.get_x() + 1 < current_line.len() {
                        end.move_right(&self.content);
                    } else if end.get_y() + 1 < self.content.lines().count() {
                        end.move_to(0, end.get_y() + 1);
                    }
                    if end.get_y() > start.get_y()
                        || (end.get_y() == start.get_y()
                            && end.get_x() > start.get_x())
                    {
                        start = end.clone();
                    }
                }
                Direction::Left => {
                    if end.get_x() > 0 {
                        end.move_left();
                    } else if end.get_y() > 0 {
                        let prev_line_len = self
                            .content
                            .lines()
                            .nth(end.get_y() - 1)
                            .unwrap_or("")
                            .len();
                        end.move_to(prev_line_len, end.get_y() - 1);
                    }
                    if end.get_y() < start.get_y()
                        || (end.get_y() == start.get_y()
                            && end.get_x() < start.get_x())
                    {
                        start = end.clone();
                    }
                }
            }

            self.selection = Some((start, end.clone()));
            self.cursor = end;
        }
    }

    fn move_cursor_normal(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.cursor.get_y() > 0 {
                    self.cursor.move_up(&self.content);
                }
            }
            Direction::Down => {
                let num_lines = self.content.lines().count();
                if self.cursor.get_y() + 1 < num_lines {
                    self.cursor.move_down(&self.content);
                }
            }
            Direction::Right => {
                let current_line =
                    self.content.lines().nth(self.cursor.get_y()).unwrap_or("");
                if self.cursor.get_x() + 1 < current_line.len() {
                    self.cursor.move_right(&self.content);
                } else if self.cursor.get_y() + 1 < self.content.lines().count()
                {
                    self.cursor.move_to(0, self.cursor.get_y() + 1);
                }
            }
            Direction::Left => {
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
        // ANSI escape codes for background color (light gray)
        const SELECTED_BG_COLOR: &str = "\x1B[48;5;252m"; // Light gray background
        const RESET_COLOR: &str = "\x1B[0m"; // Reset to default color

        let lines = self.content.lines();
        let num_lines = self.content.lines().count();

        let mut current_line_index = 0;

        let (start_cursor, end_cursor) =
            if let TerminalLayer::Visual(_) = self.layer {
                self.selection
                    .clone()
                    .unwrap_or((self.cursor.clone(), self.cursor.clone()))
            } else {
                (self.cursor.clone(), self.cursor.clone())
            };

        for line in lines {
            if current_line_index >= start_cursor.get_y()
                && current_line_index <= end_cursor.get_y()
            {
                let line_start_index =
                    if current_line_index == start_cursor.get_y() {
                        start_cursor.get_x()
                    } else {
                        0
                    };

                let line_end_index = if current_line_index == end_cursor.get_y()
                {
                    end_cursor.get_x()
                } else {
                    line.len()
                };

                // Print the line before the selection
                if line_start_index > 0 {
                    stdout.write_all(&line[..line_start_index].as_bytes())?;
                }

                // Print the selected part with background color
                if line_end_index > line_start_index {
                    stdout.write_all(SELECTED_BG_COLOR.as_bytes())?;
                    stdout.write_all(
                        &line[line_start_index..line_end_index].as_bytes(),
                    )?;
                    stdout.write_all(RESET_COLOR.as_bytes())?;
                }

                // Print the rest of the line
                if line_end_index < line.len() {
                    stdout.write_all(&line[line_end_index..].as_bytes())?;
                }
            } else {
                stdout.write_all(line.as_bytes())?;
            }

            if current_line_index < num_lines - 1 {
                stdout.write_all(b"\r\n")?;
            }
            current_line_index += 1;
        }

        // print correct cursor in visual mode
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

use crate::core::error;
use std::{path::PathBuf, usize};

use crate::file_access::FileAccess;

use super::{
    iter::BufferContent,
    motion::{Range, Scope},
};

struct Buffer {
    content: BufferContent,
    path: Option<PathBuf>,
}

impl Buffer {
    pub fn scratch() -> Self {
        Self {
            content: BufferContent::new(String::new()),
            path: None,
        }
    }

    pub fn open(path: PathBuf) -> Result<Self, error::Error> {
        let mut content = String::new();
        FileAccess::read_from_file_if_exists(&path, &mut content)?;

        Ok(Self {
            content: BufferContent::new(content),
            path: Some(path),
        })
    }

    pub fn save(&mut self) -> Result<(), error::Error> {
        match &self.path {
            Some(path) => {
                FileAccess::write_to_file(&path, self.content.inner())
            }

            // No filename
            None => Err(error::Error::Unexpected),
        }
    }

    pub fn save_to(&mut self, path: PathBuf) -> Result<(), error::Error> {
        if path.exists() {
            // File exists (add ! to override)
            return Err(error::Error::Unexpected);
        }

        self.path = Some(path);
        self.save()
    }

    pub fn save_to_forced(
        &mut self,
        path: PathBuf,
    ) -> Result<(), error::Error> {
        self.path = Some(path);
        self.save()
    }

    pub fn insert(&mut self, ch: char) {
        self.content.insert_at_current_index(&ch.to_string());
    }

    pub fn yank(&mut self, range: Range) -> Option<&str> {
        let range = self.motion_range_to_range(range)?;
        self.content.yank(range)
    }

    pub fn delete(&mut self, range: Range) -> Option<String> {
        let range = self.motion_range_to_range(range)?;
        self.content.delete(range)
    }

    fn motion_range_to_range(
        &mut self,
        range: Range,
    ) -> Option<std::ops::Range<usize>> {
        match range {
            Range::Inside(scope) => self.scope_to_range(scope),
            Range::Around(scope) | Range::Percent(scope) => {
                let mut range = self.scope_to_range(scope)?;

                assert!(
                    range.start > 0,
                    "The scope character must be to the left of the selected content."
                );

                range.start += 1;
                range.end += 1;

                Some(range)
            }
            Range::ForwardTo(ch) => self.content.find_next_range(ch),
            Range::ForwardTill(ch) => {
                let range = self.content.find_next_range(ch)?;
                Some(range.start..range.end + 1)
            }
            Range::BackwardsTo(ch) => self.content.find_previous_range(ch),
            Range::BackwardsTill(ch) => {
                let range = self.content.find_previous_range(ch)?;
                Some(range.start - 1..range.end)
            }
            Range::Word => self.content.find_next_range(' '),
            Range::Line => self.content.find_block("\n", "\n"),
            Range::SentenceEnd => todo!(),
            Range::SentenceStart => todo!(),
            Range::GoToLine(_) => todo!(),
            Range::Mark(_) => todo!(),
            Range::ForwardSearch(_) => todo!(),
            Range::BackwardSearch(_) => todo!(),
        }
    }

    fn scope_to_range(
        &mut self,
        scope: Scope,
    ) -> Option<std::ops::Range<usize>> {
        match scope {
            Scope::Parentheses => self.content.find_block("(", ")"),
            Scope::Brackets => self.content.find_block("[", "]"),
            Scope::Braces => self.content.find_block("{", "}"),
            Scope::AngleBrackets => self.content.find_block("<", ">"),
            Scope::SingleQuote => self.content.find_block("'", "'"),
            Scope::DoubleQuote => self.content.find_block("\"", "\""),
            Scope::Backtick => self.content.find_block("`", "`"),
            Scope::Word => self.content.find_block(" ", " "),
            Scope::Paragraph => self.content.find_block("\n\n", "\n\n"),
        }
    }
}

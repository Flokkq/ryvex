use crate::core::error;
use std::{path::PathBuf, usize};

use crate::file_access::FileAccess;

use super::{
    iter::BufferContent,
    motion::{NavigationMotion, Range, Scope},
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

    pub fn insert(&mut self, ch: char) -> bool {
        self.content.insert_at_current_index(&ch.to_string())
    }

    pub fn yank_range(&mut self, range: Range) -> Option<&str> {
        let range = self.motion_range_to_range(range)?;
        self.content.yank(range)
    }

    pub fn yank_navigation_motion(
        &mut self,
        motion: NavigationMotion,
    ) -> Option<&str> {
        let range = self.navigation_motion_to_range(motion)?;
        self.content.yank(range)
    }

    pub fn delete_range(&mut self, range: Range) -> Option<String> {
        let range = self.motion_range_to_range(range)?;
        self.content.delete(range)
    }

    pub fn delete_navigation_motion(
        &mut self,
        motion: NavigationMotion,
    ) -> Option<String> {
        let range = self.navigation_motion_to_range(motion)?;
        self.content.delete(range)
    }

    fn motion_range_to_range(
        &mut self,
        range: Range,
    ) -> Option<std::ops::Range<usize>> {
        match range {
            Range::Inside(scope) => {
                let range = self.scope_to_range(scope)?;
                Some(range.start + 1..range.end - 1)
            }
            Range::Around(scope) | Range::Percent(scope) => {
                self.scope_to_range(scope)
            }
            Range::ForwardTo(ch) => self.content.find_next_range(ch),
            Range::ForwardTill(ch) => {
                let range = self.content.find_next_range(ch)?;
                Some(range.start..range.end + 1)
            }
            Range::BackwardsTo(ch) => {
                let range = self.content.find_previous_range(ch)?;
                Some(range.start + 1..range.end)
            }
            Range::BackwardsTill(ch) => self.content.find_previous_range(ch),
            Range::Word => self.content.find_next_range(' '),
            Range::Line => {
                let range = self.content.find_block("\n", "\n")?;
                Some(range.start..range.end - 1)
            }
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

    fn navigation_motion_to_range(
        &mut self,
        motion: NavigationMotion,
    ) -> Option<std::ops::Range<usize>> {
        match motion {
            NavigationMotion::CharForward => self.content.next(),
            NavigationMotion::CharBackward => self.content.prev(),

            NavigationMotion::WordForward
            | NavigationMotion::EndOfWordForward => self
                .content
                .find_next_range(' ')
                .or_else(|| Some(self.content.get_index()..self.content.len())),
            NavigationMotion::WordBackward
            | NavigationMotion::EndOfWordBackward => self
                .content
                .find_previous_range(' ')
                .or_else(|| Some(0..self.content.get_index())),

            NavigationMotion::LineForward => self.content.peek_line_ahead(),
            NavigationMotion::LineBackward => self.content.peek_line_behind(),
            NavigationMotion::LineStart => {
                self.content.find_previous_range('\n')
            }
            NavigationMotion::LineEnd => self.content.find_next_range('\n'),
            NavigationMotion::Top => {
                self.content.update_index_to(0);
                self.content.peek_line_ahead()
            }
            NavigationMotion::Bottom => {
                let idx = self.content.len();
                self.content.update_index_to(idx);
                self.content
                    .find_previous_range('\n')
                    .or_else(|| self.content.get_substring_from(0))
            }
            NavigationMotion::EmptyLineAbove => todo!(),
            NavigationMotion::EmptyLineBelow => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::motion::{Range, Scope};
    use super::*;

    #[test]
    fn test_insert_single_char() {
        let mut buffer = Buffer::scratch();
        assert_eq!(buffer.content.inner(), "");

        buffer.insert('h');
        assert_eq!(buffer.content.inner(), "h");

        buffer.insert('i');
        assert_eq!(buffer.content.inner(), "hi");
    }

    #[test]
    fn test_insert_multiple_chars() {
        let mut buffer = Buffer::scratch();
        for ch in "hello world".chars() {
            buffer.insert(ch);
        }
        assert_eq!(buffer.content.inner(), "hello world");
    }

    #[test]
    fn test_delete_inside_parentheses() {
        let mut buffer = Buffer {
            content: BufferContent::new("function(param) another".to_string()),
            path: None,
        };

        let open_paren_index = buffer.content.inner().find('(').unwrap() + 1;
        buffer.content.update_index_to(open_paren_index);

        let deleted = buffer.delete_range(Range::Inside(Scope::Parentheses));
        assert_eq!(deleted, Some("param".to_string()));
        assert_eq!(buffer.content.inner(), "function() another");
    }

    #[test]
    fn test_delete_around_parentheses() {
        let mut buffer = Buffer {
            content: BufferContent::new("function(param) another".to_string()),
            path: None,
        };

        let open_paren_index = buffer.content.inner().find('(').unwrap() + 1;
        buffer.content.update_index_to(open_paren_index);

        let deleted = buffer.delete_range(Range::Around(Scope::Parentheses));
        assert_eq!(deleted, Some("(param)".to_string()));
        assert_eq!(buffer.content.inner(), "function another");
    }

    #[test]
    fn test_yank_inside_quotes() {
        let mut buffer = Buffer {
            content: BufferContent::new(r#"Hello "there" friend"#.to_string()),
            path: None,
        };

        let quote_index = buffer.content.inner().find('"').unwrap() + 1;
        buffer.content.update_index_to(quote_index);

        let yanked = buffer.yank_range(Range::Inside(Scope::DoubleQuote));
        assert_eq!(yanked, Some("there"));
        assert_eq!(buffer.content.inner(), r#"Hello "there" friend"#);
    }

    #[test]
    fn test_delete_around_quotes() {
        let mut buffer = Buffer {
            content: BufferContent::new(r#"Hello "there" friend"#.to_string()),
            path: None,
        };

        let quote_index = buffer.content.inner().find('"').unwrap() + 1;
        buffer.content.update_index_to(quote_index);

        let deleted = buffer.delete_range(Range::Around(Scope::DoubleQuote));
        assert_eq!(deleted, Some(r#""there""#.to_string()));
        assert_eq!(buffer.content.inner(), "Hello  friend");
    }

    #[test]
    fn test_yank_word() {
        let mut buffer = Buffer {
            content: BufferContent::new("one two three".to_string()),
            path: None,
        };

        let yanked = buffer.yank_range(Range::Word);
        assert_eq!(yanked, Some("one"));
        assert_eq!(buffer.content.inner(), "one two three");
    }

    #[test]
    fn test_delete_line() {
        let mut buffer = Buffer {
            content: BufferContent::new("line1\nline2\nline3".to_string()),
            path: None,
        };

        let index = buffer.content.inner().find('2').unwrap();
        buffer.content.update_index_to(index);

        let deleted = buffer.delete_range(Range::Line);
        assert_eq!(deleted, Some("\nline2".to_string()));
        assert_eq!(buffer.content.inner(), "line1\nline3");
    }

    #[test]
    fn test_delete_forward_till_char() {
        let mut buffer = Buffer {
            content: BufferContent::new("abc def".to_string()),
            path: None,
        };

        let deleted = buffer.delete_range(Range::ForwardTill('c'));
        assert_eq!(deleted, Some("abc".to_string()));
        assert_eq!(buffer.content.inner(), " def");
    }

    #[test]
    fn test_delete_backwards_till_char() {
        let mut buffer = Buffer {
            content: BufferContent::new("abc def".to_string()),
            path: None,
        };

        if let Some(c_index) = buffer.content.inner().find('c') {
            buffer.content.update_index_to(c_index);
        }

        let deleted = buffer.delete_range(Range::BackwardsTill('a'));
        assert_eq!(deleted, Some("ab".to_string()));
        assert_eq!(buffer.content.inner(), "c def");
    }

    #[test]
    fn test_delete_forward_to_char() {
        let mut buffer = Buffer {
            content: BufferContent::new("abcdef".to_string()),
            path: None,
        };

        let deleted = buffer.delete_range(Range::ForwardTo('c'));
        assert_eq!(deleted, Some("ab".to_string()));
        assert_eq!(buffer.content.inner(), "cdef");
    }

    #[test]
    fn test_delete_backwards_to_char() {
        let mut buffer = Buffer {
            content: BufferContent::new("abcdef".to_string()),
            path: None,
        };

        let f_index = buffer.content.inner().find('f').unwrap() + 1;
        buffer.content.update_index_to(f_index);

        let deleted = buffer.delete_range(Range::BackwardsTo('c'));
        assert_eq!(deleted, Some("def".to_string()));
        assert_eq!(buffer.content.inner(), "abc");
    }

    #[test]
    fn test_navigation_char_forward() {
        let mut buffer = Buffer::scratch();
        buffer.content = BufferContent::new("abcdef".to_string());
        buffer.content.update_index_to(0);
        assert_eq!(
            buffer.yank_navigation_motion(NavigationMotion::CharForward),
            Some("a")
        );
    }

    #[test]
    fn test_navigation_char_backward() {
        let mut buffer = Buffer::scratch();
        buffer.content = BufferContent::new("abcdef".to_string());
        let len = buffer.content.inner().len();
        buffer.content.update_index_to(len);
        assert_eq!(
            buffer.yank_navigation_motion(NavigationMotion::CharBackward),
            Some("f")
        );
    }

    #[test]
    fn test_navigation_word_forward() {
        let mut buffer = Buffer::scratch();
        buffer.content = BufferContent::new("hello world".to_string());
        buffer.content.update_index_to(0);
        assert_eq!(
            buffer.yank_navigation_motion(NavigationMotion::WordForward),
            Some("hello")
        );
    }

    #[test]
    fn test_navigation_word_backward() {
        let mut buffer = Buffer::scratch();
        buffer.content = BufferContent::new("hello world".to_string());
        let len = buffer.content.inner().len();
        buffer.content.update_index_to(len);
        assert_eq!(
            buffer.yank_navigation_motion(NavigationMotion::WordBackward),
            Some("world")
        );
    }

    #[test]
    fn test_navigation_end_of_word_forward() {
        let mut buffer = Buffer::scratch();
        buffer.content = BufferContent::new("hello world".to_string());
        buffer.content.update_index_to(0);
        assert_eq!(
            buffer.yank_navigation_motion(NavigationMotion::EndOfWordForward),
            Some("hello")
        );
    }

    #[test]
    fn test_navigation_end_of_word_backward() {
        let mut buffer = Buffer::scratch();
        buffer.content = BufferContent::new("hello world".to_string());
        let len = buffer.content.inner().len();
        buffer.content.update_index_to(len);
        assert_eq!(
            buffer.yank_navigation_motion(NavigationMotion::EndOfWordBackward),
            Some("world")
        );
    }

    #[test]
    fn test_navigation_line_start() {
        let mut buffer = Buffer::scratch();
        buffer.content = BufferContent::new("line1\nline2\nline3".to_string());
        let idx = buffer.content.inner().find("line2").unwrap() + 2;
        buffer.content.update_index_to(idx);
        assert_eq!(
            buffer.yank_navigation_motion(NavigationMotion::LineStart),
            Some("\nline2")
        );
    }

    #[test]
    fn test_navigation_line_end() {
        let mut buffer = Buffer::scratch();
        buffer.content = BufferContent::new("line1\nline2\nline3".to_string());
        let idx = buffer.content.inner().find("line2").unwrap() + 2;
        buffer.content.update_index_to(idx);
        assert_eq!(
            buffer.yank_navigation_motion(NavigationMotion::LineEnd),
            Some("\nline2")
        );
    }

    #[test]
    fn test_navigation_top() {
        let mut buffer = Buffer::scratch();
        buffer.content = BufferContent::new("line1\nline2\nline3".to_string());
        let idx = buffer.content.inner().find("line2").unwrap();
        let _ = buffer.content.update_index_to(idx);

        assert_eq!(
            buffer.yank_navigation_motion(NavigationMotion::Top),
            Some("line1")
        );
    }

    #[test]
    fn test_navigation_bottom() {
        let mut buffer = Buffer::scratch();
        buffer.content = BufferContent::new("line1\nline2\nline3".to_string());
        let idx = buffer.content.inner().find("line2").unwrap();
        buffer.content.update_index_to(idx);
        assert_eq!(
            buffer.yank_navigation_motion(NavigationMotion::Bottom),
            Some("line3")
        );
    }

    #[test]
    fn test_navigation_line_forward() {
        let mut buffer = Buffer::scratch();
        buffer.content = BufferContent::new("line1\nline2\nline3".to_string());
        buffer.content.update_index_to(0);
        assert_eq!(
            buffer.yank_navigation_motion(NavigationMotion::LineForward),
            Some("line1")
        );
    }

    #[test]
    fn test_navigation_line_backward() {
        let mut buffer = Buffer::scratch();
        buffer.content = BufferContent::new("line1\nline2\nline3".to_string());
        let idx = buffer.content.inner().find("line3").unwrap();
        buffer.content.update_index_to(idx);
        assert_eq!(
            buffer.yank_navigation_motion(NavigationMotion::LineBackward),
            Some("line2")
        );
    }
}

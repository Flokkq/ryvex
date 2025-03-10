use std::ops::Range;

const UNICODE_CHAR_RANGE: std::ops::RangeInclusive<usize> = 1..=4;

pub struct BufferContent {
    content: String,
    index: usize,
}

impl Iterator for BufferContent {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        for i in UNICODE_CHAR_RANGE {
            if self.index + i <= self.content.len()
                && self.content.is_char_boundary(self.index + i)
            {
                let start = self.index;
                self.index += i;
                return Some(start..self.index);
            }
        }
        None
    }
}

impl BufferContent {
    pub fn new(source: String) -> Self {
        Self {
            content: source,
            index: 0,
        }
    }

    pub fn inner(&mut self) -> &str {
        &self.content
    }

    pub fn update_index_to(&mut self, i: usize) {
        self.index = i;
    }

    pub fn yank(&self, range: Range<usize>) -> Option<&str> {
        self.content.get(range)
    }

    pub fn delete(&mut self, range: Range<usize>) -> Option<String> {
        if let Some(removed) = self.content.get(range.clone()) {
            let removed_str = removed.to_string();
            self.content.replace_range(range.clone(), "");

            if self.index > range.start {
                self.index = range.start;
            }

            Some(removed_str)
        } else {
            None
        }
    }

    pub fn insert(&mut self, index: usize, text: &str) -> bool {
        if index > self.content.len() || !self.content.is_char_boundary(index) {
            return false;
        }

        self.content.insert_str(index, text);

        if index <= self.index {
            self.index += text.len();
        }

        true
    }

    pub fn peek(&self) -> Option<Range<usize>> {
        for i in UNICODE_CHAR_RANGE {
            if self.index + i <= self.content.len()
                && self.content.is_char_boundary(self.index + i)
            {
                return Some(self.index..(self.index + i));
            }
        }
        None
    }

    pub fn consume_while<F>(&mut self, func: &F) -> Option<Range<usize>>
    where
        F: Fn(&str) -> bool,
    {
        let start_index = self.index;
        while let Some(range) = self.peek() {
            // Get the token as a &str from the range.
            let token = self.content.get(range.clone()).unwrap();
            if !func(token) {
                break;
            }
            self.next();
        }
        Some(start_index..self.index)
    }

    pub fn consume_until_tail_is(
        &mut self,
        tail: &str,
    ) -> Option<Range<usize>> {
        let start_index = self.index;
        while self.peek().is_some()
            && !self
                .content
                .get(start_index..self.index)
                .unwrap_or("")
                .ends_with(tail)
        {
            self.next();
        }
        Some(start_index..self.index)
    }

    pub fn consume_until_end(&mut self) -> Option<Range<usize>> {
        let start_index = self.index;
        while self.peek().is_some() {
            self.next();
        }
        Some(start_index..self.index)
    }

    pub fn peek_until_end(&self) -> Option<Range<usize>> {
        if self.index < self.content.len() {
            Some(self.index..self.content.len())
        } else {
            None
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_substring_from(&self, start: usize) -> Option<Range<usize>> {
        if start <= self.index {
            Some(start..self.index)
        } else {
            None
        }
    }

    pub fn get_substring_ahead(&self, end: usize) -> Option<Range<usize>> {
        if self.index <= end && end <= self.content.len() {
            Some(self.index..end)
        } else {
            None
        }
    }

    pub fn find_next(&self, pattern: &str) -> Option<usize> {
        self.content[self.index..]
            .find(pattern)
            .map(|i| i + self.index)
    }

    pub fn find_previous(&self, pattern: &str) -> Option<usize> {
        self.content[..self.index].find(pattern)
    }

    pub fn peek_line_ahead(&self) -> Option<Range<usize>> {
        if let Some(newline_index) = self.find_next("\n") {
            Some(self.index..(newline_index + 1))
        } else if self.peek().is_some() {
            Some(self.index..self.content.len())
        } else {
            None
        }
    }

    pub fn consume_line_ahead(&mut self) -> Option<Range<usize>> {
        if let Some(newline_index) = self.find_next("\n") {
            let ret_range = self.index..(newline_index + 1);
            self.index = newline_index + 1;
            Some(ret_range)
        } else if self.peek().is_some() {
            let ret_range = self.index..self.content.len();
            self.index = self.content.len();
            Some(ret_range)
        } else {
            None
        }
    }

    pub fn skip_while_true<F>(&mut self, predicate: F)
    where
        F: Fn(&str) -> bool,
    {
        while let Some(range) = self.peek() {
            let token = self.content.get(range.clone()).unwrap();
            if !predicate(token) {
                break;
            }
            self.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_does_not_advance() {
        let content = String::from("this is some plaintext");
        let mut buffer = BufferContent::new(content);
        let first_range = buffer.peek().unwrap();
        assert_eq!(buffer.yank(first_range.clone()).unwrap(), "t");
        let second_range = buffer.peek().unwrap();
        assert_eq!(buffer.yank(second_range.clone()).unwrap(), "t");
        let next_range = buffer.next().unwrap();
        assert_eq!(buffer.yank(next_range.clone()).unwrap(), "t");
    }

    #[test]
    fn peek_does_not_advance_utf() {
        let content = String::from("الْأَ");
        let mut buffer = BufferContent::new(content);
        let first_range = buffer.peek().unwrap();
        assert_eq!(buffer.yank(first_range.clone()).unwrap(), "ا");
        let second_range = buffer.peek().unwrap();
        assert_eq!(buffer.yank(second_range.clone()).unwrap(), "ا");
        let next_range = buffer.next().unwrap();
        assert_eq!(buffer.yank(next_range.clone()).unwrap(), "ا");
    }

    #[test]
    fn modern_standard_arabic_test() {
        let content = String::from("الْﺦﷺأَ");
        let mut buffer = BufferContent::new(content);
        let consumed_range = buffer.consume_line_ahead().unwrap();
        assert_eq!(buffer.yank(consumed_range.clone()).unwrap(), "الْﺦﷺأَ");
        assert!(buffer.next().is_none());
    }

    #[test]
    fn consume_until_end_consumes_full_string() {
        let content = String::from("this is some plaintext");
        let mut buffer = BufferContent::new(content.clone());
        let consumed_range = buffer.consume_until_end().unwrap();
        assert_eq!(buffer.yank(consumed_range.clone()).unwrap(), content);
        assert!(buffer.next().is_none());
    }

    #[test]
    fn next_advances_utf_correctly() {
        let content = String::from("اa木b겅c€d𐍈e");
        let mut buffer = BufferContent::new(content);
        let first = buffer.next().unwrap();
        assert_eq!(buffer.yank(first.clone()).unwrap(), "ا");
        let second = buffer.next().unwrap();
        assert_eq!(buffer.yank(second.clone()).unwrap(), "a");
        let third = buffer.next().unwrap();
        assert_eq!(buffer.yank(third.clone()).unwrap(), "木");
        let fourth = buffer.next().unwrap();
        assert_eq!(buffer.yank(fourth.clone()).unwrap(), "b");
        let fifth = buffer.next().unwrap();
        assert_eq!(buffer.yank(fifth.clone()).unwrap(), "겅");
        let sixth = buffer.next().unwrap();
        assert_eq!(buffer.yank(sixth.clone()).unwrap(), "c");
        let seventh = buffer.next().unwrap();
        assert_eq!(buffer.yank(seventh.clone()).unwrap(), "€");
        let eighth = buffer.next().unwrap();
        assert_eq!(buffer.yank(eighth.clone()).unwrap(), "d");
        let ninth = buffer.next().unwrap();
        assert_eq!(buffer.yank(ninth.clone()).unwrap(), "𐍈");
        let tenth = buffer.next().unwrap();
        assert_eq!(buffer.yank(tenth.clone()).unwrap(), "e");
    }

    #[test]
    fn test_slashes() {
        let content = String::from("¯\\\\\\\\\\¯");
        let mut buffer = BufferContent::new(content);
        let first = buffer.peek().unwrap();
        assert_eq!(buffer.yank(first.clone()).unwrap(), "¯");
        let first_next = buffer.next().unwrap();
        assert_eq!(buffer.yank(first_next.clone()).unwrap(), "¯");
        let second = buffer.peek().unwrap();
        assert_eq!(buffer.yank(second.clone()).unwrap(), "\\");
        let consumed_range = buffer.consume_while(&|c| c == "\\").unwrap();
        assert_eq!(buffer.yank(consumed_range.clone()).unwrap(), "\\\\\\\\\\");
        let third = buffer.peek().unwrap();
        assert_eq!(buffer.yank(third.clone()).unwrap(), "¯");
        let third_next = buffer.next().unwrap();
        assert_eq!(buffer.yank(third_next.clone()).unwrap(), "¯");
        assert!(buffer.next().is_none());
    }

    #[test]
    fn general_iter_test() {
        let content = String::from("this is some plaintext");
        let mut buffer = BufferContent::new(content);
        let first = buffer.peek().unwrap();
        assert_eq!(buffer.yank(first.clone()).unwrap(), "t");
        let first_again = buffer.peek().unwrap();
        assert_eq!(buffer.yank(first_again.clone()).unwrap(), "t");
        let first_next = buffer.next().unwrap();
        assert_eq!(buffer.yank(first_next.clone()).unwrap(), "t");
        let consumed_range = buffer.consume_while(&|c| c != " ").unwrap();
        assert_eq!(buffer.yank(consumed_range.clone()).unwrap(), "his");
        let consumed_range2 = buffer.consume_until_tail_is("plain").unwrap();
        assert_eq!(
            buffer.yank(consumed_range2.clone()).unwrap(),
            " is some plain"
        );
        let consumed_range3 = buffer.consume_until_end().unwrap();
        assert_eq!(buffer.yank(consumed_range3.clone()).unwrap(), "text");
        assert!(buffer.next().is_none());

        let other_text = String::from(
            "jkfsgbkfgbdklfdsbh gkhsdfbg <details> and more chars",
        );
        let mut other_buffer = BufferContent::new(other_text);
        let consumed_range_other =
            other_buffer.consume_until_tail_is("<details>").unwrap();
        assert_eq!(
            other_buffer.yank(consumed_range_other.clone()).unwrap(),
            "jkfsgbkfgbdklfdsbh gkhsdfbg <details>"
        );
        let consumed_range_other2 = other_buffer.consume_until_end().unwrap();
        assert_eq!(
            other_buffer.yank(consumed_range_other2.clone()).unwrap(),
            " and more chars"
        );
        assert!(other_buffer.peek().is_none());
    }

    #[test]
    fn consume_peek_line_test() {
        let content = String::from("this is some plaintext in a line\nAnd a new line with more content");
        let mut buffer = BufferContent::new(content);
        let line1_peek = buffer.peek_line_ahead().unwrap();
        assert_eq!(
            buffer.yank(line1_peek.clone()).unwrap(),
            "this is some plaintext in a line\n"
        );
        let line1 = buffer.consume_line_ahead().unwrap();
        assert_eq!(
            buffer.yank(line1.clone()).unwrap(),
            "this is some plaintext in a line\n"
        );
        let line2_peek = buffer.peek_line_ahead().unwrap();
        assert_eq!(
            buffer.yank(line2_peek.clone()).unwrap(),
            "And a new line with more content"
        );
        let line2 = buffer.consume_line_ahead().unwrap();
        assert_eq!(
            buffer.yank(line2.clone()).unwrap(),
            "And a new line with more content"
        );
        assert!(buffer.peek_line_ahead().is_none());
    }

    #[test]
    fn test_degenerate_newlines() {
        let content = String::from("\n\n\n\n\nfoo\n");
        let mut buffer = BufferContent::new(content);
        let line1 = buffer.consume_line_ahead().unwrap();
        assert_eq!(buffer.yank(line1.clone()).unwrap(), "\n");
        let line2 = buffer.consume_line_ahead().unwrap();
        assert_eq!(buffer.yank(line2.clone()).unwrap(), "\n");
        let line3 = buffer.consume_line_ahead().unwrap();
        assert_eq!(buffer.yank(line3.clone()).unwrap(), "\n");
        let line4 = buffer.consume_line_ahead().unwrap();
        assert_eq!(buffer.yank(line4.clone()).unwrap(), "\n");
        let line5 = buffer.consume_line_ahead().unwrap();
        assert_eq!(buffer.yank(line5.clone()).unwrap(), "\n");
        let line6 = buffer.consume_line_ahead().unwrap();
        assert_eq!(buffer.yank(line6.clone()).unwrap(), "foo\n");
        assert!(buffer.consume_line_ahead().is_none());
    }

    #[test]
    fn test_mixed_chars() {
        let content = String::from("  - foo\n\n\tbar\n");
        let mut buffer = BufferContent::new(content);
        let line1 = buffer.consume_line_ahead().unwrap();
        assert_eq!(buffer.yank(line1.clone()).unwrap(), "  - foo\n");
        let line2 = buffer.consume_line_ahead().unwrap();
        assert_eq!(buffer.yank(line2.clone()).unwrap(), "\n");
        let line3 = buffer.consume_line_ahead().unwrap();
        assert_eq!(buffer.yank(line3.clone()).unwrap(), "\tbar\n");
    }

    #[test]
    fn test_yank_and_delete() {
        let mut buffer = BufferContent::new(String::from("Hello, world!"));
        let start = buffer.content.find("world").unwrap();
        let end = start + "world".len();
        let range = start..end;
        assert_eq!(buffer.yank(range.clone()).unwrap(), "world");
        let deleted = buffer.delete(range).unwrap();
        assert_eq!(deleted, "world");
        assert_eq!(buffer.content, "Hello, !");
    }

    #[test]
    fn test_insert_middle() {
        let mut buffer = BufferContent::new(String::from("Hello world!"));

        let index = buffer.find_next("world").unwrap();
        assert!(buffer.insert(index, "beautiful "));

        assert_eq!(buffer.content, "Hello beautiful world!");
    }

    #[test]
    fn test_delete_middle() {
        let mut buffer =
            BufferContent::new(String::from("Hello beautiful world!"));

        let start = buffer.find_next("beautiful").unwrap();
        let end = start + "beautiful".len();
        let removed = buffer.delete(start..end).unwrap();

        assert_eq!(removed, "beautiful");
        assert_eq!(buffer.content, "Hello  world!");
    }

    #[test]
    fn test_insert_delete_utf8() {
        let mut buffer = BufferContent::new(String::from("Привет мир!"));

        let index = buffer.find_next("мир").unwrap();
        assert!(buffer.insert(index, "большой "));

        assert_eq!(buffer.content, "Привет большой мир!");

        let end = index + "большой ".len();
        let removed = buffer.delete(index..end).unwrap();

        assert_eq!(removed, "большой ");
        assert_eq!(buffer.content, "Привет мир!");
    }

    #[test]
    fn test_delete_moves_cursor() {
        let mut buffer = BufferContent::new(String::from("Hello world!"));

        buffer.index = buffer.find_next("world").unwrap();
        let end = buffer.index + "world".len();
        buffer.delete(buffer.index..end).unwrap();

        assert_eq!(buffer.content, "Hello !");
        assert_eq!(buffer.index, buffer.find_next("!").unwrap());
    }

    #[test]
    fn test_insert_at_cursor() {
        let mut buffer = BufferContent::new(String::from("Hello !"));

        buffer.index = buffer.find_next("!").unwrap();
        buffer.insert(buffer.index, "world");

        assert_eq!(buffer.content, "Hello world!");
        assert_eq!(buffer.index, buffer.find_next("!").unwrap());
    }
}

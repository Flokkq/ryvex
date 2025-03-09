use std::usize;

const UNICODE_RANGE: std::ops::Range<usize> = 1..4;
type Token<'a> = &'a str;

pub struct Buffer<'a> {
    content: Token<'a>,
    index: usize,
}

impl<'a> Iterator for Buffer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        for i in UNICODE_RANGE {
            if self.content.is_char_boundary(self.index + i) {
                let ret = self.content.get(self.index..self.index + i);
                self.index += 1;
                return ret;
            }
        }
        None
    }
}

impl<'a> Buffer<'a> {
    pub fn new(source: Token<'a>) -> Self {
        Self {
            content: source,
            index: 0,
        }
    }

    pub fn update_index_to(&mut self, i: usize) {
        self.index = i;
    }

    pub fn peek(&self) -> Option<Token<'a>> {
        for i in UNICODE_RANGE {
            if self.content.is_char_boundary(self.index + i) {
                return self.content.get(self.index..self.index + i);
            }
        }

        None
    }

    pub fn next_if_eq(&mut self, expected: Token<'a>) -> Option<Token<'a>> {
        if self.peek() == Some(expected) {
            return self.next();
        }
        None
    }

    pub fn consume_while_case_holds(
        &mut self,
        func: &dyn Fn(&str) -> bool,
    ) -> Option<Token<'a>> {
        let start_index = self.index;
        while self.peek().is_some() && func(self.peek().unwrap()) {
            self.next();
        }
        self.content.get(start_index..self.index)
    }

    pub fn consume_until_tail_is(&mut self, tail: &str) -> Option<Token<'a>> {
        let start_index = self.index;
        while self.peek().is_some()
            && !self
                .content
                .get(start_index..self.index)
                .unwrap_or(tail)
                .ends_with(tail)
        {
            self.next();
        }
        //unwrap_or(tail) to ensure exit in unforseen situation
        self.content.get(start_index..self.index)
    }

    pub fn consume_until_end(&mut self) -> Option<Token<'a>> {
        let start_index = self.index;
        while self.peek().is_some() {
            self.next();
        }

        self.content.get(start_index..self.index)
    }

    pub fn peek_until_end(&self) -> Option<Token<'a>> {
        self.content.get(self.index..=(self.content.len() - 1))
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_substring_from(&self, start: usize) -> Option<Token<'a>> {
        self.content.get(start..self.index)
    }

    pub fn get_substring_ahead(&self, end: usize) -> Option<Token<'a>> {
        self.content.get(self.index..end)
    }

    pub fn find_next(&self, pattern: &str) -> Option<usize> {
        self.content[self.index..].find(pattern)
    }

    pub fn find_previous(&self, pattern: &str) -> Option<usize> {
        self.content[..self.index].find(pattern)
    }

    pub fn peek_line_ahead(&self) -> Option<&'a str> {
        match self.find_next("\n") {
            Some(newline_index) => {
                self.content.get(self.index..=(self.index + newline_index))
            }
            None if self.peek().is_some() => {
                self.content.get(self.index..=(self.content.len() - 1))
            }
            _ => None,
        }
    }

    pub fn consume_line_ahead(&mut self) -> Option<&'a str> {
        match self.find_next("\n") {
            Some(newline_index) => {
                let ret =
                    self.content.get(self.index..=(self.index + newline_index));
                self.update_index_to(self.index + newline_index + 1);
                ret
            }
            None if self.peek().is_some() => {
                let ret =
                    self.content.get(self.index..=(self.content.len() - 1));
                self.update_index_to(self.content.len());
                ret
            }
            _ => None,
        }
    }

    /// Clone of [Iterator:skip_while]
    pub fn skip_while_true<F>(&mut self, predicate: F)
    where
        F: Fn(&str) -> bool,
    {
        while let Some(c) = self.peek() {
            if !predicate(c) {
                break;
            }
        }
        self.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_does_not_advance() {
        let file_content = "this is some plaintext";
        let mut buffer = Buffer::new(file_content);
        assert_eq!(Some("t"), buffer.peek());
        assert_eq!(Some("t"), buffer.peek());
        assert_eq!(Some("t"), buffer.next());
    }

    #[test]
    fn peek_does_not_advance_utf() {
        let file_content = "الْأَ";
        let mut buffer = Buffer::new(file_content);
        assert_eq!(Some("ا"), buffer.peek());
        assert_eq!(Some("ا"), buffer.peek());
        assert_eq!(Some("ا"), buffer.next());
    }

    #[test]
    fn modern_standard_arabic_test() {
        let file_content = "الْﺦﷺأَ"; // لْ is a weird character. 2 bytes are valid for the base and two more
                                    // add the little circle on top.
        let mut buffer = Buffer::new(file_content);
        assert_eq!(Some("الْﺦﷺأَ"), buffer.consume_line_ahead());
        assert_eq!(None, buffer.next());
    }

    #[test]
    fn consume_until_end_consumes_full_string() {
        let file_content = "this is some plaintext";
        let mut buffer = Buffer::new(file_content);
        assert_eq!(Some("this is some plaintext"), buffer.consume_until_end());
        assert_eq!(None, buffer.next());
    }

    #[test]
    fn next_advances_utf_correctly() {
        let file_content = "اa木b겅c€d𐍈e";
        let mut buffer = Buffer::new(file_content);
        assert_eq!(Some("ا"), buffer.next());
        assert_eq!(Some("a"), buffer.peek());
        assert_eq!(Some("a"), buffer.next());
        assert_eq!(Some("木"), buffer.peek());
        assert_eq!(Some("木"), buffer.next());
        assert_eq!(Some("b"), buffer.next());
        assert_eq!(Some("겅"), buffer.peek());
        assert_eq!(Some("겅"), buffer.next());
        assert_eq!(Some("c"), buffer.next());
        assert_eq!(Some("€"), buffer.next());
        assert_eq!(Some("d"), buffer.peek());
        assert_eq!(Some("d"), buffer.next());
        assert_eq!(Some("𐍈"), buffer.next());
        assert_eq!(Some("e"), buffer.next());
    }

    #[test]
    fn test_slashes() {
        let slashes = "¯\\\\\\\\\\¯";
        let mut slash_iter = Buffer::new(slashes);
        assert_eq!(Some("¯"), slash_iter.peek());
        assert_eq!(Some("¯"), slash_iter.next());
        assert_eq!(Some("\\"), slash_iter.peek());
        assert_eq!(
            Some("\\\\\\\\\\"),
            slash_iter.consume_while_case_holds(&|c| c == "\\")
        );
        assert_eq!(Some("¯"), slash_iter.peek());
        assert_eq!(Some("¯"), slash_iter.next());
        assert_eq!(None, slash_iter.next());
    }

    #[test]
    fn general_iter_test() {
        let file_content = "this is some plaintext";
        let mut buffer = Buffer::new(file_content);
        assert_eq!(Some("t"), buffer.peek());
        assert_eq!(Some("t"), buffer.peek());
        assert_eq!(Some("t"), buffer.next());
        assert_eq!(Some("his"), buffer.consume_while_case_holds(&|c| c != " "));
        assert_eq!(
            Some(" is some plain"),
            buffer.consume_until_tail_is("plain")
        );
        assert_eq!(Some("text"), buffer.consume_until_end());
        assert_eq!(None, buffer.next());

        let other_text = "jkfsgbkfgbdklfdsbh gkhsdfbg <details> and more chars";
        let mut other_text_iter = Buffer::new(other_text);
        assert_eq!(
            Some("jkfsgbkfgbdklfdsbh gkhsdfbg <details>"),
            other_text_iter.consume_until_tail_is("<details>")
        );
        assert_eq!(
            Some(" and more chars"),
            other_text_iter.consume_until_end()
        );
        assert_eq!(None, other_text_iter.peek());
    }

    #[test]
    fn consume_peek_line_test() {
        let file_content = "this is some plaintext in a line\nAnd a new line \
		                 with more content";
        let mut buffer = Buffer::new(file_content);
        assert_eq!(
            Some("this is some plaintext in a line\n"),
            buffer.peek_line_ahead()
        );
        assert_eq!(
            Some("this is some plaintext in a line\n"),
            buffer.consume_line_ahead()
        );
        assert_ne!(
            Some("this is some plaintext in a line\n"),
            buffer.peek_line_ahead()
        );
        assert_eq!(
            Some("And a new line with more content"),
            buffer.peek_line_ahead()
        );
        assert_eq!(
            Some("And a new line with more content"),
            buffer.consume_line_ahead()
        );
        assert_eq!(None, buffer.peek_line_ahead());
    }

    #[test]
    fn test_degenerate_newlines() {
        let file_content = "\n\n\n\n\nfoo\n";
        let mut buffer = Buffer::new(file_content);
        assert_eq!(Some("\n"), buffer.peek_line_ahead());
        assert_eq!(Some("\n"), buffer.consume_line_ahead());
        assert_eq!(Some("\n"), buffer.consume_line_ahead());
        assert_eq!(Some("\n"), buffer.consume_line_ahead());
        assert_eq!(Some("\n"), buffer.consume_line_ahead());
        assert_eq!(Some("\n"), buffer.consume_line_ahead());
        assert_eq!(Some("foo\n"), buffer.consume_line_ahead());
        assert_eq!(None, buffer.consume_line_ahead());
    }

    #[test]
    fn test_mixed_chars() {
        let file_content = "  - foo\n\n\tbar\n";
        let mut buffer = Buffer::new(file_content);
        assert_eq!(Some("  - foo\n"), buffer.consume_line_ahead());
        assert_eq!(Some("\n"), buffer.consume_line_ahead());
        assert_eq!(Some("\tbar\n"), buffer.consume_line_ahead());
    }
}

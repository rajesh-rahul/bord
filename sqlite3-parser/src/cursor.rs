use std::{iter::Peekable, str::Chars};
// TODO: Make nicer
pub struct LexCursor<'a> {
    pub abs_position: u32,
    pub input: Chars<'a>,
    token_builder: TokenBuilder<'a>,
}

pub struct TokenBuilder<'a> {
    chars: Peekable<Chars<'a>>,
    curr_byte_len: usize,
    curr_char_len: usize,
}

impl<'a> TokenBuilder<'a> {
    pub fn new(chars: Chars<'a>) -> TokenBuilder<'a> {
        TokenBuilder {
            chars: chars.peekable(),
            curr_byte_len: 0,
            curr_char_len: 0,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.chars.next() {
            self.curr_byte_len += ch.len_utf8();
            self.curr_char_len += 1;
            Some(ch)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.curr_byte_len = 0;
        self.curr_char_len = 0;
    }
}

impl<'a> LexCursor<'a> {
    pub fn new(input: &'a str) -> LexCursor {
        LexCursor {
            abs_position: 0,
            input: input.chars(),
            token_builder: TokenBuilder::new(input.chars()),
        }
    }

    pub fn next(&mut self) -> Option<char> {
        self.token_builder.next()
    }

    pub fn advance_by(&mut self, length: usize) {
        for _ in 0..length {
            self.next();
        }
    }

    pub fn first(&self) -> Option<char> {
        self.token_builder.chars.clone().next()
    }

    pub fn second(&self) -> Option<char> {
        let mut iter = self.token_builder.chars.clone();
        iter.next();

        iter.next()
    }

    pub fn third(&self) -> Option<char> {
        let mut iter = self.token_builder.chars.clone();
        iter.next();
        iter.next();

        iter.next()
    }

    fn bump_original(&mut self) {
        if let Some(_) = self.input.next() {
            self.abs_position += 1;
        }
    }

    pub fn build_token_info(&mut self) -> (&'a str, u32) {
        let (token, _) = self
            .input
            .as_str()
            .split_at(self.token_builder.curr_byte_len);

        let abs_pos = self.abs_position;

        for _ in 0..self.token_builder.curr_char_len {
            self.bump_original();
        }

        self.token_builder.reset();

        (token, abs_pos)
    }

    pub fn build_error_token_info(
        &mut self,
        is_err_predicate: impl Fn(char) -> bool,
    ) -> (&'a str, u32) {
        // Atleast one character must be consumed or else we will be stuck in an infinite loop
        if self.token_builder.curr_char_len == 0 {
            self.next();
        }

        while let Some(ch) = self.first() {
            if is_err_predicate(ch) {
                self.next();
            } else {
                break;
            }
        }

        self.build_token_info()
    }
}

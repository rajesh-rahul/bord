use std::str::Chars;

use crate::{
    grammar::common::IDEN_SET, sqlite_keywords, LexError, SqliteToken, SqliteTokenKind,
    SqliteVersion, MAX_KEYWORD_LEN,
};

use SqliteTokenKind::*;

/// Our common number system which is base 10
const DECI_RADIX: u32 = 10;

/// Hexadecimal number system which is base 16
const HEXA_RADIX: u32 = 16;

#[derive(Clone)]
pub struct SqliteLexer<'a> {
    input: &'a str,
    prev_non_triv_tk: Option<SqliteTokenKind>, // Needed for now to deal with ambiguities with Window, Filter and Over keywords
    cursor: TokenBuilder<'a>,
    version: SqliteVersion,
}

impl<'a> Iterator for SqliteLexer<'a> {
    type Item = SqliteToken;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            SqliteToken { kind: EOF, .. } => None,
            token => Some(token),
        }
    }
}

impl<'a> SqliteLexer<'a> {
    pub fn new(input: &str, version: SqliteVersion) -> SqliteLexer<'_> {
        SqliteLexer {
            // cursor: LexCursor::new(input),
            prev_non_triv_tk: None,
            cursor: TokenBuilder::new(input),
            input,
            version,
        }
    }

    pub fn lex(self) -> Vec<SqliteToken> {
        self.into_iter().collect()
    }

    #[inline(always)]
    fn cursor_is_fresh(&self) -> bool {
        self.cursor.curr_byte_len == 0
    }

    #[inline(always)]
    fn build_token(&mut self, token_kind: SqliteTokenKind) -> SqliteToken {
        let token_len = self.cursor.done();
        assert!(token_len > 0);
        let (token_text, rest_input) = self.input.split_at(token_len);
        self.input = rest_input;

        SqliteToken::new(token_kind, token_text, None)
    }

    /// `consume` variable tells us if we should continue consuming characters to reach the
    /// beginnning of next token before building the error token or to build it right away.
    fn build_err_token(&mut self, error: LexError, consume: bool) -> SqliteToken {
        if self.cursor.curr_byte_len == 0 {
            self.cursor.advance();
        }

        let token_len = self
            .cursor
            .advance_while(|ch| !is_separate_token_start(ch) && !ch.is_whitespace() && consume)
            .done();

        let (token_text, rest_input) = self.input.split_at(token_len);
        self.input = rest_input;

        SqliteToken::new(ERROR, token_text, Some(error))
    }

    fn next_token(&mut self) -> SqliteToken {
        let Some(first) = self.cursor.first() else {
            self.prev_non_triv_tk = Some(EOF);
            return SqliteToken::new(EOF, "", None);
        };

        // This is 3-tuple long because that's our longest 'fixed' token
        let tk_tuple = (first, self.cursor.second(), self.cursor.third());

        let mut build_token = |kind: SqliteTokenKind| {
            assert!(self.cursor_is_fresh());
            self.cursor.advance_by(kind.size().unwrap() as usize);

            self.build_token(kind)
        };

        let token = match tk_tuple {
            (c, ..) if c.is_whitespace() => self.process_whitespace(),
            ('"', ..) | ('`', ..) | ('[', ..) => self.process_quoted_identifier(),
            ('-', Some('>'), Some('>')) => build_token(EXTRACT_TWO),
            ('-', Some('>'), ..) => build_token(EXTRACT_ONE),
            ('|', Some('|'), ..) => build_token(DOUBLE_PIPE),
            ('<', Some('<'), ..) => build_token(L_CHEV_TWO),
            ('>', Some('>'), ..) => build_token(R_CHEV_TWO),
            ('<', Some('='), ..) => build_token(L_CHEV_EQ),
            ('>', Some('='), ..) => build_token(R_CHEV_EQ),
            ('<', Some('>'), ..) => build_token(NOT_EQ_SQL),
            ('!', Some('='), ..) => build_token(NOT_EQ),
            ('=', Some('='), ..) => build_token(EQ),
            ('=', ..) => build_token(EQ_SQL),
            ('-', Some('-'), ..) => self.process_single_line_comment(),
            ('/', Some('*'), ..) => self.process_multi_line_comment(),
            ('\'', ..) => self.process_string_literal(),

            ('0', Some('x' | 'X'), Some(d)) if d.is_digit(HEXA_RADIX) => self.process_hex_literal(),
            ('x' | 'X', Some('\''), ..) => self.process_blob_literal(),
            ('.', Some(d), ..) | (d, ..) if d.is_digit(DECI_RADIX) => {
                self.process_int_or_real_literal()
            }

            ('$' | '?' | ':' | '@', ..) => self.process_param(),
            ('(', ..) => build_token(L_PAREN),
            (')', ..) => build_token(R_PAREN),
            ('~', ..) => build_token(TILDA),
            ('+', ..) => build_token(PLUS),
            ('-', ..) => build_token(MINUS),
            ('*', ..) => build_token(STAR),
            ('/', ..) => build_token(F_SLASH),
            ('%', ..) => build_token(PERCENT),
            ('<', ..) => build_token(L_CHEV),
            ('>', ..) => build_token(R_CHEV),
            (';', ..) => build_token(SEMICOLON),
            ('|', ..) => build_token(PIPE),
            ('&', ..) => build_token(AMPERSAND),
            ('.', ..) => build_token(DOT),
            (',', ..) => build_token(COMMA),

            (c, ..) if is_identifier_start(c) => self.process_keyword_or_identifier(),

            // TODO: Special characters should have different error token behaviour?
            _ => self.build_err_token(LexError::UnknownToken, true),
        };

        if !token.is_trivia() {
            self.prev_non_triv_tk = Some(token.kind);
        }

        token
    }

    #[inline(always)]
    fn process_whitespace(&mut self) -> SqliteToken {
        debug_assert!(self.cursor_is_fresh());

        self.cursor.advance_while(|ch| ch.is_whitespace());
        self.build_token(WHITESPACE)
    }

    /// Information: https://www.sqlite.org/lang_comment.html
    fn process_single_line_comment(&mut self) -> SqliteToken {
        assert!(self.cursor_is_fresh());
        debug_assert!((self.cursor.first(), self.cursor.second()) == (Some('-'), Some('-')));

        self.cursor.advance_while(|ch| ch != '\n').advance(); // consume the newline

        self.build_token(S_LINE_COMMENT)
    }

    fn process_keyword_or_identifier(&mut self) -> SqliteToken {
        debug_assert!(self.cursor_is_fresh());
        debug_assert!(matches!(self.cursor.first(), Some(c) if is_identifier_start(c)));

        let match_keyword = |lexer: &mut SqliteLexer| {
            let mut word: [u8; MAX_KEYWORD_LEN] = [0; MAX_KEYWORD_LEN];
            let input = lexer.input.chars();

            // NOTE: The set of characters allowed in keywords is a subset of characters
            // allowed in identifiers. Therefore we can take form a word
            // with max len being MAX_KEYWORD_LEN) of iden characters and then check if the word
            // exists in sqlite_keywords()
            let mut iden_iter = input
                // NOTE: identifier_continue is a superset of identifier_start
                .take_while(|&ch| is_identifier_continue(ch))
                .map(|ch| ch.to_ascii_uppercase())
                .enumerate();

            let mut word_len = 0;

            while let Some((idx, ch)) = iden_iter.next() {
                let Ok(ch_u8) = u8::try_from(ch) else {
                    return None;
                };
                word[idx] = ch_u8;
                word_len += 1;

                if word_len >= MAX_KEYWORD_LEN {
                    break;
                }
            }

            // Account for edge case where we have an IDEN that starts with a keyword that has
            // size MAX_KEYWORD_LEN
            if word_len == MAX_KEYWORD_LEN
                && iden_iter
                    .next()
                    .is_some_and(|(_, ch)| is_identifier_continue(ch))
            {
                return None;
            }

            if let Some(keyword) = sqlite_keywords(&word[0..word_len]) {
                lexer.cursor.advance_by(word_len);

                Some(lexer.build_token(keyword))
            } else {
                None
            }
        };

        let match_iden = |lexer: &mut SqliteLexer| {
            lexer
                .cursor
                .advance() // Consume the first character because it is not the same as `identifier_continue`
                .advance_while(is_identifier_continue);

            lexer.build_token(IDEN)
        };

        let mut token = match_keyword(self).unwrap_or_else(|| match_iden(self));

        // This ugly code is necessary to check for situations where these keywords can
        // be treated as identifiers. This is supposed to match exactly to SQLite's source code:
        // (check tokenize.c)

        if token.kind == KW_WINDOW {
            let mut lexer_copy = self.clone().filter(|it| !it.is_trivia());

            if !(lexer_copy
                .next()
                .is_some_and(|it| IDEN_SET.contains(it.kind))
                && lexer_copy.next().is_some_and(|it| it.kind == KW_AS))
            {
                token.kind = IDEN;
            }
        } else if token.kind == KW_OVER {
            if !(self.prev_non_triv_tk == Some(R_PAREN)
                && self
                    .clone()
                    .filter(|it| !it.is_trivia())
                    .next()
                    .is_some_and(|it| {
                        // TODO: IDEN_SET should use the dynamic iden used in the parser
                        // NOTE: If the next token is 'over' it will be treated as a keyword
                        // because prev_non_triv of the cloned lexer is outdated. One solution
                        // to fix is to consider KW_OVER here:
                        (IDEN_SET | L_PAREN | KW_OVER).contains(it.kind) || it.kind == L_PAREN
                    }))
            {
                token.kind = IDEN;
            }
        } else if token.kind == KW_FILTER {
            if !(self.prev_non_triv_tk == Some(R_PAREN)
                && self
                    .clone()
                    .filter(|it| !it.is_trivia())
                    .next()
                    .is_some_and(|it| it.kind == L_PAREN))
            {
                token.kind = IDEN;
            }
        }

        token
    }

    /// Information: https://www.sqlite.org/lang_comment.html
    fn process_multi_line_comment(&mut self) -> SqliteToken {
        assert!(self.cursor_is_fresh());
        assert!((self.cursor.first(), self.cursor.second()) == (Some('/'), Some('*')));

        let mut prev = None;
        while let Some(ch) = self.cursor.next() {
            if prev == Some('*') && ch == '/' {
                break;
            }
            prev = Some(ch);
        }

        self.build_token(M_LINE_COMMENT)
    }

    fn process_param(&mut self) -> SqliteToken {
        assert!(self.cursor_is_fresh());
        match self.cursor.next() {
            Some('?') => {
                while self.cursor.first().is_some_and(|ch| ch.is_ascii_digit()) {
                    self.cursor.next();
                }

                self.build_token(PARAM)
            }
            Some('$' | ':' | '@') => {
                while self.cursor.first().is_some_and(is_identifier_continue) {
                    self.cursor.next();
                }

                if self.cursor.curr_byte_len == 1 {
                    self.build_err_token(LexError::MalformedParam, false)
                } else {
                    self.build_token(PARAM)
                }
            }
            _ => unreachable!(),
        }
    }

    fn process_string_literal(&mut self) -> SqliteToken {
        assert!(self.cursor_is_fresh());
        debug_assert!(self.cursor.first() == Some('\''));
        let is_terminated = advance_while_with_escape(&mut self.cursor);

        if !is_terminated {
            return self.build_err_token(LexError::UnterminatedStringLiteral, false);
        } else {
            self.build_token(STR_LIT)
        }
    }

    fn process_quoted_identifier(&mut self) -> SqliteToken {
        assert!(self.cursor_is_fresh());

        let is_terminated = match self.cursor.first() {
            Some('[') => {
                self.cursor.advance_while(|ch| ch != ']');
                self.cursor.next() == Some(']')
            }
            Some('`' | '"') => advance_while_with_escape(&mut self.cursor),
            _ => unreachable!(),
        };

        if is_terminated {
            self.build_token(IDEN)
        } else {
            return self.build_err_token(LexError::UnterminatedQuotedIdentifier, false);
        }
    }

    fn process_int_or_real_literal(&mut self) -> SqliteToken {
        debug_assert!(self.cursor_is_fresh());

        let allow_underscore = self.version.underscore_in_numerics();
        let mut has_decimal_point = false;
        let mut has_exponent = false;

        let mut match_exponent = |cursor: &mut TokenBuilder<'a>| {
            if has_exponent {
                return;
            }
            let (Some('e') | Some('E'), Some(second)) = (cursor.first(), cursor.second()) else {
                return;
            };

            match (second, cursor.third()) {
                ('+', Some(d)) | ('-', Some(d)) if d.is_digit(DECI_RADIX) => {
                    has_exponent = true;
                    cursor.advance_by(2); // Consume the `e` and the + or - sign
                    match_digit(cursor, allow_underscore, DECI_RADIX);
                }
                (d, _) if d.is_digit(DECI_RADIX) => {
                    has_exponent = true;
                    cursor.next(); // Consume the `e` but digit is consumed by match_digit
                    match_digit(cursor, allow_underscore, DECI_RADIX);
                }
                _ => return,
            }
        };

        match_digit(&mut self.cursor, allow_underscore, DECI_RADIX);
        if let Some('.') = self.cursor.first() {
            has_decimal_point = true;
            self.cursor.advance();
        }
        match_digit(&mut self.cursor, allow_underscore, DECI_RADIX);
        match_exponent(&mut self.cursor);

        // If the token succeeding the number is not an operator or whitespace, then we must
        // recognize it as an error token to follow SQLite's lexer behaviour. The actual
        // rules are bit more tricky but we will live with this approximation for now (TODO: fix)
        // Weirdly, for Hex literals, This is treated as two separate tokens
        if self
            .cursor
            .first()
            .is_some_and(|c| !is_separate_token_start(c) && !c.is_whitespace())
        {
            // We borrow postgres's error message here
            return self.build_err_token(LexError::TrailingJunkAfterNumericLiteral, true);
        }

        let kind = if has_decimal_point { REAL_LIT } else { INT_LIT };

        self.build_token(kind)
    }

    fn process_hex_literal(&mut self) -> SqliteToken {
        assert!(self.cursor_is_fresh());

        self.cursor.advance_by(2); // Consume the `0x` or `0X`
        let allow_underscore = self.version.underscore_in_numerics();

        match_digit(&mut self.cursor, allow_underscore, HEXA_RADIX);

        self.build_token(HEX_LIT)
    }

    fn process_blob_literal(&mut self) -> SqliteToken {
        assert!(self.cursor_is_fresh());

        self.cursor.advance_by(2); // Consume the `x'` or `X'`

        let mut blob_lit_len = 0;
        let mut is_malformed = false;
        let mut terminated = false;

        while let Some(ch) = self.cursor.next() {
            if ch == '\'' {
                terminated = true;
                break;
            }
            if !ch.is_digit(HEXA_RADIX) {
                is_malformed = true;
            }
            blob_lit_len += 1;
        }

        if !terminated {
            return self.build_err_token(LexError::UnterminatedBlobLiteral, false);
        } else if (blob_lit_len % 2 != 0) || is_malformed {
            return self.build_err_token(LexError::MalformedBlobLiteral, false);
        } else {
            self.build_token(BLOB_LIT)
        }
    }
}

/// In SQLite, termination characters are escaped by doubling them
/// Returns true if terminated successfully
fn advance_while_with_escape(cursor: &mut TokenBuilder<'_>) -> bool {
    assert!(cursor.curr_byte_len == 0);
    let termination_ch = cursor
        .next()
        .expect("DEV ERROR: Atleast one character is expected");

    let mut terminated_successfully = false;
    while let Some(ch) = cursor.next() {
        if ch == termination_ch {
            if cursor.first() == Some(termination_ch) {
                cursor.next();
            } else {
                terminated_successfully = true;
                break;
            }
        }
    }

    terminated_successfully
}

fn match_digit(cursor: &mut TokenBuilder<'_>, allow_underscore: bool, radix: u32) {
    while let Some(first) = cursor.first() {
        let second = cursor.second();
        let third = cursor.third();

        match (first, second, third) {
            (d1, Some('_'), Some(d2))
                if d1.is_digit(radix) && d2.is_digit(radix) && allow_underscore =>
            {
                cursor.advance_by(2);
            }
            (d, _, _) if d.is_digit(radix) => {
                cursor.advance();
            }
            _ => break,
        }
    }
}

#[derive(Clone)]
pub struct TokenBuilder<'a> {
    pub curr_byte_len: usize,
    chars: Chars<'a>,
}

impl<'a> TokenBuilder<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            curr_byte_len: 0,
        }
    }

    #[inline(always)]
    pub fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.chars.next() {
            self.curr_byte_len += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn advance(&mut self) -> &mut Self {
        self.next();

        self
    }

    #[inline(always)]
    pub fn first(&self) -> Option<char> {
        self.chars.clone().next()
    }

    #[inline(always)]
    pub fn second(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next();

        iter.next()
    }

    #[inline(always)]
    pub fn third(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next();

        iter.next()
    }

    #[inline(always)]
    pub fn advance_while(&mut self, predicate: impl Fn(char) -> bool) -> &mut Self {
        while let Some(ch) = self.first() {
            if predicate(ch) {
                self.advance();
            } else {
                break;
            }
        }

        self
    }

    #[inline(always)]
    pub fn advance_by(&mut self, length: usize) -> &mut Self {
        for _ in 0..length {
            self.advance();
        }

        self
    }

    #[inline(always)]
    pub fn done(&mut self) -> usize {
        let done_token_len = self.curr_byte_len;
        self.curr_byte_len = 0;

        done_token_len
    }
}

// Based on: https://github.com/gwenn/lemon-rs/blob/69c67e128c395992e0c805e858607c035241cac6/src/dialect/mod.rs#L85C15-L85C34
#[inline(always)]
pub(crate) fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch > '\u{7F}'
}

// Based on: https://github.com/gwenn/lemon-rs/blob/69c67e128c395992e0c805e858607c035241cac6/src/dialect/mod.rs#L89
#[inline(always)]
pub(crate) fn is_identifier_continue(ch: char) -> bool {
    ch == '$' || ch.is_ascii_alphanumeric() || ch == '_' || ch > '\u{7F}'
}

#[inline(always)]
pub(crate) fn is_separate_token_start(ch: char) -> bool {
    !is_identifier_start(ch) && ch != '$'
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! check {
        ($input:expr, $token_pat:expr) => {
            let lexer = SqliteLexer::new($input, SqliteVersion([3, 46, 0]));

            let tokens_fat = lexer.lex();

            let tokens = tokens_fat.iter().map(|t| t.kind).collect::<Vec<_>>();

            if $token_pat.into_iter().collect::<Vec<_>>() != tokens {
                panic!("Expected {:?}\n got {:?}", $token_pat, tokens_fat);
            }
        };
    }

    #[test]
    fn can_lex_hexadecimal_literal() {
        // If any of the tokens in the expected list is error, an SQL statement with that input
        // string is also an error in SQLite. Likewise, if a token is not an error, it is also
        // a valid token in SQLite
        check!("0x1234", [HEX_LIT]);
        check!("0x1234_5678", [HEX_LIT]);
        check!("0x1234_5678_9abc_def0", [HEX_LIT]);

        check!("0X1234", [HEX_LIT]);
        check!("0x", [ERROR]); // TODO: Treat this as an malformed hex literal error
        check!("0X1234_5678", [HEX_LIT]);
        check!("0X1234_5678_9abc_def0", [HEX_LIT]);

        check!("0x1_", [HEX_LIT, IDEN]);
        check!("0x1m", [HEX_LIT, IDEN]);
        check!("0x1234__5678", [HEX_LIT, IDEN]);
    }

    #[test]
    fn can_lex_integer_literal() {
        // If any of the tokens in the expected list is error, an SQL statement with that input
        // string is also an error in SQLite. Likewise, if a token is not an error, it is also
        // a valid token in SQLite
        check!("1_1", [INT_LIT]);
        check!("1", [INT_LIT]);
        check!("1234", [INT_LIT]);
        check!("1234_5678", [INT_LIT]);
        check!("1234_5678_9", [INT_LIT]);
        check!("1234_", [ERROR]);
        check!("1234__5678", [ERROR, INT_LIT]); // TODO: Not sure how to match SQLite's behaviour here
        check!("1e", [ERROR]);
        check!("1e1", [INT_LIT]);
        check!("1e1_1", [INT_LIT]);
        check!("1e+1", [INT_LIT]);
        check!("1e-1", [INT_LIT]);
        check!("1+", [INT_LIT, PLUS]);
        check!("1-", [INT_LIT, MINUS]);
        check!("1*", [INT_LIT, STAR]);
        check!("1/", [INT_LIT, F_SLASH]);
        check!("1%", [INT_LIT, PERCENT]);
        check!("1<", [INT_LIT, L_CHEV]);
        check!("1>", [INT_LIT, R_CHEV]);
        check!("1;", [INT_LIT, SEMICOLON]);
        check!("1|", [INT_LIT, PIPE]);
        check!("1||", [INT_LIT, DOUBLE_PIPE]);
        check!("1&", [INT_LIT, AMPERSAND]);
        check!("1.", [REAL_LIT]);
        check!("1,", [INT_LIT, COMMA]);
        check!("1(", [INT_LIT, L_PAREN]);
        check!("1e+1+2.", [INT_LIT, PLUS, REAL_LIT]);
        check!("1e-1-2.", [INT_LIT, MINUS, REAL_LIT]);
        check!("1e1*2.", [INT_LIT, STAR, REAL_LIT]);
        check!("1e1/2.", [INT_LIT, F_SLASH, REAL_LIT]);
        check!("1e1%2.", [INT_LIT, PERCENT, REAL_LIT]);
        check!("1e1<2.", [INT_LIT, L_CHEV, REAL_LIT]);
        check!("1$1", [ERROR, INT_LIT]); // TODO: Not sure how to match SQLite's behaviour here
        check!("1^1", [INT_LIT, ERROR, INT_LIT]);
        check!("1`1", [INT_LIT, ERROR]);
        check!("1+~1", [INT_LIT, PLUS, TILDA, INT_LIT]);
        check!("1!", [INT_LIT, ERROR]);
        check!("1m", [ERROR]);
        check!("1m,", [ERROR, COMMA]);
        check!("1m(", [ERROR, L_PAREN]);
        check!("1m$", [ERROR]);
        check!("1m😊", [ERROR]);
    }

    #[test]
    fn can_lex_real_literal() {
        check!("1234.567", [REAL_LIT]);
        check!("1.e1_1", [REAL_LIT]);
        check!("1234.5678e9", [REAL_LIT]);
        check!("1234.5678e+9", [REAL_LIT]);
        check!("1234.5678e-9", [REAL_LIT]);
        check!(".1_1e1_1", [REAL_LIT]);
        check!("1.1e11", [REAL_LIT]);
        check!(".1e-1_1", [REAL_LIT]);
        check!(".1e+1", [REAL_LIT]);
    }

    #[test]
    fn can_lex_str_literal() {
        check!("'1234.567'", [STR_LIT]);
        check!("'BLAH'", [STR_LIT]);
    }

    #[test]
    fn can_lex_blob_literal() {
        check!("x''", [BLOB_LIT]);
        check!("X''", [BLOB_LIT]);

        check!("x'1'", [ERROR]); // Blob lit needs at least two items in it
        check!("x'22'", [BLOB_LIT]);
        check!("x'afff'", [BLOB_LIT]);
        check!("x'GG'", [ERROR]);
        check!("x'AbCdEf12345678'", [BLOB_LIT]);
        check!("x'AbCdEf12345678", [ERROR]); // not terminated
        check!("x'", [ERROR]);
    }

    #[test]
    fn can_lex_keywords() {
        check!("true", [IDEN]);
        check!(
            "true FalSe SELECT select",
            [IDEN, WHITESPACE, IDEN, WHITESPACE, KW_SELECT, WHITESPACE, KW_SELECT]
        );
        check!("'true'", [STR_LIT]);
        check!(
            "NULL$ CURRENT_TIMESTAMPaa NULL CURRENT_TIMESTAMP",
            [
                IDEN,
                WHITESPACE,
                IDEN,
                WHITESPACE,
                KW_NULL,
                WHITESPACE,
                KW_CURRENT_TIMESTAMP
            ]
        );
    }

    #[test]
    fn can_lex_identifiers() {
        check!(
            "users 'users' \"users\" [users] `t3``xyz` `users` [[]] [:] [] [[] \"@$fdasf[[]][]",
            [
                IDEN, WHITESPACE, STR_LIT, WHITESPACE, IDEN, WHITESPACE, IDEN, WHITESPACE, IDEN,
                WHITESPACE, IDEN, WHITESPACE, IDEN, ERROR, WHITESPACE, IDEN, WHITESPACE, IDEN,
                WHITESPACE, IDEN, WHITESPACE, ERROR
            ]
        );
    }
}

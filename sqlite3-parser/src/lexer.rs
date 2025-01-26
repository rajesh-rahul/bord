use crate::cursor::LexCursor;
use crate::{sqlite_keywords, SqliteToken, SqliteTokenKind, SqliteVersion, MAX_KEYWORD_LEN};

use SqliteTokenKind::*;

/// Our common number system which is base 10
const DECI_RADIX: u32 = 10;

/// Hexadecimal number system which is base 16
const HEXA_RADIX: u32 = 16;

#[derive(Debug)]
pub struct LexError {
    pub message: &'static str,
    pub token_idx: usize,
}

pub struct SqliteLexer<'a> {
    cursor: LexCursor<'a>,
    version: SqliteVersion,
    tokens: Vec<SqliteToken>,
    errors: Vec<LexError>,
}

impl<'a> Iterator for SqliteLexer<'a> {
    type Item = SqliteToken;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            token @ SqliteToken {
                kind: SqliteTokenKind::EOF,
                ..
            } => Some(token),
            _ => None,
        }
    }
}

impl<'input> SqliteLexer<'input> {
    pub fn new(input: &'input str, version: SqliteVersion) -> SqliteLexer {
        SqliteLexer {
            cursor: LexCursor::new(input),
            version,
            tokens: Vec::with_capacity(100),
            errors: Vec::new(),
        }
    }

    pub fn lex(mut self) -> (Vec<SqliteToken>, Vec<LexError>) {
        loop {
            match self.next_token() {
                SqliteToken {
                    kind: SqliteTokenKind::EOF,
                    ..
                } => break,
                token => self.tokens.push(token),
            }
        }

        (self.tokens, self.errors)
    }

    fn next_token(&mut self) -> SqliteToken {
        let Some(first) = self.cursor.first() else {
            return SqliteToken::new(SqliteTokenKind::EOF, "", self.cursor.abs_position);
        };

        // token is 3-tuple long because that's our longest 'fixed' token
        let token = (first, self.cursor.second(), self.cursor.third());

        let mut build_token = |token: SqliteTokenKind| {
            self.cursor.advance_by(token.size().unwrap() as usize);

            self.build_token(token)
        };

        match token {
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

            ('0', Some('x'), Some(d)) | ('0', Some('X'), Some(d)) if d.is_digit(HEXA_RADIX) => {
                self.process_hex_literal()
            }
            ('x', Some('\''), ..) | ('X', Some('\''), ..) => self.process_blob_literal(),
            ('.', Some(d), ..) | (d, ..) if d.is_digit(DECI_RADIX) => {
                self.process_int_or_real_literal()
            }

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
            (':', ..) => build_token(COLON),

            (c, ..) if is_identifier_start(c) => self.process_keyword_or_identifier(),

            // TODO: Special characters should have different error token behaviour?
            _ => self.build_error_token("Unknown token", true),
        }
    }

    fn process_whitespace(&mut self) -> SqliteToken {
        debug_assert!(matches!(self.cursor.first(), Some(c) if c.is_whitespace()));

        while let Some(ch) = self.cursor.first() {
            if !ch.is_whitespace() {
                break;
            } else {
                self.cursor.next();
            }
        }

        self.build_token(SqliteTokenKind::WHITESPACE)
    }

    /// Information: https://www.sqlite.org/lang_comment.html
    fn process_single_line_comment(&mut self) -> SqliteToken {
        debug_assert!((self.cursor.first(), self.cursor.second()) == (Some('-'), Some('-')));

        while let Some(ch) = self.cursor.next() {
            if ch == '\n' {
                break;
            }
        }

        let (data, abs_pos) = self.cursor.build_token_info();

        SqliteToken::new(SqliteTokenKind::S_LINE_COMMENT, data, abs_pos)
    }

    fn process_keyword_or_identifier(&mut self) -> SqliteToken {
        debug_assert!(matches!(self.cursor.first(), Some(c) if is_identifier_start(c)));

        let match_keyword = |lexer: &mut SqliteLexer| {
            let mut word: [u8; MAX_KEYWORD_LEN] = [0; MAX_KEYWORD_LEN];
            let input = lexer.cursor.input.clone();

            // Note: The set of characters allowed in keywords is a subset of characters
            // allowed in identifiers. Therefore we can take form a word
            // with max len being MAX_KEYWORD_LEN) of iden characters and then check if the word
            // exists in sqlite_keywords()
            let mut iden_iter = input
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

            // Account of edge case where we have an IDEN that starts with a keyword that has
            // size MAX_KEYWORD_LEN
            if word_len == MAX_KEYWORD_LEN
                && iden_iter
                    .next()
                    .is_some_and(|(_, ch)| is_identifier_continue(ch))
            {
                return None;
            }

            if let Some(keyword) = sqlite_keywords().get(&word[0..word_len]) {
                lexer.cursor.advance_by(word_len);
                Some(lexer.build_token(*keyword))
            } else {
                None
            }
        };

        let match_iden = |lexer: &mut SqliteLexer| {
            // Consume the first character because it is not the same as `identifier_continue`
            lexer.cursor.next();

            while let Some(ch) = lexer.cursor.first() {
                if !is_identifier_continue(ch) {
                    break;
                } else {
                    lexer.cursor.next();
                }
            }

            lexer.build_token(SqliteTokenKind::IDEN)
        };

        match_keyword(self).unwrap_or_else(|| match_iden(self))
    }

    /// Information: https://www.sqlite.org/lang_comment.html
    fn process_multi_line_comment(&mut self) -> SqliteToken {
        debug_assert!((self.cursor.first(), self.cursor.second()) == (Some('/'), Some('*')));

        let mut prev = None;
        while let Some(ch) = self.cursor.next() {
            if prev == Some('*') && ch == '/' {
                break;
            }
            prev = Some(ch);
        }

        self.build_token(SqliteTokenKind::M_LINE_COMMENT)
    }

    fn process_string_literal(&mut self) -> SqliteToken {
        debug_assert!(self.cursor.first() == Some('\''));
        self.cursor.advance_by(1);

        let mut is_terminated = false;

        while let Some(ch) = self.cursor.next() {
            if ch == '\'' {
                is_terminated = true;
                break;
            }
        }

        if !is_terminated {
            return self.build_error_token("String literal not terminated", false);
        } else {
            self.build_token(SqliteTokenKind::STR_LIT)
        }
    }

    fn process_quoted_identifier(&mut self) -> SqliteToken {
        debug_assert!(matches!(
            self.cursor.first(),
            Some('"') | Some('`') | Some('[')
        ));

        let termination_ch = match self.cursor.first().unwrap() {
            '[' => ']',
            ch => ch,
        };

        self.cursor.advance_by(1);

        let mut is_terminated = false;

        while let Some(ch) = self.cursor.next() {
            if ch == termination_ch {
                is_terminated = true;
                break;
            }
        }

        if !is_terminated {
            return self.build_error_token("Quoted identifier not terminated", false);
        } else {
            self.build_token(SqliteTokenKind::IDEN)
        }
    }

    fn process_int_or_real_literal(&mut self) -> SqliteToken {
        let allow_underscore = self.version.underscore_in_numerics();
        let mut has_decimal_point = false;
        let mut has_exponent = false;

        let mut match_exponent = |cursor: &mut LexCursor| {
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
            self.cursor.next();
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
            return self.build_error_token("Trailing junk after numeric literal", true);
        }

        if has_decimal_point {
            self.build_token(SqliteTokenKind::REAL_LIT)
        } else {
            self.build_token(SqliteTokenKind::INT_LIT)
        }
    }

    fn process_hex_literal(&mut self) -> SqliteToken {
        self.cursor.advance_by(2); // Consume the `0x` or `0X`
        let allow_underscore = self.version.underscore_in_numerics();

        match_digit(&mut self.cursor, allow_underscore, HEXA_RADIX);

        self.build_token(SqliteTokenKind::HEX_LIT)
    }

    fn process_blob_literal(&mut self) -> SqliteToken {
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
            return self.build_error_token("Unterminated blob literal", false);
        } else if (blob_lit_len % 2 != 0) || is_malformed {
            return self.build_error_token("Malformed blob literal", false);
        } else {
            return self.build_token(BLOB_LIT);
        }
    }

    fn build_token(&mut self, token_kind: SqliteTokenKind) -> SqliteToken {
        let (text, abs_pos) = self.cursor.build_token_info();

        SqliteToken::new(token_kind, text, abs_pos)
    }

    /// `consume` variable tells us if we should continue consuming characters to reach the
    /// beginnning of next token before building the error token or to build it right away.
    fn build_error_token(&mut self, hint: &'static str, consume: bool) -> SqliteToken {
        let (data, abs_pos) = self.cursor.build_error_token_info(|ch| {
            !is_separate_token_start(ch) && !ch.is_whitespace() && consume
        });

        self.errors.push(LexError {
            message: hint,
            token_idx: self.tokens.len(), // TODO: Not good
        });

        SqliteToken::new(SqliteTokenKind::ERROR, data, abs_pos)
    }
}

fn match_digit(cursor: &mut LexCursor, allow_underscore: bool, radix: u32) {
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
                cursor.next();
            }
            _ => break,
        }
    }
}

// Based on: https://github.com/gwenn/lemon-rs/blob/69c67e128c395992e0c805e858607c035241cac6/src/dialect/mod.rs#L85C15-L85C34
pub(crate) fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch > '\u{7F}'
}

// Based on: https://github.com/gwenn/lemon-rs/blob/69c67e128c395992e0c805e858607c035241cac6/src/dialect/mod.rs#L89
pub(crate) fn is_identifier_continue(ch: char) -> bool {
    ch == '$' || ch.is_ascii_alphanumeric() || ch == '_' || ch > '\u{7F}'
}

pub(crate) fn is_separate_token_start(ch: char) -> bool {
    !is_identifier_start(ch) && ch != '$'
}

#[cfg(test)]
macro_rules! check {
    ($input:expr, $token_pat:pat) => {
        let lexer = SqliteLexer::new($input, SqliteVersion([3, 46, 0]));

        let (tokens_fat, _) = lexer.lex();

        let tokens = tokens_fat.iter().map(|t| t.kind).collect::<Vec<_>>();

        match tokens.as_slice() {
            $token_pat => {}
            _ => panic!(
                "Expected {:?}, got {:?}",
                stringify!($token_pat),
                tokens_fat
            ),
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
    check!("1`1", [INT_LIT, ERROR, INT_LIT]);
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
    check!("true", [KW_TRUE]);
    check!(
        "true FalSe SELECT select",
        [KW_TRUE, WHITESPACE, KW_FALSE, WHITESPACE, KW_SELECT, WHITESPACE, KW_SELECT]
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
        "users 'users' \"users\" [users] `users`",
        [IDEN, WHITESPACE, STR_LIT, WHITESPACE, IDEN, WHITESPACE, IDEN, WHITESPACE, IDEN]
    );
}

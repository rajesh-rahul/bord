use std::cell::Cell;

use enumset::EnumSet;

use crate::{
    cst::SqliteUntypedAst,
    grammar::common::{
        EXPR_BIND_PARAM_START, EXPR_LIT_START, EXPR_PREFIX_START, IDEN_SET, JOIN_KEYWORDS,
    },
    SqliteNode, SqliteToken, SqliteTokenKind, SqliteTreeKind,
};

pub struct SqliteParser {
    pub(crate) all_tokens: Vec<SqliteToken>,
    pub(crate) events: Vec<Event>,

    tokens: Vec<SqliteTokenKind>,
    pos: usize,
    all_tokens_pos: usize,
    recoverable_token: Option<SqliteTokenKind>,
    fuel: Cell<u32>,
    errors: Vec<SqliteParseError>,
    prev_token: Option<usize>,

    // The following identifier related token sets come from parse.y of SQLite. Because
    // the IDEN_SET (or `ID` in parse.y) changes depending on the build of SQLite, we put
    // it here so that we can adjust it for each parser invocation (in the future)
    #[allow(dead_code)]
    pub(crate) iden_set: EnumSet<SqliteTokenKind>,

    /// also called `id` in parse.y (NOTE: `ID` (uppercase) is iden_set. `id` is a superset of `ID`)
    #[allow(dead_code)]
    pub(crate) iden: EnumSet<SqliteTokenKind>,

    /// also called `nm` in parse.y
    pub(crate) name_token: EnumSet<SqliteTokenKind>,

    /// also called `ids` in parse.y
    pub(crate) iden_or_str: EnumSet<SqliteTokenKind>,

    /// also called `idj` in parse.y
    pub(crate) iden_or_join: EnumSet<SqliteTokenKind>,

    pub(crate) expr_start: EnumSet<SqliteTokenKind>,

    pub(crate) with_alias_start: EnumSet<SqliteTokenKind>,
}

#[derive(Debug)]
pub(crate) enum Event {
    Open { kind: SqliteTreeKind },
    Error { error_idx: u16 },
    Close,
    Advance,
}

#[derive(Clone, Copy)]
pub(crate) struct MarkOpened {
    index: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct MarkClosed {
    index: usize,
}

#[derive(Debug, Clone)]
pub struct SqliteParseError {
    pub range: (u32, u32),
    pub message: ParseErrorKind,
}

#[derive(Clone, Debug)]
pub enum ParseErrorKind {
    ExpectedItem(Vec<ExpectedItem>),
    UnknownTokens,
    OtherError(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectedItem {
    Token(SqliteTokenKind),
    Tree(SqliteTreeKind),
}

impl std::fmt::Display for SqliteParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.message))
    }
}
impl std::error::Error for SqliteParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<SqliteTokenKind> for ExpectedItem {
    fn from(value: SqliteTokenKind) -> Self {
        ExpectedItem::Token(value)
    }
}

impl From<SqliteTreeKind> for ExpectedItem {
    fn from(value: SqliteTreeKind) -> Self {
        ExpectedItem::Tree(value)
    }
}

impl<T: Into<ExpectedItem>> From<T> for ParseErrorKind {
    fn from(value: T) -> Self {
        ParseErrorKind::ExpectedItem(vec![value.into()])
    }
}

impl From<&[ExpectedItem]> for ParseErrorKind {
    fn from(value: &[ExpectedItem]) -> Self {
        ParseErrorKind::ExpectedItem(value.to_vec())
    }
}

impl SqliteParser {
    pub fn new(tokens: Vec<SqliteToken>) -> Self {
        use SqliteTokenKind::*;
        // TODO: Use a parser context to configure IDEN_SET at runtime

        let iden_set = IDEN_SET;
        let iden = iden_set | KW_INDEXED;
        let iden_or_str = iden_set | STR_LIT;

        // NOTE: in parse.y, JOIN_KW refers to multiple words and not just `JOIN`
        let iden_or_join = iden_set | KW_INDEXED | JOIN_KEYWORDS;

        // In SQLite, all token that are considered names (Like column name or table names)
        // also accept STR_LIT but we won't do that(Recommended by SQLite too).
        let name_token = iden_or_join;

        let expr_start = EXPR_LIT_START
            | EXPR_PREFIX_START
            | EXPR_BIND_PARAM_START
            | name_token
            | L_PAREN
            | KW_CAST
            | KW_NOT
            | KW_EXISTS
            | KW_CASE
            | KW_RAISE;

        let with_alias_start = iden_or_str | KW_AS;

        Self {
            tokens: tokens
                .iter()
                .filter_map(|it| if !it.is_trivia() { Some(it.kind) } else { None })
                .collect(),
            all_tokens: tokens,
            all_tokens_pos: 0,
            pos: 0,
            recoverable_token: None,
            fuel: Cell::new(256),
            events: Vec::new(),
            errors: Vec::new(),
            prev_token: None,
            iden_set,
            iden,
            name_token,
            iden_or_str,
            iden_or_join,
            expr_start,
            with_alias_start,
        }
    }

    pub(crate) fn wrap(
        &mut self,
        kind: SqliteTreeKind,
        mut child_code: impl FnMut(&mut SqliteParser),
    ) -> MarkClosed {
        let m = self.open();
        child_code(self);
        self.close(m, kind)
    }

    pub(crate) fn wrap_err(
        &mut self,
        error: impl Into<ParseErrorKind>,
        r: EnumSet<SqliteTokenKind>,
        mut child_code: impl FnMut(&mut SqliteParser),
    ) -> MarkClosed {
        let m = self.open();

        let range_start = self.curr_non_triv_token().map(|it| it.start()).unwrap_or(0);
        let mut range_end = self.curr_non_triv_token().map(|it| it.end()).unwrap_or(0);

        child_code(self);

        if let Some(tk) = self.curr_non_triv_token() {
            range_end = tk.start();
        }

        while !self.eof() && !r.contains(self.nth(0)) {
            self.advance();
            let (_, new_end) = self.prev_token().unwrap().full_range();
            range_end = new_end
        }

        self.errors.push(SqliteParseError {
            range: (range_start, range_end),
            message: error.into(),
        });

        dbg!(&self.errors);

        self.close_err(m, (self.errors.len() - 1) as u16)
    }

    pub(crate) fn build_tree<'a>(self) -> SqliteUntypedAst {
        let mut all_tokens = self.all_tokens.into_iter();
        let mut events = self.events;

        assert!(matches!(events.pop(), Some(Event::Close { .. })));

        let Some(Event::Open { kind, .. }) = events.first() else {
            panic!("Expected something in events");
        };

        let mut ast = SqliteUntypedAst::new();
        let mut curr_id = ast.next_idx();

        ast.allocate(SqliteNode::new_tree_node(*kind, None, curr_id));

        for event in &events[1..] {
            match event {
                Event::Open { kind, .. } => {
                    curr_id = curr_id.add_tree_child(&mut ast, *kind);
                }
                Event::Error { error_idx } => {
                    curr_id = curr_id.add_error_child(&mut ast, *error_idx);
                }
                Event::Close { .. } => {
                    curr_id = curr_id.as_node(&ast).parent().unwrap();
                }
                Event::Advance => {
                    let token = all_tokens.next().unwrap();

                    curr_id.add_token_child(&mut ast, token);
                }
            }
        }

        assert!(all_tokens.next().is_none());

        ast.add_errors(self.errors);

        ast
    }

    pub(crate) fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened {
            index: self.events.len(),
        };
        self.events.push(Event::Error { error_idx: 0 });
        mark
    }

    pub(crate) fn open_before(&mut self, m: MarkClosed) -> MarkOpened {
        let mark = MarkOpened { index: m.index };
        self.events.insert(m.index, Event::Error { error_idx: 0 });
        mark
    }

    pub(crate) fn wrap_parent(
        &mut self,
        child_close_m: MarkClosed,
        parent_kind: SqliteTreeKind,
    ) -> MarkClosed {
        let parent_open_m = self.open_before(child_close_m);

        self.close(parent_open_m, parent_kind)
    }

    pub(crate) fn close(&mut self, m: MarkOpened, kind: SqliteTreeKind) -> MarkClosed {
        self.events[m.index] = Event::Open { kind };
        self.events.push(Event::Close);
        MarkClosed { index: m.index }
    }

    pub(crate) fn close_err(&mut self, m: MarkOpened, error_idx: u16) -> MarkClosed {
        self.events[m.index] = Event::Error { error_idx };
        self.events.push(Event::Close);
        MarkClosed { index: m.index }
    }

    pub(crate) fn advance(&mut self) {
        assert!(!self.eof());

        self.eat_whitespace();

        if let Some(token) = self.recoverable_token {
            assert_eq!(self.all_tokens[self.all_tokens_pos].kind, token);
            self.recoverable_token = None
        }

        self.fuel.set(256);
        self.events.push(Event::Advance);
        self.prev_token = Some(self.all_tokens_pos);
        self.pos += 1;
        self.all_tokens_pos += 1;
    }

    pub(crate) fn expected(&mut self, item: impl Into<ExpectedItem>, r: EnumSet<SqliteTokenKind>) {
        self.proceed_with_err(r, item.into());
    }

    pub(crate) fn expected_one_of(
        &mut self,
        items: impl Iterator<Item = ExpectedItem>,
        r: EnumSet<SqliteTokenKind>,
    ) {
        self.proceed_with_err(r, ParseErrorKind::ExpectedItem(items.collect()));
    }

    pub(crate) fn tokens(&self) -> &[SqliteTokenKind] {
        &self.tokens[self.pos..]
    }

    pub(crate) fn go_back_all_tokens_by(&self, n: usize) -> Option<&SqliteToken> {
        self.all_tokens.get(self.all_tokens_pos - n - 1)
    }

    pub(crate) fn advance_by(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    pub(crate) fn prev_token(&self) -> Option<&SqliteToken> {
        self.prev_token.map(|it| &self.all_tokens[it])
    }

    // pub(crate) fn proceed_with_err2(&mut self, expected_)
    // TODO: Non sensical values should be combined
    pub(crate) fn proceed_with_err(
        &mut self,
        r: EnumSet<SqliteTokenKind>,
        error: impl Into<ParseErrorKind>,
    ) {
        let m = self.open();

        let curr_token_kind = self.nth(0);

        let can_recover = r.contains(curr_token_kind);

        // let range = if !self.eof() && !can_recover {
        //     self.advance();
        //     self.prev_token()
        //         .expect("IMPOSSIBLE: advance call guaranteed prev token")
        //         .full_range()
        // } else {
        //     self.prev_token()
        //         .map(|tk| (tk.end(), tk.end()))
        //         .unwrap_or((0, 0))
        // };

        let mut range = self
            .prev_token()
            .map(|tk| (tk.end(), tk.end()))
            .unwrap_or((0, 0));
        while !self.eof() && !r.contains(self.nth(0)) {
            self.advance();
            let (_, new_end) = self.prev_token().unwrap().full_range();
            range = (range.0, new_end);
        }

        self.errors.push(SqliteParseError {
            range,
            message: error.into(),
        });
        self.close_err(m, (self.errors.len() - 1) as u16);
    }

    pub(crate) fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    pub(crate) fn nth(&self, lookahead: usize) -> SqliteTokenKind {
        if self.fuel.get() == 0 {
            panic!("parser is stuck")
        }
        self.fuel.set(self.fuel.get() - 1);

        self.tokens
            .get(self.pos + lookahead)
            .map_or(SqliteTokenKind::EOF, |it| *it)
    }

    pub(crate) fn curr_non_triv_token(&self) -> Option<&SqliteToken> {
        self.all_tokens[self.all_tokens_pos..]
            .iter()
            .find(|it| !it.is_trivia())
    }

    pub(crate) fn at(&self, kind: SqliteTokenKind) -> bool {
        self.nth(0) == kind
    }

    pub(crate) fn at_any(&self, set: EnumSet<SqliteTokenKind>) -> bool {
        set.contains(self.nth(0))
    }

    pub(crate) fn eat(&mut self, kind: SqliteTokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(crate) fn eat_whitespace(&mut self) {
        while self.all_tokens_pos < self.all_tokens.len()
            && self.all_tokens[self.all_tokens_pos].is_trivia()
        {
            self.all_tokens_pos += 1;
            self.events.push(Event::Advance);
        }
    }

    pub(crate) fn eat_any(&mut self, set: EnumSet<SqliteTokenKind>) -> bool {
        if self.at_any(set) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Used to eat tokens that are guaranteed to be eaten. Its a logic bug to not be eaten
    pub(crate) fn guaranteed(&mut self, tk: SqliteTokenKind) {
        if !self.eat(tk) {
            unreachable!("DEV ERROR: {tk:?} was guaranteed to be eaten")
        }
    }

    /// Used to eat token sets that are guaranteed to be eaten. Its a logic bug to not be eaten
    pub(crate) fn guaranteed_any(&mut self, set: EnumSet<SqliteTokenKind>) {
        if !self.eat_any(set) {
            unreachable!("DEV ERROR: set was guaranteed to be eaten")
        }
    }

    pub(crate) fn must_eat(&mut self, kind: SqliteTokenKind, r: EnumSet<SqliteTokenKind>) {
        if !self.eat(kind) {
            let m = self.open();

            let mut range = self
                .prev_token()
                .map(|tk| (tk.end(), tk.end()))
                .unwrap_or((0, 0));

            while !self.eof() && !r.contains(self.nth(0)) {
                self.advance();
                let (_, new_end) = self.prev_token().unwrap().full_range();
                range = (range.0, new_end);
            }

            if self.at(kind) {
                self.errors.push(SqliteParseError {
                    range,
                    message: ParseErrorKind::UnknownTokens,
                });
            } else {
                self.errors.push(SqliteParseError {
                    range,
                    message: kind.into(),
                });
            }
            self.close_err(m, (self.errors.len() - 1) as u16);
            self.eat(kind);
        }
    }

    pub(crate) fn must_eat_any(
        &mut self,
        kinds: EnumSet<SqliteTokenKind>,
        expected: SqliteTreeKind,
        r: EnumSet<SqliteTokenKind>,
    ) {
        if !self.at_any(kinds) {
            let m = self.open();

            let mut range = self
                .prev_token()
                .map(|tk| (tk.end(), tk.end()))
                .unwrap_or((0, 0));

            while !self.eof() && !r.contains(self.nth(0)) {
                self.advance();
                let (_, new_end) = self.prev_token().unwrap().full_range();
                range = (range.0, new_end);
            }

            if self.at_any(kinds) {
                self.errors.push(SqliteParseError {
                    range,
                    message: ParseErrorKind::UnknownTokens,
                });
                let err_close_m = self.close_err(m, (self.errors.len() - 1) as u16);

                let m = self.open_before(err_close_m);
                self.guaranteed_any(kinds);
                self.close(m, expected);
            } else {
                self.errors.push(SqliteParseError {
                    range,
                    message: expected.into(),
                });
                self.close_err(m, (self.errors.len() - 1) as u16);
            }
        } else {
            let m = self.open();
            self.guaranteed_any(kinds);
            self.close(m, expected);
        }
    }

    pub(crate) fn at_any_now_or_later(
        &self,
        kinds: EnumSet<SqliteTokenKind>,
        r: EnumSet<SqliteTokenKind>,
    ) -> bool {
        if self.at_any(kinds) {
            return true;
        } else {
            self.tokens()
                .iter()
                .skip_while(|&&it| !r.contains(it))
                .next()
                .is_some_and(|&it| kinds.contains(it))
        }
    }
}

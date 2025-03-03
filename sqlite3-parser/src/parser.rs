use std::{cell::Cell, collections::VecDeque};

use enumset::EnumSet;
use text_size::TextSize;

use crate::{
    grammar::common::{EXPR_LIT_START, EXPR_PREFIX_START, IDEN_SET, JOIN_KEYWORDS},
    CstTrait, SqliteLexer, SqliteToken, SqliteTokenKind, SqliteTreeKind, SqliteTreeTag,
    SqliteVersion, T,
};

pub struct SqliteParser<T> {
    pub(crate) events: Vec<Event>,
    pub lexer: T,
    pub(crate) abs_pos: TextSize,

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

    pub(crate) arg_expr_start: EnumSet<SqliteTokenKind>,

    pub(crate) with_alias_start: EnumSet<SqliteTokenKind>,

    pub(crate) table_or_subquery_start: EnumSet<SqliteTokenKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Open {
        kind: SqliteTreeKind,
        tag: SqliteTreeTag,
        close_idx: usize,
    },
    Error(ParseErrorKind),
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SqliteParseError {
    pub range: (u32, u32),
    pub message: ParseErrorKind,
}

impl Default for SqliteParseError {
    fn default() -> Self {
        Self {
            range: Default::default(),
            message: ParseErrorKind::UnknownTokens,
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct SqliteParseError2 {
//     pub message: ParseErrorKind,
// }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseErrorKind {
    ExpectedItems(Vec<ExpectedItem>),
    UnknownTokens,
    IllegalJoinOperator, // OtherError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        ParseErrorKind::ExpectedItems(vec![value.into()])
    }
}

impl From<&'static [ExpectedItem]> for ParseErrorKind {
    fn from(value: &'static [ExpectedItem]) -> Self {
        ParseErrorKind::ExpectedItems(value.to_vec())
    }
}

impl ParseErrorKind {
    pub fn is_missing_semicolon_err(&self) -> bool {
        match self {
            ParseErrorKind::ExpectedItems(items) => {
                matches!(items.as_slice(), &[ExpectedItem::Token(T![;])])
            }
            _ => false,
        }
    }
}

pub trait Lexer {
    fn all_tokens(self) -> Vec<SqliteToken>;
    fn eof(&self) -> bool;
    fn tokens_so_far(&self) -> &[SqliteToken];
    fn prev_non_triv_token(&self) -> Option<&SqliteToken>;
    // fn first(&self) -> SqliteTokenKind;
    // fn second(&self) -> SqliteTokenKind;
    // fn third(&self) -> SqliteTokenKind;
    fn nth(&self, lookahead: usize) -> SqliteTokenKind;
    // fn non_triv_token_kinds(&self) -> impl Iterator<Item = SqliteTokenKind>;
    // fn all_tokens_iter(&self) -> impl Iterator<Item = SqliteToken>;
    /// Returns how many tokens were eaten
    fn eat_trivia(&mut self) -> usize;

    /// Returns how many tokens were eaten
    fn advance(&mut self) -> usize;

    fn curr_byte_len(&self) -> TextSize;
}

pub struct OnDemandLexer<'a>
where
// IA: Iterator<Item = SqliteToken> + Clone,
{
    inner: Cell<Option<PeekableLexer<'a>>>,
    all_tokens: Vec<SqliteToken>,
    fuel: Cell<u32>,
    curr_byte_len: TextSize,
    prev_non_triv_tk: Option<usize>,
}

// TODO: Room for optimization
struct PeekableLexer<'a> {
    queue: VecDeque<SqliteToken>,

    lexer: SqliteLexer<'a>,
}

impl<'a> PeekableLexer<'a> {
    fn new(lexer: SqliteLexer<'a>) -> Self {
        Self {
            queue: VecDeque::with_capacity(64),
            lexer,
        }
    }

    fn peek_nth(&mut self, idx: usize) -> Option<&SqliteToken> {
        if idx < self.queue.len() {
            Some(&self.queue[idx])
        } else {
            let peek_size = self.queue.len().abs_diff(idx + 1);
            self.queue.extend((&mut self.lexer).take(peek_size));

            self.queue.get(idx)
        }
    }

    fn next_if(&mut self, go_to_next: fn(&SqliteToken) -> bool) -> Option<SqliteToken> {
        if self.peek_nth(0).is_some_and(go_to_next) {
            self.queue.pop_front()
        } else {
            None
        }
    }

    fn next(&mut self) -> Option<SqliteToken> {
        self.peek_nth(0);

        self.queue.pop_front()
    }
}

pub struct NormalLexer {
    all_tokens: Vec<SqliteToken>,
    pos: usize,
    fuel: Cell<u32>,
    prev_non_triv_tk: Option<usize>,
    all_tokens_pos: usize,
    curr_byte_len: TextSize,
    tokens: Vec<SqliteTokenKind>,
}

pub fn new_on_demand_lexer<'a>(text: &'a str, version: SqliteVersion) -> OnDemandLexer<'a> {
    let lexer = SqliteLexer::new(text, version);

    OnDemandLexer::from(lexer.clone())
}
impl<'a> From<SqliteLexer<'a>> for OnDemandLexer<'a>
// IA: Iterator<Item = SqliteToken> + Clone,
{
    fn from(value: SqliteLexer<'a>) -> Self {
        OnDemandLexer {
            inner: Cell::new(Some(PeekableLexer::new(value))),
            prev_non_triv_tk: None,
            curr_byte_len: TextSize::new(0),
            all_tokens: Vec::with_capacity(32),
            fuel: Cell::new(256),
        }
    }
}

impl<'a> From<SqliteLexer<'a>> for NormalLexer {
    fn from(value: SqliteLexer<'a>) -> Self {
        let all_tokens = value.lex();

        NormalLexer {
            pos: 0,
            prev_non_triv_tk: None,
            all_tokens_pos: 0,
            curr_byte_len: TextSize::new(0),
            tokens: all_tokens
                .iter()
                .filter_map(|it| if !it.is_trivia() { Some(it.kind) } else { None })
                .collect(),
            all_tokens,
            fuel: Cell::new(256),
        }
    }
}

impl<'a> Lexer for OnDemandLexer<'a>
where
// IA: Iterator<Item = SqliteToken> + Clone,
{
    fn eat_trivia(&mut self) -> usize {
        let initial_size = self.all_tokens.len();

        while let Some(tk) = self
            .inner
            .get_mut()
            .as_mut()
            .unwrap()
            .next_if(|it| it.is_trivia())
        {
            self.curr_byte_len += tk.text_len();
            self.all_tokens.push(tk);
        }
        // self.inner.set(Some(l));

        return self.all_tokens.len() - initial_size;
    }

    fn eof(&self) -> bool {
        self.nth(0) == SqliteTokenKind::EOF
    }

    fn advance(&mut self) -> usize {
        let num_trivia_tk_eaten = self.eat_trivia();

        if let Some(tk) = self.inner.get_mut().as_mut().unwrap().next() {
            self.fuel.set(256);
            self.prev_non_triv_tk = Some(self.all_tokens.len());
            self.curr_byte_len += tk.text_len();
            self.all_tokens.push(tk);
            // self.inner_token_peek.get_mut().as_mut().unwrap().next().unwrap();
        } else {
            panic!("Unexpected EOF");
        }

        return num_trivia_tk_eaten + 1;
    }

    fn prev_non_triv_token(&self) -> Option<&SqliteToken> {
        self.prev_non_triv_tk.map(|it| &self.all_tokens[it])
    }

    fn tokens_so_far(&self) -> &[SqliteToken] {
        &self.all_tokens
    }

    fn all_tokens(self) -> Vec<SqliteToken> {
        // assert!(self.eof());
        self.all_tokens
    }

    fn curr_byte_len(&self) -> TextSize {
        self.curr_byte_len
    }

    fn nth(&self, lookahead: usize) -> SqliteTokenKind {
        if self.fuel.get() == 0 {
            panic!("parser is stuck")
        }
        self.fuel.set(self.fuel.get() - 1);

        let mut l = self.inner.take().unwrap();

        let mut i = 0;
        let mut actual_idx = 0;
        loop {
            match l.peek_nth(actual_idx) {
                Some(tk) if tk.is_trivia() => {
                    actual_idx += 1;
                }
                Some(_) => {
                    if i == lookahead {
                        break;
                    } else {
                        i += 1;
                        actual_idx += 1;
                    }
                }
                None => break,
            }
        }

        let result = l
            .peek_nth(actual_idx)
            .map_or(SqliteTokenKind::EOF, |it| it.kind);

        self.inner.set(Some(l));

        result
    }

    // fn non_triv_token_kinds(&self) -> impl Iterator<Item = SqliteTokenKind> {
    //     let iter = self.inner.take().unwrap();
    //     self.inner.set(Some(iter.clone()));

    //     iter.filter(|it| !it.is_trivia()).map(|it| it.kind)
    // }

    // fn all_tokens_iter(&self) -> impl Iterator<Item = SqliteToken> {
    //     let iter = self.inner.take().unwrap();
    //     self.inner.set(Some(iter.clone()));

    //     iter
    // }
}

impl Lexer for NormalLexer {
    fn eat_trivia(&mut self) -> usize {
        let initial_pos = self.all_tokens_pos;
        while self.all_tokens_pos < self.all_tokens.len()
            && self.all_tokens[self.all_tokens_pos].is_trivia()
        {
            self.curr_byte_len += self.all_tokens[self.all_tokens_pos].text_len();
            self.all_tokens_pos += 1;
        }

        return self.all_tokens_pos - initial_pos;
    }

    fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    fn advance(&mut self) -> usize {
        assert!(!self.eof());

        let num_trivia_tk_eaten = self.eat_trivia();

        self.fuel.set(256);
        self.prev_non_triv_tk = Some(self.all_tokens_pos);
        self.pos += 1;
        self.curr_byte_len += self.all_tokens[self.all_tokens_pos].text_len();
        self.all_tokens_pos += 1;

        return num_trivia_tk_eaten + 1;
    }

    fn curr_byte_len(&self) -> TextSize {
        self.curr_byte_len
    }

    fn prev_non_triv_token(&self) -> Option<&SqliteToken> {
        self.prev_non_triv_tk.map(|it| &self.all_tokens[it])
    }

    fn tokens_so_far(&self) -> &[SqliteToken] {
        &self.all_tokens[..self.all_tokens_pos]
    }

    fn all_tokens(self) -> Vec<SqliteToken> {
        assert!(self.eof());
        self.all_tokens
    }

    fn nth(&self, lookahead: usize) -> SqliteTokenKind {
        if self.fuel.get() == 0 {
            panic!("parser is stuck")
        }
        self.fuel.set(self.fuel.get() - 1);

        self.tokens
            .get(self.pos + lookahead)
            .map_or(SqliteTokenKind::EOF, |it| *it)
    }

    // fn non_triv_token_kinds(&self) -> impl Iterator<Item = SqliteTokenKind> {
    //     self.tokens[self.pos..].iter().copied()
    // }

    // fn all_tokens_iter(&self) -> impl Iterator<Item = SqliteToken> {
    //     self.all_tokens[self.all_tokens_pos..].iter().cloned()
    // }
}

impl<T: Lexer> SqliteParser<T> {
    pub fn new(lexer: T) -> Self {
        Self::_new(lexer, TextSize::new(0))
    }

    pub fn with_abs_pos(lexer: T, abs_pos: TextSize) -> Self {
        Self::_new(lexer, abs_pos)
    }

    pub fn _new(lexer: T, abs_pos: TextSize) -> Self {
        use SqliteTokenKind::*;
        // TODO: Use a parser context to configure IDEN_SET at runtime

        let mut iden_set = IDEN_SET;

        let omit_compound_select = false;

        if !omit_compound_select {
            iden_set = iden_set.difference(KW_EXCEPT | KW_INTERSECT | KW_UNION);
        }

        let omit_window_func = false;

        if omit_window_func {
            iden_set = iden_set.difference(
                KW_CURRENT
                    | KW_FOLLOWING
                    | KW_PARTITION
                    | KW_PRECEDING
                    | KW_RANGE
                    | KW_UNBOUNDED
                    | KW_EXCLUDE
                    | KW_GROUPS
                    | KW_OTHERS
                    | KW_TIES,
            );
        }

        let omit_generated_columns = false;

        if omit_generated_columns {
            iden_set = iden_set.difference(KW_GENERATED | KW_ALWAYS);
        }

        let iden = iden_set | KW_INDEXED;
        let iden_or_str = iden_set | STR_LIT;

        // NOTE: in parse.y, JOIN_KW refers to multiple words and not just `JOIN`
        let iden_or_join = iden_set | KW_INDEXED | JOIN_KEYWORDS;

        // TODO: In SQLite, all token that are considered names (Like column name or table names)
        // also accept STR_LIT but we won't do that(Recommended by SQLite too).
        let name_token = iden_or_join | STR_LIT;

        let expr_start = EXPR_LIT_START
            | EXPR_PREFIX_START
            | PARAM
            | name_token
            | L_PAREN
            | KW_CAST
            | KW_NOT
            | KW_EXISTS
            | KW_CASE
            | KW_RAISE;

        let with_alias_start = iden_or_str | KW_AS;

        let table_or_subquery_start = name_token | T!['('];

        let arg_expr_start = KW_DISTINCT | KW_ALL | KW_ORDER | expr_start;

        Self {
            lexer,
            events: Vec::new(),
            iden_set,
            iden,
            abs_pos, // TODO: unnecessary?
            name_token,
            iden_or_str,
            iden_or_join,
            expr_start,
            arg_expr_start,
            table_or_subquery_start,
            with_alias_start,
        }
    }

    pub(crate) fn wrap(
        &mut self,
        kind: SqliteTreeKind,
        mut child_code: impl FnMut(&mut SqliteParser<T>),
    ) -> MarkClosed {
        let m = self.open();
        child_code(self);
        self.close(m, kind)
    }

    pub(crate) fn wrap_err(
        &mut self,
        error: impl Into<ParseErrorKind>,
        r: EnumSet<SqliteTokenKind>,
        mut child_code: impl FnMut(&mut SqliteParser<T>),
    ) -> MarkClosed {
        let m = self.open();

        // let range_start = self.peek_non_triv_token().map(|it| it.start()).unwrap_or(0);
        // let mut range_end = self.peek_non_triv_token().map(|it| it.end()).unwrap_or(0);

        child_code(self);

        // if let Some(tk) = self.peek_non_triv_token() {
        //     range_end = tk.start();
        // }

        while !self.lexer.eof() && !r.contains(self.nth(0)) {
            self.advance();
            // let (_, new_end) = self.lexer.prev_non_triv_token().unwrap().full_range();
            // range_end = new_end
        }

        // self.errors.push(SqliteParseError {
        //     range: (range_start, range_end),
        //     message: error.into(),
        // });

        self.close_err(m, error.into())
    }

    pub(crate) fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened {
            index: self.events.len(),
        };
        // Note: Unknown Tokens is a dummy value, really error kind is determined on `close_err`
        self.events
            .push(Event::Error(ParseErrorKind::UnknownTokens));
        mark
    }

    pub(crate) fn open_before(&mut self, m: MarkClosed) -> MarkOpened {
        let mark = MarkOpened { index: m.index };
        // Note: Event::Error is a dummy value. Real value is determined on `close`
        self.events
            .insert(m.index, Event::Error(ParseErrorKind::UnknownTokens));
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
        self.close_with_tag(m, kind, SqliteTreeTag::NoTag)
    }

    pub(crate) fn close_with_tag(
        &mut self,
        m: MarkOpened,
        kind: SqliteTreeKind,
        tag: SqliteTreeTag,
    ) -> MarkClosed {
        self.events[m.index] = Event::Open {
            kind,
            tag,
            close_idx: self.events.len() - m.index,
        };
        self.events.push(Event::Close);
        MarkClosed { index: m.index }
    }

    pub(crate) fn close_err(&mut self, m: MarkOpened, error: ParseErrorKind) -> MarkClosed {
        self.events[m.index] = Event::Error(error);
        self.events.push(Event::Close);
        MarkClosed { index: m.index }
    }

    pub(crate) fn advance(&mut self) {
        let num_advances = self.lexer.advance();

        self.events
            .extend(std::iter::repeat(Event::Advance).take(num_advances));
    }

    // pub(crate) fn expected(&mut self, item: impl Into<ExpectedItem>, r: EnumSet<SqliteTokenKind>) {
    //     self.proceed_with_err(r, ParseErrorKind::ExpectedItem(item.into()));
    // }

    pub(crate) fn expected_one_of(
        &mut self,
        items: &'static [ExpectedItem],
        r: EnumSet<SqliteTokenKind>,
    ) {
        self.proceed_with_err(r, ParseErrorKind::ExpectedItems(items.to_vec()));
    }

    // pub(crate) fn tokens(&self) -> &[SqliteTokenKind] {
    //     &self.tokens[self.pos..]
    // }

    pub(crate) fn go_back_all_tokens_by(&self, n: usize) -> Option<&SqliteToken> {
        // self.all_tokens.get(self.all_tokens_pos - n - 1)
        self.lexer
            .tokens_so_far()
            .get(self.lexer.tokens_so_far().len() - n - 1)
    }

    pub(crate) fn advance_by(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    // pub(crate) fn prev_token(&self) -> Option<&SqliteToken> {
    //     self.prev_token.map(|it| &self.all_tokens[it])
    // }

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

        if !self.eof() && !can_recover {
            self.advance();
        }

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

        // let mut range = self
        //     .lexer
        //     .prev_non_triv_token()
        //     .map(|tk| (tk.end(), tk.end()))
        //     .unwrap_or((0, 0));
        while !self.lexer.eof() && !r.contains(self.nth(0)) {
            self.advance();
            // let (_, new_end) = self.lexer.prev_non_triv_token().unwrap().full_range();
            // range = (range.0, new_end);
        }

        self.close_err(m, error.into());
    }

    pub(crate) fn curr_byte_len(&self) -> TextSize {
        self.lexer.curr_byte_len()
    }

    pub(crate) fn nth(&self, lookahead: usize) -> SqliteTokenKind {
        self.lexer.nth(lookahead)
    }

    // pub(crate) fn peek_non_triv_token(&self) -> Option<SqliteToken> {
    //     self.lexer.all_tokens_iter().find(|it| !it.is_trivia())
    // }

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

    pub(crate) fn eat_trivia(&mut self) {
        let num_advances = self.lexer.eat_trivia();
        self.events.reserve(num_advances);

        let end = self.lexer.tokens_so_far().len();
        let start = end - num_advances;

        (0..num_advances).for_each(|_| self.events.push(Event::Advance));
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

            // let mut range = self
            //     .lexer
            //     .prev_non_triv_token()
            //     .map(|tk| (tk.end(), tk.end()))
            //     .unwrap_or((0, 0));

            while !self.lexer.eof() && !r.contains(self.nth(0)) {
                self.advance();
                // let (_, new_end) = self.lexer.prev_non_triv_token().unwrap().full_range();
                // range = (range.0, new_end);
            }

            // let error = if self.at(kind) {
            //     ParseErrorKind::UnknownTokens
            // } else {
            //     kind.into()
            // };

            self.close_err(m, kind.into());
            // self.eat(kind);
        }
    }

    pub(crate) fn must_eat2(&mut self, kind: SqliteTokenKind, lr: &mut EnumSet<SqliteTokenKind>) {
        if !self.eat(kind) {
            let m = self.open();

            // let mut range = self
            //     .lexer
            //     .prev_non_triv_token()
            //     .map(|tk| (tk.end(), tk.end()))
            //     .unwrap_or((0, 0));

            while !self.lexer.eof() && !lr.contains(self.nth(0)) {
                self.advance();
                // let (_, new_end) = self.lexer.prev_non_triv_token().unwrap().full_range();
                // range = (range.0, new_end);
            }

            // let error = if self.at(kind) {
            //     ParseErrorKind::UnknownTokens
            // } else {
            //     kind.into()
            // };

            self.close_err(m, kind.into());
            // self.eat(kind);
        } else {
            lr.remove(kind);
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

            // let mut range = self
            //     .lexer
            //     .prev_non_triv_token()
            //     .map(|tk| (tk.end(), tk.end()))
            //     .unwrap_or((0, 0));

            while !self.lexer.eof() && !r.contains(self.nth(0)) {
                self.advance();
                // let (_, new_end) = self.lexer.prev_non_triv_token().unwrap().full_range();
                // range = (range.0, new_end);
            }

            // if self.at_any(kinds) {
            //     // self.errors.push(SqliteParseError {
            //     //     range,
            //     //     message: ParseErrorKind::UnknownTokens,
            //     // });
            //     let err_close_m = self.close_err(m, ParseErrorKind::UnknownTokens);

            //     let m = self.open_before(err_close_m);
            //     self.guaranteed_any(kinds);
            //     self.close(m, expected);
            // } else {
            //     self.close_err(m, expected.into());
            // }
            self.close_err(m, expected.into());
        } else {
            let m = self.open();
            self.guaranteed_any(kinds);
            self.close(m, expected);
        }
    }

    // pub(crate) fn at_any_now_or_later(
    //     &self,
    //     kinds: EnumSet<SqliteTokenKind>,
    //     r: EnumSet<SqliteTokenKind>,
    // ) -> bool {
    //     if self.at_any(kinds) {
    //         return true;
    //     } else {
    //         self.lexer
    //             .non_triv_token_kinds()
    //             .skip_while(|&it| !r.contains(it))
    //             .next()
    //             .is_some_and(|it| kinds.contains(it))
    //     }
    // }

    pub(crate) fn eof(&self) -> bool {
        self.lexer.eof()
    }

    pub fn to_events_and_tokens(self) -> (Vec<Event>, Vec<SqliteToken>) {
        (self.events, self.lexer.all_tokens())
    }

    pub(crate) fn build_cst<CST: CstTrait>(self) -> CST {
        let all_tokens = self.lexer.all_tokens().into_iter();

        CST::build(self.abs_pos, all_tokens, self.events)
    }

    // pub(crate) fn tag_last_closeda(&mut self, tag_to_assign: SqliteTreeTag) {
    //     let Some(Event::Close) = self.events.last().cloned() else {
    //         panic!("DEV ERROR: Only call tag_latest right after closing a tree")
    //     };

    //     if let Event::Open {
    //         kind, close_idx, ..
    //     } = self.events[start_idx]
    //     {
    //         self.events[start_idx] = Event::Open {
    //             kind,
    //             tag: tag_to_assign,
    //             close_idx,
    //         };
    //     }
    // }

    pub(crate) fn tag_last_closed2(&mut self, m: MarkClosed, tag_to_assign: SqliteTreeTag) {
        let Event::Open {
            kind, close_idx, ..
        } = self.events[m.index]
        else {
            panic!("DEV ERROR: Mark Closed not pointing to an open event")
        };

        self.events[m.index] = Event::Open {
            kind,
            tag: tag_to_assign,
            close_idx,
        };
    }
}

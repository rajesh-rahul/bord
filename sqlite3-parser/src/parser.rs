use std::cell::Cell;

use enumset::EnumSet;

use crate::{cst::SqliteUntypedAst, SqliteNode, SqliteToken, SqliteTokenKind, SqliteTreeKind};


pub struct SqliteParser {
    pub(crate) all_tokens: Vec<SqliteToken>,
    pub(crate) events: Vec<Event>,

    tokens: Vec<SqliteTokenKind>,
    pos: usize,
    all_tokens_pos: usize,
    fuel: Cell<u32>,
    errors: Vec<SqliteParseError>,
    prev_token: Option<usize>,
}


#[derive(Debug)]
pub(crate) enum Event {
    Open { kind: SqliteTreeKind },
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

#[derive(Debug)]
pub struct SqliteParseError {
    pub range: (u32, u32),
    pub message: String,
}



impl SqliteParser {
    pub fn new(tokens: Vec<SqliteToken>) -> Self {
        Self {
            tokens: tokens
                .iter()
                .filter_map(|it| if !it.is_trivia() { Some(it.kind) } else { None })
                .collect(),
            all_tokens: tokens,
            pos: 0,
            all_tokens_pos: 0,
            fuel: Cell::new(256),
            events: Vec::new(),
            errors: Vec::new(),
            prev_token: None,
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

    pub(crate) fn build_tree<'a>(self) -> (SqliteUntypedAst, Vec<SqliteParseError>) {
        let mut all_tokens = self.all_tokens.into_iter();
        let mut events = self.events;

        assert!(matches!(events.pop(), Some(Event::Close { .. })));

        let Some(Event::Open { kind }) = events.first() else {
            panic!("Expected something in events");
        };

        let mut ast = SqliteUntypedAst::new();

        let mut curr_id = ast.allocate(SqliteNode::new_tree_node(*kind, None));

        for event in &events[1..] {
            match event {
                Event::Open { kind } => {
                    curr_id = curr_id.add_tree_child(&mut ast, *kind);
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

        (ast, self.errors)
    }

    pub(crate) fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened {
            index: self.events.len(),
        };
        self.events.push(Event::Open {
            kind: SqliteTreeKind::ErrorTree,
        });
        mark
    }

    pub(crate) fn open_before(&mut self, m: MarkClosed) -> MarkOpened {
        let mark = MarkOpened { index: m.index };
        self.events.insert(
            m.index,
            Event::Open {
                kind: SqliteTreeKind::ErrorTree,
            },
        );
        mark
    }

    pub(crate) fn wrap_as_parent(
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

    pub(crate) fn advance(&mut self) {
        assert!(!self.eof());

        self.eat_whitespace();

        self.fuel.set(256);
        self.events.push(Event::Advance);
        self.prev_token = Some(self.all_tokens_pos);
        self.pos += 1;
        self.all_tokens_pos += 1;
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

    // TODO: Non sensical values should be combined
    pub(crate) fn try_or_adv(&mut self, error: impl Into<String>, r: EnumSet<SqliteTokenKind>) {
        let m = self.open();

        let curr_token_kind = self.nth(0);

        let can_recover = r.contains(curr_token_kind);

        let mut range = if !self.eof() && !can_recover {
            self.advance();
            self.prev_token()
                .expect("IMPOSSIBLE: advance call guaranteed prev token")
                .full_range()
        } else {
            self.prev_token().map(|tk| tk.end_range()).unwrap_or((0, 0))
        };

        while !self.eof() && !r.contains(self.nth(0)) {
            self.advance();
            let (_, new_end) = self.prev_token().unwrap().full_range();
            range = (range.0, new_end);
        }

        self.errors.push(SqliteParseError {
            range,
            message: error.into(),
        });

        self.close(m, SqliteTreeKind::ErrorTree);
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

    #[allow(dead_code)]
    pub(crate) fn eat_seq(&mut self, seq: &[SqliteTokenKind]) -> bool {
        let sequence_matches = self.tokens.iter().take(seq.len()).eq(seq.iter());

        if sequence_matches {
            self.advance_by(seq.len());
            true
        } else {
            false
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
    pub(crate) fn expect(&mut self, tk: SqliteTokenKind) {
        if !self.eat(tk) {
            unreachable!("DEV ERROR: {tk:?} was guaranteed to be eaten")
        }
    }

    pub(crate) fn expect_or_advance(&mut self, kind: SqliteTokenKind, r: EnumSet<SqliteTokenKind>) {
        if !self.eat(kind) {
            self.try_or_adv(format!("expected {kind:?}"), r);
        }
    }
}
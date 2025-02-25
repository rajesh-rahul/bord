//! Initally based on: https://github.com/mamcx/tree-flat

pub mod batch;
pub mod incr;
pub mod slot;
pub mod slot_list;
use smol_str::SmolStr;
use text_size::TextSize;

use crate::{
    parser::{Event, ParseErrorKind},
    SqliteTokenKind, SqliteTreeKind,
};

#[derive(Debug, Clone, PartialEq, Eq)] // TODO: Make copy
pub enum CstNodeDataKind {
    Tree(SqliteTreeKind),
    Token(SqliteToken),
    Error(ParseErrorKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CstNodeData {
    pub relative_pos: TextSize,
    pub kind: CstNodeDataKind,
}

impl std::default::Default for CstNodeData {
    fn default() -> Self {
        Self {
            relative_pos: Default::default(),
            kind: CstNodeDataKind::Tree(SqliteTreeKind::File),
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum LexError {
    UnknownToken,
    UnterminatedBlobLiteral,
    MalformedBlobLiteral,
    TrailingJunkAfterNumericLiteral,
    UnterminatedQuotedIdentifier,
    UnterminatedStringLiteral,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SqliteToken {
    pub kind: SqliteTokenKind,
    pub text: SmolStr,
    // pub abs_pos: u32,
    // Ideally you would put this info in SqliteTokenKind but we need to use EnumSet with it
    pub error: Option<LexError>,
}

#[derive(Debug, Clone, Copy)]
pub struct TextPatch<T, B> {
    pub relex_start: T,
    pub affected_node_byte_len: B,
    pub start: TextSize,
    pub size: TextSize,
    pub kind: TextPatchKind,
}

#[derive(Debug, Clone, Copy)]
pub enum TextPatchKind {
    Insert,
    Replace { end: TextSize },
}

/// Represents the IDs of a branches that were modified during a text document edit. Always
/// represents the maximal range. For instance, if 3 branches were removed and 2 were added
/// (or vice versa),the range size would be 3. The goal
#[derive(Debug)]
pub struct ModifiedBranchesInfo {
    pub splice_range: std::ops::Range<usize>,
    pub num_new_branches: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompareCstNode {
    position: (TextSize, TextSize),
    kind: CstNodeDataKind,
    children: Vec<CompareCstNode>,
}

pub trait CstTrait: std::fmt::Display {
    type Node<'a>: CstNodeTrait<'a>
    where
        Self: 'a;

    type Mut<'a>: CstMutTrait<'a>
    where
        Self: 'a;

    fn with_capacity(abs_pos: TextSize, capacity: usize) -> Self;
    fn root<'a>(&'a self) -> Self::Node<'a>;
    fn root_mut<'a>(&'a mut self) -> Self::Mut<'a>;
    fn use_tree_capacity() -> bool;
    fn errors<'a>(&'a self) -> impl DoubleEndedIterator<Item = Self::Node<'a>> {
        self.root()
            .me_and_descendants()
            .filter(|it| it.error().is_some())
    }

    fn statements<'a>(&'a self) -> impl Iterator<Item = Self::Node<'a>> {
        self.root()
            .children()
            .filter(|it| it.tree() == Some(SqliteTreeKind::Statement))
    }

    fn to_events_and_tokens<'a>(&'a self) -> (Vec<Event>, Vec<SqliteToken>) {
        let mut events = Vec::new();
        let mut tokens = Vec::new();

        helper(&mut events, &mut tokens, self.root());

        (events, tokens)
    }

    fn build<'a>(
        abs_pos: TextSize,
        all_tokens: impl Iterator<Item = SqliteToken>,
        events: Vec<Event>,
    ) -> Self
    where
        Self: Sized,
    {
        let capacity = events
            .iter()
            .filter(|it| !matches!(it, Event::Close { .. }))
            .count();

        let mut cst = Self::with_capacity(abs_pos, capacity);
        let cst_mut_handle = populate_cst(
            cst.root_mut(),
            Self::use_tree_capacity(),
            all_tokens,
            events,
        );

        std::mem::drop(cst_mut_handle);
        cst
    }
}

pub(crate) fn populate_cst<'a, T: CstMutTrait<'a>>(
    mut builder: T,
    use_tree_capacity: bool,
    mut all_tokens: impl Iterator<Item = SqliteToken>,
    mut events: Vec<Event>,
) -> T {
    assert!(matches!(events.pop(), Some(Event::Close { .. })));

    let Some(Event::Open { .. }) = events.first() else {
        panic!("Expected something in events");
    };

    for (idx, event) in events[1..].iter().enumerate() {
        match event {
            Event::Open { kind, close_idx } => {
                assert!(matches!(events[idx + close_idx + 1], Event::Close));
                let capacity = if use_tree_capacity {
                    events[idx + 1..idx + close_idx + 1]
                        .iter()
                        .filter(|it| !matches!(it, Event::Close { .. }))
                        .count()
                } else {
                    1
                };

                builder = builder.push_tree(*kind, capacity);
            }
            Event::Error(error) => {
                builder = builder.push_error(error.clone(), 4);
                // curr = curr.push_error(*error_idx as usize);
            }
            Event::Close { .. } => {
                builder = builder.parent_mut();
            }
            Event::Advance => {
                let token = all_tokens.next().unwrap();

                builder.push_token(token);
            }
        }
    }

    assert!(all_tokens.next().is_none());

    builder
}

fn build_comparable_node<'a>(node: impl CstNodeTrait<'a>) -> CompareCstNode {
    CompareCstNode {
        position: (node.start_pos(), node.end_pos()),
        kind: node.data().kind.clone(),
        children: node.children().map(build_comparable_node).collect(),
    }
}

fn helper<'a>(events: &mut Vec<Event>, tokens: &mut Vec<SqliteToken>, node: impl CstNodeTrait<'a>) {
    match &node.data().kind {
        CstNodeDataKind::Tree(tree_kind) => {
            let open_idx = events.len();
            events.push(Event::Error(ParseErrorKind::UnknownTokens));

            for child in node.children() {
                helper(events, tokens, child);
            }

            let close_idx = events.len() - open_idx;
            events[open_idx] = Event::Open {
                kind: *tree_kind,
                close_idx,
            };
            events.push(Event::Close);
        }
        CstNodeDataKind::Token(tk) => {
            events.push(Event::Advance);
            tokens.push(tk.clone())
        }
        CstNodeDataKind::Error(err) => {
            events.push(Event::Error(err.clone()));

            for child in node.children() {
                helper(events, tokens, child);
            }
            events.push(Event::Close);
        }
    }
}

pub trait CstMutTrait<'a>: std::fmt::Debug {
    fn parent_mut(self) -> Self;

    fn push_tree(self, tree: SqliteTreeKind, capacity: usize) -> Self;

    fn push_token(&mut self, token: SqliteToken);

    fn push_error(self, error: ParseErrorKind, capacity: usize) -> Self;
}

pub trait CstNodeTrait<'a>
where
    Self: Sized + std::fmt::Debug + Copy + std::fmt::Display,
{
    type Id: std::hash::Hash + std::cmp::Eq + std::cmp::PartialEq;

    fn data(&self) -> &'a CstNodeData;
    fn id(&self) -> Self::Id;

    fn equals(&self, other: &Self) -> bool {
        self.id() == other.id()
    }

    fn token(&self) -> Option<&'a SqliteToken> {
        match &self.data().kind {
            CstNodeDataKind::Token(tk) => Some(tk),
            _ => None,
        }
    }

    fn token_kind(&self) -> Option<SqliteTokenKind> {
        match &self.data().kind {
            CstNodeDataKind::Token(tk) => Some(tk.kind),
            _ => None,
        }
    }

    fn error(&self) -> Option<&'a ParseErrorKind> {
        match &self.data().kind {
            CstNodeDataKind::Error(err) => Some(err),
            _ => None,
        }
    }

    fn tree(&self) -> Option<SqliteTreeKind> {
        match &self.data().kind {
            CstNodeDataKind::Tree(tree) => Some(*tree),
            _ => None,
        }
    }

    /// Panics if self is root
    fn parent(self) -> Self;

    fn children(self) -> impl DoubleEndedIterator<Item = Self>;

    fn non_trivial_children(&self) -> impl DoubleEndedIterator<Item = Self> {
        self.children()
            .filter(|it| !it.token().is_some_and(|it| it.is_trivia()))
    }

    fn valid_children(&self) -> impl DoubleEndedIterator<Item = Self> {
        self.non_trivial_children()
            .filter(|it| it.error().is_none())
    }

    fn find_children(&self, key: impl ChildNodeKey) -> impl Iterator<Item = Self> {
        key.find_children(*self)
    }

    // Iterate over earlier siblings (In insertion order)
    /// Panics if node is root
    fn left_siblings(&self) -> impl DoubleEndedIterator<Item = Self>;

    // Iterate over later siblings (In insertion order)
    /// Panics if node is root
    fn right_siblings(&self) -> impl DoubleEndedIterator<Item = Self>;

    fn has_errors(&self) -> bool {
        self.me_and_descendants()
            .find_map(|it| it.error())
            .is_some()
    }

    fn byte_len(&self) -> TextSize {
        let end = self.end_pos();
        let start = self.start_pos();
        assert!(end >= start);

        end - start
    }

    // Iterate over siblings (In insertion order), skipping ourselves
    fn siblings(&self) -> impl DoubleEndedIterator<Item = Self> {
        self.left_siblings().chain(self.right_siblings())
    }

    fn ancestors(&self) -> AncestorIter<Self> {
        AncestorIter { curr: *self }
    }

    fn me_and_descendants(self) -> impl DoubleEndedIterator<Item = Self>;

    fn is_root(&self) -> bool;

    fn is_trivia(&self) -> bool {
        self.token().is_some_and(|it| it.is_trivia())
    }

    fn start_pos(&self) -> TextSize;

    fn start_pos_skip_trivia(&self) -> TextSize;

    fn end_pos(&self) -> TextSize;

    fn end_pos_skip_trivia(&self) -> TextSize;

    fn as_str(&self) -> &'static str {
        match &self.data().kind {
            CstNodeDataKind::Tree(tree_kind) => tree_kind.as_str(),
            CstNodeDataKind::Token(tk) => tk.kind.as_str(),
            CstNodeDataKind::Error(_) => "Error",
        }
    }

    fn to_text(&self) -> String {
        self.me_and_descendants()
            .filter_map(|it| match &it.data().kind {
                CstNodeDataKind::Token(token) => Some(token.text.as_str()),
                _ => None,
            })
            .collect()
    }

    fn to_text_with_capacity(&self, capacity: usize) -> String {
        let mut s = String::with_capacity(capacity);

        self.me_and_descendants()
            .filter_map(|it| match &it.data().kind {
                CstNodeDataKind::Token(token) => Some(token.text.as_str()),
                _ => None,
            })
            .for_each(|it| s.push_str(it));

        s
    }

    fn width(&self) -> TextSize {
        let start = self.start_pos();
        let end = self.end_pos();
        assert!(end >= start);

        end - start
    }

    fn comparable(&self) -> CompareCstNode {
        build_comparable_node(self.clone())
    }

    fn print_subtree(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print = |curr: &Self, custom_root: Self::Id| {
            let mut s = format!("{}", curr.data());

            if curr.id() == custom_root {
                return writeln!(
                    f,
                    "{s}({}..{})",
                    u32::from(curr.start_pos()),
                    u32::from(curr.end_pos())
                );
            }

            let parent_id = curr.parent().id();

            for ancestor in curr.ancestors() {
                if ancestor.id() == parent_id {
                    let start: u32 = curr.start_pos().into();
                    let end: u32 = curr.end_pos().into();

                    if ancestor.children().last().map(|it| it.id()) == Some(curr.id()) {
                        s = format!("└───{s}({start}..{end})");
                    } else {
                        s = format!("├───{s}({start}..{end})");
                    }
                } else {
                    match ancestor.children().last() {
                        Some(ancestor) if curr.end_pos() <= ancestor.start_pos() => {
                            s = format!("├   {s}")
                        }
                        _ => s = format!("    {s}"),
                    }
                }

                if ancestor.id() == custom_root {
                    break;
                }
            }

            return writeln!(f, "{s}");
        };

        for descendant in self.me_and_descendants() {
            print(&descendant, self.id())?;
        }

        Ok(())
    }
}

impl SqliteToken {
    pub fn new(kind: SqliteTokenKind, text: &str, error: Option<LexError>) -> Self {
        Self {
            kind,
            text: text.into(),
            // abs_pos,
            error,
        }
    }

    pub fn is_trivia(&self) -> bool {
        self.kind.is_trivia()
    }

    pub fn is_eof(&self) -> bool {
        self.kind == SqliteTokenKind::EOF
    }

    pub fn is_error(&self) -> bool {
        matches!(self.kind, SqliteTokenKind::ERROR)
    }

    pub fn text_matches(&self, other: &str) -> bool {
        self.text.eq_ignore_ascii_case(other)
    }

    pub fn text_len(&self) -> TextSize {
        TextSize::new(self.text.len() as u32)
    }
}

impl std::fmt::Display for CstNodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            CstNodeDataKind::Tree(tree_kind) => std::write!(f, "{:?}", tree_kind),
            CstNodeDataKind::Token(SqliteToken {
                kind: SqliteTokenKind::IDEN,
                text,
                ..
            }) => std::write!(f, "IDEN({})", text),
            CstNodeDataKind::Token(SqliteToken { kind, .. }) => std::write!(f, "{:?}", kind),
            CstNodeDataKind::Error(err) => f.write_str(&format!("Error: {err:?}")),
        }
    }
}

pub struct AncestorIter<N> {
    curr: N,
}

impl<'a, N: CstNodeTrait<'a>> Iterator for AncestorIter<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_root() {
            None
        } else {
            let parent = self.curr.parent();
            self.curr = parent;

            Some(parent)
        }
    }
}

impl CstNodeData {
    pub fn is_trivial(&self) -> bool {
        match &self.kind {
            CstNodeDataKind::Token(tk) => tk.is_trivia(),
            _ => false,
        }
    }
}

pub trait ChildNodeKey {
    fn find_children<'a, N: CstNodeTrait<'a>>(self, node: N) -> impl Iterator<Item = N>;
}

impl ChildNodeKey for SqliteTokenKind {
    fn find_children<'a, N: CstNodeTrait<'a>>(self, node: N) -> impl Iterator<Item = N> {
        node.children()
            .filter(move |child| child.token().is_some_and(|it| it.kind == self))
    }
}

impl ChildNodeKey for SqliteTreeKind {
    fn find_children<'a, N: CstNodeTrait<'a>>(self, node: N) -> impl Iterator<Item = N> {
        node.children()
            .filter(move |child| child.tree() == Some(self))
    }
}

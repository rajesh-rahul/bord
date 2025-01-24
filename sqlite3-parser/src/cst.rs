use crate::{SqliteParseError, SqliteTokenKind, SqliteTreeKind};
use enumset::EnumSet;
use smol_str::SmolStr;
use std::fmt::Write;

/// Tree and Token node terminology comes from matklad's [error resilient parser article]
/// (https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html)
pub struct SqliteUntypedAst {
    nodes: Vec<SqliteNode>,
    pub errors: Vec<SqliteParseError>,
}

#[derive(Debug)]
pub enum SqliteNode {
    Tree(TreeChild),
    Token(TokenChild),
    Error(ErrorChild),
}

#[derive(Debug)]
pub struct TreeChild {
    pub kind: SqliteTreeKind,
    pub children: Vec<NodeId>,
    pub parent: Option<NodeId>,
    pub idx: NodeId,
}

#[derive(Debug)]
pub struct TokenChild {
    pub token: SqliteToken,
    pub parent: NodeId,
    pub idx: NodeId,
}

#[derive(Debug)]
pub struct ErrorChild {
    pub error_idx: u16,
    pub children: Vec<NodeId>,
    pub parent: Option<NodeId>,
    pub idx: NodeId,
}

impl TokenChild {
    fn full_text(&self, text: &mut String) {
        text.push_str(&self.token.text);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SqliteToken {
    pub kind: SqliteTokenKind,
    pub text: SmolStr,
    pub abs_pos: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

pub struct AncestorIter<'a> {
    curr: Option<&'a SqliteNode>,
    ast: &'a SqliteUntypedAst,
}

pub struct NodePrinter<'a> {
    node: &'a SqliteNode,
    ast: &'a SqliteUntypedAst,
}

impl<'a> Iterator for AncestorIter<'a> {
    type Item = (NodeId, &'a SqliteNode);

    fn next(&mut self) -> Option<Self::Item> {
        let parent = self.curr?.parent().map(|it| (it, it.as_node(self.ast)));
        self.curr = parent.map(|(_parent_id, parent)| parent);

        return parent;
    }
}

impl SqliteUntypedAst {
    pub fn root(&self) -> &SqliteNode {
        debug_assert!(!self.nodes.is_empty());
        &self.nodes[0]
    }

    pub fn nodes(&self) -> impl DoubleEndedIterator<Item = (NodeId, &SqliteNode)> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(idx, node)| (NodeId::new(idx), node))
    }

    pub fn allocate(&mut self, node: SqliteNode) {
        self.nodes.push(node);
    }

    pub fn next_idx(&self) -> NodeId {
        NodeId(self.nodes.len())
    }

    pub fn node_ref(&self, id: NodeId) -> &SqliteNode {
        &self.nodes[id.0]
    }

    pub fn new() -> Self {
        SqliteUntypedAst {
            nodes: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn find_associated_err(&self, node: &SqliteNode) -> Option<&SqliteParseError> {
        node.error_child()
            .and_then(|it| self.errors.get(it.error_idx as usize))
    }

    pub fn add_child(&mut self, parent: NodeId, child: NodeId) {
        match &mut self.nodes[parent.0] {
            node @ SqliteNode::Tree { .. } => node.add_child(child),
            node @ SqliteNode::Error { .. } => node.add_child(child),
            _ => panic!("Node is a not a tree node"),
        }
    }

    pub fn add_errors(&mut self, mut errors: Vec<SqliteParseError>) {
        self.errors = std::mem::take(&mut errors)
    }
}

impl std::fmt::Debug for SqliteUntypedAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        self.root().print(&mut buf, 0, self);
        write!(f, "{}", buf)
    }
}

impl<'a> std::fmt::Debug for NodePrinter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        self.node.print(&mut buf, 0, self.ast);
        write!(f, "{}", buf)
    }
}

impl NodeId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
    pub fn add_tree_child<'i, 'a>(
        &self,
        ast: &mut SqliteUntypedAst,
        kind: SqliteTreeKind,
    ) -> NodeId {
        let child_idx = ast.next_idx();
        ast.allocate(SqliteNode::new_tree_node(kind, Some(*self), child_idx));

        ast.add_child(*self, child_idx);

        child_idx
    }

    pub fn add_token_child<'i, 'a>(
        &self,
        ast: &mut SqliteUntypedAst,
        token: SqliteToken,
    ) -> NodeId {
        let child_idx = ast.next_idx();
        ast.allocate(SqliteNode::new_token_node(token, *self, child_idx));

        ast.add_child(*self, child_idx);

        child_idx
    }

    pub fn add_error_child<'i, 'a>(&self, ast: &mut SqliteUntypedAst, error_idx: u16) -> NodeId {
        let child_idx = ast.next_idx();
        ast.allocate(SqliteNode::new_error_node(
            error_idx,
            Some(*self),
            child_idx,
        ));

        ast.add_child(*self, child_idx);

        child_idx
    }

    pub fn as_node<'i, 'a>(&self, ast: &'a SqliteUntypedAst) -> &'a SqliteNode {
        ast.node_ref(*self)
    }
}

impl From<usize> for NodeId {
    fn from(value: usize) -> Self {
        NodeId(value)
    }
}

impl ErrorChild {
    pub fn children<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
    ) -> impl DoubleEndedIterator<Item = &'a SqliteNode> {
        self.children.iter().map(|child_id| child_id.as_node(ast))
    }

    pub fn full_text(&self, ast: &SqliteUntypedAst, text: &mut String) {
        for child in self.children(ast) {
            child.full_text(ast, text);
        }
    }
}

impl TreeChild {
    pub fn children<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
    ) -> impl DoubleEndedIterator<Item = &'a SqliteNode> {
        self.children.iter().map(|child_id| child_id.as_node(ast))
    }

    pub fn non_trivial_children<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
    ) -> impl DoubleEndedIterator<Item = &'a SqliteNode> {
        self.children
            .iter()
            .map(|child_id| child_id.as_node(ast))
            .filter(|it| !it.is_trivial_node())
    }

    pub fn valid_children<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
    ) -> impl DoubleEndedIterator<Item = &'a SqliteNode> {
        self.children(ast)
            .filter(|it| !it.is_trivial_node() && it.error_child().is_none())
    }

    pub fn find_child<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
        key: impl ChildNodeKey,
    ) -> Option<&'a SqliteNode> {
        key.find_children(self, ast).next()
    }

    pub fn find_token_child<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
        key: SqliteTokenKind,
    ) -> impl DoubleEndedIterator<Item = &'a SqliteNode> {
        self.children(ast)
            .filter(move |child| child.token_kind() == Some(key))
    }

    pub fn find_token_child_any<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
        key: EnumSet<SqliteTokenKind>,
    ) -> Option<&'a SqliteNode> {
        self.children(ast).find_map(|it| match it {
            SqliteNode::Token(leaf) => {
                if key.contains(leaf.token.kind) {
                    Some(it)
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    /// NOTE: Trival tokens
    pub fn find_token_group<'a, T: PartialEq + Copy>(
        &'a self,
        ast: &'a SqliteUntypedAst,
        needle: &[T],
        func: impl Fn(&'a SqliteToken, T) -> bool,
    ) -> Option<&[NodeId]> {
        assert!(!needle.is_empty());

        'outer: for window_start_idx in 0..self.children.len() {
            let mut window = self.children[window_start_idx..]
                .iter()
                .map(|it| it.as_node(ast).token_child());

            let mut result_group_end = window_start_idx;

            let mut needle_iter = needle.iter().peekable();

            while let Some(n) = needle_iter.peek() {
                // NOTE: The order of these match clauses matter. We allow having trivial token
                // kinds in the needle. As
                match window.next().flatten() {
                    Some(TokenChild { token, .. }) if func(token, **n) => {
                        needle_iter.next();
                        result_group_end += 1
                    }
                    // We only include trivial tokens in the result group if they result group
                    // already contains a non trivial token (i.e. our result group length > 0)
                    Some(TokenChild { token, .. })
                        if token.is_trivia() && result_group_end != window_start_idx =>
                    {
                        result_group_end += 1;
                    }
                    _ => continue 'outer,
                }
            }

            return Some(&self.children[window_start_idx..result_group_end]);
        }

        None
    }

    pub fn find_children<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
        key: impl ChildNodeKey,
    ) -> impl Iterator<Item = &'a SqliteNode> {
        key.find_children(self, ast)
    }

    /// NOTE: Trival tokens
    pub fn find_token_kind_group<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
        needle: &[SqliteTokenKind],
    ) -> Option<&[NodeId]> {
        self.find_token_group(ast, needle, |lhs, rhs| lhs.kind == rhs)
    }

    /// WARNING: Skips trival nodes, case insensitive
    pub fn find_token_text_group<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
        needle: &[&'static str],
    ) -> Option<&'a [NodeId]> {
        self.find_token_group(ast, needle, |lhs, rhs| lhs.text_matches(rhs))
    }

    pub fn find_token_text<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
        text_to_find: &'static str,
    ) -> Option<&'a SqliteNode> {
        self.children(ast).find(|it| {
            it.token_child()
                .is_some_and(|it| it.token.text_matches(text_to_find))
        })
    }

    pub fn first_token_descendant<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
    ) -> Option<&'a TokenChild> {
        let childless_parent = |node: &&SqliteNode| match node {
            SqliteNode::Error(n) => n.children.is_empty(),
            SqliteNode::Tree(n) => n.children.is_empty(),
            _ => false,
        };

        let mut curr_node = self.children(ast).skip_while(childless_parent).next();

        loop {
            match curr_node {
                Some(SqliteNode::Tree(tree)) => {
                    curr_node = tree.children(ast).skip_while(childless_parent).next();
                }
                Some(SqliteNode::Error(err)) => {
                    curr_node = err.children(ast).skip_while(childless_parent).next();
                }
                Some(SqliteNode::Token(leaf)) => return Some(leaf),
                None => return None,
            }
        }
    }

    pub fn last_token_descendant<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
    ) -> Option<&'a TokenChild> {
        let childless_parent = |node: &&SqliteNode| match node {
            SqliteNode::Error(n) => n.children.is_empty(),
            SqliteNode::Tree(n) => n.children.is_empty(),
            _ => false,
        };

        let mut curr_node = self.children(ast).rev().skip_while(childless_parent).next();

        loop {
            match curr_node {
                Some(SqliteNode::Tree(tree)) => {
                    curr_node = tree.children(ast).rev().skip_while(childless_parent).next();
                }
                Some(SqliteNode::Error(err)) => {
                    curr_node = err.children(ast).rev().skip_while(childless_parent).next();
                }
                Some(SqliteNode::Token(leaf)) => return Some(leaf),
                None => return None,
            }
        }
    }

    pub fn node_start(&self, ast: &SqliteUntypedAst) -> Option<u32> {
        self.first_token_descendant(&ast).map(|n| n.token.start())
    }

    pub fn node_end(&self, ast: &SqliteUntypedAst) -> Option<u32> {
        self.last_token_descendant(&ast).map(|n| n.token.end())
    }

    pub fn node_range(&self, ast: &SqliteUntypedAst) -> Option<(u32, u32)> {
        let start = self.node_start(ast)?;
        let end = self.node_end(ast)?;

        Some((start, end))
    }

    pub fn full_text(&self, ast: &SqliteUntypedAst, text: &mut String) {
        for child in self.children(ast) {
            child.full_text(ast, text);
        }
    }
}

impl SqliteNode {
    pub fn new_tree_node(kind: SqliteTreeKind, parent: Option<NodeId>, idx: NodeId) -> Self {
        Self::Tree(TreeChild {
            kind,
            children: Vec::new(),
            parent,
            idx,
        })
    }

    fn new_error_node(error_idx: u16, parent: Option<NodeId>, idx: NodeId) -> SqliteNode {
        Self::Error(ErrorChild {
            error_idx,
            children: Vec::new(),
            parent,
            idx,
        })
    }

    pub fn new_token_node(token: SqliteToken, parent: NodeId, idx: NodeId) -> Self {
        Self::Token(TokenChild { token, parent, idx })
    }

    pub fn tree_child(&self) -> Option<&TreeChild> {
        match self {
            SqliteNode::Tree(tree) => Some(tree),
            _ => None,
        }
    }

    pub fn error_child(&self) -> Option<&ErrorChild> {
        match self {
            SqliteNode::Error(err) => Some(err),
            _ => None,
        }
    }

    pub fn token_child(&self) -> Option<&TokenChild> {
        match self {
            SqliteNode::Token(leaf) => Some(leaf),
            _ => None,
        }
    }

    pub fn add_child(&mut self, child: NodeId) {
        match self {
            Self::Tree(tree) => tree.children.push(child),
            Self::Error(err) => err.children.push(child),
            Self::Token { .. } => panic!("Cannot add child to token node"),
        }
    }

    fn print(&self, buf: &mut String, level: usize, ast: &SqliteUntypedAst) {
        let indent = "  ".repeat(level);

        match self {
            Self::Tree(tree) => {
                std::write!(buf, "{indent}{:?}\n", tree.kind).unwrap();
                for child in tree.children(ast) {
                    child.print(buf, level + 1, ast)
                }
            }
            Self::Error(err) => {
                std::write!(buf, "{indent}{:?}\n", "Error").unwrap();
                for child in err.children(ast) {
                    child.print(buf, level + 1, ast)
                }
            }
            Self::Token(leaf) => {
                if !leaf.token.is_trivia() {
                    std::write!(
                        buf,
                        "{indent}'{}` - {:?}\n",
                        leaf.token.text,
                        leaf.token.kind
                    )
                    .unwrap();
                }
            }
        }
        assert!(buf.ends_with('\n'));
    }

    pub fn parent(&self) -> Option<NodeId> {
        match self {
            Self::Tree(tree) => tree.parent,
            Self::Error(err) => err.parent,
            Self::Token(leaf) => Some(leaf.parent),
        }
    }

    pub fn idx(&self) -> NodeId {
        match self {
            SqliteNode::Tree(tree_child) => tree_child.idx,
            SqliteNode::Token(token_child) => token_child.idx,
            SqliteNode::Error(error_child) => error_child.idx,
        }
    }

    pub fn ancestors<'a>(&'a self, ast: &'a SqliteUntypedAst) -> AncestorIter<'a> {
        AncestorIter {
            curr: Some(self),
            ast,
        }
    }

    pub fn parent_as_node<'a>(&'a self, ast: &'a SqliteUntypedAst) -> Option<&'a SqliteNode> {
        self.parent().map(|n_id| n_id.as_node(ast))
    }

    pub fn tree_kind(&self) -> Option<SqliteTreeKind> {
        match self {
            Self::Tree(tree) => Some(tree.kind),
            _ => None,
        }
    }

    pub fn token_kind(&self) -> Option<SqliteTokenKind> {
        match self {
            Self::Token(leaf) => Some(leaf.token.kind),
            _ => None,
        }
    }

    pub fn token_text(&self) -> Option<&str> {
        match self {
            Self::Token(leaf) => Some(leaf.token.text.as_str()),
            _ => None,
        }
    }

    // TODO: Optimize
    pub fn as_str(&self) -> &str {
        match self {
            SqliteNode::Tree(tree) => tree.kind.into(),
            SqliteNode::Token(leaf) => leaf.token.kind.as_str(),
            SqliteNode::Error(_) => "Error",
        }
    }

    pub fn printer<'a>(&'a self, ast: &'a SqliteUntypedAst) -> NodePrinter<'a> {
        NodePrinter { node: self, ast }
    }

    pub fn full_text(&self, ast: &SqliteUntypedAst, text: &mut String) {
        match self {
            SqliteNode::Tree(tree) => tree.full_text(ast, text),
            SqliteNode::Token(token) => token.full_text(text),
            SqliteNode::Error(err) => err.full_text(ast, text),
        }
    }

    pub fn is_trivial_node(&self) -> bool {
        matches!(self, SqliteNode::Token(leaf) if leaf.token.is_trivia())
    }

    pub fn has_parse_error(&self, ast: &SqliteUntypedAst) -> bool {
        match self {
            SqliteNode::Error(_) => true,
            SqliteNode::Tree(tree) => tree.children(ast).any(|it| it.has_parse_error(ast)),
            SqliteNode::Token(_) => false,
        }
    }
}

pub trait ChildNodeKey {
    fn find_children<'a>(
        self,
        node: &'a TreeChild,
        ast: &'a SqliteUntypedAst,
    ) -> impl Iterator<Item = &'a SqliteNode>;
}

impl ChildNodeKey for SqliteTokenKind {
    fn find_children<'a>(
        self,
        node: &'a TreeChild,
        ast: &'a SqliteUntypedAst,
    ) -> impl Iterator<Item = &'a SqliteNode> {
        node.children(ast)
            .filter(move |child| child.token_kind() == Some(self))
    }
}

impl ChildNodeKey for SqliteTreeKind {
    fn find_children<'a>(
        self,
        node: &'a TreeChild,
        ast: &'a SqliteUntypedAst,
    ) -> impl Iterator<Item = &'a SqliteNode> {
        node.children(ast)
            .filter(move |child| child.tree_kind() == Some(self))
    }
}

impl ChildNodeKey for EnumSet<SqliteTokenKind> {
    fn find_children<'a>(
        self,
        node: &'a TreeChild,
        ast: &'a SqliteUntypedAst,
    ) -> impl Iterator<Item = &'a SqliteNode> {
        node.children(ast)
            .filter(move |child| child.token_kind().is_some_and(|it| self.contains(it)))
    }
}

impl SqliteToken {
    pub fn new(kind: SqliteTokenKind, text: &str, abs_pos: u32) -> Self {
        Self {
            kind,
            text: text.into(),
            abs_pos,
        }
    }

    pub fn full_range(&self) -> (u32, u32) {
        (self.abs_pos, self.abs_pos + self.text.len() as u32 - 1)
    }

    pub fn start(&self) -> u32 {
        self.abs_pos
    }

    pub fn end(&self) -> u32 {
        self.abs_pos + self.text.len() as u32
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
}

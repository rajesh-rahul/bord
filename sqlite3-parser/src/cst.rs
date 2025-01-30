//! Initally based on: https://github.com/mamcx/tree-flat
use std::num::NonZeroUsize;

use smol_str::SmolStr;

use crate::{ast, SqliteParseError, SqliteTokenKind, SqliteTreeKind};

#[derive(Debug)]
pub enum CstNodeData {
    Tree(SqliteTreeKind),
    Token(SqliteToken),
    Error(usize),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SqliteToken {
    pub kind: SqliteTokenKind,
    pub text: SmolStr,
    pub abs_pos: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(NonZeroUsize);

#[derive(Debug, Clone, Copy)]
pub struct CstNode<'a> {
    pub id: NodeId,
    pub data: &'a CstNodeData,
    pub cst: &'a SqliteUntypedCst,
}

#[derive(Debug)]
pub struct CstMut<'a> {
    pub id: NodeId,
    pub parent: NodeId,
    pub cst: &'a mut SqliteUntypedCst,
}

#[derive(Debug)]
pub struct SqliteUntypedCst {
    pub(crate) data: Vec<CstNodeData>,
    // pub(crate) level: Vec<usize>,
    pub(crate) parent: Vec<usize>,
    pub(crate) children: Vec<tinyvec::TinyVec<[usize; 4]>>,
    pub(crate) errors: Vec<SqliteParseError>,
}

impl NodeId {
    pub fn from_index(n: usize) -> Self {
        NodeId(NonZeroUsize::new(n + 1).unwrap())
    }

    pub fn to_index(self) -> usize {
        self.0.get() - 1
    }

    pub fn is_root(&self) -> bool {
        self.to_index() == 0
    }
}

impl SqliteUntypedCst {
    pub fn new(root: SqliteTreeKind) -> Self {
        Self::with_capacity(root, 1)
    }

    pub fn with_capacity(root: SqliteTreeKind, capacity: usize) -> Self {
        let mut cst = SqliteUntypedCst {
            data: Vec::with_capacity(capacity),
            // level: Vec::with_capacity(capacity),
            parent: Vec::with_capacity(capacity),
            children: Vec::with_capacity(capacity),
            errors: Vec::new(),
        };
        cst.push(CstNodeData::Tree(root), NodeId::from_index(0));

        cst
    }

    pub(crate) fn add_errors(&mut self, errors: Vec<SqliteParseError>) {
        self.errors = errors;
    }

    pub fn errors(&self) -> &[SqliteParseError] {
        &self.errors
    }

    pub fn typed_ast(&self) -> ast::File {
        ast::File::cast(self.root()).unwrap()
    }

    pub fn push(&mut self, data: CstNodeData, parent: NodeId) -> NodeId {
        let parent_idx = parent.to_index();

        self.data.push(data);
        // self.level.push(level);
        self.parent.push(parent_idx);
        self.children.push(Default::default());

        let new_node_idx = self.data.len() - 1;

        if new_node_idx != parent_idx {
            self.children[parent_idx].push(new_node_idx);
        }

        NodeId::from_index(new_node_idx)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    // pub fn get_level(&self, id: NodeId) -> usize {
    //     if id.to_index() == 0 {
    //         0
    //     } else {
    //         self.level[id.to_index()]
    //     }
    // }

    pub fn node(&self, id: NodeId) -> CstNode {
        assert!(id.to_index() < self.data.len());
        CstNode {
            id,
            data: &self.data[id.to_index()],
            cst: self,
        }
    }

    pub fn nodes<'a>(&'a self) -> impl DoubleEndedIterator<Item = CstNode<'a>> {
        (0..self.data.len()).map(|it| self.node(NodeId::from_index(it)))
    }

    pub fn node_mut(&mut self, id: NodeId) -> CstMut {
        assert!(id.to_index() < self.data.len());

        let parent = NodeId::from_index(self.parent[id.to_index()]);

        CstMut {
            id,
            parent,
            cst: self,
        }
    }

    pub fn root(&self) -> CstNode {
        assert!(!self.data.is_empty());

        self.node(NodeId::from_index(0))
    }

    pub fn root_mut(&mut self) -> CstMut {
        assert!(!self.data.is_empty());

        self.node_mut(NodeId::from_index(0))
    }

    pub fn statements(&self) -> impl Iterator<Item = CstNode<'_>> {
        self.root()
            .children()
            .filter(|it| it.tree() == Some(SqliteTreeKind::Statement))
    }
}

impl<'a> CstMut<'a> {
    pub fn parent_mut(self) -> CstMut<'a> {
        assert!(self.id.to_index() != 0);

        self.cst.node_mut(self.parent)
    }

    pub fn push_tree(mut self, tree: SqliteTreeKind) -> CstMut<'a> {
        let new_node_id = self.append(CstNodeData::Tree(tree));

        CstMut {
            id: new_node_id,
            parent: self.id,
            cst: self.cst,
        }
    }

    pub fn push_token(&mut self, token: SqliteToken) -> CstMut {
        let new_node_id = self.append(CstNodeData::Token(token));

        CstMut {
            id: new_node_id,
            parent: self.id,
            cst: self.cst,
        }
    }

    pub fn push_error(mut self, error_idx: usize) -> CstMut<'a> {
        let new_node_id = self.append(CstNodeData::Error(error_idx));

        CstMut {
            id: new_node_id,
            parent: self.id,
            cst: self.cst,
        }
    }

    pub fn append(&mut self, data: CstNodeData) -> NodeId {
        self.cst.push(data, self.id)
    }
}

impl CstNodeData {
    pub fn is_trivial(&self) -> bool {
        match self {
            CstNodeData::Token(tk) => tk.is_trivia(),
            _ => false,
        }
    }
}

impl<'a> CstNode<'a> {
    pub fn token(&self) -> Option<&'a SqliteToken> {
        match &self.data {
            CstNodeData::Token(tk) => Some(tk),
            _ => None,
        }
    }

    pub fn token_kind(&self) -> Option<SqliteTokenKind> {
        match &self.data {
            CstNodeData::Token(tk) => Some(tk.kind),
            _ => None,
        }
    }

    pub fn error(&self) -> Option<&'a SqliteParseError> {
        match &self.data {
            CstNodeData::Error(err_idx) => self.cst.errors.get(*err_idx),
            _ => None,
        }
    }

    pub fn tree(&self) -> Option<SqliteTreeKind> {
        match &self.data {
            CstNodeData::Tree(tree) => Some(*tree),
            _ => None,
        }
    }

    /// Panics if self is root
    pub fn parent(self) -> CstNode<'a> {
        assert!(!self.id.is_root());

        let parent_id = NodeId::from_index(self.cst.parent[self.id.to_index()]);
        self.cst.node(parent_id)
    }

    pub fn children(&self) -> impl DoubleEndedIterator<Item = CstNode<'a>> {
        self.cst.children[self.id.to_index()]
            .iter()
            .map(|it| self.cst.node(NodeId::from_index(*it)))
    }

    pub fn non_trivial_children(&self) -> impl DoubleEndedIterator<Item = CstNode<'a>> {
        self.children()
            .filter(|it| !it.token().is_some_and(|it| it.is_trivia()))
    }

    pub fn valid_children(&self) -> impl DoubleEndedIterator<Item = CstNode<'a>> {
        self.non_trivial_children()
            .filter(|it| it.error().is_none())
    }

    pub fn find_children(&self, key: impl ChildNodeKey) -> impl Iterator<Item = CstNode<'a>> {
        key.find_children(*self)
    }

    // Iterate over earlier siblings (In insertion order)
    /// Panics if node is root
    pub fn left_siblings(&self) -> impl Iterator<Item = CstNode<'a>> + '_ {
        self.parent().children().take_while(|it| it.id != self.id)
    }

    // Iterate over later siblings (In insertion order)
    /// Panics if node is root
    pub fn right_siblings(&self) -> impl Iterator<Item = CstNode<'a>> + '_ {
        // Run the iterator until we find ourselves
        let mut iter = self.parent().children();
        iter.find(|it| it.id == self.id).unwrap();

        iter
    }

    pub fn has_errors(&self) -> bool {
        // End search at the end of the array or at the node immediately to the left of
        // our first right sibling(if we have one)
        let search_end_idx = if self.is_root() {
            self.cst.data.len() - 1
        } else {
            self.right_siblings()
                .next()
                .map(|it| it.id.to_index() - 1)
                .unwrap_or(self.cst.data.len() - 1)
        };

        let search_start_idx = self.id.to_index();

        self.cst.data[search_start_idx..=search_end_idx]
            .iter()
            .any(|it| matches!(it, CstNodeData::Error(_)))
    }

    // Iterate over siblings (In insertion order), skipping ourselves
    pub fn siblings(&self) -> impl Iterator<Item = CstNode<'a>> + '_ {
        self.left_siblings().chain(self.right_siblings())
    }

    pub fn ancestors(&self) -> AncestorIter<'a> {
        AncestorIter { curr: *self }
    }

    pub fn descendants<'b>(&self) -> impl DoubleEndedIterator<Item = CstNode<'a>> {
        let mut curr = *self;

        let mut last_non_triv_desc = self.id.to_index();
        while let Some(descendant) = curr.last_non_trivial_child_idx() {
            last_non_triv_desc = descendant;
            curr = self.cst.node(NodeId::from_index(descendant));
        }

        let start = self.id.to_index() + 1;
        let end = last_non_triv_desc;

        // NOTE: if start is greater than end, we will get an empty iterator
        (start..=end).map(|it| self.cst.node(NodeId::from_index(it)))
    }

    // If this node has no descendants, return index of this node
    // TODO: Can be optimized look at start_pos/end_pos
    fn last_descendant_idx(&self) -> usize {
        let mut last_descendant = self.id.to_index();

        while !self.cst.children[last_descendant].is_empty() {
            let children = &self.cst.children[last_descendant];
            last_descendant = children[children.len() - 1] // safe because children not empty
        }

        last_descendant
    }

    fn last_non_trivial_child_idx(&self) -> Option<usize> {
        self.non_trivial_children()
            .last()
            .map(|it| it.id.to_index())
    }

    fn is_root(&self) -> bool {
        self.id == NodeId::from_index(0)
    }

    fn is_trivia(&self) -> bool {
        self.token().is_some_and(|it| it.is_trivia())
    }

    pub fn start_pos(&self) -> Option<u32> {
        self.start_pos_configurable(true)
    }

    pub fn start_pos_skip_trivia(&self) -> Option<u32> {
        self.start_pos_configurable(false)
    }

    pub fn end_pos(&self) -> Option<u32> {
        self.end_pos_configurable(true)
    }

    pub fn end_pos_skip_trivia(&self) -> Option<u32> {
        self.end_pos_configurable(false)
    }

    /// Use `allow_trivial` to include trivial tokens such as whitespace in end_pos calculation.
    ///
    /// This may not be desired in cases such as when we need to show error squiggly lines
    /// in the editor - having the squiggly line extend past text and into whitespace is unsightly
    // NOTE: We can also implement this by recursively calling start_pos on the first child
    // until we find a token node - but this gotta be faster
    fn start_pos_configurable(&self, allow_trivial: bool) -> Option<u32> {
        // End search at the end of the array or at the node immediately to the left of
        // our first right sibling(if we have one)
        let search_end_idx = if self.is_root() {
            self.cst.data.len() - 1
        } else {
            self.right_siblings()
                .next()
                .map(|it| it.id.to_index() - 1)
                .unwrap_or(self.cst.data.len() - 1)
        };

        let search_start_idx = self.id.to_index(); // Start search from this node

        // Don't skip ourselves. Because if we are a token node, then we are what we are looking for
        for node_data in &self.cst.data[search_start_idx..=search_end_idx] {
            match node_data {
                CstNodeData::Token(tk) if allow_trivial => return Some(tk.start()),
                CstNodeData::Token(tk) if !allow_trivial && !tk.is_trivia() => {
                    return Some(tk.start())
                }
                _ => {}
            }
        }

        None
    }

    /// Use `allow_trivial` to include trivial tokens such as whitespace in end_pos calculation.
    ///
    /// This may not be desired in cases such as when we need to show error squiggly lines
    /// in the editor - having the squiggly line extend past text and into whitespace is unsightly
    fn end_pos_configurable(&self, allow_trivial: bool) -> Option<u32> {
        // Start search from the end of the array or from the node immediately to the left of
        // our first right sibling(if we have one)
        let search_start_idx = if self.is_root() {
            self.cst.data.len() - 1
        } else {
            self.right_siblings()
                .next()
                .map(|it| it.id.to_index() - 1)
                .unwrap_or(self.cst.data.len() - 1)
        };

        let search_end_idx = self.id.to_index(); // End search when we reach this node(inclusive)

        // Don't skip ourselves. Because if we are a token node, then we are what we are looking for
        for node_data in self.cst.data[search_end_idx..=search_start_idx]
            .iter()
            .rev()
        {
            match node_data {
                CstNodeData::Token(tk) if allow_trivial => return Some(tk.end()),
                CstNodeData::Token(tk) if !allow_trivial && !tk.is_trivia() => {
                    return Some(tk.end())
                }
                _ => {}
            }
        }

        None
    }

    pub fn print_subtree(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(f, self.id)?;

        for descendant in self.descendants().filter(|it| !it.is_trivia()) {
            descendant.print(f, self.id)?;
        }

        Ok(())
    }

    pub fn as_str(&self) -> &'static str {
        match self.data {
            CstNodeData::Tree(tree_kind) => tree_kind.as_str(),
            CstNodeData::Token(tk) => tk.kind.as_str(),
            CstNodeData::Error(_) => "Error",
        }
    }

    pub fn to_string(&self) -> String {
        let start = self.id.to_index();
        let end = self.last_descendant_idx() + 1;

        let text = self.cst.data[start..end]
            .iter()
            .filter_map(|it| match it {
                CstNodeData::Token(token) => Some(token.text.as_str()),
                _ => None,
            })
            .collect();

        text
    }

    pub fn print(&self, f: &mut std::fmt::Formatter<'_>, custom_root: NodeId) -> std::fmt::Result {
        let mut s = format!("{}", self.data);

        if self.id == custom_root {
            return writeln!(f, "{s}");
        }

        let this_idx = self.id.to_index();
        let parent_id = self.parent().id;

        for ancestor in self.ancestors() {
            if ancestor.id == parent_id {
                if ancestor.last_non_trivial_child_idx() == Some(this_idx) {
                    s = format!("└───{}", s);
                } else {
                    s = format!("├───{}", s);
                }
            } else {
                match ancestor.last_non_trivial_child_idx() {
                    Some(idx) if idx > this_idx => s = format!("├   {s}"),
                    _ => s = format!("    {s}"),
                }
            }

            if ancestor.id == custom_root {
                break;
            }
        }

        return writeln!(f, "{s}");
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

impl std::fmt::Display for SqliteUntypedCst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root().print_subtree(f)
    }
}

impl std::fmt::Display for CstNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_subtree(f)
    }
}

impl std::fmt::Display for CstNodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CstNodeData::Tree(tree_kind) => std::write!(f, "{:?}", tree_kind),
            CstNodeData::Token(token) => std::write!(f, "`{}` - {:?}", token.text, token.kind),
            CstNodeData::Error(_) => f.write_str("Error"),
        }
    }
}

pub struct AncestorIter<'a> {
    curr: CstNode<'a>,
}

impl<'a> Iterator for AncestorIter<'a> {
    type Item = CstNode<'a>;

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

pub trait ChildNodeKey {
    fn find_children<'a>(self, node: CstNode<'a>) -> impl Iterator<Item = CstNode<'a>>;
}

impl ChildNodeKey for SqliteTokenKind {
    fn find_children<'a>(self, node: CstNode<'a>) -> impl Iterator<Item = CstNode<'a>> {
        node.children()
            .filter(move |child| child.token().is_some_and(|it| it.kind == self))
    }
}

impl ChildNodeKey for SqliteTreeKind {
    fn find_children<'a>(self, node: CstNode<'a>) -> impl Iterator<Item = CstNode<'a>> {
        node.children()
            .filter(move |child| child.tree() == Some(self))
    }
}

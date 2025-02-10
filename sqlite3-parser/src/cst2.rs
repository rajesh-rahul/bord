//! Initally based on: https://github.com/mamcx/tree-flat

use itertools::{Either, Itertools};
use smol_str::SmolStr;
use tinyvec::TinyVec;

use crate::{parser::ParseErrorKind, SqliteParseError, SqliteTokenKind, SqliteTreeKind, T};

#[derive(Debug, Clone, PartialEq, Eq)] // TODO: Make copy
pub enum CstNodeDataKind {
    Tree(SqliteTreeKind),
    Token(SqliteToken),
    Error(ParseErrorKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CstNodeData {
    pub relative_pos: usize,
    pub kind: CstNodeDataKind,
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
pub struct CstNode<'a> {
    pub fat_id: NodeId,
    pub data: &'a CstNodeData,
    pub cst: &'a SqliteUntypedCst,
}

#[derive(Debug)]
pub struct CstMut<'a> {
    pub fat_id: NodeId,
    pub parent: Option<usize>,
    pub cst: &'a mut SqliteUntypedCst,
}

// #[derive(Debug)]
// pub enum CstMutKind {
//     Root,
//     NonRoot {
//         fat_id: NodeId,
//         parent: Option<usize>,
//     },
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {
    pub branch_id: usize,
    pub id: usize,
}

impl NodeId {
    pub const fn new(branch_id: usize, id: usize) -> NodeId {
        NodeId { branch_id, id }
    }

    pub const fn is_root(&self) -> bool {
        matches!(
            self,
            NodeId {
                branch_id: 0,
                id: 0
            }
        )
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct ResolvedNodeId {
//     pub branch_id: usize,
//     pub id: usize,
// }

#[derive(Clone, Eq, PartialEq)]
pub struct SqliteUntypedCst {
    pub(crate) branches: Vec<CstBranch>,
    pub(crate) branch_positions: Vec<usize>,
    pub(crate) errors: Vec<SqliteParseError>,
    pub(crate) byte_len: usize,
    pub(crate) abs_pos: usize,
}

mod private {
    use super::*;

    // #[derive(Clone, Debug, Default, Eq, PartialEq)]
    // pub struct CstBranch {
    //     pub data: Vec<CstNodeData>,
    //     parents: Vec<Option<usize>>,
    //     children: Vec<TinyVec<[usize; 4]>>,
    // }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum CstBranch {
        Token(CstNodeData),
        Tree {
            data: Vec<CstNodeData>,
            parents: Vec<Option<usize>>,
            children: Vec<TinyVec<[usize; 4]>>,
        },
    }

    impl CstBranch {
        pub fn with_capacity(node_data: CstNodeData, capacity: usize) -> CstBranch {
            match &node_data.kind {
                CstNodeDataKind::Token(_) => CstBranch::Token(node_data),
                _ => {
                    let mut data = Vec::with_capacity(capacity);
                    let mut parents = Vec::with_capacity(capacity);
                    let mut children = Vec::with_capacity(capacity);

                    data.push(node_data);
                    parents.push(None);
                    children.push(TinyVec::new());

                    CstBranch::Tree {
                        data,
                        parents,
                        children,
                    }
                }
            }
        }

        pub fn root_branch() -> CstBranch {
            CstBranch::Tree {
                data: vec![CstNodeData {
                    relative_pos: 0,
                    kind: CstNodeDataKind::Tree(SqliteTreeKind::File),
                }],
                parents: Vec::new(),
                children: Vec::new(),
            }
        }

        pub fn len(&self) -> usize {
            match self {
                CstBranch::Tree { data, .. } => data.len(),
                CstBranch::Token(_) => 1,
            }
        }

        pub fn is_root_branch(&self) -> bool {
            match self {
                CstBranch::Tree {
                    parents, children, ..
                } => parents.is_empty() && children.is_empty(),
                _ => false,
            }
        }

        pub fn push_child(&mut self, parent: usize, node_data: CstNodeData) -> usize {
            assert!(!self.is_root_branch());

            match self {
                CstBranch::Token(_) => panic!("Cannot add child to token node"),
                CstBranch::Tree {
                    data,
                    parents,
                    children,
                } => {
                    let child_idx = data.len();

                    data.push(node_data);
                    parents.push(Some(parent));
                    children.push(TinyVec::new());
                    children[parent].push(child_idx);

                    child_idx
                }
            }
        }

        pub fn parent(&self, id: usize) -> Option<usize> {
            if self.is_root_branch() {
                panic!("Root node do not have parent")
            } else {
                match self {
                    CstBranch::Token(_) => None,
                    CstBranch::Tree { parents, .. } => parents.get(id).and_then(|it| *it),
                }
            }
        }

        pub fn data(&self, id: usize) -> &CstNodeData {
            match self {
                CstBranch::Token(data) => {
                    assert!(id == 0);
                    &data
                }
                CstBranch::Tree { data, .. } => &data[id],
            }
        }

        pub fn children_slice(&self, id: usize) -> &[usize] {
            if self.is_root_branch() {
                panic!("Root branch do not directly store children")
            } else {
                match self {
                    CstBranch::Token(_) => &[],
                    CstBranch::Tree { children, .. } => children[id].as_slice(),
                }
            }
        }
    }
}
pub use private::*;

#[derive(Debug, Clone, Copy)]
pub struct TextPatch<T, B> {
    pub relex_start: T,
    pub affected_node_byte_len: B,
    pub start: usize,
    pub size: usize,
    pub kind: TextPatchKind,
}

#[derive(Debug, Clone, Copy)]
pub enum TextPatchKind {
    Insert,
    Replace { end: usize },
}

/// Represents the IDs of a branches that were modified during a text document edit. Always
/// represents the maximal range. For instance, if 3 branches were removed and 2 were added
/// (or vice versa),the range size would be 3. The goal
#[derive(Debug)]
pub struct ModifiedBranchesInfo {
    pub splice_range: std::ops::Range<usize>,
    pub num_new_branches: usize,
}

impl SqliteUntypedCst {
    pub fn with_capacity(abs_pos: usize, capacity: usize) -> Self {
        let mut branches = Vec::with_capacity(capacity + 1);
        let mut branch_positions = Vec::with_capacity(capacity + 1);
        // First branch is a special branch that belongs to the root
        branches.push(CstBranch::root_branch());
        branch_positions.push(0);

        SqliteUntypedCst {
            branches,
            branch_positions,
            abs_pos,
            byte_len: abs_pos,
            errors: Vec::new(),
        }
    }

    pub fn push_cst_branch(&mut self, data: CstNodeData, capacity: usize) -> NodeId {
        // self.branches[0].data.push(data.clone());
        // self.branches[0].parents.push(Some(0));
        // self.branches[0].children.push(TinyVec::new());

        let branch_id = self.branches.len();
        self.branches.push(CstBranch::with_capacity(data, capacity));

        self.branch_positions.push(self.byte_len);
        // self.branches[0].children[0].push(branch_id);

        NodeId::new(branch_id, 0)
    }

    pub fn push_child(&mut self, branch_id: usize, parent: usize, data: CstNodeData) -> NodeId {
        assert!(branch_id < self.branches.len());

        let child_id = self.branches[branch_id].push_child(parent, data);

        NodeId::new(branch_id, child_id)
    }

    pub fn num_branches(&self) -> usize {
        self.branches.len() - 1
    }

    pub fn has_branch(&self, branch_id: usize) -> bool {
        branch_id < self.num_branches() && branch_id > 0
    }

    pub fn updated_text_patch(&self, patch: TextPatch<(), ()>) -> TextPatch<usize, usize> {
        let find_node_with_pos = |pos: usize| {
            let p_point = self.branch_positions[1..].partition_point(|it| *it < pos) + 1;
            if self.has_branch(p_point) && self.branch_positions[p_point] == pos {
                return Some(self.node(NodeId::new(p_point, 0)));
            } else if self.has_branch(p_point - 1) {
                return Some(self.node(NodeId::new(p_point - 1, 0)));
            } else {
                return None;
            }
        };

        let relex_start = find_node_with_pos(patch.start)
            .and_then(|it| {
                it.left_siblings()
                    .rev()
                    .find(|it| it.token_kind() == Some(T![;]))
            })
            .map(|it| it.end_pos().expect("Semicolons always have end pos") as usize)
            .unwrap_or(0);

        let affected_node_byte_len = match patch {
            TextPatch {
                start,
                kind: TextPatchKind::Insert,
                ..
            } => find_node_with_pos(start)
                .map(|it| it.byte_len())
                .unwrap_or(0),
            TextPatch {
                kind: TextPatchKind::Replace { end },
                ..
            } => find_node_with_pos(end).map(|it| it.byte_len()).unwrap_or(0),
        };

        TextPatch {
            relex_start,
            affected_node_byte_len,
            start: patch.start,
            size: patch.size,
            kind: patch.kind,
        }
    }

    // pub fn with_capacity(root: SqliteTreeKind, capacity: usize) -> Self {
    //     let mut cst = SqliteUntypedCst {
    //         data: Vec::with_capacity(capacity),
    //         // level: Vec::with_capacity(capacity),
    //         parent: Vec::with_capacity(capacity),
    //         children: Vec::with_capacity(capacity),
    //         errors: Vec::new(),
    //     };
    //     cst.push(CstNodeData::Tree(root), NodeId::from_index(0));

    //     cst
    // }

    pub fn errors(&self) -> &[SqliteParseError] {
        &self.errors
    }

    // pub fn typed_ast(&self) -> ast::File {
    //     ast::File::cast(self.root()).unwrap()
    // }

    pub fn node<'a>(&'a self, NodeId { branch_id, id }: NodeId) -> CstNode<'a> {
        assert!(branch_id < self.branches.len());
        assert!(id < self.branches[branch_id].len());

        let data = self.branches[branch_id].data(id);

        CstNode {
            fat_id: NodeId::new(branch_id, id),
            data,
            cst: self,
        }
    }

    pub fn node_mut<'a>(&'a mut self, NodeId { branch_id, id }: NodeId) -> CstMut<'a> {
        assert!(branch_id < self.branches.len());
        assert!(id < self.branches[branch_id].len());

        let parent = self.branches[branch_id].parent(id);

        CstMut {
            fat_id: NodeId::new(branch_id, id),
            parent,
            cst: self,
        }
    }

    pub fn root<'a>(&'a self) -> CstNode<'a> {
        static ROOT: CstNodeData = CstNodeData {
            kind: CstNodeDataKind::Tree(SqliteTreeKind::File),
            relative_pos: 0,
        };

        CstNode {
            fat_id: NodeId::new(0, 0),
            data: &ROOT,
            cst: self,
        }
    }

    pub fn root_mut(&mut self) -> CstMut {
        CstMut {
            fat_id: NodeId::new(0, 0),
            parent: None,
            cst: self,
        }
    }

    pub fn statements(&self) -> impl Iterator<Item = CstNode<'_>> {
        self.root()
            .children()
            .filter(|it| it.tree() == Some(SqliteTreeKind::Statement))
    }

    pub fn merge_cst(
        &mut self,
        new_cst: SqliteUntypedCst,
        patch: TextPatch<usize, usize>,
    ) -> ModifiedBranchesInfo {
        let (start, end, grow_size) = {
            let TextPatch {
                relex_start,
                start,
                size,
                kind,
                ..
            } = patch;

            let partition_point =
                self.branch_positions[1..].partition_point(|&it| it < relex_start) + 1;

            // TODO: Should we be incorporating affected_node_byte_len here? (like we do in incremental parse)
            let expected_len = start + size - relex_start;
            let actual_len = new_cst.byte_len - new_cst.abs_pos;
            assert!(actual_len >= expected_len);

            let spillover = actual_len - expected_len;

            let (grow_size, end_text_pos) = match kind {
                TextPatchKind::Insert => {
                    let end_text_pos = start + spillover;

                    (size as isize, end_text_pos)
                }
                TextPatchKind::Replace { end } => {
                    // NOTE: size is not the same as `end - start`. `size` is the length of
                    // the new text and `end - start` represents the length of the text its
                    // replacing
                    let delta = (size as isize) - (end - start) as isize;
                    let end_text_pos = end + spillover;

                    (delta, end_text_pos)
                }
            };

            let mut affected_end =
                self.branch_positions[1..].partition_point(|&it| it < end_text_pos) + 1; // TODO: is <= correct?

            while affected_end < self.branches.len()
                && self.branches[affected_end].len() == 1
                && !matches!(self.branches[affected_end], CstBranch::Token(_))
            {
                affected_end += 1
            }

            (partition_point, affected_end, grow_size)
        };

        let modified_info = ModifiedBranchesInfo {
            splice_range: start..end,
            num_new_branches: new_cst.num_branches(),
        };

        self.byte_len = self
            .byte_len
            .checked_add_signed(grow_size)
            .expect("byte_len is always positive");

        self.branch_positions[end..].iter_mut().for_each(|it| {
            *it = it
                .checked_add_signed(grow_size)
                .expect("Expected addition to be >= 0")
        });

        self.branch_positions
            .splice(start..end, new_cst.branch_positions.into_iter().skip(1));

        self.branches
            .splice(start..end, new_cst.branches.into_iter().skip(1));

        modified_info
    }
}

impl<'a> CstMut<'a> {
    pub fn parent_mut(self) -> CstMut<'a> {
        if self.fat_id.is_root() {
            panic!("Root node do not have parent")
        }

        if let Some(parent) = self.parent {
            self.cst
                .node_mut(NodeId::new(self.fat_id.branch_id, parent))
        } else {
            self.cst.root_mut()
        }
    }

    pub fn push_tree(mut self, tree: SqliteTreeKind, capacity: usize) -> CstMut<'a> {
        let fat_id = self.append(CstNodeDataKind::Tree(tree), capacity);

        self.cst.node_mut(fat_id)
    }

    pub fn push_token(&mut self, token: SqliteToken) -> CstMut<'_> {
        let byte_len_to_add = token.text.len();
        let fat_id = self.append(CstNodeDataKind::Token(token), 1);
        self.cst.byte_len += byte_len_to_add;

        self.cst.node_mut(fat_id)
    }

    pub fn push_error(mut self, error: ParseErrorKind, capacity: usize) -> CstMut<'a> {
        let fat_id = self.append(CstNodeDataKind::Error(error), capacity);

        self.cst.node_mut(fat_id)
    }

    pub fn append(&mut self, kind: CstNodeDataKind, capacity: usize) -> NodeId {
        if self.fat_id.is_root() {
            self.cst.push_cst_branch(
                CstNodeData {
                    relative_pos: 0,
                    kind,
                },
                capacity,
            )
        } else {
            let parent = self.fat_id.id;
            let offset = self.cst.branch_positions[self.fat_id.branch_id];
            let relative_pos = self.cst.byte_len - offset;

            self.cst.push_child(
                self.fat_id.branch_id,
                parent,
                CstNodeData { relative_pos, kind },
            )
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

impl<'a> CstNode<'a> {
    pub fn token(&self) -> Option<&'a SqliteToken> {
        match &self.data.kind {
            CstNodeDataKind::Token(tk) => Some(tk),
            _ => None,
        }
    }

    pub fn token_kind(&self) -> Option<SqliteTokenKind> {
        match &self.data.kind {
            CstNodeDataKind::Token(tk) => Some(tk.kind),
            _ => None,
        }
    }

    pub fn error(&self) -> Option<&'a ParseErrorKind> {
        match &self.data.kind {
            CstNodeDataKind::Error(err) => Some(err),
            _ => None,
        }
    }

    pub fn tree(&self) -> Option<SqliteTreeKind> {
        match &self.data.kind {
            CstNodeDataKind::Tree(tree) => Some(*tree),
            _ => None,
        }
    }

    /// Panics if self is root
    pub fn parent(self) -> CstNode<'a> {
        let NodeId { branch_id, id } = self.fat_id.into();

        match self.cst.branches[branch_id].parent(id) {
            Some(parent) => self.cst.node(NodeId::new(branch_id, parent)),
            None => self.cst.root(),
        }
    }

    pub fn children(
        self,
    ) -> Either<
        impl DoubleEndedIterator<Item = CstNode<'a>>,
        impl DoubleEndedIterator<Item = CstNode<'a>>,
    > {
        let NodeId { branch_id, id } = self.fat_id.into();

        if self.fat_id.is_root() {
            Either::Left(
                self.cst
                    .branches
                    .iter()
                    .enumerate()
                    .skip(1)
                    .map(|(branch_id, _)| self.cst.node(NodeId::new(branch_id, 0))),
            )
        } else {
            Either::Right(
                self.cst.branches[branch_id]
                    .children_slice(id)
                    .iter()
                    .copied()
                    .map(move |child_id| self.cst.node(NodeId::new(branch_id, child_id))),
            )
        }
    }

    pub fn non_trivial_children(&self) -> impl DoubleEndedIterator<Item = CstNode<'a>> + '_ {
        self.children()
            .filter(|it| !it.token().is_some_and(|it| it.is_trivia()))
    }

    pub fn valid_children(&self) -> impl DoubleEndedIterator<Item = CstNode<'a>> + '_ {
        self.non_trivial_children()
            .filter(|it| it.error().is_none())
    }

    pub fn find_children(&self, key: impl ChildNodeKey) -> impl Iterator<Item = CstNode<'a>> {
        key.find_children(*self)
    }

    // Iterate over earlier siblings (In insertion order)
    /// Panics if node is root
    pub fn left_siblings(
        &self,
    ) -> Either<
        impl DoubleEndedIterator<Item = CstNode<'a>>,
        impl DoubleEndedIterator<Item = CstNode<'a>> + '_,
    > {
        let parent: NodeId = self.parent().fat_id;

        if parent.is_root() {
            Either::Left(
                (1..self.fat_id.branch_id)
                    .map(|branch_id| self.cst.node(NodeId::new(branch_id, 0))),
            )
        } else {
            let parent_children = &self.cst.branches[parent.branch_id].children_slice(parent.id);

            let idx = parent_children
                .binary_search(&self.fat_id.id)
                .expect("Expected parent to have child");

            Either::Right(
                // NOTE: This indexing will not panic because in Rust `list.len()..` returns empty slice
                parent_children[..idx]
                    .iter()
                    .map(move |&id| self.cst.node(NodeId::new(parent.branch_id, id))),
            )
        }
    }

    // Iterate over later siblings (In insertion order)
    /// Panics if node is root
    pub fn right_siblings(
        &self,
    ) -> Either<
        impl DoubleEndedIterator<Item = CstNode<'a>>,
        impl DoubleEndedIterator<Item = CstNode<'a>> + '_,
    > {
        let parent: NodeId = self.parent().fat_id;

        if parent.is_root() {
            Either::Left(
                (self.fat_id.branch_id + 1..self.cst.num_branches())
                    .map(|branch_id| self.cst.node(NodeId::new(branch_id, 0))),
            )
        } else {
            let parent_children = &self.cst.branches[parent.branch_id].children_slice(parent.id);

            let idx = parent_children
                .binary_search(&self.fat_id.id)
                .expect("Expected parent to have child");

            Either::Right(
                // NOTE: This indexing will not panic because in Rust `list.len()..` returns empty slice
                parent_children[idx + 1..]
                    .iter()
                    .map(move |&id| self.cst.node(NodeId::new(parent.branch_id, id))),
            )
        }
    }

    pub fn has_errors(&self) -> bool {
        self.me_and_descendants()
            .find_map(|it| it.error())
            .is_some()
    }

    pub fn byte_len(&self) -> usize {
        let end = self.end_pos().unwrap_or(0);
        let start = self.start_pos().unwrap_or(0);
        assert!(end >= start);

        return (end - start) as usize;
    }

    // Iterate over siblings (In insertion order), skipping ourselves
    pub fn siblings(&self) -> impl DoubleEndedIterator<Item = CstNode<'a>> + '_ {
        self.left_siblings().chain(self.right_siblings())
    }

    pub fn ancestors(&self) -> AncestorIter<'a> {
        AncestorIter { curr: *self }
    }

    pub fn me_and_descendants(
        self,
    ) -> Either<
        impl DoubleEndedIterator<Item = CstNode<'a>>,
        impl DoubleEndedIterator<Item = CstNode<'a>>,
    > {
        if self.is_root() {
            Either::Left(
                std::iter::once(self.cst.root()).chain(self.children().flat_map(move |it| {
                    let branch_id = it.fat_id.branch_id;
                    (0..it.cst.branches[branch_id].len())
                        .map(move |id| self.cst.node(NodeId::new(branch_id, id)))
                })),
            )
        } else {
            let NodeId { branch_id, id } = self.fat_id;

            let end = if self.parent().is_root() {
                self.cst.branches[branch_id].len()
            } else {
                match self.right_siblings().next() {
                    Some(sibling) => sibling.fat_id.id,
                    None => self.cst.branches[branch_id].len(),
                }
            };

            let start = id;
            // NOTE: if start is greater than end, we will get an empty iterator
            Either::Right((start..end).map(move |id| self.cst.node(NodeId::new(branch_id, id))))
        }
    }

    fn is_root(&self) -> bool {
        self.fat_id.is_root()
    }

    fn is_trivia(&self) -> bool {
        self.token().is_some_and(|it| it.is_trivia())
    }

    pub fn start_pos(&self) -> Option<usize> {
        self.start_pos_configurable(true)
    }

    pub fn start_pos_skip_trivia(&self) -> Option<usize> {
        self.start_pos_configurable(false)
    }

    pub fn end_pos(&self) -> Option<usize> {
        self.end_pos_configurable(true)
    }

    pub fn end_pos_skip_trivia(&self) -> Option<usize> {
        self.end_pos_configurable(false)
    }

    /// Use `allow_trivial` to include trivial tokens such as whitespace in end_pos calculation.
    ///
    /// This may not be desired in cases such as when we need to show error squiggly lines
    /// in the editor - having the squiggly line extend past text and into whitespace is unsightly
    // NOTE: We can also implement this by recursively calling start_pos on the first child
    // until we find a token node - but this gotta be faster
    fn start_pos_configurable(&self, allow_trivial: bool) -> Option<usize> {
        let offset = self.cst.branch_positions[self.fat_id.branch_id];
        let start_pos = |node: CstNode| match &node.data.kind {
            CstNodeDataKind::Token(tk) if allow_trivial || (!allow_trivial && !tk.is_trivia()) => {
                Some(offset + node.data.relative_pos)
            }
            _ => None,
        };

        // Don't skip ourselves. Because if we are a token node, then we are what we are looking for
        self.me_and_descendants().find_map(start_pos)
    }

    /// Use `allow_trivial` to include trivial tokens such as whitespace in end_pos calculation.
    ///
    /// This may not be desired in cases such as when we need to show error squiggly lines
    /// in the editor - having the squiggly line extend past text and into whitespace is unsightly
    fn end_pos_configurable(&self, allow_trivial: bool) -> Option<usize> {
        let offset = self.cst.branch_positions[self.fat_id.branch_id];
        let end_pos = |node: CstNode| match &node.data.kind {
            CstNodeDataKind::Token(tk) if allow_trivial || (!allow_trivial && !tk.is_trivia()) => {
                Some(offset + node.data.relative_pos + tk.text.len())
            }
            _ => None,
        };

        // Don't skip ourselves. Because if we are a token node, then we are what we are looking for
        self.me_and_descendants().rev().find_map(end_pos)
    }

    pub fn print_subtree(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for descendant in self.me_and_descendants().filter(|it| !it.is_trivia()) {
            descendant.print(f, self.fat_id)?;
        }

        Ok(())
    }

    pub fn as_str(&self) -> &'static str {
        match &self.data.kind {
            CstNodeDataKind::Tree(tree_kind) => tree_kind.as_str(),
            CstNodeDataKind::Token(tk) => tk.kind.as_str(),
            CstNodeDataKind::Error(_) => "Error",
        }
    }

    pub fn to_string(&self) -> String {
        self.me_and_descendants()
            .filter_map(|it| match &it.data.kind {
                CstNodeDataKind::Token(token) => Some(token.text.as_str()),
                _ => None,
            })
            .collect()
    }

    pub fn to_string_with_capacity(&self, capacity: usize) -> String {
        let mut s = String::with_capacity(capacity);

        self.me_and_descendants()
            .filter_map(|it| match &it.data.kind {
                CstNodeDataKind::Token(token) => Some(token.text.as_str()),
                _ => None,
            })
            .for_each(|it| s.push_str(it));

        s
    }

    pub fn print(&self, f: &mut std::fmt::Formatter<'_>, custom_root: NodeId) -> std::fmt::Result {
        let mut s = format!("{}", self.data);

        if self.fat_id == custom_root {
            return writeln!(f, "{s}");
        }

        let parent_id = self.parent().fat_id;
        let last_non_triv_child_id =
            |node: CstNode| node.non_trivial_children().last().map(|it| it.fat_id);

        for ancestor in self.ancestors() {
            if ancestor.fat_id == parent_id {
                if last_non_triv_child_id(ancestor) == Some(self.fat_id) {
                    s = format!("└───{}", s);
                } else {
                    s = format!("├───{}", s);
                }
            } else {
                match last_non_triv_child_id(ancestor) {
                    Some(fat_id)
                        if fat_id.branch_id > self.fat_id.branch_id
                            || fat_id.id > self.fat_id.id =>
                    {
                        s = format!("├   {s}")
                    }
                    _ => s = format!("    {s}"),
                }
            }

            if ancestor.fat_id == custom_root {
                break;
            }
        }

        return writeln!(f, "{s}");
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
        match &self.kind {
            CstNodeDataKind::Tree(tree_kind) => std::write!(f, "{:?}", tree_kind),
            CstNodeDataKind::Token(SqliteToken {
                kind: SqliteTokenKind::IDEN,
                text,
                ..
            }) => std::write!(f, "IDEN({})", text),
            CstNodeDataKind::Token(SqliteToken { kind, .. }) => std::write!(f, "{:?}", kind),
            CstNodeDataKind::Error(_) => f.write_str("Error"),
        }
    }
}

impl std::fmt::Debug for SqliteUntypedCst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, branch) in self.branches.iter().enumerate() {
            writeln!(
                f,
                "{} -> [{}]",
                self.branch_positions[idx],
                (0..branch.len()).map(|idx| branch.data(idx)).join(", ")
            )?;
        }
        writeln!(f, "byte_len: {}", self.byte_len)?;
        writeln!(f, "positions: {:?}", self.branch_positions)?;
        Ok(())
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

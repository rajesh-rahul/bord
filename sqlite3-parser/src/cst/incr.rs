//! Initally based on: https://github.com/mamcx/tree-flat

use itertools::{Either, Itertools};
use text_size::TextSize;
use tinyvec::TinyVec;

use super::*;
use crate::{parser::ParseErrorKind, SqliteTokenKind, SqliteTreeKind, T};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BranchId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NodeId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FatId {
    branch_id: BranchId,
    id: NodeId,
}

#[derive(Debug, Clone, Copy)]
pub struct IncrCstNode<'a> {
    fat_id: FatId,
    data: &'a CstNodeData,
    cst: &'a IncrSqlCst,
}

#[derive(Debug)]
pub struct IncrCstMut<'a> {
    fat_id: FatId,
    parent: Option<NodeId>,
    cst: &'a mut IncrSqlCst,
}

impl FatId {
    pub const fn new(branch_id: BranchId, id: NodeId) -> FatId {
        FatId { branch_id, id }
    }

    pub const fn is_root(&self) -> bool {
        matches!(
            self,
            FatId {
                branch_id: BranchId(0),
                id: NodeId(0)
            }
        )
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct IncrSqlCst {
    // TODO: Switch to linkedlist slotmap
    branches: Vec<CstBranch>,
    branch_positions: Vec<TextSize>,
    byte_len: TextSize,
    abs_pos: TextSize,
}

mod private {
    use super::*;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum CstBranch {
        Token(CstNodeData),
        Tree {
            data: Vec<CstNodeData>,
            parents: Vec<Option<NodeId>>,
            children: Vec<TinyVec<[NodeId; 8]>>,
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
                    relative_pos: TextSize::new(0),
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

        pub fn push_child(&mut self, parent: NodeId, node_data: CstNodeData) -> NodeId {
            assert!(!self.is_root_branch());

            match self {
                CstBranch::Token(_) => panic!("Cannot add child to token node"),
                CstBranch::Tree {
                    data,
                    parents,
                    children,
                } => {
                    let child_idx = NodeId::new(data.len());

                    data.push(node_data);
                    parents.push(Some(parent));
                    children.push(TinyVec::new());
                    children[parent].push(child_idx);

                    child_idx
                }
            }
        }

        #[inline(always)]
        pub fn parent(&self, id: NodeId) -> Option<NodeId> {
            if self.is_root_branch() {
                panic!("Root node do not have parent")
            } else {
                match self {
                    CstBranch::Token(_) => None,
                    CstBranch::Tree { parents, .. } => parents[id],
                }
            }
        }

        #[inline(always)]
        pub fn data(&self, id: NodeId) -> &CstNodeData {
            match self {
                CstBranch::Token(data) => {
                    assert!(id == NodeId(0));
                    &data
                }
                CstBranch::Tree { data, .. } => &data[id],
            }
        }

        pub fn children_slice(&self, id: NodeId) -> &[NodeId] {
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

impl IncrSqlCst {
    fn push_cst_branch(&mut self, data: CstNodeData, capacity: usize) -> FatId {
        let branch_id = self.branches.len();
        self.branches.push(CstBranch::with_capacity(data, capacity));

        self.branch_positions.push(self.byte_len);

        FatId::new(BranchId::new(branch_id), NodeId::new(0))
    }

    fn branch_root<'a>(&'a self, branch_id: BranchId) -> IncrCstNode<'a> {
        self.node(branch_id, NodeId(0))
    }

    fn push_child(&mut self, branch_id: BranchId, parent: NodeId, data: CstNodeData) -> FatId {
        let child_id = self.branches[branch_id].push_child(parent, data);

        FatId::new(branch_id, child_id)
    }

    pub fn num_branches(&self) -> usize {
        self.branches.len() - 1
    }

    fn has_branch(&self, branch_id: usize) -> bool {
        branch_id < self.num_branches() && branch_id > 0
    }

    #[inline(always)]
    fn node_mut<'a>(&'a mut self, fat_id: FatId) -> IncrCstMut<'a> {
        IncrCstMut {
            fat_id,
            parent: self.branches[fat_id.branch_id].parent(fat_id.id),
            cst: self,
        }
    }

    #[inline(always)]
    pub fn node<'a>(&'a self, branch_id: BranchId, id: NodeId) -> IncrCstNode<'a> {
        IncrCstNode {
            fat_id: FatId::new(branch_id, id),
            data: self.branches[branch_id].data(id),
            cst: self,
        }
    }

    pub fn updated_text_patch(&self, patch: TextPatch<(), ()>) -> TextPatch<TextSize, TextSize> {
        let find_node_with_pos = |pos: TextSize| {
            let p_point = self.branch_positions[1..].partition_point(|it| *it < pos) + 1;
            if self.has_branch(p_point) && self.branch_positions[p_point] == pos {
                return Some(self.branch_root(BranchId::new(p_point)));
            } else if self.has_branch(p_point - 1) {
                return Some(self.branch_root(BranchId::new(p_point - 1)));
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
            .map(|it| it.end_pos())
            .unwrap_or(0.into());

        let affected_node_byte_len = match patch {
            TextPatch {
                start,
                kind: TextPatchKind::Insert,
                ..
            } => find_node_with_pos(start)
                .map(|it| it.byte_len())
                .unwrap_or(0.into()),
            TextPatch {
                kind: TextPatchKind::Replace { end },
                ..
            } => find_node_with_pos(end)
                .map(|it| it.byte_len())
                .unwrap_or(0.into()),
        };

        TextPatch {
            relex_start,
            affected_node_byte_len,
            start: patch.start,
            size: patch.size,
            kind: patch.kind,
        }
    }

    pub fn merge_cst(
        &mut self,
        new_cst: IncrSqlCst,
        patch: TextPatch<TextSize, TextSize>,
    ) -> ModifiedBranchesInfo {
        enum GrowSize {
            Pos(TextSize),
            Neg(TextSize),
        }

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

                    (GrowSize::Pos(size), end_text_pos)
                }
                TextPatchKind::Replace { end } => {
                    // NOTE: size is not the same as `end - start`. `size` is the length of
                    // the new text and `end - start` represents the length of the text its
                    // replacing
                    let new_size = size;
                    let old_size = end - start;

                    let grow_size = if new_size >= old_size {
                        GrowSize::Pos(new_size - old_size)
                    } else {
                        let diff = u32::from(new_size).abs_diff(old_size.into());
                        GrowSize::Neg(diff.into())
                    };

                    let end_text_pos = end + spillover;

                    (grow_size, end_text_pos)
                }
            };

            let is_affected = |idx: usize| {
                if self.branch_positions[idx] < end_text_pos {
                    true
                } else if self.branch_positions[idx] == end_text_pos {
                    let curr_branch = self.branch_root(BranchId::new(idx));

                    curr_branch
                        .error()
                        .is_some_and(|err| err.is_missing_semicolon_err())
                } else {
                    false
                }
            };

            let mut affected_end =
                self.branch_positions[1..].partition_point(|&it| it < end_text_pos) + 1; // TODO: is <= correct?

            while affected_end < self.branches.len()
                && self
                    .branch_root(BranchId::new(affected_end))
                    .error()
                    .is_some_and(|err| err.is_missing_semicolon_err())
            {
                affected_end += 1
            }

            (partition_point, affected_end, grow_size)
        };

        let modified_info = ModifiedBranchesInfo {
            splice_range: start..end,
            num_new_branches: new_cst.num_branches(),
        };

        match grow_size {
            GrowSize::Pos(ts) => self.byte_len += ts,
            GrowSize::Neg(ts) => self.byte_len -= ts,
        }

        self.branch_positions[end..].iter_mut().for_each(|it| {
            // TODO: Use saturating add?
            match grow_size {
                GrowSize::Pos(ts) => *it += ts,
                GrowSize::Neg(ts) => *it -= ts,
            }
            // *it = it.saturating_add_signed(grow_size);
        });

        let start_T = std::time::Instant::now();
        self.branch_positions
            .splice(start..end, new_cst.branch_positions.into_iter().skip(1));

        self.branches
            .splice(start..end, new_cst.branches.into_iter().skip(1));
        eprintln!("splce time: {}", start_T.elapsed().as_micros());

        self.branches.shrink_to_fit();
        self.branches.shrink_to_fit();

        modified_info
    }
}

impl CstTrait for IncrSqlCst {
    type Node<'a> = IncrCstNode<'a>;
    type Mut<'a> = IncrCstMut<'a>;

    fn use_tree_capacity() -> bool {
        true
    }

    fn with_capacity(abs_pos: TextSize, _capacity: usize) -> Self {
        let mut branches = Vec::with_capacity(10);
        let mut branch_positions = Vec::with_capacity(10);
        // First branch is a special branch that belongs to the root
        branches.push(CstBranch::root_branch());
        branch_positions.push(TextSize::new(0));

        IncrSqlCst {
            branches,
            branch_positions,
            abs_pos,
            byte_len: abs_pos,
        }
    }

    #[inline(always)]
    fn root_mut<'a>(&'a mut self) -> IncrCstMut<'a> {
        IncrCstMut {
            fat_id: FatId::new(BranchId(0), NodeId(0)),
            parent: None,
            cst: self,
        }
    }

    fn root<'a>(&'a self) -> IncrCstNode<'a> {
        static ROOT: CstNodeData = CstNodeData {
            kind: CstNodeDataKind::Tree(SqliteTreeKind::File),
            relative_pos: TextSize::new(0),
        };

        IncrCstNode {
            fat_id: FatId::new(BranchId(0), NodeId(0)),
            data: &ROOT,
            cst: self,
        }
    }
}

impl IncrCstMut<'_> {
    fn append(&mut self, kind: CstNodeDataKind, capacity: usize) -> FatId {
        if self.fat_id.is_root() {
            self.cst.push_cst_branch(
                CstNodeData {
                    relative_pos: TextSize::new(0),
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
impl<'a> CstMutTrait<'a> for IncrCstMut<'a> {
    #[inline(always)]
    fn parent_mut(self) -> IncrCstMut<'a> {
        if self.fat_id.is_root() {
            panic!("Root node do not have parent")
        }

        if let Some(parent) = self.parent {
            self.cst.node_mut(FatId::new(self.fat_id.branch_id, parent))
        } else {
            self.cst.root_mut()
        }
    }

    #[inline(always)]
    fn push_tree(mut self, tree: SqliteTreeKind, capacity: usize) -> IncrCstMut<'a> {
        let fat_id = self.append(CstNodeDataKind::Tree(tree), capacity);

        self.cst.node_mut(fat_id)
    }

    #[inline(always)]
    fn push_token(&mut self, token: SqliteToken) {
        let byte_len_to_add = token.text_len();
        let _ = self.append(CstNodeDataKind::Token(token), 1);
        self.cst.byte_len += byte_len_to_add;
    }

    #[inline(always)]
    fn push_error(mut self, error: ParseErrorKind, capacity: usize) -> IncrCstMut<'a> {
        let fat_id = self.append(CstNodeDataKind::Error(error), capacity);

        self.cst.node_mut(fat_id)
    }
}

impl<'a> IncrCstNode<'a> {
    fn offset(&self) -> TextSize {
        self.cst.branch_positions[self.fat_id.branch_id]
    }

    /// Use `allow_trivial` to include trivial tokens such as whitespace in end_pos calculation.
    ///
    /// This may not be desired in cases such as when we need to show error squiggly lines
    /// in the editor - having the squiggly line extend past text and into whitespace is unsightly
    // NOTE: We can also implement this by recursively calling start_pos on the first child
    // until we find a token node - but this gotta be faster
    fn start_pos_configurable(&self, allow_trivial: bool) -> TextSize {
        // let is_root = self.is_root();
        // let offset = self.cst.branch_positions[self.fat_id.branch_id];
        // // let start_pos = |node: CstNode| match &node.data.kind {
        // //     CstNodeDataKind::Token(tk) if allow_trivial || (!allow_trivial && !tk.is_trivia()) => {
        // //         Some(offset + node.data.relative_pos)
        // //     }
        // //     _ => None,
        // // };

        // Don't skip ourselves. Because if we are a token node, then we are what we are looking for
        self.me_and_descendants()
            .skip_while(|it| !allow_trivial && it.is_trivia())
            .next()
            .map(|it| it.data.relative_pos + it.offset())
            .unwrap_or(self.offset() + self.data.relative_pos)

        // if allow_trivial {
        //     let offset = self.cst.branch_positions[self.fat_id.branch_id];

        //     offset + self.data.relative_pos
        // } else {
        //     self.me_and_descendants()
        //         .skip_while(|it| it.is_trivia())

        // }
    }

    /// Use `allow_trivial` to include trivial tokens such as whitespace in end_pos calculation.
    ///
    /// This may not be desired in cases such as when we need to show error squiggly lines
    /// in the editor - having the squiggly line extend past text and into whitespace is unsightly
    fn end_pos_configurable(&self, allow_trivial: bool) -> TextSize {
        self.me_and_descendants()
            .rev()
            .skip_while(|it| !allow_trivial && it.is_trivia())
            .next()
            .map(|it| {
                if let Some(tk) = it.token() {
                    it.offset() + it.data.relative_pos + tk.text_len()
                } else {
                    it.offset() + it.data.relative_pos
                }
            })
            .unwrap_or(self.offset() + self.data.relative_pos)
    }
}
impl<'a> CstNodeTrait<'a> for IncrCstNode<'a> {
    type Id = FatId;

    #[inline(always)]
    fn data(&self) -> &'a CstNodeData {
        &self.data
    }

    #[inline(always)]
    fn id(&self) -> Self::Id {
        self.fat_id
    }

    fn parent(self) -> IncrCstNode<'a> {
        let FatId { branch_id, id } = self.fat_id.into();

        match self.cst.branches[branch_id].parent(id) {
            Some(parent) => self.cst.node(branch_id, parent),
            None => self.cst.root(),
        }
    }

    fn children(self) -> impl DoubleEndedIterator<Item = IncrCstNode<'a>> {
        let FatId { branch_id, id } = self.fat_id;

        if self.fat_id.is_root() {
            Either::Left(
                self.cst
                    .branches
                    .iter()
                    .enumerate()
                    .skip(1)
                    .map(|(branch_id, _)| self.cst.branch_root(BranchId::new(branch_id))),
            )
        } else {
            Either::Right(
                self.cst.branches[branch_id]
                    .children_slice(id)
                    .iter()
                    .copied()
                    .map(move |child_id| self.cst.node(branch_id, child_id)),
            )
        }
    }

    // Iterate over earlier siblings (In insertion order)
    /// Panics if node is root
    fn left_siblings(&self) -> impl DoubleEndedIterator<Item = IncrCstNode<'a>> {
        let parent: FatId = self.parent().fat_id;

        if parent.is_root() {
            Either::Left(
                (1..usize::from(self.fat_id.branch_id))
                    .map(|branch_id| self.cst.branch_root(BranchId::new(branch_id))),
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
                    .map(move |&id| self.cst.node(parent.branch_id, id)),
            )
        }
    }

    // Iterate over later siblings (In insertion order)
    /// Panics if node is root
    fn right_siblings(&self) -> impl DoubleEndedIterator<Item = IncrCstNode<'a>> {
        let parent: FatId = self.parent().fat_id;

        if parent.is_root() {
            Either::Left(
                (usize::from(self.fat_id.branch_id) + 1..self.cst.num_branches())
                    .map(|branch_id| self.cst.branch_root(BranchId::new(branch_id))),
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
                    .map(move |&id| self.cst.node(parent.branch_id, id)),
            )
        }
    }

    fn me_and_descendants(self) -> impl DoubleEndedIterator<Item = IncrCstNode<'a>> {
        if self.is_root() {
            Either::Left(
                std::iter::once(self.cst.root()).chain(self.children().flat_map(move |it| {
                    let branch_id = it.fat_id.branch_id;
                    (0..it.cst.branches[branch_id].len())
                        .map(move |id| self.cst.node(branch_id, NodeId::new(id)))
                })),
            )
        } else {
            let FatId { branch_id, id } = self.fat_id;

            let end = if self.parent().is_root() {
                self.cst.branches[branch_id].len()
            } else {
                // An optimization if we have right sibling
                if let Some(sibling) = self.right_siblings().next() {
                    sibling.fat_id.id.into()
                } else {
                    let mut last_descendant = self;

                    while let Some(descendant) = last_descendant.children().last() {
                        last_descendant = descendant;
                    }

                    usize::from(last_descendant.fat_id.id) + 1
                }
            };

            let start = id.into();
            // NOTE: if start is greater than end, we will get an empty iterator
            Either::Right((start..end).map(move |id| self.cst.node(branch_id, NodeId::new(id))))
        }
    }

    fn is_root(&self) -> bool {
        self.fat_id.is_root()
    }

    fn start_pos(&self) -> TextSize {
        self.start_pos_configurable(true)
    }

    fn start_pos_skip_trivia(&self) -> TextSize {
        self.start_pos_configurable(false)
    }

    fn end_pos(&self) -> TextSize {
        self.end_pos_configurable(true)
    }

    fn end_pos_skip_trivia(&self) -> TextSize {
        self.end_pos_configurable(false)
    }
}

impl std::fmt::Display for IncrSqlCst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root().print_subtree(f)
    }
}

impl std::fmt::Display for IncrCstNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_subtree(f)
    }
}

impl std::fmt::Debug for IncrSqlCst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (branch_id, branch) in self.branches.iter().enumerate() {
            writeln!(
                f,
                "{} -> [{}]",
                u32::from(self.branch_positions[branch_id]),
                (0..branch.len())
                    .map(|id| branch.data(NodeId::new(id)))
                    .join(", ")
            )?;
        }
        writeln!(f, "byte_len: {}", u32::from(self.byte_len))?;
        writeln!(f, "positions: {:?}", self.branch_positions)?;
        Ok(())
    }
}

macro_rules! derive_index {
    ($idx:ty => $($res:ty ,)*) => {
        $(
            impl std::ops::Index<$idx> for Vec<$res> {
                type Output = $res;

                #[inline(always)]
                fn index(&self, index: $idx) -> &$res {
                    &self[index.0 as usize]
                }
            }

            impl std::ops::IndexMut<$idx> for Vec<$res> {
                #[inline(always)]
                fn index_mut(&mut self, index: $idx) -> &mut $res {
                    &mut self[index.0 as usize]
                }
            }
        )*
    };
}

// derive_index!{ NodeId, CstNodeDataKind }
derive_index! { NodeId => CstNodeData, TinyVec<[NodeId; 8]>, Option<NodeId>,}
derive_index! { BranchId => CstBranch, TextSize, }

impl From<NodeId> for usize {
    #[inline(always)]
    fn from(value: NodeId) -> Self {
        value.0 as usize
    }
}

impl From<BranchId> for usize {
    #[inline(always)]
    fn from(value: BranchId) -> Self {
        value.0 as usize
    }
}

impl BranchId {
    #[inline(always)]
    pub fn new(idx: usize) -> Self {
        BranchId(idx as u32)
    }
}

impl NodeId {
    #[inline(always)]
    pub fn new(idx: usize) -> Self {
        NodeId(idx as u32)
    }
}

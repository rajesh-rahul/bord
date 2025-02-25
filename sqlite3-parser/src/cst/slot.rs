//! Initally based on: https://github.com/mamcx/tree-flat

// use std::collections::FxHashMap;

use itertools::{Either, Itertools};
use slotmap::{DefaultKey, SecondaryMap};
// use slab::Slab;
// use slotmap::{SecondaryMap, SlotMap};
use tinyvec::TinyVec;

use super::{slot_list::SlotList, *};
use crate::{parser::ParseErrorKind, SqliteTreeKind, T};

#[derive(Debug, Clone, Copy)]
pub struct SlotCstNode<'a> {
    pub fat_id: NodeId,
    pub data: &'a CstNodeData,
    pub cst: &'a SlotIncrSqlCst,
}

#[derive(Debug)]
pub struct SlotCstMut<'a> {
    pub parent: Option<usize>,
    pub cst: &'a mut SlotIncrSqlCst,
    pub curr: NodeId,
    pub cursor: DefaultKey,
    pub abs_start: TextSize,
    pub abs_end: TextSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {
    pub branch_key: DefaultKey,
    pub id: usize,
}

impl NodeId {
    pub const fn new(branch_key: DefaultKey, id: usize) -> NodeId {
        NodeId { branch_key, id }
    }
}

#[derive(Clone)]
pub struct SlotIncrSqlCst {
    // TODO: Switch to linkedlist slotmap
    branches: SlotList<LinkedCstBranch>,
    branch_positions: SecondaryMap<DefaultKey, TextSize>,
    // pub(crate) byte_len: usize,
    abs_pos: TextSize,
}

impl std::cmp::PartialEq for SlotIncrSqlCst {
    fn eq(&self, other: &Self) -> bool {
        self.branches.eq(&other.branches)
            && self
                .branches
                .iter()
                .map(|key| self.branch_positions[key])
                .eq(other.branches.iter().map(|key| other.branch_positions[key]))
        // self.abs_pos == other.abs_pos
        //     && self.branches[self.head_branch] == other.branches[other.head_branch]
        //     && self.branches[self.tail_branch] == other.branches[other.tail_branch]
        //     && self
        //         .branch_iter()
        //         .map(|key| self.branch_positions[key])
        //         .eq(other.branch_iter().map(|key| other.branch_positions[key]))
        //     && self
        //         .branch_iter()
        //         .rev()
        //         .map(|key| self.branch_positions[key])
        //         .eq(other
        //             .branch_iter()
        //             .rev()
        //             .map(|key| other.branch_positions[key]))
        //     && self
        //         .branch_iter()
        //         .map(|key| &self.branches[key])
        //         .eq(other.branch_iter().map(|key| &other.branches[key]))
        //     && self
        //         .branch_iter()
        //         .rev()
        //         .map(|key| &self.branches[key])
        //         .eq(other.branch_iter().rev().map(|key| &other.branches[key]))
    }
}

mod private {

    use super::*;

    // #[derive(Clone, Debug, Eq, PartialEq)]
    // pub enum CstBranch {
    //     Token(CstNodeData),
    //     Tree {
    //         data: Vec<CstNodeData>,
    //         parents: Vec<Option<usize>>,
    //         children: Vec<TinyVec<[usize; 4]>>,
    //     },
    // }

    #[derive(Clone, Debug)]
    pub(super) struct LinkedCstBranch {
        data: TinyVec<[CstNodeData; 1]>,
        parents: TinyVec<[Option<usize>; 1]>,
        children: Vec<TinyVec<[usize; 4]>>,
    }

    impl std::cmp::PartialEq for LinkedCstBranch {
        fn eq(&self, other: &Self) -> bool {
            self.data == other.data
                && self.parents == other.parents
                && self.children == other.children
        }
    }

    impl LinkedCstBranch {
        pub fn with_capacity(node_data: CstNodeData, capacity: usize) -> LinkedCstBranch {
            let mut data = TinyVec::with_capacity(capacity);
            let mut parents = TinyVec::with_capacity(capacity);
            let mut children = Vec::with_capacity(capacity);

            data.push(node_data);
            parents.push(None);
            children.push(TinyVec::new());

            LinkedCstBranch {
                data,
                parents,
                children,
            }
        }

        pub fn root_branch() -> LinkedCstBranch {
            LinkedCstBranch {
                data: tinyvec::tiny_vec![CstNodeData {
                    relative_pos: TextSize::new(0),
                    kind: CstNodeDataKind::Tree(SqliteTreeKind::File),
                }],
                parents: Default::default(),
                children: Default::default(),
            }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_root_branch(&self) -> bool {
            self.parents.is_empty() && self.children.is_empty()
        }

        pub fn is_token_branch(&self) -> bool {
            matches!(&self.data[0].kind, CstNodeDataKind::Token(_))
        }

        pub fn push_child(&mut self, parent: usize, node_data: CstNodeData) -> usize {
            assert!(!self.is_root_branch());
            assert!(!self.is_token_branch());

            let child_idx = self.data.len();

            self.data.push(node_data);
            self.parents.push(Some(parent));
            self.children.push(TinyVec::new());
            self.children[parent].push(child_idx);

            child_idx
        }

        pub fn parent(&self, id: usize) -> Option<usize> {
            if self.is_root_branch() {
                None
            } else {
                self.parents.get(id).and_then(|it| *it)
                // match self {
                //     CstBranch::Token(_) => None,
                //     CstBranch::Tree { parents, .. } => parents.get(id).and_then(|it| *it),
                // }
            }
        }

        pub fn data(&self, id: usize) -> &CstNodeData {
            &self.data[id]
        }

        pub fn children_slice(&self, id: usize) -> &[usize] {
            if self.is_root_branch() {
                panic!("Root branch do not directly store children")
            } else {
                self.children[id].as_slice()
            }
        }
    }
}
pub use private::*;

impl SlotIncrSqlCst {
    pub fn num_branches(&self) -> usize {
        self.branches.len() - 1
    }

    fn has_branch(&self, branch_id: usize) -> bool {
        branch_id < self.num_branches() && branch_id > 0
    }

    // fn next_branch_key(&self, branch_key: DefaultKey) -> Option<DefaultKey> {
    //     self.branches.get(branch_key).and_then(|it| it.next_branch)
    // }

    // fn prev_branch_key(&self, branch_key: DefaultKey) -> Option<DefaultKey> {
    //     self.branches.get(branch_key).and_then(|it| it.prev_branch)
    // }

    pub fn node<'a>(&'a self, NodeId { branch_key, id }: NodeId) -> SlotCstNode<'a> {
        let data = self.branches[branch_key].data(id);

        SlotCstNode {
            fat_id: NodeId::new(branch_key, id),
            data,
            cst: self,
        }
    }

    pub fn updated_text_patch(&self, patch: TextPatch<(), ()>) -> TextPatch<TextSize, TextSize> {
        let find_node_with_pos = |pos: TextSize| {
            self.branches
                .iter()
                .skip(1)
                .take_while(|it| self.branch_positions[*it] < pos)
                .last()
                .map(|it| self.node(NodeId::new(it, 0)))
            // let p_point = self.branch_positions[1..].partition_point(|it| *it < pos) + 1;
            // if self.has_branch(p_point) && self.branch_positions[p_point] == pos {
            //     return Some(self.node(NodeId::new(p_point, 0)));
            // } else if self.has_branch(p_point - 1) {
            //     return Some(self.node(NodeId::new(p_point - 1, 0)));
            // } else {
            //     return None;
            // }
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

    pub fn insert_events(
        &mut self,
        patch: TextPatch<TextSize, TextSize>,
        events: Vec<Event>,
        tokens: Vec<SqliteToken>,
    ) {
        let TextPatch {
            relex_start,
            start,
            size,
            kind,
            ..
        } = patch;

        enum GrowSize {
            Pos(TextSize),
            Neg(TextSize),
        }

        let st = std::time::Instant::now();
        let before_start_node = self
            .branches
            .iter()
            .skip(1)
            .take_while(|it| self.branch_positions[*it] < relex_start)
            .last()
            .map(|key| self.node(NodeId::new(key, 0)))
            .unwrap_or(self.root());

        let abs_start = if before_start_node.is_root() {
            0.into()
        } else {
            before_start_node.end_pos()
        };
        eprintln!("before_start_node: {}", st.elapsed().as_micros());

        let before_start_key = before_start_node.fat_id.branch_key;
        let cst_mut = SlotCstMut {
            abs_start,
            parent: None,
            cursor: before_start_key,
            curr: self.root().fat_id,
            cst: self,
            abs_end: abs_start,
        };

        let st = std::time::Instant::now();
        let cst_mut = populate_cst(
            cst_mut,
            Self::use_tree_capacity(),
            tokens.into_iter(),
            events,
        );
        eprintln!("population: {}", st.elapsed().as_micros());

        // TODO: Should we be incorporating affected_node_byte_len here? (like we do in incremental parse)
        let expected_len = start + size - relex_start;
        let actual_len = cst_mut.width();
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

        let end_branch_key = cst_mut.cursor;
        std::mem::drop(cst_mut);

        // for key in self
        //     .branches
        //     .iter_custom(Some(before_start_key), Some(end_branch_key))
        //     .skip(1)
        // {
        //     println!("added: {}", self.node(NodeId::new(key, 0)));
        // }
        let st = std::time::Instant::now();
        for key in self
            .branches
            .iter_custom(Some(end_branch_key), Some(self.branches.tail()))
            .skip(1)
        {
            let pos = self.branch_positions.get_mut(key).unwrap();
            match grow_size {
                GrowSize::Pos(ts) => *pos += ts,
                GrowSize::Neg(ts) => *pos -= ts,
            }
        }
        eprintln!("update positions: {}", st.elapsed().as_micros());

        let end_text_pos = self.node(NodeId::new(end_branch_key, 0)).end_pos();

        let is_affected = |branch_key: DefaultKey| {
            if self.branch_positions[branch_key] < end_text_pos {
                true
            } else if self.branch_positions[branch_key] == end_text_pos {
                let curr_branch = self.node(NodeId::new(branch_key, 0));

                curr_branch
                    .error()
                    .is_some_and(|err| err.is_missing_semicolon_err())
            } else {
                false
            }

            // || (self.branches[branch_key].len() == 1
            //     && self.node(NodeId::new(branch_key, 0)).token().is_none())
        };
        let st = std::time::Instant::now();
        let branches_to_remove = self
            .branches
            .iter_custom(Some(end_branch_key), Some(self.branches.tail()))
            .skip(1)
            .take_while(|branch_key| is_affected(*branch_key))
            .collect_vec();

        for branch_key in branches_to_remove {
            // println!("REmoved: {}", self.node(NodeId::new(branch_key, 0)));
            self.branches.remove(branch_key);
            // self.branch_positions.remove(branch_key);
            // NOTE: No need to remove from branch positions
        }
        eprintln!("remove_branches: {}", st.elapsed().as_micros());
    }
}

impl CstTrait for SlotIncrSqlCst {
    type Node<'a> = SlotCstNode<'a>;
    type Mut<'a> = SlotCstMut<'a>;

    fn use_tree_capacity() -> bool {
        true
    }

    fn root_mut<'a>(&'a mut self) -> Self::Mut<'a> {
        SlotCstMut {
            parent: None,
            curr: NodeId::new(self.branches.head(), 0),
            cursor: self.branches.head(),
            abs_start: self.abs_pos,
            cst: self,
            abs_end: TextSize::new(0),
        }
    }

    fn with_capacity(abs_pos: TextSize, capacity: usize) -> Self {
        let branches = SlotList::with_capacity(LinkedCstBranch::root_branch(), capacity);
        let mut branch_positions = SecondaryMap::with_capacity(capacity);
        branch_positions.insert(branches.head(), abs_pos);

        SlotIncrSqlCst {
            branches,
            branch_positions,
            // byte_len: 0,
            abs_pos,
        }
    }

    fn root<'a>(&'a self) -> SlotCstNode<'a> {
        static ROOT: CstNodeData = CstNodeData {
            kind: CstNodeDataKind::Tree(SqliteTreeKind::File),
            relative_pos: TextSize::new(0),
        };

        SlotCstNode {
            fat_id: NodeId::new(self.branches.head(), 0),
            data: &ROOT,
            cst: self,
        }
    }
}

impl<'a> SlotCstMut<'a> {
    fn append(&mut self, kind: CstNodeDataKind, capacity: usize) -> NodeId {
        let curr = &mut self.cst.branches[self.curr.branch_key];

        if curr.is_root_branch() {
            let new_curr_key = self.insert_cst_branch(
                CstNodeData {
                    relative_pos: TextSize::new(0),
                    kind,
                },
                capacity,
            );
            self.cursor = new_curr_key;

            NodeId::new(new_curr_key, 0)
        } else {
            let parent = self.curr.id;
            let abs_offset = self.cst.branch_positions.get(self.curr.branch_key).unwrap();
            let relative_pos = self.abs_end - abs_offset;

            let id = curr.push_child(parent, CstNodeData { relative_pos, kind });

            NodeId::new(self.curr.branch_key, id)
        }
    }

    fn width(&self) -> TextSize {
        self.abs_end - self.abs_start
    }

    fn node_mut(mut self, node_id: NodeId) -> SlotCstMut<'a> {
        let parent = self.cst.branches[node_id.branch_key].parent(node_id.id);

        self.parent = parent;
        self.curr = node_id;

        self
    }

    fn insert_cst_branch(&mut self, data: CstNodeData, capacity: usize) -> DefaultKey {
        let new = LinkedCstBranch::with_capacity(data, capacity);
        let new_key = self.cst.branches.insert_after(self.cursor, new);
        self.cursor = new_key;

        self.cst.branch_positions.insert(new_key, self.abs_end);
        new_key
    }
}

impl<'a> CstMutTrait<'a> for SlotCstMut<'a> {
    fn parent_mut(self) -> SlotCstMut<'a> {
        if self.curr.branch_key == self.cst.branches.head() {
            panic!("Root node do not have parent")
        }
        if let Some(parent) = self.parent {
            let parent_id = NodeId::new(self.curr.branch_key, parent);
            self.node_mut(parent_id)
        } else {
            let parent_id = NodeId::new(self.cst.branches.head(), 0);
            self.node_mut(parent_id)
        }
    }

    fn push_tree(mut self, tree: SqliteTreeKind, capacity: usize) -> SlotCstMut<'a> {
        let node_id = self.append(CstNodeDataKind::Tree(tree), capacity);

        self.node_mut(node_id)
    }

    fn push_token(&mut self, token: SqliteToken) {
        let byte_len_to_add = token.text_len();
        let _ = self.append(CstNodeDataKind::Token(token), 1);
        // self.cst.byte_len += byte_len_to_add;
        self.abs_end += byte_len_to_add;
    }

    fn push_error(mut self, error: ParseErrorKind, capacity: usize) -> SlotCstMut<'a> {
        let node_id = self.append(CstNodeDataKind::Error(error), capacity);

        self.node_mut(node_id)
    }
}

impl<'a> SlotCstNode<'a> {
    fn offset(&self) -> TextSize {
        *self
            .cst
            .branch_positions
            .get(self.fat_id.branch_key)
            .unwrap()
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

impl<'a> CstNodeTrait<'a> for SlotCstNode<'a> {
    type Id = NodeId;

    fn data(&self) -> &'a CstNodeData {
        &self.data
    }

    fn id(&self) -> Self::Id {
        self.fat_id
    }

    fn parent(self) -> SlotCstNode<'a> {
        let NodeId { branch_key, id } = self.fat_id.into();

        match self.cst.branches[branch_key].parent(id) {
            Some(parent) => self.cst.node(NodeId::new(branch_key, parent)),
            None => self.cst.root(),
        }
    }

    fn children(self) -> impl DoubleEndedIterator<Item = SlotCstNode<'a>> {
        // let NodeId { branch_key, id } = self.fat_id.into();
        if self.is_root() {
            // Skip root branch
            let mut iter = self.cst.branches.iter();
            iter.next();

            Either::Left(iter.map(move |branch_key| {
                // if self.cst.branches.get(branch_key).is_none() {
                //     // dbg!(&self.cst.branches);
                //     println!("START");
                //     let mut iter = self.cst.branch_iter();
                //     println!("items: {:?}", iter.map(|it| it).rev().collect_vec());
                //     println!("END");
                //     // dbg!(self.cst);
                //     panic!();
                // }
                self.cst.node(NodeId::new(branch_key, 0))
            }))
        } else {
            Either::Right(
                self.cst.branches[self.fat_id.branch_key]
                    .children_slice(self.fat_id.id)
                    .iter()
                    .copied()
                    .map(move |child_id| {
                        self.cst.node(NodeId::new(self.fat_id.branch_key, child_id))
                    }),
            )
        }
    }

    // Iterate over earlier siblings (In insertion order)
    /// Panics if node is root
    fn left_siblings(&self) -> impl DoubleEndedIterator<Item = SlotCstNode<'a>> {
        if self.parent().is_root() {
            let head = match self.cst.root().children().next() {
                Some(SlotCstNode { fat_id, .. }) if fat_id != self.fat_id => {
                    Some(fat_id.branch_key)
                }
                _ => None,
            };

            let tail = match self.cst.branches.prev_key(self.fat_id.branch_key) {
                Some(key) if key != self.cst.branches.head() => Some(key),
                _ => None,
            };

            // dbg!(head, tail);
            Either::Left(
                self.cst
                    .branches
                    .iter_custom(head, tail)
                    .map(|branch_key| self.cst.node(NodeId::new(branch_key, 0))),
            )
        } else {
            let parent_id = self.parent().fat_id;
            let parent_children =
                &self.cst.branches[parent_id.branch_key].children_slice(parent_id.id);

            let idx = parent_children
                .binary_search(&self.fat_id.id)
                .expect("Expected parent to have child");

            Either::Right(
                // NOTE: This indexing will not panic because in Rust `list.len()..` returns empty slice
                parent_children[..idx]
                    .iter()
                    .map(move |&id| self.cst.node(NodeId::new(parent_id.branch_key, id))),
            )
        }
    }

    // Iterate over later siblings (In insertion order)
    /// Panics if node is root
    fn right_siblings(&self) -> impl DoubleEndedIterator<Item = SlotCstNode<'a>> {
        if self.parent().is_root() {
            let head = self.cst.branches.next_key(self.fat_id.branch_key);
            let tail = match self.cst.root().children().next_back() {
                Some(SlotCstNode { fat_id, .. }) if fat_id != self.fat_id => {
                    Some(fat_id.branch_key)
                }
                _ => None,
            };

            Either::Left(
                self.cst
                    .branches
                    .iter_custom(head, tail)
                    .map(|branch_key| self.cst.node(NodeId::new(branch_key, 0))),
            )
        } else {
            let parent_id = self.parent().fat_id;
            let parent_children =
                &self.cst.branches[parent_id.branch_key].children_slice(parent_id.id);

            let idx = parent_children
                .binary_search(&self.fat_id.id)
                .expect("Expected parent to have child");

            Either::Right(
                // NOTE: This indexing will not panic because in Rust `list.len()..` returns empty slice
                parent_children[idx + 1..]
                    .iter()
                    .map(move |&id| self.cst.node(NodeId::new(parent_id.branch_key, id))),
            )
        }
    }

    fn me_and_descendants(self) -> impl DoubleEndedIterator<Item = SlotCstNode<'a>> {
        if self.is_root() {
            Either::Left(
                std::iter::once(self.cst.root()).chain(self.children().flat_map(move |it| {
                    let branch_key = it.fat_id.branch_key;
                    (0..it.cst.branches[branch_key].len())
                        .map(move |id| self.cst.node(NodeId::new(branch_key, id)))
                })),
            )
        } else {
            let NodeId { branch_key, id } = self.fat_id;

            let end = if self.parent().is_root() {
                self.cst.branches[branch_key].len()
            } else {
                // An optimization if we have right sibling
                if let Some(sibling) = self.right_siblings().next() {
                    sibling.fat_id.id
                } else {
                    let mut last_descendant = self;

                    while let Some(descendant) = last_descendant.children().last() {
                        last_descendant = descendant;
                    }

                    last_descendant.fat_id.id + 1
                }
            };

            let start = id;
            // NOTE: if start is greater than end, we will get an empty iterator
            Either::Right((start..end).map(move |id| self.cst.node(NodeId::new(branch_key, id))))
        }
    }

    fn is_root(&self) -> bool {
        self.fat_id.branch_key == self.cst.branches.head() && self.fat_id.id == 0
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

impl std::fmt::Display for SlotIncrSqlCst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root().print_subtree(f)
    }
}

impl std::fmt::Display for SlotCstNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_subtree(f)
    }
}

impl std::fmt::Debug for SlotIncrSqlCst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for branch_key in self.branches.iter() {
            let branch = &self.branches[branch_key];
            writeln!(
                f,
                "{branch_key:?}({}) -> [{}]",
                u32::from(self.branch_positions[branch_key]),
                (0..branch.len()).map(|idx| branch.data(idx)).join(", ")
            )?;
        }
        writeln!(
            f,
            "byte_len: {}",
            u32::from(self.root().end_pos() - self.root().start_pos())
        )?;
        writeln!(
            f,
            "positions: {:?}",
            self.branches
                .iter()
                .map(|key| self.branch_positions[key])
                .collect_vec()
        )?;
        Ok(())
    }
}

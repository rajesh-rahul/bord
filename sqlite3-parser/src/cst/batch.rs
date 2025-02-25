use text_size::TextSize;
use tinyvec::TinyVec;

use crate::{ParseErrorKind, SqliteTreeKind};

use super::{CstMutTrait, CstNodeData, CstNodeDataKind, CstNodeTrait, CstTrait, SqliteToken};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NodeId(u32);

#[derive(Debug)]
pub struct SqlCst {
    pub(crate) data: Vec<CstNodeData>,
    pub(crate) parent: Vec<NodeId>,
    pub(crate) children: Vec<TinyVec<[NodeId; 12]>>,
    pub(crate) abs_pos: TextSize,
    pub(crate) byte_len: TextSize,
}

#[derive(Debug, Clone, Copy)]
pub struct CstNode<'a> {
    pub id: NodeId,
    pub data: &'a CstNodeData,
    pub cst: &'a SqlCst,
}

#[derive(Debug)]
pub struct CstMut<'a> {
    pub id: NodeId,
    pub parent: NodeId,
    pub cst: &'a mut SqlCst,
}

impl CstTrait for SqlCst {
    type Node<'a> = CstNode<'a>;

    type Mut<'a> = CstMut<'a>;

    fn use_tree_capacity() -> bool {
        false
    }

    fn with_capacity(abs_pos: TextSize, capacity: usize) -> Self {
        let mut cst = SqlCst {
            data: Vec::with_capacity(capacity),
            parent: Vec::with_capacity(capacity),
            children: Vec::with_capacity(capacity),
            abs_pos,
            byte_len: TextSize::new(0),
        };

        cst.push(
            CstNodeData {
                relative_pos: TextSize::new(0),
                kind: CstNodeDataKind::Tree(SqliteTreeKind::File),
            },
            NodeId(0),
        );

        cst
    }

    fn root<'a>(&'a self) -> Self::Node<'a> {
        self.node(0.into())
    }

    fn root_mut<'a>(&'a mut self) -> Self::Mut<'a> {
        self.node_mut(0.into())
    }
}

impl SqlCst {
    #[inline(always)]
    pub fn node(&self, id: NodeId) -> CstNode {
        CstNode {
            id,
            data: &self.data[id],
            cst: self,
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline(always)]
    fn node_mut(&mut self, id: NodeId) -> CstMut {
        CstMut {
            id,
            parent: self.parent[id].into(),
            cst: self,
        }
    }

    pub fn push(&mut self, data: CstNodeData, parent_idx: NodeId) -> NodeId {
        self.data.push(data);
        self.parent.push(parent_idx);
        self.children.push(Default::default());

        let new_node_idx: NodeId = (self.data.len() - 1).into();

        if new_node_idx != parent_idx {
            self.children[parent_idx].push(new_node_idx);
        }

        new_node_idx.into()
    }
}

impl<'a> CstNodeTrait<'a> for CstNode<'a> {
    type Id = NodeId;

    fn data(&self) -> &'a CstNodeData {
        self.data
    }

    fn id(&self) -> Self::Id {
        self.id
    }

    fn parent(self) -> CstNode<'a> {
        let parent_id = self.cst.parent[self.id];

        self.cst.node(parent_id.into())
    }

    fn children(self) -> impl DoubleEndedIterator<Item = Self> {
        self.cst.children[self.id]
            .iter()
            .copied()
            .map(|it| self.cst.node(it.into()))
    }

    fn left_siblings(&self) -> impl DoubleEndedIterator<Item = Self> {
        let parent_children = self.parent().children_slice();

        let idx = parent_children
            .binary_search(&self.id)
            .expect("We should be present in our parent's children list");

        parent_children[0..idx]
            .iter()
            .map(|&it| self.cst.node(it.into()))
    }

    fn right_siblings(&self) -> impl DoubleEndedIterator + Iterator<Item = Self> {
        let parent_children = self.parent().children_slice();

        let idx = parent_children
            .binary_search(&self.id)
            .expect("We should be present in our parent's children list");

        // NOTE: idx + 1 is safe, even if we are the last element
        parent_children[idx + 1..]
            .iter()
            .map(|&it| self.cst.node(it.into()))
    }

    fn me_and_descendants(self) -> impl DoubleEndedIterator + Iterator<Item = Self> {
        let start = self.id.into();

        let end: usize = if self.is_root() {
            self.cst.data.len()
        } else {
            // An optimization if we have right sibling
            if let Some(sibling) = self.right_siblings().next() {
                sibling.id.into()
            } else {
                let mut last_descendant = self;

                while let Some(descendant) = last_descendant.children().last() {
                    last_descendant = descendant;
                }

                usize::from(last_descendant.id) + 1
            }
        };

        (start..end).map(move |id| self.cst.node(id.into()))
    }

    fn is_root(&self) -> bool {
        self.id == NodeId(0)
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

impl<'a> CstNode<'a> {
    fn children_slice(&self) -> &'a [NodeId] {
        self.cst.children[self.id].as_slice()
    }

    fn start_pos_configurable(&self, allow_trivial: bool) -> TextSize {
        self.me_and_descendants()
            .filter(|it| allow_trivial || it.token().is_some_and(|it| !it.is_trivia()))
            .next()
            .map(|it| self.cst.abs_pos + it.data.relative_pos)
            .unwrap_or(self.cst.abs_pos + self.data.relative_pos)
    }

    fn end_pos_configurable(&self, allow_trivial: bool) -> TextSize {
        // TODO: Does `last()` simplify to `rev().next()` for DoubleEndedIterator?
        self.me_and_descendants()
            .rev()
            .filter(|it| allow_trivial || it.token().is_some_and(|it| !it.is_trivia()))
            .next()
            .map(|it| {
                if let Some(tk) = it.token() {
                    self.cst.abs_pos + it.data.relative_pos + tk.text_len()
                } else {
                    self.cst.abs_pos + it.data.relative_pos
                }
            })
            .unwrap_or(self.cst.abs_pos + self.data.relative_pos)
    }
}

impl<'a> CstMutTrait<'a> for CstMut<'a> {
    #[inline(always)]
    fn parent_mut(self) -> CstMut<'a> {
        assert!(self.id != NodeId(0), "Cannot ask for Root's parent");

        self.cst.node_mut(self.parent)
    }

    #[inline(always)]
    fn push_tree(mut self, tree: SqliteTreeKind, _capacity: usize) -> CstMut<'a> {
        let new_node_id = self.append(CstNodeDataKind::Tree(tree));

        CstMut {
            id: new_node_id,
            parent: self.id,
            cst: self.cst,
        }
    }

    #[inline(always)]
    fn push_token(&mut self, token: SqliteToken) {
        let byte_len_to_add = token.text_len();
        let _ = self.append(CstNodeDataKind::Token(token));
        self.cst.byte_len += byte_len_to_add;
    }

    #[inline(always)]
    fn push_error(mut self, error: ParseErrorKind, _capacity: usize) -> CstMut<'a> {
        let new_node_id = self.append(CstNodeDataKind::Error(error));

        CstMut {
            id: new_node_id,
            parent: self.id,
            cst: self.cst,
        }
    }
}

impl<'a> CstMut<'a> {
    #[inline(always)]
    pub fn append(&mut self, kind: CstNodeDataKind) -> NodeId {
        self.cst.push(
            CstNodeData {
                relative_pos: self.cst.byte_len,
                kind,
            },
            self.id.into(),
        )
    }
}

impl std::fmt::Display for SqlCst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root().print_subtree(f)
    }
}

impl std::fmt::Display for CstNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_subtree(f)
    }
}

impl From<usize> for NodeId {
    #[inline(always)]
    fn from(value: usize) -> Self {
        NodeId(value as u32)
    }
}

impl From<NodeId> for usize {
    #[inline(always)]
    fn from(value: NodeId) -> Self {
        value.0 as usize
    }
}

impl std::ops::Index<NodeId> for Vec<NodeId> {
    type Output = NodeId;

    #[inline(always)]
    fn index(&self, index: NodeId) -> &NodeId {
        &self[index.0 as usize]
    }
}

impl std::ops::Index<NodeId> for Vec<CstNodeData> {
    type Output = CstNodeData;

    #[inline(always)]
    fn index(&self, index: NodeId) -> &CstNodeData {
        &self[index.0 as usize]
    }
}

impl std::ops::IndexMut<NodeId> for Vec<CstNodeData> {
    #[inline(always)]
    fn index_mut(&mut self, index: NodeId) -> &mut CstNodeData {
        &mut self[index.0 as usize]
    }
}

impl std::ops::Index<NodeId> for Vec<TinyVec<[NodeId; 12]>> {
    type Output = TinyVec<[NodeId; 12]>;

    #[inline(always)]
    fn index(&self, index: NodeId) -> &TinyVec<[NodeId; 12]> {
        &self[index.0 as usize]
    }
}

impl std::ops::IndexMut<NodeId> for Vec<TinyVec<[NodeId; 12]>> {
    #[inline(always)]
    fn index_mut(&mut self, index: NodeId) -> &mut TinyVec<[NodeId; 12]> {
        &mut self[index.0 as usize]
    }
}

use crate::{ParseErrorKind, SqliteTreeKind};

use super::{CstMutTrait, CstNodeData, CstNodeDataKind, CstNodeTrait, CstTrait, SqliteToken};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(usize);

#[derive(Debug)]
pub struct SqlCst {
    pub(crate) data: Vec<CstNodeData>,
    pub(crate) parent: Vec<usize>,
    pub(crate) children: Vec<tinyvec::TinyVec<[usize; 6]>>,
    pub(crate) abs_pos: usize,
    pub(crate) byte_len: usize,
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

    fn with_capacity(abs_pos: usize, capacity: usize) -> Self {
        let mut cst = SqlCst {
            data: Vec::with_capacity(capacity),
            parent: Vec::with_capacity(capacity),
            children: Vec::with_capacity(capacity),
            abs_pos,
            byte_len: 0,
        };

        cst.push(
            CstNodeData {
                relative_pos: 0,
                kind: CstNodeDataKind::Tree(SqliteTreeKind::File),
            },
            0,
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
    fn node(&self, id: NodeId) -> CstNode {
        assert!(id.0 < self.data.len());

        CstNode {
            id,
            data: &self.data[id.0],
            cst: self,
        }
    }

    fn node_mut(&mut self, id: NodeId) -> CstMut {
        assert!(id.0 < self.data.len());

        let parent = self.parent[id.0].into();

        CstMut {
            id,
            parent,
            cst: self,
        }
    }

    pub fn push(&mut self, data: CstNodeData, parent_idx: usize) -> NodeId {
        self.data.push(data);
        self.parent.push(parent_idx);
        self.children.push(Default::default());

        let new_node_idx = self.data.len() - 1;

        if new_node_idx != parent_idx {
            self.children[parent_idx].push(new_node_idx);
        }

        new_node_idx.into()
    }
}

impl<'a> CstNodeTrait<'a> for CstNode<'a> {
    fn data(&self) -> &'a CstNodeData {
        self.data
    }

    fn parent(self) -> CstNode<'a> {
        assert!(self.id.0 != 0);

        let parent_id = self.cst.parent[self.id.0];
        self.cst.node(parent_id.into())
    }

    fn children(self) -> impl DoubleEndedIterator<Item = Self> {
        self.cst.children[self.id.0]
            .iter()
            .copied()
            .map(|it| self.cst.node(it.into()))
    }

    fn left_siblings(&self) -> impl DoubleEndedIterator<Item = Self> {
        let parent_children = self.parent().children_slice();

        let idx = parent_children
            .binary_search(&self.id.0)
            .expect("We should be present in our parent's children list");

        parent_children[0..idx]
            .iter()
            .map(|&it| self.cst.node(it.into()))
    }

    fn right_siblings(&self) -> impl DoubleEndedIterator + Iterator<Item = Self> {
        let parent_children = self.parent().children_slice();

        let idx = parent_children
            .binary_search(&self.id.0)
            .expect("We should be present in our parent's children list");

        // NOTE: idx + 1 is safe, even if we are the last element
        parent_children[idx + 1..]
            .iter()
            .map(|&it| self.cst.node(it.into()))
    }

    fn me_and_descendants(self) -> impl DoubleEndedIterator + Iterator<Item = Self> {
        let start: usize = self.id.into();

        let end = if self.is_root() {
            self.cst.data.len()
        } else {
            // An optimization if we have right sibling
            if let Some(sibling) = self.right_siblings().next() {
                sibling.id.0
            } else {
                let mut last_descendant = self;

                while let Some(descendant) = last_descendant.children().last() {
                    last_descendant = descendant;
                }

                last_descendant.id.0 + 1
            }
        };

        (start..end).map(move |id| self.cst.node(id.into()))
    }

    fn is_root(&self) -> bool {
        self.id.0 == 0
    }

    fn start_pos(&self) -> usize {
        self.start_pos_configurable(true)
    }

    fn start_pos_skip_trivia(&self) -> usize {
        self.start_pos_configurable(false)
    }

    fn end_pos(&self) -> usize {
        self.end_pos_configurable(true)
    }

    fn end_pos_skip_trivia(&self) -> usize {
        self.end_pos_configurable(false)
    }
}

impl<'a> CstNode<'a> {
    fn children_slice(&self) -> &'a [usize] {
        self.cst.children[self.id.0].as_slice()
    }

    fn start_pos_configurable(&self, allow_trivial: bool) -> usize {
        self.me_and_descendants()
            .filter(|it| allow_trivial || it.token().is_some_and(|it| !it.is_trivia()))
            .next()
            .map(|it| self.cst.abs_pos + it.data.relative_pos)
            .unwrap_or(self.cst.abs_pos + self.data.relative_pos)
    }

    fn end_pos_configurable(&self, allow_trivial: bool) -> usize {
        // TODO: is does `last()` simplify to `rev().next()` for DoubleEndedIterator?
        self.me_and_descendants()
            .rev()
            .filter(|it| allow_trivial || it.token().is_some_and(|it| !it.is_trivia()))
            .next()
            .map(|it| {
                if let Some(tk) = it.token() {
                    self.cst.abs_pos + it.data.relative_pos + tk.text.len()
                } else {
                    self.cst.abs_pos + it.data.relative_pos
                }
            })
            .unwrap_or(self.cst.abs_pos + self.data.relative_pos)
    }

    pub fn print_subtree(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for descendant in self.me_and_descendants().filter(|it| !it.is_trivia()) {
            descendant.print(f, self.id)?;
        }

        Ok(())
    }

    pub fn print(&self, f: &mut std::fmt::Formatter<'_>, custom_root: NodeId) -> std::fmt::Result {
        let mut s = format!("{}", self.data);

        if self.id == custom_root {
            return writeln!(f, "{s}({}..{})", self.start_pos(), self.end_pos());
        }

        let parent_id = self.parent().id;
        let last_non_triv_child_id =
            |node: CstNode| node.non_trivial_children().last().map(|it| it.id);
        for ancestor in self.ancestors() {
            if ancestor.id == parent_id {
                let start = self.start_pos();
                let end = self.end_pos();

                if last_non_triv_child_id(ancestor) == Some(self.id) {
                    s = format!("└───{s}({start}..{end})");
                } else {
                    s = format!("├───{s}({start}..{end})");
                }
            } else {
                match last_non_triv_child_id(ancestor) {
                    Some(idx) if idx > self.id => s = format!("├   {s}"),
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

impl<'a> CstMutTrait<'a> for CstMut<'a> {
    fn parent_mut(self) -> CstMut<'a> {
        assert!(self.id.0 != 0);

        self.cst.node_mut(self.parent)
    }

    fn push_tree(mut self, tree: SqliteTreeKind, _capacity: usize) -> CstMut<'a> {
        let new_node_id = self.append(CstNodeDataKind::Tree(tree));

        CstMut {
            id: new_node_id,
            parent: self.id,
            cst: self.cst,
        }
    }

    fn push_token(&mut self, token: SqliteToken) {
        let byte_len_to_add = token.text.len();
        let _ = self.append(CstNodeDataKind::Token(token));
        self.cst.byte_len += byte_len_to_add;
    }

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

// impl std::cmp::PartialEq<usize> for NodeId {
//     fn eq(&self, other: &usize) -> bool {
//         self.0 == *other
//     }
// }

impl From<usize> for NodeId {
    fn from(value: usize) -> Self {
        NodeId(value)
    }
}

impl From<NodeId> for usize {
    fn from(value: NodeId) -> Self {
        value.0
    }
}

use crate::{SqliteTokenKind, SqliteTreeKind};
use smol_str::SmolStr;
use std::fmt::Write;

/// Tree and Token node terminology comes from matklad's [error resilient parser article]
/// (https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html)
pub struct SqliteUntypedAst(Vec<SqliteNode>);

#[derive(Debug)]
pub enum SqliteNode {
    Tree {
        kind: SqliteTreeKind,
        children: Vec<NodeId>,
        parent: Option<NodeId>,
    },
    Token {
        token: SqliteToken,
        parent: NodeId,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SqliteToken {
    pub kind: SqliteTokenKind,
    pub text: SmolStr,
    pub abs_pos: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct NodeId(usize);

impl SqliteUntypedAst {
    pub fn root(&self) -> &SqliteNode {
        debug_assert!(!self.0.is_empty());

        &self.0[0]
    }

    pub fn nodes(&self) -> &[SqliteNode] {
        self.0.as_slice()
    }

    pub fn allocate(&mut self, node: SqliteNode) -> NodeId {
        self.0.push(node);

        NodeId(self.0.len() - 1)
    }

    pub fn node_ref(&self, id: NodeId) -> &SqliteNode {
        &self.0[id.0]
    }

    pub fn new() -> Self {
        SqliteUntypedAst(Vec::new())
    }

    pub fn tree_node_mut(&mut self, id: NodeId) -> &mut SqliteNode {
        match &mut self.0[id.0] {
            node @ SqliteNode::Tree { .. } => node,
            _ => panic!("Node is a not a tree node"),
        }
    }
}

impl std::fmt::Debug for SqliteUntypedAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        self.root().print(&mut buf, 0, self);
        write!(f, "{}", buf)
    }
}

impl NodeId {
    pub fn add_tree_child<'i, 'a>(
        &self,
        ast: &mut SqliteUntypedAst,
        kind: SqliteTreeKind,
    ) -> NodeId {
        let child_id = ast.allocate(SqliteNode::new_tree_node(kind, Some(*self)));

        let curr_node = ast.tree_node_mut(*self);
        curr_node.add_child(child_id);

        child_id
    }

    pub fn add_token_child<'i, 'a>(
        &self,
        ast: &mut SqliteUntypedAst,
        token: SqliteToken,
    ) -> NodeId {
        let child_id = ast.allocate(SqliteNode::new_token_node(token, *self));

        let curr_node = ast.tree_node_mut(*self);
        curr_node.add_child(child_id);

        child_id
    }

    pub fn as_node<'i, 'a>(&self, ast: &'a SqliteUntypedAst) -> &'a SqliteNode {
        ast.node_ref(*self)
    }
}

impl SqliteNode {
    pub fn new_tree_node(kind: SqliteTreeKind, parent: Option<NodeId>) -> Self {
        Self::Tree {
            kind,
            children: Vec::new(),
            parent,
        }
    }

    pub fn new_token_node(token: SqliteToken, parent: NodeId) -> Self {
        Self::Token { token, parent }
    }

    /// # Panics
    /// Panics if the node is not a tree node
    pub fn children<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
    ) -> impl Iterator<Item = &'a SqliteNode> {
        match self {
            Self::Tree { children, .. } => children.iter().map(|child_id| child_id.as_node(ast)),
            Self::Token { .. } => panic!("DEV ERROR: Cannot call children on token nodes"),
        }
    }

    pub fn add_child(&mut self, child: NodeId) {
        match self {
            Self::Tree { children, .. } => children.push(child),
            Self::Token { .. } => panic!("Cannot add child to token node"),
        }
    }

    fn print(&self, buf: &mut String, level: usize, ast: &SqliteUntypedAst) {
        let indent = "  ".repeat(level);

        match self {
            Self::Tree { kind, .. } => {
                std::write!(buf, "{indent}{:?}\n", kind).unwrap();
                for child in self.children(ast) {
                    child.print(buf, level + 1, ast)
                }
            }
            Self::Token { token, .. } => {
                if !token.is_trivia() {
                    std::write!(buf, "{indent}  '{}` - {:?}\n", token.text, token.kind).unwrap();
                }
            }
        }
        assert!(buf.ends_with('\n'));
    }

    pub fn parent(&self) -> Option<NodeId> {
        match self {
            Self::Tree { parent, .. } => parent.clone(),
            Self::Token { parent, .. } => Some(*parent),
        }
    }

    pub fn tree_kind(&self) -> Option<SqliteTreeKind> {
        match self {
            Self::Tree { kind, .. } => Some(*kind),
            _ => None,
        }
    }

    pub fn find_child<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
        key: impl ChildNodeKey,
    ) -> Option<&'a SqliteNode> {
        key.find_children(self, ast).next()
    }

    pub fn find_children<'a>(
        &'a self,
        ast: &'a SqliteUntypedAst,
        key: impl ChildNodeKey,
    ) -> impl Iterator<Item = &'a SqliteNode> {
        key.find_children(self, ast)
    }

    pub fn token_kind(&self) -> Option<SqliteTokenKind> {
        match self {
            Self::Token { token, .. } => Some(token.kind),
            _ => None,
        }
    }
}

pub trait ChildNodeKey {
    fn find_children<'i, 'a>(
        self,
        node: &'a SqliteNode,
        ast: &'a SqliteUntypedAst,
    ) -> impl Iterator<Item = &'a SqliteNode>;
}

impl ChildNodeKey for SqliteTokenKind {
    fn find_children<'i, 'a>(
        self,
        node: &'a SqliteNode,
        ast: &'a SqliteUntypedAst,
    ) -> impl Iterator<Item = &'a SqliteNode> {
        assert!(matches!(node, SqliteNode::Tree { .. }));

        node.children(ast)
            .filter(move |child| child.token_kind() == Some(self))
    }
}

impl ChildNodeKey for SqliteTreeKind {
    fn find_children<'i, 'a>(
        self,
        node: &'a SqliteNode,
        ast: &'a SqliteUntypedAst,
    ) -> impl Iterator<Item = &'a SqliteNode> {
        assert!(matches!(node, SqliteNode::Tree { .. }));

        node.children(ast)
            .filter(move |child| child.tree_kind() == Some(self))
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

    pub fn start_range(&self) -> (u32, u32) {
        (self.abs_pos, self.abs_pos)
    }

    pub fn end_range(&self) -> (u32, u32) {
        let end = self.abs_pos + self.text.len() as u32;

        (end, end)
    }

    pub fn is_trivia(&self) -> bool {
        matches!(
            self.kind,
            SqliteTokenKind::WHITESPACE
                | SqliteTokenKind::S_LINE_COMMENT
                | SqliteTokenKind::M_LINE_COMMENT
        )
    }

    pub fn is_eof(&self) -> bool {
        self.kind == SqliteTokenKind::EOF
    }

    pub fn is_error(&self) -> bool {
        matches!(self.kind, SqliteTokenKind::ERROR)
    }

    pub fn as_str(&self) -> &str {
        use SqliteTokenKind::*;

        match self.kind {
            KW_ABORT => "ABORT",
            KW_ACTION => "ACTION",
            KW_ADD => "ADD",
            KW_AFTER => "AFTER",
            KW_ALL => "ALL",
            KW_ALTER => "ALTER",
            KW_ALWAYS => "ALWAYS",
            KW_ANALYZE => "ANALYZE",
            KW_AND => "AND",
            KW_AS => "AS",
            KW_ASC => "ASC",
            KW_ATTACH => "ATTACH",
            KW_AUTOINCREMENT => "AUTOINCREMENT",
            KW_BEFORE => "BEFORE",
            KW_BEGIN => "BEGIN",
            KW_BETWEEN => "BETWEEN",
            KW_BY => "BY",
            KW_CASCADE => "CASCADE",
            KW_CASE => "CASE",
            KW_CAST => "CAST",
            KW_CHECK => "CHECK",
            KW_COLLATE => "COLLATE",
            KW_COLUMN => "COLUMN",
            KW_COMMIT => "COMMIT",
            KW_CONFLICT => "CONFLICT",
            KW_CONSTRAINT => "CONSTRAINT",
            KW_CREATE => "CREATE",
            KW_CROSS => "CROSS",
            KW_CURRENT => "CURRENT",
            KW_CURRENT_DATE => "CURRENT_DATE",
            KW_CURRENT_TIME => "CURRENT_TIME",
            KW_CURRENT_TIMESTAMP => "CURRENT_TIMESTAMP",
            KW_DATABASE => "DATABASE",
            KW_DEFAULT => "DEFAULT",
            KW_DEFERRABLE => "DEFERRABLE",
            KW_DEFERRED => "DEFERRED",
            KW_DELETE => "DELETE",
            KW_DESC => "DESC",
            KW_DETACH => "DETACH",
            KW_DISTINCT => "DISTINCT",
            KW_DO => "DO",
            KW_DROP => "DROP",
            KW_EACH => "EACH",
            KW_ELSE => "ELSE",
            KW_END => "END",
            KW_ESCAPE => "ESCAPE",
            KW_EXCEPT => "EXCEPT",
            KW_EXCLUDE => "EXCLUDE",
            KW_EXCLUSIVE => "EXCLUSIVE",
            KW_EXISTS => "EXISTS",
            KW_EXPLAIN => "EXPLAIN",
            KW_FAIL => "FAIL",
            KW_FILTER => "FILTER",
            KW_FIRST => "FIRST",
            KW_FOLLOWING => "FOLLOWING",
            KW_FOR => "FOR",
            KW_FOREIGN => "FOREIGN",
            KW_FROM => "FROM",
            KW_FULL => "FULL",
            KW_GENERATED => "GENERATED",
            KW_GLOB => "GLOB",
            KW_GROUP => "GROUP",
            KW_GROUPS => "GROUPS",
            KW_HAVING => "HAVING",
            KW_IF => "IF",
            KW_IGNORE => "IGNORE",
            KW_IMMEDIATE => "IMMEDIATE",
            KW_IN => "IN",
            KW_INDEX => "INDEX",
            KW_INDEXED => "INDEXED",
            KW_INITIALLY => "INITIALLY",
            KW_INNER => "INNER",
            KW_INSERT => "INSERT",
            KW_INSTEAD => "INSTEAD",
            KW_INTERSECT => "INTERSECT",
            KW_INTO => "INTO",
            KW_IS => "IS",
            KW_ISNULL => "ISNULL",
            KW_JOIN => "JOIN",
            KW_KEY => "KEY",
            KW_LAST => "LAST",
            KW_LEFT => "LEFT",
            KW_LIKE => "LIKE",
            KW_LIMIT => "LIMIT",
            KW_MATCH => "MATCH",
            KW_MATERIALIZED => "MATERIALIZED",
            KW_NATURAL => "NATURAL",
            KW_NO => "NO",
            KW_NOT => "NOT",
            KW_NOTHING => "NOTHING",
            KW_NOTNULL => "NOTNULL",
            KW_NULL => "NULL",
            KW_NULLS => "NULLS",
            KW_OF => "OF",
            KW_OFFSET => "OFFSET",
            KW_ON => "ON",
            KW_OR => "OR",
            KW_ORDER => "ORDER",
            KW_OTHERS => "OTHERS",
            KW_OUTER => "OUTER",
            KW_OVER => "OVER",
            KW_PARTITION => "PARTITION",
            KW_PLAN => "PLAN",
            KW_PRAGMA => "PRAGMA",
            KW_PRECEDING => "PRECEDING",
            KW_PRIMARY => "PRIMARY",
            KW_QUERY => "QUERY",
            KW_RAISE => "RAISE",
            KW_RANGE => "RANGE",
            KW_RECURSIVE => "RECURSIVE",
            KW_REFERENCES => "REFERENCES",
            KW_REGEXP => "REGEXP",
            KW_REINDEX => "REINDEX",
            KW_RELEASE => "RELEASE",
            KW_RENAME => "RENAME",
            KW_REPLACE => "REPLACE",
            KW_RESTRICT => "RESTRICT",
            KW_RETURNING => "RETURNING",
            KW_RIGHT => "RIGHT",
            KW_ROLLBACK => "ROLLBACK",
            KW_ROW => "ROW",
            KW_ROWS => "ROWS",
            KW_SAVEPOINT => "SAVEPOINT",
            KW_SELECT => "SELECT",
            KW_SET => "SET",
            KW_TABLE => "TABLE",
            KW_TEMP => "TEMP",
            KW_TEMPORARY => "TEMPORARY",
            KW_THEN => "THEN",
            KW_TIES => "TIES",
            KW_TO => "TO",
            KW_TRANSACTION => "TRANSACTION",
            KW_TRIGGER => "TRIGGER",
            KW_UNBOUNDED => "UNBOUNDED",
            KW_UNION => "UNION",
            KW_UNIQUE => "UNIQUE",
            KW_UPDATE => "UPDATE",
            KW_USING => "USING",
            KW_VACUUM => "VACUUM",
            KW_VALUES => "VALUES",
            KW_VIEW => "VIEW",
            KW_VIRTUAL => "VIRTUAL",
            KW_WHEN => "WHEN",
            KW_WHERE => "WHERE",
            KW_WINDOW => "WINDOW",
            KW_WITH => "WITH",
            KW_WITHOUT => "WITHOUT",
            WHITESPACE => self.text.as_str(),
            S_LINE_COMMENT => self.text.as_str(),
            M_LINE_COMMENT => self.text.as_str(),
            STR_LIT => self.text.as_str(),
            REAL_LIT => self.text.as_str(),
            IDEN => self.text.as_str(),
            DOT => ".",
            STAR => "*",
            L_PAREN => "(",
            R_PAREN => ")",
            COMMA => ",",
            SEMICOLON => ";",
            COLON => ":",
            EQ_SQL => "=",
            EQ => "==",
            NOT_EQ_SQL => "<>",
            NOT_EQ => "!=",
            PLUS => "+",
            MINUS => "-",
            F_SLASH => "/",
            PERCENT => "%",
            EOF => "",
            EXTRACT_TWO => "->>",
            EXTRACT_ONE => "->",
            L_CHEV => "<",
            R_CHEV => ">",
            L_CHEV_EQ => "<=",
            R_CHEV_EQ => ">=",
            DOUBLE_PIPE => "||",
            TILDA => "~",
            L_CHEV_TWO => "<<",
            R_CHEV_TWO => ">>",
            PIPE => "|",
            AMPERSAND => "&",
            Q_MARK => "?",
            AT_MARK => "@",
            INT_LIT => self.text.as_str(),
            HEX_LIT => self.text.as_str(),
            ERROR => self.text.as_str(),
        }
    }
}

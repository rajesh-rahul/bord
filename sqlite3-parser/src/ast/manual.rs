use crate::{grammar::common::NUMERIC_LIT, SqliteToken, SqliteTokenKind, SqliteTreeKind};

use crate::CstNodeTrait;

use super::generated::*;

pub struct PragmaStmt<N> {
    inner: N,
}

impl<'a, N: CstNodeTrait<'a>> PragmaStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::PragmaStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }

    pub fn full_pragma_name(&self) -> Option<FullPragmaName<N>> {
        self.inner.children().flat_map(FullPragmaName::cast).next()
    }

    pub fn pragma_value(&self) -> Option<PragmaValue<N>> {
        self.inner.children().flat_map(PragmaValue::cast).next()
    }
}

pub struct Offset<N> {
    inner: N,
}
impl<'a, N: CstNodeTrait<'a>> Offset<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::Offset) {
            Some(Self { inner: node })
        } else {
            None
        }
    }

    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner.children().flat_map(Expr::cast).next()
    }
}

pub struct SignedNumber<N> {
    pub(crate) inner: N,
}
impl<'a, N: CstNodeTrait<'a>> SignedNumber<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SignedNumber) {
            Some(Self { inner: node })
        } else {
            None
        }
    }

    pub fn number_sign(&self) -> impl Iterator<Item = NumberSign<N>> + use<'_, 'a, N> {
        self.inner.children().flat_map(NumberSign::cast)
    }

    pub fn number(&self) -> Option<&'a SqliteToken> {
        self.inner.non_trivial_children().find_map(|it| {
            if it.token_kind().is_some_and(|it| NUMERIC_LIT.contains(it)) {
                Some(it.token().unwrap())
            } else {
                None
            }
        })
    }
}

pub enum NumberSign<N> {
    PLUS(N),
    MINUS(N),
}

impl<'a, N: CstNodeTrait<'a>> NumberSign<N> {
    pub fn cast(node: N) -> Option<Self> {
        use SqliteTokenKind as TokenKind;

        if node.token_kind() == Some(TokenKind::PLUS) {
            return Some(Self::PLUS(node));
        } else if node.token_kind() == Some(TokenKind::MINUS) {
            return Some(Self::MINUS(node));
        } else {
            return None;
        }
    }
}

pub struct TableOrSubquery<N> {
    inner: N,
}

impl<'a, N: CstNodeTrait<'a>> TableOrSubquery<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableOrSubquery) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        self.inner
    }
    pub fn table_or_subquery_kind(&self) -> Option<TableOrSubqueryKind<N>> {
        self.inner
            .valid_children()
            .flat_map(TableOrSubqueryKind::cast)
            .next()
    }
    pub fn with_alias(&self) -> Option<WithAlias<N>> {
        self.inner
            .find_children(SqliteTreeKind::WithAlias)
            .flat_map(WithAlias::cast)
            .next()
    }
}

pub enum TableOrSubqueryKind<N> {
    QualifiedTableName(QualifiedTableName<N>),
    FromClauseTableValueFunction(FromClauseTableValueFunction<N>),
    SelectStmtWithCte(SelectStmtWithCte<N>),
    JoinClause(JoinClause<N>),
    TableOrSubquery(N), // To avoid boxing
}

impl<'a, N: CstNodeTrait<'a>> TableOrSubqueryKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match node.tree() {
            Some(SqliteTreeKind::QualifiedTableName) => Some(Self::QualifiedTableName(
                QualifiedTableName::cast(node).unwrap(),
            )),
            Some(SqliteTreeKind::FromClauseTableValueFunction) => {
                Some(Self::FromClauseTableValueFunction(
                    FromClauseTableValueFunction::cast(node).unwrap(),
                ))
            }
            Some(SqliteTreeKind::SelectStmtWithCte) => Some(Self::SelectStmtWithCte(
                SelectStmtWithCte::cast(node).unwrap(),
            )),
            Some(SqliteTreeKind::JoinClause) => {
                Some(Self::JoinClause(JoinClause::cast(node).unwrap()))
            }
            Some(SqliteTreeKind::TableOrSubquery) => Some(Self::TableOrSubquery(node)),
            _ => None,
        }
    }
}

pub struct JoinClause<N> {
    inner: N,
}
impl<'a, N: CstNodeTrait<'a>> JoinClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::JoinClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn left_operand(&self) -> Option<FromClauseValue<N>> {
        self.inner
            .valid_children()
            .take_while(|it| it.tree() != Some(SqliteTreeKind::JoinOperator))
            .flat_map(FromClauseValue::cast)
            .next()
    }
    pub fn join_operator(&self) -> Option<JoinOperator<N>> {
        self.inner
            .find_children(SqliteTreeKind::JoinOperator)
            .flat_map(JoinOperator::cast)
            .next()
    }
    pub fn right_operand(&self) -> Option<FromClauseValue<N>> {
        let mut child_iter = self.inner.valid_children();

        child_iter.find(|it| it.tree() == Some(SqliteTreeKind::JoinOperator));

        child_iter.find_map(FromClauseValue::cast)
    }

    pub fn join_constraint(&self) -> Option<JoinConstraint<N>> {
        self.inner
            .find_children(SqliteTreeKind::JoinConstraint)
            .flat_map(JoinConstraint::cast)
            .next()
    }
}

macro_rules! derive_deref {
    ($($struct_name:ident)*) => {
        $(
            impl<'a, N: CstNodeTrait<'a> + Copy> $struct_name<N> {
                pub fn untyped(&self) -> N {
                    self.inner
                }
            }
        )*
    };
}

pub struct JoinOperator<N> {
    inner: N,
}

pub enum JoinOperatorKind {
    Comma,
    Cross,
    Inner,
    Left,
    Right,
    Full,
    NaturalInner,
    NaturalLeft,
    NaturalRight,
    NaturalFull,
}

impl<'a, N: CstNodeTrait<'a>> JoinOperator<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::JoinClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        self.inner
    }

    pub fn kind(&self) -> Option<JoinOperatorKind> {
        unimplemented!()
    }
}

derive_deref! { JoinClause Offset PragmaStmt}

impl<'a, N: CstNodeTrait<'a> + Copy> TableOrSubqueryKind<N> {
    pub fn untyped(&self) -> N {
        match self {
            Self::QualifiedTableName(n) => n.untyped(),
            Self::FromClauseTableValueFunction(n) => n.untyped(),
            Self::SelectStmtWithCte(n) => n.untyped(),
            Self::JoinClause(n) => n.untyped(),
            Self::TableOrSubquery(n) => *n,
        }
    }
}

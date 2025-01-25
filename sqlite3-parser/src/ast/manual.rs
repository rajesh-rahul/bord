use crate::{
    grammar::common::NUMERIC_LIT, CstNode, SqliteToken, SqliteTokenKind, SqliteTreeKind, T,
};

use super::generated::*;

pub struct PragmaStmt<'a> {
    inner: CstNode<'a>,
}
impl<'a> PragmaStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::PragmaStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }

    pub fn full_pragma_name(&self) -> Option<FullPragmaName<'a>> {
        self.inner.children().flat_map(FullPragmaName::cast).next()
    }

    pub fn pragma_value(&self) -> Option<PragmaValue<'a>> {
        self.inner.children().flat_map(PragmaValue::cast).next()
    }
}

pub struct Offset<'a> {
    inner: CstNode<'a>,
}
impl<'a> Offset<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::Offset) {
            Some(Self { inner: node })
        } else {
            None
        }
    }

    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner.children().flat_map(Expr::cast).next()
    }
}

pub struct TypeName<'a> {
    inner: CstNode<'a>,
}
impl<'a> TypeName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TypeName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }

    pub fn type_name_words(&self) -> impl Iterator<Item = TypeNameWord<'a>> {
        self.inner.children().flat_map(TypeNameWord::cast)
    }

    pub fn left_signed_number(&self) -> Option<SignedNumber<'a>> {
        self.inner
            .non_trivial_children()
            .take_while(|it| it.token_kind() != Some(T![,]))
            .find(|it| it.tree() == Some(SqliteTreeKind::SignedNumber))
            .and_then(SignedNumber::cast)
    }

    pub fn right_signed_number(&self) -> Option<SignedNumber<'a>> {
        let mut child_iter = self.inner.non_trivial_children();

        // Navigate to the comma child if it exists (This will allow us to skip over the left
        // signed number)
        child_iter.find(|it| it.token_kind() == Some(T![,]));

        child_iter.find_map(SignedNumber::cast)
    }
}

pub struct SignedNumber<'a> {
    inner: CstNode<'a>,
}
impl<'a> SignedNumber<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SignedNumber) {
            Some(Self { inner: node })
        } else {
            None
        }
    }

    pub fn number_sign(&self) -> impl Iterator<Item = NumberSign<'a>> {
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

pub enum NumberSign<'a> {
    PLUS(&'a SqliteToken),
    MINUS(&'a SqliteToken),
}
impl<'a> NumberSign<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;

        if node.token_kind() == Some(TokenKind::PLUS) {
            return Some(Self::PLUS(node.token().unwrap()));
        } else if node.token_kind() == Some(TokenKind::MINUS) {
            return Some(Self::MINUS(node.token().unwrap()));
        } else {
            return None;
        }
    }
}

pub struct OpBetweenAnd<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpBetweenAnd<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpBetweenAnd) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn target_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }

    pub fn low_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();

        child_iter.find(|it| it.token_kind() == Some(SqliteTokenKind::KW_BETWEEN));

        child_iter
            .take_while(|it| it.token_kind() != Some(SqliteTokenKind::KW_AND))
            .find_map(Expr::cast)
    }

    pub fn high_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();

        child_iter.find(|it| it.token_kind() == Some(SqliteTokenKind::KW_AND));

        child_iter.find_map(Expr::cast)
    }
}

pub struct OpNotBetweenAnd<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpNotBetweenAnd<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotBetweenAnd) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn target_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }

    pub fn low_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();

        child_iter.find(|it| it.token_kind() == Some(SqliteTokenKind::KW_BETWEEN));

        child_iter
            .take_while(|it| it.token_kind() != Some(SqliteTokenKind::KW_AND))
            .find_map(Expr::cast)
    }

    pub fn high_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();

        child_iter.find(|it| it.token_kind() == Some(SqliteTokenKind::KW_AND));

        child_iter.find_map(Expr::cast)
    }
}

pub struct CaseWhenClause<'a> {
    inner: CstNode<'a>,
}
impl<'a> CaseWhenClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CaseWhenClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }

    pub fn when_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();

        child_iter.find(|it| it.token_kind() == Some(SqliteTokenKind::KW_WHEN));

        child_iter
            .take_while(|it| it.token_kind() != Some(SqliteTokenKind::KW_THEN))
            .find_map(Expr::cast)
    }

    pub fn then_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();

        child_iter.find(|it| it.token_kind() == Some(SqliteTokenKind::KW_THEN));

        child_iter.find_map(Expr::cast)
    }
}
pub struct FromClause<'a> {
    inner: CstNode<'a>,
}
impl<'a> FromClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FromClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<FromClauseValue<'a>> {
        self.inner.children().flat_map(FromClauseValue::cast).next()
    }
}

pub enum TableOrSubquery<'a> {
    QualifiedTableName(QualifiedTableName<'a>),
    FromClauseTableValueFunction(FromClauseTableValueFunction<'a>),
    FromClauseSelectStmt(FromClauseSelectStmt<'a>),
    JoinClause(JoinClause<'a>),
    TableOrSubquery(CstNode<'a>), // To avoid boxing
}

impl<'a> TableOrSubquery<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        match node.tree() {
            Some(SqliteTreeKind::QualifiedTableName) => Some(Self::QualifiedTableName(
                QualifiedTableName::cast(node).unwrap(),
            )),
            Some(SqliteTreeKind::FromClauseTableValueFunction) => {
                Some(Self::FromClauseTableValueFunction(
                    FromClauseTableValueFunction::cast(node).unwrap(),
                ))
            }
            Some(SqliteTreeKind::FromClauseSelectStmt) => Some(Self::FromClauseSelectStmt(
                FromClauseSelectStmt::cast(node).unwrap(),
            )),
            Some(SqliteTreeKind::JoinClause) => {
                Some(Self::JoinClause(JoinClause::cast(node).unwrap()))
            }
            Some(SqliteTreeKind::TableOrSubquery) => Some(Self::TableOrSubquery(node)),
            _ => None,
        }
    }
}

pub enum FromClauseValue<'a> {
    JoinClause(JoinClause<'a>),
    TableOrSubquery(TableOrSubquery<'a>),
}

impl<'a> FromClauseValue<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        match node.tree() {
            Some(SqliteTreeKind::JoinClause) => {
                Some(Self::JoinClause(JoinClause::cast(node).unwrap()))
            }
            Some(SqliteTreeKind::TableOrSubquery) => {
                Some(Self::TableOrSubquery(TableOrSubquery::cast(node).unwrap()))
            }
            _ => None,
        }
    }
}

pub struct JoinClause<'a> {
    inner: CstNode<'a>,
}
impl<'a> JoinClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::JoinClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn left_operand(&self) -> Option<FromClauseValue<'a>> {
        self.inner
            .valid_children()
            .take_while(|it| it.tree() != Some(SqliteTreeKind::JoinOperator))
            .flat_map(FromClauseValue::cast)
            .next()
    }
    pub fn join_operator(&self) -> Option<JoinOperator<'a>> {
        self.inner
            .find_children(SqliteTreeKind::JoinOperator)
            .flat_map(JoinOperator::cast)
            .next()
    }
    pub fn right_operand(&self) -> Option<FromClauseValue<'a>> {
        let mut child_iter = self.inner.valid_children();

        child_iter.find(|it| it.tree() == Some(SqliteTreeKind::JoinOperator));

        child_iter.find_map(FromClauseValue::cast)
    }

    pub fn join_constraint(&self) -> Option<JoinConstraint<'a>> {
        self.inner
            .find_children(SqliteTreeKind::JoinConstraint)
            .flat_map(JoinConstraint::cast)
            .next()
    }
}

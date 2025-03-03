#![allow(non_camel_case_types)]
use super::manual::*;
use crate::{
    CstNodeDataKind, CstNodeTrait, SqliteToken, SqliteTokenKind, SqliteTreeKind, SqliteTreeTag,
};
use SqliteTokenKind as TokenKind;
use SqliteTreeKind as TreeKind;
pub enum ExprInfix<N> {
    OpConcat(OpConcat<N>),
    OpExtractOne(OpExtractOne<N>),
    OpExtractTwo(OpExtractTwo<N>),
    OpMultiply(OpMultiply<N>),
    OpDivide(OpDivide<N>),
    OpModulus(OpModulus<N>),
    OpAdd(OpAdd<N>),
    OpSubtract(OpSubtract<N>),
    OpBinAnd(OpBinAnd<N>),
    OpBinOr(OpBinOr<N>),
    OpBinLShift(OpBinLShift<N>),
    OpBinRShift(OpBinRShift<N>),
    OpLT(OpLT<N>),
    OpGT(OpGT<N>),
    OpLTE(OpLTE<N>),
    OpGTE(OpGTE<N>),
    OpEq(OpEq<N>),
    OpNotEq(OpNotEq<N>),
    OpAnd(OpAnd<N>),
    OpOr(OpOr<N>),
    OpMatch(OpMatch<N>),
    OpLike(OpLike<N>),
    OpRegexp(OpRegexp<N>),
    OpGlob(OpGlob<N>),
    OpBetweenAnd(OpBetweenAnd<N>),
    OpNotMatch(OpNotMatch<N>),
    OpNotLike(OpNotLike<N>),
    OpNotRegexp(OpNotRegexp<N>),
    OpNotGlob(OpNotGlob<N>),
    OpNotBetweenAnd(OpNotBetweenAnd<N>),
    OpIsNotDistinctFrom(OpIsNotDistinctFrom<N>),
    OpIsDistinctFrom(OpIsDistinctFrom<N>),
    OpIsNot(OpIsNot<N>),
    OpIs(OpIs<N>),
    OpIn(OpIn<N>),
    OpNotIn(OpNotIn<N>),
}
impl<'a, N: CstNodeTrait<'a>> ExprInfix<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::ExprInfix) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::OpConcat, _) => {
                    Some(Self::OpConcat(OpConcat::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpExtractOne, _) => {
                    Some(Self::OpExtractOne(OpExtractOne::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpExtractTwo, _) => {
                    Some(Self::OpExtractTwo(OpExtractTwo::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpMultiply, _) => {
                    Some(Self::OpMultiply(OpMultiply::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpDivide, _) => {
                    Some(Self::OpDivide(OpDivide::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpModulus, _) => {
                    Some(Self::OpModulus(OpModulus::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpAdd, _) => Some(Self::OpAdd(OpAdd::cast(child)?)),
                CstNodeDataKind::Tree(TreeKind::OpSubtract, _) => {
                    Some(Self::OpSubtract(OpSubtract::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpBinAnd, _) => {
                    Some(Self::OpBinAnd(OpBinAnd::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpBinOr, _) => {
                    Some(Self::OpBinOr(OpBinOr::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpBinLShift, _) => {
                    Some(Self::OpBinLShift(OpBinLShift::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpBinRShift, _) => {
                    Some(Self::OpBinRShift(OpBinRShift::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpLT, _) => Some(Self::OpLT(OpLT::cast(child)?)),
                CstNodeDataKind::Tree(TreeKind::OpGT, _) => Some(Self::OpGT(OpGT::cast(child)?)),
                CstNodeDataKind::Tree(TreeKind::OpLTE, _) => Some(Self::OpLTE(OpLTE::cast(child)?)),
                CstNodeDataKind::Tree(TreeKind::OpGTE, _) => Some(Self::OpGTE(OpGTE::cast(child)?)),
                CstNodeDataKind::Tree(TreeKind::OpEq, _) => Some(Self::OpEq(OpEq::cast(child)?)),
                CstNodeDataKind::Tree(TreeKind::OpNotEq, _) => {
                    Some(Self::OpNotEq(OpNotEq::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpAnd, _) => Some(Self::OpAnd(OpAnd::cast(child)?)),
                CstNodeDataKind::Tree(TreeKind::OpOr, _) => Some(Self::OpOr(OpOr::cast(child)?)),
                CstNodeDataKind::Tree(TreeKind::OpMatch, _) => {
                    Some(Self::OpMatch(OpMatch::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpLike, _) => {
                    Some(Self::OpLike(OpLike::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpRegexp, _) => {
                    Some(Self::OpRegexp(OpRegexp::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpGlob, _) => {
                    Some(Self::OpGlob(OpGlob::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpBetweenAnd, _) => {
                    Some(Self::OpBetweenAnd(OpBetweenAnd::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpNotMatch, _) => {
                    Some(Self::OpNotMatch(OpNotMatch::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpNotLike, _) => {
                    Some(Self::OpNotLike(OpNotLike::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpNotRegexp, _) => {
                    Some(Self::OpNotRegexp(OpNotRegexp::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpNotGlob, _) => {
                    Some(Self::OpNotGlob(OpNotGlob::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpNotBetweenAnd, _) => {
                    Some(Self::OpNotBetweenAnd(OpNotBetweenAnd::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpIsNotDistinctFrom, _) => {
                    Some(Self::OpIsNotDistinctFrom(OpIsNotDistinctFrom::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpIsDistinctFrom, _) => {
                    Some(Self::OpIsDistinctFrom(OpIsDistinctFrom::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpIsNot, _) => {
                    Some(Self::OpIsNot(OpIsNot::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpIs, _) => Some(Self::OpIs(OpIs::cast(child)?)),
                CstNodeDataKind::Tree(TreeKind::OpIn, _) => Some(Self::OpIn(OpIn::cast(child)?)),
                CstNodeDataKind::Tree(TreeKind::OpNotIn, _) => {
                    Some(Self::OpNotIn(OpNotIn::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::OpConcat(node) => node.untyped(),
            Self::OpExtractOne(node) => node.untyped(),
            Self::OpExtractTwo(node) => node.untyped(),
            Self::OpMultiply(node) => node.untyped(),
            Self::OpDivide(node) => node.untyped(),
            Self::OpModulus(node) => node.untyped(),
            Self::OpAdd(node) => node.untyped(),
            Self::OpSubtract(node) => node.untyped(),
            Self::OpBinAnd(node) => node.untyped(),
            Self::OpBinOr(node) => node.untyped(),
            Self::OpBinLShift(node) => node.untyped(),
            Self::OpBinRShift(node) => node.untyped(),
            Self::OpLT(node) => node.untyped(),
            Self::OpGT(node) => node.untyped(),
            Self::OpLTE(node) => node.untyped(),
            Self::OpGTE(node) => node.untyped(),
            Self::OpEq(node) => node.untyped(),
            Self::OpNotEq(node) => node.untyped(),
            Self::OpAnd(node) => node.untyped(),
            Self::OpOr(node) => node.untyped(),
            Self::OpMatch(node) => node.untyped(),
            Self::OpLike(node) => node.untyped(),
            Self::OpRegexp(node) => node.untyped(),
            Self::OpGlob(node) => node.untyped(),
            Self::OpBetweenAnd(node) => node.untyped(),
            Self::OpNotMatch(node) => node.untyped(),
            Self::OpNotLike(node) => node.untyped(),
            Self::OpNotRegexp(node) => node.untyped(),
            Self::OpNotGlob(node) => node.untyped(),
            Self::OpNotBetweenAnd(node) => node.untyped(),
            Self::OpIsNotDistinctFrom(node) => node.untyped(),
            Self::OpIsDistinctFrom(node) => node.untyped(),
            Self::OpIsNot(node) => node.untyped(),
            Self::OpIs(node) => node.untyped(),
            Self::OpIn(node) => node.untyped(),
            Self::OpNotIn(node) => node.untyped(),
        }
    }
    pub fn op_concat(self) -> Option<OpConcat<N>> {
        match self {
            Self::OpConcat(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_extract_one(self) -> Option<OpExtractOne<N>> {
        match self {
            Self::OpExtractOne(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_extract_two(self) -> Option<OpExtractTwo<N>> {
        match self {
            Self::OpExtractTwo(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_multiply(self) -> Option<OpMultiply<N>> {
        match self {
            Self::OpMultiply(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_divide(self) -> Option<OpDivide<N>> {
        match self {
            Self::OpDivide(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_modulus(self) -> Option<OpModulus<N>> {
        match self {
            Self::OpModulus(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_add(self) -> Option<OpAdd<N>> {
        match self {
            Self::OpAdd(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_subtract(self) -> Option<OpSubtract<N>> {
        match self {
            Self::OpSubtract(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_bin_and(self) -> Option<OpBinAnd<N>> {
        match self {
            Self::OpBinAnd(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_bin_or(self) -> Option<OpBinOr<N>> {
        match self {
            Self::OpBinOr(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_bin_l_shift(self) -> Option<OpBinLShift<N>> {
        match self {
            Self::OpBinLShift(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_bin_r_shift(self) -> Option<OpBinRShift<N>> {
        match self {
            Self::OpBinRShift(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_lt(self) -> Option<OpLT<N>> {
        match self {
            Self::OpLT(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_gt(self) -> Option<OpGT<N>> {
        match self {
            Self::OpGT(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_lte(self) -> Option<OpLTE<N>> {
        match self {
            Self::OpLTE(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_gte(self) -> Option<OpGTE<N>> {
        match self {
            Self::OpGTE(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_eq(self) -> Option<OpEq<N>> {
        match self {
            Self::OpEq(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_eq(self) -> Option<OpNotEq<N>> {
        match self {
            Self::OpNotEq(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_and(self) -> Option<OpAnd<N>> {
        match self {
            Self::OpAnd(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_or(self) -> Option<OpOr<N>> {
        match self {
            Self::OpOr(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_match(self) -> Option<OpMatch<N>> {
        match self {
            Self::OpMatch(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_like(self) -> Option<OpLike<N>> {
        match self {
            Self::OpLike(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_regexp(self) -> Option<OpRegexp<N>> {
        match self {
            Self::OpRegexp(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_glob(self) -> Option<OpGlob<N>> {
        match self {
            Self::OpGlob(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_between_and(self) -> Option<OpBetweenAnd<N>> {
        match self {
            Self::OpBetweenAnd(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_match(self) -> Option<OpNotMatch<N>> {
        match self {
            Self::OpNotMatch(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_like(self) -> Option<OpNotLike<N>> {
        match self {
            Self::OpNotLike(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_regexp(self) -> Option<OpNotRegexp<N>> {
        match self {
            Self::OpNotRegexp(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_glob(self) -> Option<OpNotGlob<N>> {
        match self {
            Self::OpNotGlob(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_between_and(self) -> Option<OpNotBetweenAnd<N>> {
        match self {
            Self::OpNotBetweenAnd(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_is_not_distinct_from(self) -> Option<OpIsNotDistinctFrom<N>> {
        match self {
            Self::OpIsNotDistinctFrom(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_is_distinct_from(self) -> Option<OpIsDistinctFrom<N>> {
        match self {
            Self::OpIsDistinctFrom(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_is_not(self) -> Option<OpIsNot<N>> {
        match self {
            Self::OpIsNot(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_is(self) -> Option<OpIs<N>> {
        match self {
            Self::OpIs(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_in(self) -> Option<OpIn<N>> {
        match self {
            Self::OpIn(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_in(self) -> Option<OpNotIn<N>> {
        match self {
            Self::OpNotIn(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FuncArguments<N> {
    ArgExpr(ArgExpr<N>),
    ArgStar(ArgStar<N>),
}
impl<'a, N: CstNodeTrait<'a>> FuncArguments<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::FuncArguments) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::ArgExpr, _) => {
                    Some(Self::ArgExpr(ArgExpr::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ArgStar, _) => {
                    Some(Self::ArgStar(ArgStar::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::ArgExpr(node) => node.untyped(),
            Self::ArgStar(node) => node.untyped(),
        }
    }
    pub fn arg_expr(self) -> Option<ArgExpr<N>> {
        match self {
            Self::ArgExpr(item) => Some(item),
            _ => None,
        }
    }
    pub fn arg_star(self) -> Option<ArgStar<N>> {
        match self {
            Self::ArgStar(item) => Some(item),
            _ => None,
        }
    }
}
pub enum NullsPosition<N> {
    KW_FIRST(N),
    KW_LAST(N),
}
impl<'a, N: CstNodeTrait<'a>> NullsPosition<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_FIRST,
                ..
            }) => Some(Self::KW_FIRST(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_LAST,
                ..
            }) => Some(Self::KW_LAST(node)),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_FIRST(node) => node.clone(),
            Self::KW_LAST(node) => node.clone(),
        }
    }
    pub fn kw_first(self) -> Option<N> {
        match self {
            Self::KW_FIRST(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_last(self) -> Option<N> {
        match self {
            Self::KW_LAST(item) => Some(item),
            _ => None,
        }
    }
}
pub enum InsertValueKind<N> {
    InsertValuesClause(InsertValuesClause<N>),
    InsertSelectClause(InsertSelectClause<N>),
    InsertDefaultValuesClause(InsertDefaultValuesClause<N>),
}
impl<'a, N: CstNodeTrait<'a>> InsertValueKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::InsertValueKind) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::InsertValuesClause, _) => {
                    Some(Self::InsertValuesClause(InsertValuesClause::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::InsertSelectClause, _) => {
                    Some(Self::InsertSelectClause(InsertSelectClause::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::InsertDefaultValuesClause, _) => Some(
                    Self::InsertDefaultValuesClause(InsertDefaultValuesClause::cast(child)?),
                ),
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::InsertValuesClause(node) => node.untyped(),
            Self::InsertSelectClause(node) => node.untyped(),
            Self::InsertDefaultValuesClause(node) => node.untyped(),
        }
    }
    pub fn insert_values_clause(self) -> Option<InsertValuesClause<N>> {
        match self {
            Self::InsertValuesClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn insert_select_clause(self) -> Option<InsertSelectClause<N>> {
        match self {
            Self::InsertSelectClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn insert_default_values_clause(self) -> Option<InsertDefaultValuesClause<N>> {
        match self {
            Self::InsertDefaultValuesClause(item) => Some(item),
            _ => None,
        }
    }
}
pub enum SetColumnKind<N> {
    ColumnName(ColumnName<N>),
    ColNameList(ColNameList<N>),
}
impl<'a, N: CstNodeTrait<'a>> SetColumnKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::ColumnName, _) => {
                Some(Self::ColumnName(ColumnName::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::ColNameList, _) => {
                Some(Self::ColNameList(ColNameList::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::ColumnName(node) => node.untyped(),
            Self::ColNameList(node) => node.untyped(),
        }
    }
    pub fn column_name(self) -> Option<ColumnName<N>> {
        match self {
            Self::ColumnName(item) => Some(item),
            _ => None,
        }
    }
    pub fn col_name_list(self) -> Option<ColNameList<N>> {
        match self {
            Self::ColNameList(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ReturningClauseKind<N> {
    STAR(N),
    ReturningClauseExpr(ReturningClauseExpr<N>),
}
impl<'a, N: CstNodeTrait<'a>> ReturningClauseKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::ReturningClauseKind) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::STAR,
                    ..
                }) => Some(Self::STAR(node)),
                CstNodeDataKind::Tree(TreeKind::ReturningClauseExpr, _) => {
                    Some(Self::ReturningClauseExpr(ReturningClauseExpr::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::STAR(node) => node.clone(),
            Self::ReturningClauseExpr(node) => node.untyped(),
        }
    }
    pub fn star(self) -> Option<N> {
        match self {
            Self::STAR(item) => Some(item),
            _ => None,
        }
    }
    pub fn returning_clause_expr(self) -> Option<ReturningClauseExpr<N>> {
        match self {
            Self::ReturningClauseExpr(item) => Some(item),
            _ => None,
        }
    }
}
pub enum IndexDetails<N> {
    TableNameIndexedBy(TableNameIndexedBy<N>),
    TableNameNotIndexed(TableNameNotIndexed<N>),
}
impl<'a, N: CstNodeTrait<'a>> IndexDetails<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::TableNameIndexedBy, _) => {
                Some(Self::TableNameIndexedBy(TableNameIndexedBy::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::TableNameNotIndexed, _) => {
                Some(Self::TableNameNotIndexed(TableNameNotIndexed::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::TableNameIndexedBy(node) => node.untyped(),
            Self::TableNameNotIndexed(node) => node.untyped(),
        }
    }
    pub fn table_name_indexed_by(self) -> Option<TableNameIndexedBy<N>> {
        match self {
            Self::TableNameIndexedBy(item) => Some(item),
            _ => None,
        }
    }
    pub fn table_name_not_indexed(self) -> Option<TableNameNotIndexed<N>> {
        match self {
            Self::TableNameNotIndexed(item) => Some(item),
            _ => None,
        }
    }
}
pub enum InsertStmtKind<N> {
    KW_REPLACE(N),
    InsertOrAction(InsertOrAction<N>),
}
impl<'a, N: CstNodeTrait<'a>> InsertStmtKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::InsertStmtKind) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_REPLACE,
                    ..
                }) => Some(Self::KW_REPLACE(node)),
                CstNodeDataKind::Tree(TreeKind::InsertOrAction, _) => {
                    Some(Self::InsertOrAction(InsertOrAction::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_REPLACE(node) => node.clone(),
            Self::InsertOrAction(node) => node.untyped(),
        }
    }
    pub fn kw_replace(self) -> Option<N> {
        match self {
            Self::KW_REPLACE(item) => Some(item),
            _ => None,
        }
    }
    pub fn insert_or_action(self) -> Option<InsertOrAction<N>> {
        match self {
            Self::InsertOrAction(item) => Some(item),
            _ => None,
        }
    }
}
pub enum SelectDistinct<N> {
    KW_DISTINCT(N),
    KW_ALL(N),
}
impl<'a, N: CstNodeTrait<'a>> SelectDistinct<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_DISTINCT,
                ..
            }) => Some(Self::KW_DISTINCT(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_ALL,
                ..
            }) => Some(Self::KW_ALL(node)),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_DISTINCT(node) => node.clone(),
            Self::KW_ALL(node) => node.clone(),
        }
    }
    pub fn kw_distinct(self) -> Option<N> {
        match self {
            Self::KW_DISTINCT(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_all(self) -> Option<N> {
        match self {
            Self::KW_ALL(item) => Some(item),
            _ => None,
        }
    }
}
pub enum Range<N> {
    KW_RANGE(N),
    KW_ROWS(N),
    KW_GROUPS(N),
}
impl<'a, N: CstNodeTrait<'a>> Range<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_RANGE,
                ..
            }) => Some(Self::KW_RANGE(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_ROWS,
                ..
            }) => Some(Self::KW_ROWS(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_GROUPS,
                ..
            }) => Some(Self::KW_GROUPS(node)),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_RANGE(node) => node.clone(),
            Self::KW_ROWS(node) => node.clone(),
            Self::KW_GROUPS(node) => node.clone(),
        }
    }
    pub fn kw_range(self) -> Option<N> {
        match self {
            Self::KW_RANGE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_rows(self) -> Option<N> {
        match self {
            Self::KW_ROWS(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_groups(self) -> Option<N> {
        match self {
            Self::KW_GROUPS(item) => Some(item),
            _ => None,
        }
    }
}
pub enum InExprKind<N> {
    EmptyableExprList(EmptyableExprList<N>),
    ExprSelect(ExprSelect<N>),
    InTableFunc(InTableFunc<N>),
    InTable(InTable<N>),
}
impl<'a, N: CstNodeTrait<'a>> InExprKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::EmptyableExprList, _) => {
                Some(Self::EmptyableExprList(EmptyableExprList::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::ExprSelect, _) => {
                Some(Self::ExprSelect(ExprSelect::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::InTableFunc, _) => {
                Some(Self::InTableFunc(InTableFunc::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::InTable, _) => {
                Some(Self::InTable(InTable::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::EmptyableExprList(node) => node.untyped(),
            Self::ExprSelect(node) => node.untyped(),
            Self::InTableFunc(node) => node.untyped(),
            Self::InTable(node) => node.untyped(),
        }
    }
    pub fn emptyable_expr_list(self) -> Option<EmptyableExprList<N>> {
        match self {
            Self::EmptyableExprList(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_select(self) -> Option<ExprSelect<N>> {
        match self {
            Self::ExprSelect(item) => Some(item),
            _ => None,
        }
    }
    pub fn in_table_func(self) -> Option<InTableFunc<N>> {
        match self {
            Self::InTableFunc(item) => Some(item),
            _ => None,
        }
    }
    pub fn in_table(self) -> Option<InTable<N>> {
        match self {
            Self::InTable(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ResultColumn<N> {
    ResultColumnExpr(ResultColumnExpr<N>),
    ResultColumnAll(ResultColumnAll<N>),
    ResultColumnTableAll(ResultColumnTableAll<N>),
}
impl<'a, N: CstNodeTrait<'a>> ResultColumn<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::ResultColumn) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::ResultColumnExpr, _) => {
                    Some(Self::ResultColumnExpr(ResultColumnExpr::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ResultColumnAll, _) => {
                    Some(Self::ResultColumnAll(ResultColumnAll::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ResultColumnTableAll, _) => Some(
                    Self::ResultColumnTableAll(ResultColumnTableAll::cast(child)?),
                ),
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::ResultColumnExpr(node) => node.untyped(),
            Self::ResultColumnAll(node) => node.untyped(),
            Self::ResultColumnTableAll(node) => node.untyped(),
        }
    }
    pub fn result_column_expr(self) -> Option<ResultColumnExpr<N>> {
        match self {
            Self::ResultColumnExpr(item) => Some(item),
            _ => None,
        }
    }
    pub fn result_column_all(self) -> Option<ResultColumnAll<N>> {
        match self {
            Self::ResultColumnAll(item) => Some(item),
            _ => None,
        }
    }
    pub fn result_column_table_all(self) -> Option<ResultColumnTableAll<N>> {
        match self {
            Self::ResultColumnTableAll(item) => Some(item),
            _ => None,
        }
    }
}
pub enum PlusOrMinus<N> {
    PLUS(N),
    MINUS(N),
}
impl<'a, N: CstNodeTrait<'a>> PlusOrMinus<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::PLUS,
                ..
            }) => Some(Self::PLUS(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::MINUS,
                ..
            }) => Some(Self::MINUS(node)),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::PLUS(node) => node.clone(),
            Self::MINUS(node) => node.clone(),
        }
    }
    pub fn plus(self) -> Option<N> {
        match self {
            Self::PLUS(item) => Some(item),
            _ => None,
        }
    }
    pub fn minus(self) -> Option<N> {
        match self {
            Self::MINUS(item) => Some(item),
            _ => None,
        }
    }
}
pub enum TableColumns<N> {
    TableDetails(TableDetails<N>),
    CreateTableSelect(CreateTableSelect<N>),
}
impl<'a, N: CstNodeTrait<'a>> TableColumns<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::TableDetails, _) => {
                Some(Self::TableDetails(TableDetails::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::CreateTableSelect, _) => {
                Some(Self::CreateTableSelect(CreateTableSelect::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::TableDetails(node) => node.untyped(),
            Self::CreateTableSelect(node) => node.untyped(),
        }
    }
    pub fn table_details(self) -> Option<TableDetails<N>> {
        match self {
            Self::TableDetails(item) => Some(item),
            _ => None,
        }
    }
    pub fn create_table_select(self) -> Option<CreateTableSelect<N>> {
        match self {
            Self::CreateTableSelect(item) => Some(item),
            _ => None,
        }
    }
}
pub enum Expr<N> {
    ExprParen(ExprParen<N>),
    ExprLit(ExprLit<N>),
    ExprColumnName(ExprColumnName<N>),
    ExprPrefix(ExprPrefix<N>),
    ExprPostfix(ExprPostfix<N>),
    ExprInfix(ExprInfix<N>),
    ExprBindParam(ExprBindParam<N>),
    ExprFunc(ExprFunc<N>),
    ExprExistsSelect(ExprExistsSelect<N>),
    ExprList(ExprList<N>),
    ExprCast(ExprCast<N>),
    ExprCase(ExprCase<N>),
    RaiseFunc(RaiseFunc<N>),
    ExprSelect(ExprSelect<N>),
}
impl<'a, N: CstNodeTrait<'a>> Expr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::Expr) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::ExprParen, _) => {
                    Some(Self::ExprParen(ExprParen::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ExprLit, _) => {
                    Some(Self::ExprLit(ExprLit::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ExprColumnName, _) => {
                    Some(Self::ExprColumnName(ExprColumnName::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ExprPrefix, _) => {
                    Some(Self::ExprPrefix(ExprPrefix::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ExprPostfix, _) => {
                    Some(Self::ExprPostfix(ExprPostfix::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ExprInfix, _) => {
                    Some(Self::ExprInfix(ExprInfix::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ExprBindParam, _) => {
                    Some(Self::ExprBindParam(ExprBindParam::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ExprFunc, _) => {
                    Some(Self::ExprFunc(ExprFunc::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ExprExistsSelect, _) => {
                    Some(Self::ExprExistsSelect(ExprExistsSelect::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ExprList, _) => {
                    Some(Self::ExprList(ExprList::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ExprCast, _) => {
                    Some(Self::ExprCast(ExprCast::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ExprCase, _) => {
                    Some(Self::ExprCase(ExprCase::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::RaiseFunc, _) => {
                    Some(Self::RaiseFunc(RaiseFunc::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ExprSelect, _) => {
                    Some(Self::ExprSelect(ExprSelect::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::ExprParen(node) => node.untyped(),
            Self::ExprLit(node) => node.untyped(),
            Self::ExprColumnName(node) => node.untyped(),
            Self::ExprPrefix(node) => node.untyped(),
            Self::ExprPostfix(node) => node.untyped(),
            Self::ExprInfix(node) => node.untyped(),
            Self::ExprBindParam(node) => node.untyped(),
            Self::ExprFunc(node) => node.untyped(),
            Self::ExprExistsSelect(node) => node.untyped(),
            Self::ExprList(node) => node.untyped(),
            Self::ExprCast(node) => node.untyped(),
            Self::ExprCase(node) => node.untyped(),
            Self::RaiseFunc(node) => node.untyped(),
            Self::ExprSelect(node) => node.untyped(),
        }
    }
    pub fn expr_paren(self) -> Option<ExprParen<N>> {
        match self {
            Self::ExprParen(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_lit(self) -> Option<ExprLit<N>> {
        match self {
            Self::ExprLit(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_column_name(self) -> Option<ExprColumnName<N>> {
        match self {
            Self::ExprColumnName(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_prefix(self) -> Option<ExprPrefix<N>> {
        match self {
            Self::ExprPrefix(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_postfix(self) -> Option<ExprPostfix<N>> {
        match self {
            Self::ExprPostfix(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_infix(self) -> Option<ExprInfix<N>> {
        match self {
            Self::ExprInfix(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_bind_param(self) -> Option<ExprBindParam<N>> {
        match self {
            Self::ExprBindParam(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_func(self) -> Option<ExprFunc<N>> {
        match self {
            Self::ExprFunc(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_exists_select(self) -> Option<ExprExistsSelect<N>> {
        match self {
            Self::ExprExistsSelect(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_list(self) -> Option<ExprList<N>> {
        match self {
            Self::ExprList(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_cast(self) -> Option<ExprCast<N>> {
        match self {
            Self::ExprCast(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_case(self) -> Option<ExprCase<N>> {
        match self {
            Self::ExprCase(item) => Some(item),
            _ => None,
        }
    }
    pub fn raise_func(self) -> Option<RaiseFunc<N>> {
        match self {
            Self::RaiseFunc(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_select(self) -> Option<ExprSelect<N>> {
        match self {
            Self::ExprSelect(item) => Some(item),
            _ => None,
        }
    }
}
pub enum Order<N> {
    KW_ASC(N),
    KW_DESC(N),
}
impl<'a, N: CstNodeTrait<'a>> Order<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::Order) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_ASC,
                    ..
                }) => Some(Self::KW_ASC(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_DESC,
                    ..
                }) => Some(Self::KW_DESC(node)),
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_ASC(node) => node.clone(),
            Self::KW_DESC(node) => node.clone(),
        }
    }
    pub fn kw_asc(self) -> Option<N> {
        match self {
            Self::KW_ASC(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_desc(self) -> Option<N> {
        match self {
            Self::KW_DESC(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FrameSpecExcludeKind<N> {
    FrameSpecNoOthers(FrameSpecNoOthers<N>),
    FrameSpecCurrentRow(FrameSpecCurrentRow<N>),
    KW_GROUP(N),
    KW_TIES(N),
}
impl<'a, N: CstNodeTrait<'a>> FrameSpecExcludeKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::FrameSpecNoOthers, _) => {
                Some(Self::FrameSpecNoOthers(FrameSpecNoOthers::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::FrameSpecCurrentRow, _) => {
                Some(Self::FrameSpecCurrentRow(FrameSpecCurrentRow::cast(node)?))
            }
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_GROUP,
                ..
            }) => Some(Self::KW_GROUP(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_TIES,
                ..
            }) => Some(Self::KW_TIES(node)),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::FrameSpecNoOthers(node) => node.untyped(),
            Self::FrameSpecCurrentRow(node) => node.untyped(),
            Self::KW_GROUP(node) => node.clone(),
            Self::KW_TIES(node) => node.clone(),
        }
    }
    pub fn frame_spec_no_others(self) -> Option<FrameSpecNoOthers<N>> {
        match self {
            Self::FrameSpecNoOthers(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_current_row(self) -> Option<FrameSpecCurrentRow<N>> {
        match self {
            Self::FrameSpecCurrentRow(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_group(self) -> Option<N> {
        match self {
            Self::KW_GROUP(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_ties(self) -> Option<N> {
        match self {
            Self::KW_TIES(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ExprLit<N> {
    INT_LIT(N),
    HEX_LIT(N),
    STR_LIT(N),
    REAL_LIT(N),
    BLOB_LIT(N),
    KW_NULL(N),
    KW_CURRENT_TIME(N),
    KW_CURRENT_DATE(N),
    KW_CURRENT_TIMESTAMP(N),
}
impl<'a, N: CstNodeTrait<'a>> ExprLit<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::ExprLit) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::INT_LIT,
                    ..
                }) => Some(Self::INT_LIT(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::HEX_LIT,
                    ..
                }) => Some(Self::HEX_LIT(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::STR_LIT,
                    ..
                }) => Some(Self::STR_LIT(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::REAL_LIT,
                    ..
                }) => Some(Self::REAL_LIT(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::BLOB_LIT,
                    ..
                }) => Some(Self::BLOB_LIT(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_NULL,
                    ..
                }) => Some(Self::KW_NULL(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_CURRENT_TIME,
                    ..
                }) => Some(Self::KW_CURRENT_TIME(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_CURRENT_DATE,
                    ..
                }) => Some(Self::KW_CURRENT_DATE(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_CURRENT_TIMESTAMP,
                    ..
                }) => Some(Self::KW_CURRENT_TIMESTAMP(node)),
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::INT_LIT(node) => node.clone(),
            Self::HEX_LIT(node) => node.clone(),
            Self::STR_LIT(node) => node.clone(),
            Self::REAL_LIT(node) => node.clone(),
            Self::BLOB_LIT(node) => node.clone(),
            Self::KW_NULL(node) => node.clone(),
            Self::KW_CURRENT_TIME(node) => node.clone(),
            Self::KW_CURRENT_DATE(node) => node.clone(),
            Self::KW_CURRENT_TIMESTAMP(node) => node.clone(),
        }
    }
    pub fn int_lit(self) -> Option<N> {
        match self {
            Self::INT_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn hex_lit(self) -> Option<N> {
        match self {
            Self::HEX_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn str_lit(self) -> Option<N> {
        match self {
            Self::STR_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn real_lit(self) -> Option<N> {
        match self {
            Self::REAL_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn blob_lit(self) -> Option<N> {
        match self {
            Self::BLOB_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_null(self) -> Option<N> {
        match self {
            Self::KW_NULL(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_current_time(self) -> Option<N> {
        match self {
            Self::KW_CURRENT_TIME(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_current_date(self) -> Option<N> {
        match self {
            Self::KW_CURRENT_DATE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_current_timestamp(self) -> Option<N> {
        match self {
            Self::KW_CURRENT_TIMESTAMP(item) => Some(item),
            _ => None,
        }
    }
}
pub enum TriggerActionKind<N> {
    KW_DELETE(N),
    KW_INSERT(N),
    TriggerUpdateAction(TriggerUpdateAction<N>),
}
impl<'a, N: CstNodeTrait<'a>> TriggerActionKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::TriggerActionKind) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_DELETE,
                    ..
                }) => Some(Self::KW_DELETE(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_INSERT,
                    ..
                }) => Some(Self::KW_INSERT(node)),
                CstNodeDataKind::Tree(TreeKind::TriggerUpdateAction, _) => {
                    Some(Self::TriggerUpdateAction(TriggerUpdateAction::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_DELETE(node) => node.clone(),
            Self::KW_INSERT(node) => node.clone(),
            Self::TriggerUpdateAction(node) => node.untyped(),
        }
    }
    pub fn kw_delete(self) -> Option<N> {
        match self {
            Self::KW_DELETE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_insert(self) -> Option<N> {
        match self {
            Self::KW_INSERT(item) => Some(item),
            _ => None,
        }
    }
    pub fn trigger_update_action(self) -> Option<TriggerUpdateAction<N>> {
        match self {
            Self::TriggerUpdateAction(item) => Some(item),
            _ => None,
        }
    }
}
pub enum AnalyzeTarget<N> {
    SchemaOrIdxOrTableName(SchemaOrIdxOrTableName<N>),
    TableOrIdxNameWithSchema(TableOrIdxNameWithSchema<N>),
}
impl<'a, N: CstNodeTrait<'a>> AnalyzeTarget<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::SchemaOrIdxOrTableName, _) => Some(
                Self::SchemaOrIdxOrTableName(SchemaOrIdxOrTableName::cast(node)?),
            ),
            CstNodeDataKind::Tree(TreeKind::TableOrIdxNameWithSchema, _) => Some(
                Self::TableOrIdxNameWithSchema(TableOrIdxNameWithSchema::cast(node)?),
            ),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::SchemaOrIdxOrTableName(node) => node.untyped(),
            Self::TableOrIdxNameWithSchema(node) => node.untyped(),
        }
    }
    pub fn schema_or_idx_or_table_name(self) -> Option<SchemaOrIdxOrTableName<N>> {
        match self {
            Self::SchemaOrIdxOrTableName(item) => Some(item),
            _ => None,
        }
    }
    pub fn table_or_idx_name_with_schema(self) -> Option<TableOrIdxNameWithSchema<N>> {
        match self {
            Self::TableOrIdxNameWithSchema(item) => Some(item),
            _ => None,
        }
    }
}
pub enum OverClauseKind<N> {
    WindowName(WindowName<N>),
    WindowDef(WindowDef<N>),
}
impl<'a, N: CstNodeTrait<'a>> OverClauseKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::WindowName, _) => {
                Some(Self::WindowName(WindowName::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::WindowDef, _) => {
                Some(Self::WindowDef(WindowDef::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::WindowName(node) => node.untyped(),
            Self::WindowDef(node) => node.untyped(),
        }
    }
    pub fn window_name(self) -> Option<WindowName<N>> {
        match self {
            Self::WindowName(item) => Some(item),
            _ => None,
        }
    }
    pub fn window_def(self) -> Option<WindowDef<N>> {
        match self {
            Self::WindowDef(item) => Some(item),
            _ => None,
        }
    }
}
pub enum TriggerWhen<N> {
    KW_BEFORE(N),
    KW_AFTER(N),
    TriggerInsteadOf(TriggerInsteadOf<N>),
}
impl<'a, N: CstNodeTrait<'a>> TriggerWhen<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_BEFORE,
                ..
            }) => Some(Self::KW_BEFORE(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_AFTER,
                ..
            }) => Some(Self::KW_AFTER(node)),
            CstNodeDataKind::Tree(TreeKind::TriggerInsteadOf, _) => {
                Some(Self::TriggerInsteadOf(TriggerInsteadOf::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_BEFORE(node) => node.clone(),
            Self::KW_AFTER(node) => node.clone(),
            Self::TriggerInsteadOf(node) => node.untyped(),
        }
    }
    pub fn kw_before(self) -> Option<N> {
        match self {
            Self::KW_BEFORE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_after(self) -> Option<N> {
        match self {
            Self::KW_AFTER(item) => Some(item),
            _ => None,
        }
    }
    pub fn trigger_instead_of(self) -> Option<TriggerInsteadOf<N>> {
        match self {
            Self::TriggerInsteadOf(item) => Some(item),
            _ => None,
        }
    }
}
pub enum JoinConstraint<N> {
    OnConstraint(OnConstraint<N>),
    UsingConstraint(UsingConstraint<N>),
}
impl<'a, N: CstNodeTrait<'a>> JoinConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::JoinConstraint) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::OnConstraint, _) => {
                    Some(Self::OnConstraint(OnConstraint::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::UsingConstraint, _) => {
                    Some(Self::UsingConstraint(UsingConstraint::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::OnConstraint(node) => node.untyped(),
            Self::UsingConstraint(node) => node.untyped(),
        }
    }
    pub fn on_constraint(self) -> Option<OnConstraint<N>> {
        match self {
            Self::OnConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn using_constraint(self) -> Option<UsingConstraint<N>> {
        match self {
            Self::UsingConstraint(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ConstraintType<N> {
    PrimaryConstraint(PrimaryConstraint<N>),
    NullConstraint(NullConstraint<N>),
    UniqueConstraint(UniqueConstraint<N>),
    CheckConstraint(CheckConstraint<N>),
    DefaultConstraint(DefaultConstraint<N>),
    Collation(Collation<N>),
    ColumnGenerated(ColumnGenerated<N>),
    FkClause(FkClause<N>),
}
impl<'a, N: CstNodeTrait<'a>> ConstraintType<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::PrimaryConstraint, _) => {
                Some(Self::PrimaryConstraint(PrimaryConstraint::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::NullConstraint, _) => {
                Some(Self::NullConstraint(NullConstraint::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::UniqueConstraint, _) => {
                Some(Self::UniqueConstraint(UniqueConstraint::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::CheckConstraint, _) => {
                Some(Self::CheckConstraint(CheckConstraint::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::DefaultConstraint, _) => {
                Some(Self::DefaultConstraint(DefaultConstraint::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::Collation, _) => {
                Some(Self::Collation(Collation::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::ColumnGenerated, _) => {
                Some(Self::ColumnGenerated(ColumnGenerated::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::FkClause, _) => {
                Some(Self::FkClause(FkClause::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::PrimaryConstraint(node) => node.untyped(),
            Self::NullConstraint(node) => node.untyped(),
            Self::UniqueConstraint(node) => node.untyped(),
            Self::CheckConstraint(node) => node.untyped(),
            Self::DefaultConstraint(node) => node.untyped(),
            Self::Collation(node) => node.untyped(),
            Self::ColumnGenerated(node) => node.untyped(),
            Self::FkClause(node) => node.untyped(),
        }
    }
    pub fn primary_constraint(self) -> Option<PrimaryConstraint<N>> {
        match self {
            Self::PrimaryConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn null_constraint(self) -> Option<NullConstraint<N>> {
        match self {
            Self::NullConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn unique_constraint(self) -> Option<UniqueConstraint<N>> {
        match self {
            Self::UniqueConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn check_constraint(self) -> Option<CheckConstraint<N>> {
        match self {
            Self::CheckConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn default_constraint(self) -> Option<DefaultConstraint<N>> {
        match self {
            Self::DefaultConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn collation(self) -> Option<Collation<N>> {
        match self {
            Self::Collation(item) => Some(item),
            _ => None,
        }
    }
    pub fn column_generated(self) -> Option<ColumnGenerated<N>> {
        match self {
            Self::ColumnGenerated(item) => Some(item),
            _ => None,
        }
    }
    pub fn fk_clause(self) -> Option<FkClause<N>> {
        match self {
            Self::FkClause(item) => Some(item),
            _ => None,
        }
    }
}
pub enum CtePrependable<N> {
    SelectStmt(SelectStmt<N>),
    InsertStmt(InsertStmt<N>),
    UpdateStmt(UpdateStmt<N>),
    DeleteStmt(DeleteStmt<N>),
}
impl<'a, N: CstNodeTrait<'a>> CtePrependable<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::SelectStmt, _) => {
                Some(Self::SelectStmt(SelectStmt::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::InsertStmt, _) => {
                Some(Self::InsertStmt(InsertStmt::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::UpdateStmt, _) => {
                Some(Self::UpdateStmt(UpdateStmt::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::DeleteStmt, _) => {
                Some(Self::DeleteStmt(DeleteStmt::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::SelectStmt(node) => node.untyped(),
            Self::InsertStmt(node) => node.untyped(),
            Self::UpdateStmt(node) => node.untyped(),
            Self::DeleteStmt(node) => node.untyped(),
        }
    }
    pub fn select_stmt(self) -> Option<SelectStmt<N>> {
        match self {
            Self::SelectStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn insert_stmt(self) -> Option<InsertStmt<N>> {
        match self {
            Self::InsertStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn update_stmt(self) -> Option<UpdateStmt<N>> {
        match self {
            Self::UpdateStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn delete_stmt(self) -> Option<DeleteStmt<N>> {
        match self {
            Self::DeleteStmt(item) => Some(item),
            _ => None,
        }
    }
}
pub enum CompoundOperator<N> {
    UnionCompoundOperator(UnionCompoundOperator<N>),
    KW_INTERSECT(N),
    KW_EXCEPT(N),
}
impl<'a, N: CstNodeTrait<'a>> CompoundOperator<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::CompoundOperator) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::UnionCompoundOperator, _) => Some(
                    Self::UnionCompoundOperator(UnionCompoundOperator::cast(child)?),
                ),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_INTERSECT,
                    ..
                }) => Some(Self::KW_INTERSECT(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_EXCEPT,
                    ..
                }) => Some(Self::KW_EXCEPT(node)),
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::UnionCompoundOperator(node) => node.untyped(),
            Self::KW_INTERSECT(node) => node.clone(),
            Self::KW_EXCEPT(node) => node.clone(),
        }
    }
    pub fn union_compound_operator(self) -> Option<UnionCompoundOperator<N>> {
        match self {
            Self::UnionCompoundOperator(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_intersect(self) -> Option<N> {
        match self {
            Self::KW_INTERSECT(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_except(self) -> Option<N> {
        match self {
            Self::KW_EXCEPT(item) => Some(item),
            _ => None,
        }
    }
}
pub enum UpsertClauseAction<N> {
    UpsertDoUpdate(UpsertDoUpdate<N>),
    KW_NOTHING(N),
}
impl<'a, N: CstNodeTrait<'a>> UpsertClauseAction<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::UpsertDoUpdate, _) => {
                Some(Self::UpsertDoUpdate(UpsertDoUpdate::cast(node)?))
            }
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_NOTHING,
                ..
            }) => Some(Self::KW_NOTHING(node)),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::UpsertDoUpdate(node) => node.untyped(),
            Self::KW_NOTHING(node) => node.clone(),
        }
    }
    pub fn upsert_do_update(self) -> Option<UpsertDoUpdate<N>> {
        match self {
            Self::UpsertDoUpdate(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_nothing(self) -> Option<N> {
        match self {
            Self::KW_NOTHING(item) => Some(item),
            _ => None,
        }
    }
}
pub enum TableOptions<N> {
    TableOptWithoutRowId(TableOptWithoutRowId<N>),
    KW_STRICT(N),
}
impl<'a, N: CstNodeTrait<'a>> TableOptions<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::TableOptions) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::TableOptWithoutRowId, _) => Some(
                    Self::TableOptWithoutRowId(TableOptWithoutRowId::cast(child)?),
                ),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_STRICT,
                    ..
                }) => Some(Self::KW_STRICT(node)),
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::TableOptWithoutRowId(node) => node.untyped(),
            Self::KW_STRICT(node) => node.clone(),
        }
    }
    pub fn table_opt_without_row_id(self) -> Option<TableOptWithoutRowId<N>> {
        match self {
            Self::TableOptWithoutRowId(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_strict(self) -> Option<N> {
        match self {
            Self::KW_STRICT(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FrameSpecKind<N> {
    FrameSpecBetweenClause(FrameSpecBetweenClause<N>),
    FrameSpecUnboundedPreceding(FrameSpecUnboundedPreceding<N>),
    FrameSpecPreceding(FrameSpecPreceding<N>),
    FrameSpecCurrentRow(FrameSpecCurrentRow<N>),
}
impl<'a, N: CstNodeTrait<'a>> FrameSpecKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::FrameSpecBetweenClause, _) => Some(
                Self::FrameSpecBetweenClause(FrameSpecBetweenClause::cast(node)?),
            ),
            CstNodeDataKind::Tree(TreeKind::FrameSpecUnboundedPreceding, _) => Some(
                Self::FrameSpecUnboundedPreceding(FrameSpecUnboundedPreceding::cast(node)?),
            ),
            CstNodeDataKind::Tree(TreeKind::FrameSpecPreceding, _) => {
                Some(Self::FrameSpecPreceding(FrameSpecPreceding::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::FrameSpecCurrentRow, _) => {
                Some(Self::FrameSpecCurrentRow(FrameSpecCurrentRow::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::FrameSpecBetweenClause(node) => node.untyped(),
            Self::FrameSpecUnboundedPreceding(node) => node.untyped(),
            Self::FrameSpecPreceding(node) => node.untyped(),
            Self::FrameSpecCurrentRow(node) => node.untyped(),
        }
    }
    pub fn frame_spec_between_clause(self) -> Option<FrameSpecBetweenClause<N>> {
        match self {
            Self::FrameSpecBetweenClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_unbounded_preceding(self) -> Option<FrameSpecUnboundedPreceding<N>> {
        match self {
            Self::FrameSpecUnboundedPreceding(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_preceding(self) -> Option<FrameSpecPreceding<N>> {
        match self {
            Self::FrameSpecPreceding(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_current_row(self) -> Option<FrameSpecCurrentRow<N>> {
        match self {
            Self::FrameSpecCurrentRow(item) => Some(item),
            _ => None,
        }
    }
}
pub enum TableConstraintKind<N> {
    TablePkConstraint(TablePkConstraint<N>),
    TableUqConstraint(TableUqConstraint<N>),
    CheckConstraint(CheckConstraint<N>),
    TableFkConstraint(TableFkConstraint<N>),
}
impl<'a, N: CstNodeTrait<'a>> TableConstraintKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::TablePkConstraint, _) => {
                Some(Self::TablePkConstraint(TablePkConstraint::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::TableUqConstraint, _) => {
                Some(Self::TableUqConstraint(TableUqConstraint::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::CheckConstraint, _) => {
                Some(Self::CheckConstraint(CheckConstraint::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::TableFkConstraint, _) => {
                Some(Self::TableFkConstraint(TableFkConstraint::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::TablePkConstraint(node) => node.untyped(),
            Self::TableUqConstraint(node) => node.untyped(),
            Self::CheckConstraint(node) => node.untyped(),
            Self::TableFkConstraint(node) => node.untyped(),
        }
    }
    pub fn table_pk_constraint(self) -> Option<TablePkConstraint<N>> {
        match self {
            Self::TablePkConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn table_uq_constraint(self) -> Option<TableUqConstraint<N>> {
        match self {
            Self::TableUqConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn check_constraint(self) -> Option<CheckConstraint<N>> {
        match self {
            Self::CheckConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn table_fk_constraint(self) -> Option<TableFkConstraint<N>> {
        match self {
            Self::TableFkConstraint(item) => Some(item),
            _ => None,
        }
    }
}
pub enum StatementNoCte<N> {
    CreateTableStmt(CreateTableStmt<N>),
    AlterTableStmt(AlterTableStmt<N>),
    AnalyzeStmt(AnalyzeStmt<N>),
    AttachDbStmt(AttachDbStmt<N>),
    BeginStmt(BeginStmt<N>),
    CommitStmt(CommitStmt<N>),
    CreateIndexStmt(CreateIndexStmt<N>),
    CreateTriggerStmt(CreateTriggerStmt<N>),
    CreateViewStmt(CreateViewStmt<N>),
    CreateVirtualTableStmt(CreateVirtualTableStmt<N>),
    DetachStmt(DetachStmt<N>),
    DropIndexStmt(DropIndexStmt<N>),
    DropViewStmt(DropViewStmt<N>),
    DropTableStmt(DropTableStmt<N>),
    DropTriggerStmt(DropTriggerStmt<N>),
    PragmaStmt(PragmaStmt<N>),
    ReIndexStmt(ReIndexStmt<N>),
    ReleaseStmt(ReleaseStmt<N>),
    RollbackStmt(RollbackStmt<N>),
    SavepointStmt(SavepointStmt<N>),
    VacuumStmt(VacuumStmt<N>),
}
impl<'a, N: CstNodeTrait<'a>> StatementNoCte<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::StatementNoCte) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::CreateTableStmt, _) => {
                    Some(Self::CreateTableStmt(CreateTableStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::AlterTableStmt, _) => {
                    Some(Self::AlterTableStmt(AlterTableStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::AnalyzeStmt, _) => {
                    Some(Self::AnalyzeStmt(AnalyzeStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::AttachDbStmt, _) => {
                    Some(Self::AttachDbStmt(AttachDbStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::BeginStmt, _) => {
                    Some(Self::BeginStmt(BeginStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::CommitStmt, _) => {
                    Some(Self::CommitStmt(CommitStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::CreateIndexStmt, _) => {
                    Some(Self::CreateIndexStmt(CreateIndexStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::CreateTriggerStmt, _) => {
                    Some(Self::CreateTriggerStmt(CreateTriggerStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::CreateViewStmt, _) => {
                    Some(Self::CreateViewStmt(CreateViewStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::CreateVirtualTableStmt, _) => Some(
                    Self::CreateVirtualTableStmt(CreateVirtualTableStmt::cast(child)?),
                ),
                CstNodeDataKind::Tree(TreeKind::DetachStmt, _) => {
                    Some(Self::DetachStmt(DetachStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::DropIndexStmt, _) => {
                    Some(Self::DropIndexStmt(DropIndexStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::DropViewStmt, _) => {
                    Some(Self::DropViewStmt(DropViewStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::DropTableStmt, _) => {
                    Some(Self::DropTableStmt(DropTableStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::DropTriggerStmt, _) => {
                    Some(Self::DropTriggerStmt(DropTriggerStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::PragmaStmt, _) => {
                    Some(Self::PragmaStmt(PragmaStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ReIndexStmt, _) => {
                    Some(Self::ReIndexStmt(ReIndexStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ReleaseStmt, _) => {
                    Some(Self::ReleaseStmt(ReleaseStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::RollbackStmt, _) => {
                    Some(Self::RollbackStmt(RollbackStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::SavepointStmt, _) => {
                    Some(Self::SavepointStmt(SavepointStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::VacuumStmt, _) => {
                    Some(Self::VacuumStmt(VacuumStmt::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::CreateTableStmt(node) => node.untyped(),
            Self::AlterTableStmt(node) => node.untyped(),
            Self::AnalyzeStmt(node) => node.untyped(),
            Self::AttachDbStmt(node) => node.untyped(),
            Self::BeginStmt(node) => node.untyped(),
            Self::CommitStmt(node) => node.untyped(),
            Self::CreateIndexStmt(node) => node.untyped(),
            Self::CreateTriggerStmt(node) => node.untyped(),
            Self::CreateViewStmt(node) => node.untyped(),
            Self::CreateVirtualTableStmt(node) => node.untyped(),
            Self::DetachStmt(node) => node.untyped(),
            Self::DropIndexStmt(node) => node.untyped(),
            Self::DropViewStmt(node) => node.untyped(),
            Self::DropTableStmt(node) => node.untyped(),
            Self::DropTriggerStmt(node) => node.untyped(),
            Self::PragmaStmt(node) => node.untyped(),
            Self::ReIndexStmt(node) => node.untyped(),
            Self::ReleaseStmt(node) => node.untyped(),
            Self::RollbackStmt(node) => node.untyped(),
            Self::SavepointStmt(node) => node.untyped(),
            Self::VacuumStmt(node) => node.untyped(),
        }
    }
    pub fn create_table_stmt(self) -> Option<CreateTableStmt<N>> {
        match self {
            Self::CreateTableStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn alter_table_stmt(self) -> Option<AlterTableStmt<N>> {
        match self {
            Self::AlterTableStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn analyze_stmt(self) -> Option<AnalyzeStmt<N>> {
        match self {
            Self::AnalyzeStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn attach_db_stmt(self) -> Option<AttachDbStmt<N>> {
        match self {
            Self::AttachDbStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn begin_stmt(self) -> Option<BeginStmt<N>> {
        match self {
            Self::BeginStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn commit_stmt(self) -> Option<CommitStmt<N>> {
        match self {
            Self::CommitStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn create_index_stmt(self) -> Option<CreateIndexStmt<N>> {
        match self {
            Self::CreateIndexStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn create_trigger_stmt(self) -> Option<CreateTriggerStmt<N>> {
        match self {
            Self::CreateTriggerStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn create_view_stmt(self) -> Option<CreateViewStmt<N>> {
        match self {
            Self::CreateViewStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn create_virtual_table_stmt(self) -> Option<CreateVirtualTableStmt<N>> {
        match self {
            Self::CreateVirtualTableStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn detach_stmt(self) -> Option<DetachStmt<N>> {
        match self {
            Self::DetachStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn drop_index_stmt(self) -> Option<DropIndexStmt<N>> {
        match self {
            Self::DropIndexStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn drop_view_stmt(self) -> Option<DropViewStmt<N>> {
        match self {
            Self::DropViewStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn drop_table_stmt(self) -> Option<DropTableStmt<N>> {
        match self {
            Self::DropTableStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn drop_trigger_stmt(self) -> Option<DropTriggerStmt<N>> {
        match self {
            Self::DropTriggerStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn pragma_stmt(self) -> Option<PragmaStmt<N>> {
        match self {
            Self::PragmaStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn re_index_stmt(self) -> Option<ReIndexStmt<N>> {
        match self {
            Self::ReIndexStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn release_stmt(self) -> Option<ReleaseStmt<N>> {
        match self {
            Self::ReleaseStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn rollback_stmt(self) -> Option<RollbackStmt<N>> {
        match self {
            Self::RollbackStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn savepoint_stmt(self) -> Option<SavepointStmt<N>> {
        match self {
            Self::SavepointStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn vacuum_stmt(self) -> Option<VacuumStmt<N>> {
        match self {
            Self::VacuumStmt(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FkOnOrMatch<N> {
    FkOnAction(FkOnAction<N>),
    FkMatchAction(FkMatchAction<N>),
}
impl<'a, N: CstNodeTrait<'a>> FkOnOrMatch<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::FkOnAction, _) => {
                Some(Self::FkOnAction(FkOnAction::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::FkMatchAction, _) => {
                Some(Self::FkMatchAction(FkMatchAction::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::FkOnAction(node) => node.untyped(),
            Self::FkMatchAction(node) => node.untyped(),
        }
    }
    pub fn fk_on_action(self) -> Option<FkOnAction<N>> {
        match self {
            Self::FkOnAction(item) => Some(item),
            _ => None,
        }
    }
    pub fn fk_match_action(self) -> Option<FkMatchAction<N>> {
        match self {
            Self::FkMatchAction(item) => Some(item),
            _ => None,
        }
    }
}
pub enum CommitStartKw<N> {
    KW_COMMIT(N),
    KW_END(N),
}
impl<'a, N: CstNodeTrait<'a>> CommitStartKw<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_COMMIT,
                ..
            }) => Some(Self::KW_COMMIT(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_END,
                ..
            }) => Some(Self::KW_END(node)),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_COMMIT(node) => node.clone(),
            Self::KW_END(node) => node.clone(),
        }
    }
    pub fn kw_commit(self) -> Option<N> {
        match self {
            Self::KW_COMMIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_end(self) -> Option<N> {
        match self {
            Self::KW_END(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FrameSpecBetweenLeft<N> {
    FrameSpecUnboundedPreceding(FrameSpecUnboundedPreceding<N>),
    FrameSpecPreceding(FrameSpecPreceding<N>),
    FrameSpecCurrentRow(FrameSpecCurrentRow<N>),
    FrameSpecFollowing(FrameSpecFollowing<N>),
}
impl<'a, N: CstNodeTrait<'a>> FrameSpecBetweenLeft<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::FrameSpecBetweenLeft) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::FrameSpecUnboundedPreceding, _) => Some(
                    Self::FrameSpecUnboundedPreceding(FrameSpecUnboundedPreceding::cast(child)?),
                ),
                CstNodeDataKind::Tree(TreeKind::FrameSpecPreceding, _) => {
                    Some(Self::FrameSpecPreceding(FrameSpecPreceding::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::FrameSpecCurrentRow, _) => {
                    Some(Self::FrameSpecCurrentRow(FrameSpecCurrentRow::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::FrameSpecFollowing, _) => {
                    Some(Self::FrameSpecFollowing(FrameSpecFollowing::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::FrameSpecUnboundedPreceding(node) => node.untyped(),
            Self::FrameSpecPreceding(node) => node.untyped(),
            Self::FrameSpecCurrentRow(node) => node.untyped(),
            Self::FrameSpecFollowing(node) => node.untyped(),
        }
    }
    pub fn frame_spec_unbounded_preceding(self) -> Option<FrameSpecUnboundedPreceding<N>> {
        match self {
            Self::FrameSpecUnboundedPreceding(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_preceding(self) -> Option<FrameSpecPreceding<N>> {
        match self {
            Self::FrameSpecPreceding(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_current_row(self) -> Option<FrameSpecCurrentRow<N>> {
        match self {
            Self::FrameSpecCurrentRow(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_following(self) -> Option<FrameSpecFollowing<N>> {
        match self {
            Self::FrameSpecFollowing(item) => Some(item),
            _ => None,
        }
    }
}
pub enum Temporary<N> {
    KW_TEMP(N),
    KW_TEMPORARY(N),
}
impl<'a, N: CstNodeTrait<'a>> Temporary<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_TEMP,
                ..
            }) => Some(Self::KW_TEMP(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_TEMPORARY,
                ..
            }) => Some(Self::KW_TEMPORARY(node)),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_TEMP(node) => node.clone(),
            Self::KW_TEMPORARY(node) => node.clone(),
        }
    }
    pub fn kw_temp(self) -> Option<N> {
        match self {
            Self::KW_TEMP(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_temporary(self) -> Option<N> {
        match self {
            Self::KW_TEMPORARY(item) => Some(item),
            _ => None,
        }
    }
}
pub enum BeginStmtKind<N> {
    KW_DEFERRED(N),
    KW_IMMEDIATE(N),
    KW_EXCLUSIVE(N),
}
impl<'a, N: CstNodeTrait<'a>> BeginStmtKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_DEFERRED,
                ..
            }) => Some(Self::KW_DEFERRED(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_IMMEDIATE,
                ..
            }) => Some(Self::KW_IMMEDIATE(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_EXCLUSIVE,
                ..
            }) => Some(Self::KW_EXCLUSIVE(node)),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_DEFERRED(node) => node.clone(),
            Self::KW_IMMEDIATE(node) => node.clone(),
            Self::KW_EXCLUSIVE(node) => node.clone(),
        }
    }
    pub fn kw_deferred(self) -> Option<N> {
        match self {
            Self::KW_DEFERRED(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_immediate(self) -> Option<N> {
        match self {
            Self::KW_IMMEDIATE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_exclusive(self) -> Option<N> {
        match self {
            Self::KW_EXCLUSIVE(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ExprPrefix<N> {
    OpBinComplement(OpBinComplement<N>),
    OpUnaryPlus(OpUnaryPlus<N>),
    OpUnaryMinus(OpUnaryMinus<N>),
    OpNot(OpNot<N>),
}
impl<'a, N: CstNodeTrait<'a>> ExprPrefix<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::ExprPrefix) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::OpBinComplement, _) => {
                    Some(Self::OpBinComplement(OpBinComplement::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpUnaryPlus, _) => {
                    Some(Self::OpUnaryPlus(OpUnaryPlus::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpUnaryMinus, _) => {
                    Some(Self::OpUnaryMinus(OpUnaryMinus::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpNot, _) => Some(Self::OpNot(OpNot::cast(child)?)),
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::OpBinComplement(node) => node.untyped(),
            Self::OpUnaryPlus(node) => node.untyped(),
            Self::OpUnaryMinus(node) => node.untyped(),
            Self::OpNot(node) => node.untyped(),
        }
    }
    pub fn op_bin_complement(self) -> Option<OpBinComplement<N>> {
        match self {
            Self::OpBinComplement(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_unary_plus(self) -> Option<OpUnaryPlus<N>> {
        match self {
            Self::OpUnaryPlus(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_unary_minus(self) -> Option<OpUnaryMinus<N>> {
        match self {
            Self::OpUnaryMinus(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not(self) -> Option<OpNot<N>> {
        match self {
            Self::OpNot(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FromClauseValue<N> {
    TableOrSubquery(TableOrSubquery<N>),
    JoinClause(JoinClause<N>),
}
impl<'a, N: CstNodeTrait<'a>> FromClauseValue<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::TableOrSubquery, _) => {
                Some(Self::TableOrSubquery(TableOrSubquery::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::JoinClause, _) => {
                Some(Self::JoinClause(JoinClause::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::TableOrSubquery(node) => node.untyped(),
            Self::JoinClause(node) => node.untyped(),
        }
    }
    pub fn table_or_subquery(self) -> Option<TableOrSubquery<N>> {
        match self {
            Self::TableOrSubquery(item) => Some(item),
            _ => None,
        }
    }
    pub fn join_clause(self) -> Option<JoinClause<N>> {
        match self {
            Self::JoinClause(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ExprPostfix<N> {
    OpNotSpaceNull(OpNotSpaceNull<N>),
    OpCollate(OpCollate<N>),
    OpNotNull(OpNotNull<N>),
    OpIsNull(OpIsNull<N>),
}
impl<'a, N: CstNodeTrait<'a>> ExprPostfix<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::ExprPostfix) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::OpNotSpaceNull, _) => {
                    Some(Self::OpNotSpaceNull(OpNotSpaceNull::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpCollate, _) => {
                    Some(Self::OpCollate(OpCollate::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpNotNull, _) => {
                    Some(Self::OpNotNull(OpNotNull::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::OpIsNull, _) => {
                    Some(Self::OpIsNull(OpIsNull::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::OpNotSpaceNull(node) => node.untyped(),
            Self::OpCollate(node) => node.untyped(),
            Self::OpNotNull(node) => node.untyped(),
            Self::OpIsNull(node) => node.untyped(),
        }
    }
    pub fn op_not_space_null(self) -> Option<OpNotSpaceNull<N>> {
        match self {
            Self::OpNotSpaceNull(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_collate(self) -> Option<OpCollate<N>> {
        match self {
            Self::OpCollate(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_null(self) -> Option<OpNotNull<N>> {
        match self {
            Self::OpNotNull(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_is_null(self) -> Option<OpIsNull<N>> {
        match self {
            Self::OpIsNull(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ColumnGeneratedKind<N> {
    KW_STORED(N),
    KW_VIRTUAL(N),
}
impl<'a, N: CstNodeTrait<'a>> ColumnGeneratedKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::ColumnGeneratedKind) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_STORED,
                    ..
                }) => Some(Self::KW_STORED(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_VIRTUAL,
                    ..
                }) => Some(Self::KW_VIRTUAL(node)),
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_STORED(node) => node.clone(),
            Self::KW_VIRTUAL(node) => node.clone(),
        }
    }
    pub fn kw_stored(self) -> Option<N> {
        match self {
            Self::KW_STORED(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_virtual(self) -> Option<N> {
        match self {
            Self::KW_VIRTUAL(item) => Some(item),
            _ => None,
        }
    }
}
pub enum RaiseAction<N> {
    KW_IGNORE(N),
    RaiseActionRollBack(RaiseActionRollBack<N>),
    RaiseActionAbort(RaiseActionAbort<N>),
    RaiseActionFail(RaiseActionFail<N>),
}
impl<'a, N: CstNodeTrait<'a>> RaiseAction<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::RaiseAction) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_IGNORE,
                    ..
                }) => Some(Self::KW_IGNORE(node)),
                CstNodeDataKind::Tree(TreeKind::RaiseActionRollBack, _) => {
                    Some(Self::RaiseActionRollBack(RaiseActionRollBack::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::RaiseActionAbort, _) => {
                    Some(Self::RaiseActionAbort(RaiseActionAbort::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::RaiseActionFail, _) => {
                    Some(Self::RaiseActionFail(RaiseActionFail::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_IGNORE(node) => node.clone(),
            Self::RaiseActionRollBack(node) => node.untyped(),
            Self::RaiseActionAbort(node) => node.untyped(),
            Self::RaiseActionFail(node) => node.untyped(),
        }
    }
    pub fn kw_ignore(self) -> Option<N> {
        match self {
            Self::KW_IGNORE(item) => Some(item),
            _ => None,
        }
    }
    pub fn raise_action_roll_back(self) -> Option<RaiseActionRollBack<N>> {
        match self {
            Self::RaiseActionRollBack(item) => Some(item),
            _ => None,
        }
    }
    pub fn raise_action_abort(self) -> Option<RaiseActionAbort<N>> {
        match self {
            Self::RaiseActionAbort(item) => Some(item),
            _ => None,
        }
    }
    pub fn raise_action_fail(self) -> Option<RaiseActionFail<N>> {
        match self {
            Self::RaiseActionFail(item) => Some(item),
            _ => None,
        }
    }
}
pub enum PragmaValue<N> {
    SignedNumber(SignedNumber<N>),
    PragmaValueName(PragmaValueName<N>),
    KW_ON(N),
    KW_DELETE(N),
    KW_DEFAULT(N),
}
impl<'a, N: CstNodeTrait<'a>> PragmaValue<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::PragmaValue) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::SignedNumber, _) => {
                    Some(Self::SignedNumber(SignedNumber::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::PragmaValueName, _) => {
                    Some(Self::PragmaValueName(PragmaValueName::cast(child)?))
                }
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_ON,
                    ..
                }) => Some(Self::KW_ON(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_DELETE,
                    ..
                }) => Some(Self::KW_DELETE(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_DEFAULT,
                    ..
                }) => Some(Self::KW_DEFAULT(node)),
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::SignedNumber(node) => node.untyped(),
            Self::PragmaValueName(node) => node.untyped(),
            Self::KW_ON(node) => node.clone(),
            Self::KW_DELETE(node) => node.clone(),
            Self::KW_DEFAULT(node) => node.clone(),
        }
    }
    pub fn signed_number(self) -> Option<SignedNumber<N>> {
        match self {
            Self::SignedNumber(item) => Some(item),
            _ => None,
        }
    }
    pub fn pragma_value_name(self) -> Option<PragmaValueName<N>> {
        match self {
            Self::PragmaValueName(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_on(self) -> Option<N> {
        match self {
            Self::KW_ON(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_delete(self) -> Option<N> {
        match self {
            Self::KW_DELETE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_default(self) -> Option<N> {
        match self {
            Self::KW_DEFAULT(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ModuleArg<N> {
    STR_LIT(N),
    INT_LIT(N),
    REAL_LIT(N),
    HEX_LIT(N),
}
impl<'a, N: CstNodeTrait<'a>> ModuleArg<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::ModuleArg) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::STR_LIT,
                    ..
                }) => Some(Self::STR_LIT(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::INT_LIT,
                    ..
                }) => Some(Self::INT_LIT(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::REAL_LIT,
                    ..
                }) => Some(Self::REAL_LIT(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::HEX_LIT,
                    ..
                }) => Some(Self::HEX_LIT(node)),
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::STR_LIT(node) => node.clone(),
            Self::INT_LIT(node) => node.clone(),
            Self::REAL_LIT(node) => node.clone(),
            Self::HEX_LIT(node) => node.clone(),
        }
    }
    pub fn str_lit(self) -> Option<N> {
        match self {
            Self::STR_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn int_lit(self) -> Option<N> {
        match self {
            Self::INT_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn real_lit(self) -> Option<N> {
        match self {
            Self::REAL_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn hex_lit(self) -> Option<N> {
        match self {
            Self::HEX_LIT(item) => Some(item),
            _ => None,
        }
    }
}
pub enum Target<N> {
    TableOrIdxOrCollationName(TableOrIdxOrCollationName<N>),
    TableOrIdxNameWithSchema(TableOrIdxNameWithSchema<N>),
}
impl<'a, N: CstNodeTrait<'a>> Target<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::TableOrIdxOrCollationName, _) => Some(
                Self::TableOrIdxOrCollationName(TableOrIdxOrCollationName::cast(node)?),
            ),
            CstNodeDataKind::Tree(TreeKind::TableOrIdxNameWithSchema, _) => Some(
                Self::TableOrIdxNameWithSchema(TableOrIdxNameWithSchema::cast(node)?),
            ),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::TableOrIdxOrCollationName(node) => node.untyped(),
            Self::TableOrIdxNameWithSchema(node) => node.untyped(),
        }
    }
    pub fn table_or_idx_or_collation_name(self) -> Option<TableOrIdxOrCollationName<N>> {
        match self {
            Self::TableOrIdxOrCollationName(item) => Some(item),
            _ => None,
        }
    }
    pub fn table_or_idx_name_with_schema(self) -> Option<TableOrIdxNameWithSchema<N>> {
        match self {
            Self::TableOrIdxNameWithSchema(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ConflictAction<N> {
    KW_ROLLBACK(N),
    KW_ABORT(N),
    KW_FAIL(N),
    KW_IGNORE(N),
    KW_REPLACE(N),
}
impl<'a, N: CstNodeTrait<'a>> ConflictAction<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::ConflictAction) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_ROLLBACK,
                    ..
                }) => Some(Self::KW_ROLLBACK(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_ABORT,
                    ..
                }) => Some(Self::KW_ABORT(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_FAIL,
                    ..
                }) => Some(Self::KW_FAIL(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_IGNORE,
                    ..
                }) => Some(Self::KW_IGNORE(node)),
                CstNodeDataKind::Token(SqliteToken {
                    kind: TokenKind::KW_REPLACE,
                    ..
                }) => Some(Self::KW_REPLACE(node)),
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_ROLLBACK(node) => node.clone(),
            Self::KW_ABORT(node) => node.clone(),
            Self::KW_FAIL(node) => node.clone(),
            Self::KW_IGNORE(node) => node.clone(),
            Self::KW_REPLACE(node) => node.clone(),
        }
    }
    pub fn kw_rollback(self) -> Option<N> {
        match self {
            Self::KW_ROLLBACK(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_abort(self) -> Option<N> {
        match self {
            Self::KW_ABORT(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_fail(self) -> Option<N> {
        match self {
            Self::KW_FAIL(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_ignore(self) -> Option<N> {
        match self {
            Self::KW_IGNORE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_replace(self) -> Option<N> {
        match self {
            Self::KW_REPLACE(item) => Some(item),
            _ => None,
        }
    }
}
pub enum AlterTableKind<N> {
    RenameTable(RenameTable<N>),
    RenameColumn(RenameColumn<N>),
    AddColumn(AddColumn<N>),
    DropColumn(DropColumn<N>),
}
impl<'a, N: CstNodeTrait<'a>> AlterTableKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::RenameTable, _) => {
                Some(Self::RenameTable(RenameTable::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::RenameColumn, _) => {
                Some(Self::RenameColumn(RenameColumn::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::AddColumn, _) => {
                Some(Self::AddColumn(AddColumn::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::DropColumn, _) => {
                Some(Self::DropColumn(DropColumn::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::RenameTable(node) => node.untyped(),
            Self::RenameColumn(node) => node.untyped(),
            Self::AddColumn(node) => node.untyped(),
            Self::DropColumn(node) => node.untyped(),
        }
    }
    pub fn rename_table(self) -> Option<RenameTable<N>> {
        match self {
            Self::RenameTable(item) => Some(item),
            _ => None,
        }
    }
    pub fn rename_column(self) -> Option<RenameColumn<N>> {
        match self {
            Self::RenameColumn(item) => Some(item),
            _ => None,
        }
    }
    pub fn add_column(self) -> Option<AddColumn<N>> {
        match self {
            Self::AddColumn(item) => Some(item),
            _ => None,
        }
    }
    pub fn drop_column(self) -> Option<DropColumn<N>> {
        match self {
            Self::DropColumn(item) => Some(item),
            _ => None,
        }
    }
}
pub enum DeferKind<N> {
    KW_DEFERRED(N),
    KW_IMMEDIATE(N),
}
impl<'a, N: CstNodeTrait<'a>> DeferKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_DEFERRED,
                ..
            }) => Some(Self::KW_DEFERRED(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_IMMEDIATE,
                ..
            }) => Some(Self::KW_IMMEDIATE(node)),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_DEFERRED(node) => node.clone(),
            Self::KW_IMMEDIATE(node) => node.clone(),
        }
    }
    pub fn kw_deferred(self) -> Option<N> {
        match self {
            Self::KW_DEFERRED(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_immediate(self) -> Option<N> {
        match self {
            Self::KW_IMMEDIATE(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FkAction<N> {
    FkSetNull(FkSetNull<N>),
    FkSetDefault(FkSetDefault<N>),
    FkCascade(FkCascade<N>),
    FkRestrict(FkRestrict<N>),
    FkNoAction(FkNoAction<N>),
}
impl<'a, N: CstNodeTrait<'a>> FkAction<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::FkSetNull, _) => {
                Some(Self::FkSetNull(FkSetNull::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::FkSetDefault, _) => {
                Some(Self::FkSetDefault(FkSetDefault::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::FkCascade, _) => {
                Some(Self::FkCascade(FkCascade::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::FkRestrict, _) => {
                Some(Self::FkRestrict(FkRestrict::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::FkNoAction, _) => {
                Some(Self::FkNoAction(FkNoAction::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::FkSetNull(node) => node.untyped(),
            Self::FkSetDefault(node) => node.untyped(),
            Self::FkCascade(node) => node.untyped(),
            Self::FkRestrict(node) => node.untyped(),
            Self::FkNoAction(node) => node.untyped(),
        }
    }
    pub fn fk_set_null(self) -> Option<FkSetNull<N>> {
        match self {
            Self::FkSetNull(item) => Some(item),
            _ => None,
        }
    }
    pub fn fk_set_default(self) -> Option<FkSetDefault<N>> {
        match self {
            Self::FkSetDefault(item) => Some(item),
            _ => None,
        }
    }
    pub fn fk_cascade(self) -> Option<FkCascade<N>> {
        match self {
            Self::FkCascade(item) => Some(item),
            _ => None,
        }
    }
    pub fn fk_restrict(self) -> Option<FkRestrict<N>> {
        match self {
            Self::FkRestrict(item) => Some(item),
            _ => None,
        }
    }
    pub fn fk_no_action(self) -> Option<FkNoAction<N>> {
        match self {
            Self::FkNoAction(item) => Some(item),
            _ => None,
        }
    }
}
pub enum SelectCore<N> {
    TraditionalSelect(TraditionalSelect<N>),
    ValuesSelect(ValuesSelect<N>),
}
impl<'a, N: CstNodeTrait<'a>> SelectCore<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::SelectCore) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::TraditionalSelect, _) => {
                    Some(Self::TraditionalSelect(TraditionalSelect::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::ValuesSelect, _) => {
                    Some(Self::ValuesSelect(ValuesSelect::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::TraditionalSelect(node) => node.untyped(),
            Self::ValuesSelect(node) => node.untyped(),
        }
    }
    pub fn traditional_select(self) -> Option<TraditionalSelect<N>> {
        match self {
            Self::TraditionalSelect(item) => Some(item),
            _ => None,
        }
    }
    pub fn values_select(self) -> Option<ValuesSelect<N>> {
        match self {
            Self::ValuesSelect(item) => Some(item),
            _ => None,
        }
    }
}
pub enum DefaultValueKind<N> {
    DefaultConstraintExpr(DefaultConstraintExpr<N>),
    DefaultConstraintLiteral(DefaultConstraintLiteral<N>),
    DefaultConstraintIden(DefaultConstraintIden<N>),
}
impl<'a, N: CstNodeTrait<'a>> DefaultValueKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::DefaultConstraintExpr, _) => Some(
                Self::DefaultConstraintExpr(DefaultConstraintExpr::cast(node)?),
            ),
            CstNodeDataKind::Tree(TreeKind::DefaultConstraintLiteral, _) => Some(
                Self::DefaultConstraintLiteral(DefaultConstraintLiteral::cast(node)?),
            ),
            CstNodeDataKind::Tree(TreeKind::DefaultConstraintIden, _) => Some(
                Self::DefaultConstraintIden(DefaultConstraintIden::cast(node)?),
            ),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::DefaultConstraintExpr(node) => node.untyped(),
            Self::DefaultConstraintLiteral(node) => node.untyped(),
            Self::DefaultConstraintIden(node) => node.untyped(),
        }
    }
    pub fn default_constraint_expr(self) -> Option<DefaultConstraintExpr<N>> {
        match self {
            Self::DefaultConstraintExpr(item) => Some(item),
            _ => None,
        }
    }
    pub fn default_constraint_literal(self) -> Option<DefaultConstraintLiteral<N>> {
        match self {
            Self::DefaultConstraintLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn default_constraint_iden(self) -> Option<DefaultConstraintIden<N>> {
        match self {
            Self::DefaultConstraintIden(item) => Some(item),
            _ => None,
        }
    }
}
pub enum TriggerBodyStmt<N> {
    UpdateStmt(UpdateStmt<N>),
    InsertStmt(InsertStmt<N>),
    DeleteStmt(DeleteStmt<N>),
    SelectStmtWithCte(SelectStmtWithCte<N>),
}
impl<'a, N: CstNodeTrait<'a>> TriggerBodyStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::TriggerBodyStmt) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::UpdateStmt, _) => {
                    Some(Self::UpdateStmt(UpdateStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::InsertStmt, _) => {
                    Some(Self::InsertStmt(InsertStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::DeleteStmt, _) => {
                    Some(Self::DeleteStmt(DeleteStmt::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::SelectStmtWithCte, _) => {
                    Some(Self::SelectStmtWithCte(SelectStmtWithCte::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::UpdateStmt(node) => node.untyped(),
            Self::InsertStmt(node) => node.untyped(),
            Self::DeleteStmt(node) => node.untyped(),
            Self::SelectStmtWithCte(node) => node.untyped(),
        }
    }
    pub fn update_stmt(self) -> Option<UpdateStmt<N>> {
        match self {
            Self::UpdateStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn insert_stmt(self) -> Option<InsertStmt<N>> {
        match self {
            Self::InsertStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn delete_stmt(self) -> Option<DeleteStmt<N>> {
        match self {
            Self::DeleteStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn select_stmt_with_cte(self) -> Option<SelectStmtWithCte<N>> {
        match self {
            Self::SelectStmtWithCte(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FkFailKind<N> {
    KW_DELETE(N),
    KW_UPDATE(N),
}
impl<'a, N: CstNodeTrait<'a>> FkFailKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_DELETE,
                ..
            }) => Some(Self::KW_DELETE(node)),
            CstNodeDataKind::Token(SqliteToken {
                kind: TokenKind::KW_UPDATE,
                ..
            }) => Some(Self::KW_UPDATE(node)),
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::KW_DELETE(node) => node.clone(),
            Self::KW_UPDATE(node) => node.clone(),
        }
    }
    pub fn kw_delete(self) -> Option<N> {
        match self {
            Self::KW_DELETE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_update(self) -> Option<N> {
        match self {
            Self::KW_UPDATE(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FrameSpecBetweenRight<N> {
    FrameSpecUnboundedFollowing(FrameSpecUnboundedFollowing<N>),
    FrameSpecPreceding(FrameSpecPreceding<N>),
    FrameSpecCurrentRow(FrameSpecCurrentRow<N>),
    FrameSpecFollowing(FrameSpecFollowing<N>),
}
impl<'a, N: CstNodeTrait<'a>> FrameSpecBetweenRight<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() != Some(TreeKind::FrameSpecBetweenRight) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match &child.data().kind {
                CstNodeDataKind::Tree(TreeKind::FrameSpecUnboundedFollowing, _) => Some(
                    Self::FrameSpecUnboundedFollowing(FrameSpecUnboundedFollowing::cast(child)?),
                ),
                CstNodeDataKind::Tree(TreeKind::FrameSpecPreceding, _) => {
                    Some(Self::FrameSpecPreceding(FrameSpecPreceding::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::FrameSpecCurrentRow, _) => {
                    Some(Self::FrameSpecCurrentRow(FrameSpecCurrentRow::cast(child)?))
                }
                CstNodeDataKind::Tree(TreeKind::FrameSpecFollowing, _) => {
                    Some(Self::FrameSpecFollowing(FrameSpecFollowing::cast(child)?))
                }
                _ => None,
            })
            .next()
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::FrameSpecUnboundedFollowing(node) => node.untyped(),
            Self::FrameSpecPreceding(node) => node.untyped(),
            Self::FrameSpecCurrentRow(node) => node.untyped(),
            Self::FrameSpecFollowing(node) => node.untyped(),
        }
    }
    pub fn frame_spec_unbounded_following(self) -> Option<FrameSpecUnboundedFollowing<N>> {
        match self {
            Self::FrameSpecUnboundedFollowing(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_preceding(self) -> Option<FrameSpecPreceding<N>> {
        match self {
            Self::FrameSpecPreceding(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_current_row(self) -> Option<FrameSpecCurrentRow<N>> {
        match self {
            Self::FrameSpecCurrentRow(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_following(self) -> Option<FrameSpecFollowing<N>> {
        match self {
            Self::FrameSpecFollowing(item) => Some(item),
            _ => None,
        }
    }
}
pub enum StatementKind<N> {
    StatementNoCte(StatementNoCte<N>),
    StatementWithCte(StatementWithCte<N>),
}
impl<'a, N: CstNodeTrait<'a>> StatementKind<N> {
    pub fn cast(node: N) -> Option<Self> {
        match &node.data().kind {
            CstNodeDataKind::Tree(TreeKind::StatementNoCte, _) => {
                Some(Self::StatementNoCte(StatementNoCte::cast(node)?))
            }
            CstNodeDataKind::Tree(TreeKind::StatementWithCte, _) => {
                Some(Self::StatementWithCte(StatementWithCte::cast(node)?))
            }
            _ => None,
        }
    }
    pub fn untyped(&self) -> N
    where
        N: Copy,
    {
        match self {
            Self::StatementNoCte(node) => node.untyped(),
            Self::StatementWithCte(node) => node.untyped(),
        }
    }
    pub fn statement_no_cte(self) -> Option<StatementNoCte<N>> {
        match self {
            Self::StatementNoCte(item) => Some(item),
            _ => None,
        }
    }
    pub fn statement_with_cte(self) -> Option<StatementWithCte<N>> {
        match self {
            Self::StatementWithCte(item) => Some(item),
            _ => None,
        }
    }
}
pub struct WithAlias<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> WithAlias<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WithAlias) {
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
    pub fn alias_name(&self) -> Option<AliasName<N>> {
        self.inner
            .find_children(SqliteTreeKind::AliasName)
            .flat_map(AliasName::cast)
            .next()
    }
}
pub struct TableFunctionName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableFunctionName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableFunctionName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct FullTriggerName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FullTriggerName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FullTriggerName) {
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
    pub fn schema_name(&self) -> Option<SchemaName<N>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn trigger(&self) -> Option<TriggerName<N>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerName)
            .flat_map(TriggerName::cast)
            .next()
    }
}
pub struct ColumnName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ColumnName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ColumnName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct MaterializedCte<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> MaterializedCte<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::MaterializedCte) {
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
    pub fn not(&self) -> Option<N> {
        self.inner
            .find_children(SqliteTokenKind::KW_NOT)
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct TableFkConstraint<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableFkConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableFkConstraint) {
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
    pub fn col_name_list(&self) -> Option<ColNameList<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColNameList)
            .flat_map(ColNameList::cast)
            .next()
    }
    pub fn fk_clause(&self) -> Option<FkClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::FkClause)
            .flat_map(FkClause::cast)
            .next()
    }
}
pub struct ReturningClauseExpr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ReturningClauseExpr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ReturningClauseExpr) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn col_alias(&self) -> Option<AliasName<N>> {
        self.inner
            .find_children(SqliteTreeKind::AliasName)
            .flat_map(AliasName::cast)
            .next()
    }
}
pub struct InsertStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> InsertStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InsertStmt) {
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
    pub fn insert_stmt_kind(&self) -> Option<InsertStmtKind<N>> {
        self.inner
            .find_children(SqliteTreeKind::InsertStmtKind)
            .flat_map(InsertStmtKind::cast)
            .next()
    }
    pub fn full_table_name(&self) -> Option<FullTableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
    pub fn with_alias(&self) -> Option<WithAlias<N>> {
        self.inner
            .find_children(SqliteTreeKind::WithAlias)
            .flat_map(WithAlias::cast)
            .next()
    }
    pub fn col_name_list(&self) -> Option<ColNameList<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColNameList)
            .flat_map(ColNameList::cast)
            .next()
    }
    pub fn insert_value_kind(&self) -> Option<InsertValueKind<N>> {
        self.inner
            .find_children(SqliteTreeKind::InsertValueKind)
            .flat_map(InsertValueKind::cast)
            .next()
    }
    pub fn returning_clause(&self) -> Option<ReturningClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::ReturningClause)
            .flat_map(ReturningClause::cast)
            .next()
    }
}
pub struct CreateTriggerStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CreateTriggerStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CreateTriggerStmt) {
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
    pub fn temporary(&self) -> Option<Temporary<N>> {
        self.inner.children().flat_map(Temporary::cast).next()
    }
    pub fn if_not_exists(&self) -> Option<IfNotExists<N>> {
        self.inner
            .find_children(SqliteTreeKind::IfNotExists)
            .flat_map(IfNotExists::cast)
            .next()
    }
    pub fn full_trigger_name(&self) -> Option<FullTriggerName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullTriggerName)
            .flat_map(FullTriggerName::cast)
            .next()
    }
    pub fn trigger_when(&self) -> Option<TriggerWhen<N>> {
        self.inner.children().flat_map(TriggerWhen::cast).next()
    }
    pub fn trigger_action_kind(&self) -> Option<TriggerActionKind<N>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerActionKind)
            .flat_map(TriggerActionKind::cast)
            .next()
    }
    pub fn full_table_name(&self) -> Option<FullTableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
    pub fn trigger_for_each_row(&self) -> Option<TriggerForEachRow<N>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerForEachRow)
            .flat_map(TriggerForEachRow::cast)
            .next()
    }
    pub fn trigger_when_expr(&self) -> Option<TriggerWhenExpr<N>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerWhenExpr)
            .flat_map(TriggerWhenExpr::cast)
            .next()
    }
    pub fn trigger_body_stmt_list(&self) -> Option<TriggerBodyStmtList<N>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerBodyStmtList)
            .flat_map(TriggerBodyStmtList::cast)
            .next()
    }
}
pub struct TableNameIndexedBy<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableNameIndexedBy<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableNameIndexedBy) {
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
    pub fn index_name(&self) -> Option<IndexName<N>> {
        self.inner
            .find_children(SqliteTreeKind::IndexName)
            .flat_map(IndexName::cast)
            .next()
    }
}
pub struct OpIn<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpIn<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpIn) {
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
    pub fn lhs(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn in_expr_kind(&self) -> Option<InExprKind<N>> {
        self.inner.children().flat_map(InExprKind::cast).next()
    }
}
pub struct SetColumnExpr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> SetColumnExpr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SetColumnExpr) {
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
    pub fn set_column_kind(&self) -> Option<SetColumnKind<N>> {
        self.inner.children().flat_map(SetColumnKind::cast).next()
    }
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct FrameSpecUnboundedFollowing<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FrameSpecUnboundedFollowing<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecUnboundedFollowing) {
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
}
pub struct OpNotGlob<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpNotGlob<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotGlob) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct AlterTableStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> AlterTableStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::AlterTableStmt) {
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
    pub fn full_table_name(&self) -> Option<FullTableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
    pub fn alter_table_kind(&self) -> Option<AlterTableKind<N>> {
        self.inner.children().flat_map(AlterTableKind::cast).next()
    }
}
pub struct WindowBaseName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> WindowBaseName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WindowBaseName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct WindowPartitionByClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> WindowPartitionByClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WindowPartitionByClause) {
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
    pub fn items(&self) -> impl Iterator<Item = Expr<N>> + use<'_, 'a, N> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(|it| it.children())
            .flat_map(Expr::cast)
    }
}
pub struct WhereClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> WhereClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WhereClause) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct SavepointStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> SavepointStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SavepointStmt) {
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
    pub fn savepoint_name(&self) -> Option<SavepointName<N>> {
        self.inner
            .find_children(SqliteTreeKind::SavepointName)
            .flat_map(SavepointName::cast)
            .next()
    }
}
pub struct CreateTableSelect<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CreateTableSelect<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CreateTableSelect) {
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
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<N>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
}
pub struct TriggerName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TriggerName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct TraditionalSelect<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TraditionalSelect<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TraditionalSelect) {
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
    pub fn select_distinct(&self) -> Option<SelectDistinct<N>> {
        self.inner.children().flat_map(SelectDistinct::cast).next()
    }
    pub fn result_column_list(&self) -> Option<ResultColumnList<N>> {
        self.inner
            .find_children(SqliteTreeKind::ResultColumnList)
            .flat_map(ResultColumnList::cast)
            .next()
    }
    pub fn from_clause(&self) -> Option<FromClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::FromClause)
            .flat_map(FromClause::cast)
            .next()
    }
    pub fn where_clause(&self) -> Option<WhereClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
    pub fn group_by_clause(&self) -> Option<GroupByClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::GroupByClause)
            .flat_map(GroupByClause::cast)
            .next()
    }
    pub fn having_clause(&self) -> Option<HavingClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::HavingClause)
            .flat_map(HavingClause::cast)
            .next()
    }
    pub fn window_clause(&self) -> Option<WindowClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::WindowClause)
            .flat_map(WindowClause::cast)
            .next()
    }
}
pub struct OnConstraint<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OnConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OnConstraint) {
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
    pub fn on_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct TableOrIndexName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableOrIndexName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableOrIndexName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct ExprCast<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ExprCast<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprCast) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn type_name(&self) -> Option<TypeName<N>> {
        self.inner
            .find_children(SqliteTreeKind::TypeName)
            .flat_map(TypeName::cast)
            .next()
    }
}
pub struct CaseTargetExpr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CaseTargetExpr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CaseTargetExpr) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct IndexName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> IndexName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::IndexName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct OpUnaryPlus<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpUnaryPlus<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpUnaryPlus) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct OpIsNotDistinctFrom<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpIsNotDistinctFrom<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpIsNotDistinctFrom) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct OpLTE<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpLTE<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpLTE) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct TableOptWithoutRowId<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableOptWithoutRowId<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableOptWithoutRowId) {
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
}
pub struct SelectStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> SelectStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SelectStmt) {
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
    pub fn select_core(&self) -> Option<SelectCore<N>> {
        self.inner
            .find_children(SqliteTreeKind::SelectCore)
            .flat_map(SelectCore::cast)
            .next()
    }
    pub fn compound_selects(&self) -> impl Iterator<Item = CompoundSelect<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(CompoundSelect::cast)
    }
    pub fn order_by_clause(&self) -> Option<OrderByClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::OrderByClause)
            .flat_map(OrderByClause::cast)
            .next()
    }
    pub fn limit_clause(&self) -> Option<LimitClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::LimitClause)
            .flat_map(LimitClause::cast)
            .next()
    }
}
pub struct PasswordExpr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> PasswordExpr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::PasswordExpr) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct CreateIndexStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CreateIndexStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CreateIndexStmt) {
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
    pub fn unique(&self) -> Option<N> {
        self.inner
            .find_children(SqliteTokenKind::KW_UNIQUE)
            .next()
            .filter(|it| it.token().is_some())
    }
    pub fn if_not_exists(&self) -> Option<IfNotExists<N>> {
        self.inner
            .find_children(SqliteTreeKind::IfNotExists)
            .flat_map(IfNotExists::cast)
            .next()
    }
    pub fn full_index_name(&self) -> Option<FullIndexName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullIndexName)
            .flat_map(FullIndexName::cast)
            .next()
    }
    pub fn table_name(&self) -> Option<TableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
    pub fn indexed_col_list(&self) -> Option<IndexedColList<N>> {
        self.inner
            .find_children(SqliteTreeKind::IndexedColList)
            .flat_map(IndexedColList::cast)
            .next()
    }
    pub fn where_clause(&self) -> Option<WhereClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
}
pub struct TriggerUpdateAffectCols<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TriggerUpdateAffectCols<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerUpdateAffectCols) {
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
    pub fn columns(&self) -> impl Iterator<Item = ColumnName<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(ColumnName::cast)
    }
}
pub struct FrameSpecCurrentRow<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FrameSpecCurrentRow<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecCurrentRow) {
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
}
pub struct IndexedCol<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> IndexedCol<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::IndexedCol) {
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
    pub fn index_column(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn collation(&self) -> Option<Collation<N>> {
        self.inner
            .find_children(SqliteTreeKind::Collation)
            .flat_map(Collation::cast)
            .next()
    }
    pub fn order(&self) -> Option<Order<N>> {
        self.inner
            .find_children(SqliteTreeKind::Order)
            .flat_map(Order::cast)
            .next()
    }
}
pub struct SchemaNameExpr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> SchemaNameExpr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SchemaNameExpr) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct DeleteStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DeleteStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DeleteStmt) {
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
    pub fn qualified_table_name(&self) -> Option<QualifiedTableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::QualifiedTableName)
            .flat_map(QualifiedTableName::cast)
            .next()
    }
    pub fn where_clause(&self) -> Option<WhereClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
    pub fn returning_clause(&self) -> Option<ReturningClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::ReturningClause)
            .flat_map(ReturningClause::cast)
            .next()
    }
    pub fn delete_stmt_limited(&self) -> Option<DeleteStmtLimited<N>> {
        self.inner
            .find_children(SqliteTreeKind::DeleteStmtLimited)
            .flat_map(DeleteStmtLimited::cast)
            .next()
    }
}
pub struct OpCollate<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpCollate<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpCollate) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn collation(&self) -> Option<Collation<N>> {
        self.inner
            .find_children(SqliteTreeKind::Collation)
            .flat_map(Collation::cast)
            .next()
    }
}
pub struct ViewName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ViewName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ViewName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct FkMatchAction<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FkMatchAction<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkMatchAction) {
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
    pub fn any_valid_name(&self) -> Option<AnyValidName<N>> {
        self.inner
            .find_children(SqliteTreeKind::AnyValidName)
            .flat_map(AnyValidName::cast)
            .next()
    }
}
pub struct AnalyzeStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> AnalyzeStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::AnalyzeStmt) {
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
    pub fn analyze_target(&self) -> Option<AnalyzeTarget<N>> {
        self.inner.children().flat_map(AnalyzeTarget::cast).next()
    }
}
pub struct OpGTE<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpGTE<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpGTE) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct DropIndexStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DropIndexStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DropIndexStmt) {
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
    pub fn full_index_name(&self) -> Option<FullIndexName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullIndexName)
            .flat_map(FullIndexName::cast)
            .next()
    }
}
pub struct ColNameList<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ColNameList<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ColNameList) {
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
    pub fn items(&self) -> impl Iterator<Item = ColumnName<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(ColumnName::cast)
    }
}
pub struct PrimaryConstraint<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> PrimaryConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::PrimaryConstraint) {
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
    pub fn order(&self) -> Option<Order<N>> {
        self.inner
            .find_children(SqliteTreeKind::Order)
            .flat_map(Order::cast)
            .next()
    }
    pub fn conflict_clause(&self) -> Option<ConflictClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictClause)
            .flat_map(ConflictClause::cast)
            .next()
    }
    pub fn auto_increment(&self) -> Option<N> {
        self.inner
            .find_children(SqliteTokenKind::KW_AUTOINCREMENT)
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct OpIsDistinctFrom<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpIsDistinctFrom<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpIsDistinctFrom) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct FromClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FromClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FromClause) {
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
    pub fn from_clause_value(&self) -> Option<FromClauseValue<N>> {
        self.inner.children().flat_map(FromClauseValue::cast).next()
    }
}
pub struct ColumnConstraintName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ColumnConstraintName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ColumnConstraintName) {
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
    pub fn constraint_name(&self) -> Option<ConstraintName<N>> {
        self.inner
            .find_children(SqliteTreeKind::ConstraintName)
            .flat_map(ConstraintName::cast)
            .next()
    }
}
pub struct SchemaOrIdxOrTableName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> SchemaOrIdxOrTableName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SchemaOrIdxOrTableName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct FullTableFunctionName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FullTableFunctionName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FullTableFunctionName) {
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
    pub fn schema_name(&self) -> Option<SchemaName<N>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn table_func(&self) -> Option<TableFunctionName<N>> {
        self.inner
            .find_children(SqliteTreeKind::TableFunctionName)
            .flat_map(TableFunctionName::cast)
            .next()
    }
}
pub struct ExprList<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ExprList<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprList) {
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
    pub fn items(&self) -> impl Iterator<Item = Expr<N>> + use<'_, 'a, N> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(|it| it.children())
            .flat_map(Expr::cast)
    }
}
pub struct AnyValidName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> AnyValidName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::AnyValidName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct NewColumnName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> NewColumnName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::NewColumnName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct OrderByClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OrderByClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OrderByClause) {
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
    pub fn ordering_term_list(&self) -> Option<OrderingTermList<N>> {
        self.inner
            .find_children(SqliteTreeKind::OrderingTermList)
            .flat_map(OrderingTermList::cast)
            .next()
    }
}
pub struct DropTableStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DropTableStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DropTableStmt) {
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
    pub fn full_table_name(&self) -> Option<FullTableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
}
pub struct CreateTableStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CreateTableStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CreateTableStmt) {
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
    pub fn temporary(&self) -> Option<Temporary<N>> {
        self.inner.children().flat_map(Temporary::cast).next()
    }
    pub fn if_not_exists(&self) -> Option<IfNotExists<N>> {
        self.inner
            .find_children(SqliteTreeKind::IfNotExists)
            .flat_map(IfNotExists::cast)
            .next()
    }
    pub fn full_table_name(&self) -> Option<FullTableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
    pub fn table_columns(&self) -> Option<TableColumns<N>> {
        self.inner.children().flat_map(TableColumns::cast).next()
    }
}
pub struct OpSubtract<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpSubtract<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpSubtract) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct FrameSpecNoOthers<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FrameSpecNoOthers<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecNoOthers) {
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
}
pub struct DefaultConstraintLiteral<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DefaultConstraintLiteral<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DefaultConstraintLiteral) {
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
    pub fn plus_or_minus(&self) -> Option<PlusOrMinus<N>> {
        self.inner.children().flat_map(PlusOrMinus::cast).next()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct TriggerBodyStmtList<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TriggerBodyStmtList<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerBodyStmtList) {
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
    pub fn items(&self) -> impl Iterator<Item = TriggerBodyStmt<N>> + use<'_, 'a, N> {
        self.inner
            .find_children(SqliteTreeKind::TriggerBodyStmt)
            .flat_map(|it| it.children())
            .flat_map(TriggerBodyStmt::cast)
    }
}
pub struct OpBetweenAnd<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpBetweenAnd<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpBetweenAnd) {
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
    pub fn target_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Target)
            .and_then(Expr::cast)
    }
    pub fn low_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Low)
            .and_then(Expr::cast)
    }
    pub fn high_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::High)
            .and_then(Expr::cast)
    }
}
pub struct WindowClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> WindowClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WindowClause) {
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
    pub fn functions(&self) -> impl Iterator<Item = WindowFunction<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(WindowFunction::cast)
    }
}
pub struct TableName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct FrameSpecExcludeClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FrameSpecExcludeClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecExcludeClause) {
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
    pub fn frame_spec_exclude_kind(&self) -> Option<FrameSpecExcludeKind<N>> {
        self.inner
            .children()
            .flat_map(FrameSpecExcludeKind::cast)
            .next()
    }
}
pub struct Statement<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> Statement<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::Statement) {
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
    pub fn explain_clause(&self) -> Option<ExplainClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::ExplainClause)
            .flat_map(ExplainClause::cast)
            .next()
    }
    pub fn statement_kind(&self) -> Option<StatementKind<N>> {
        self.inner.children().flat_map(StatementKind::cast).next()
    }
}
pub struct ValuesSelect<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ValuesSelect<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ValuesSelect) {
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
    pub fn expr_lists(&self) -> impl Iterator<Item = ExprList<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(ExprList::cast)
    }
}
pub struct WindowFunction<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> WindowFunction<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WindowFunction) {
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
    pub fn window_name(&self) -> Option<WindowName<N>> {
        self.inner
            .find_children(SqliteTreeKind::WindowName)
            .flat_map(WindowName::cast)
            .next()
    }
    pub fn window_def(&self) -> Option<WindowDef<N>> {
        self.inner
            .find_children(SqliteTreeKind::WindowDef)
            .flat_map(WindowDef::cast)
            .next()
    }
}
pub struct PragmaName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> PragmaName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::PragmaName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct TypeNameWord<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TypeNameWord<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TypeNameWord) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct FrameSpec<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FrameSpec<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpec) {
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
    pub fn range(&self) -> Option<Range<N>> {
        self.inner.children().flat_map(Range::cast).next()
    }
    pub fn frame_spec_kind(&self) -> Option<FrameSpecKind<N>> {
        self.inner.children().flat_map(FrameSpecKind::cast).next()
    }
    pub fn frame_spec_exclude_clause(&self) -> Option<FrameSpecExcludeClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::FrameSpecExcludeClause)
            .flat_map(FrameSpecExcludeClause::cast)
            .next()
    }
}
pub struct UpsertClauseConflictTarget<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> UpsertClauseConflictTarget<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UpsertClauseConflictTarget) {
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
    pub fn indexed_col_list(&self) -> Option<IndexedColList<N>> {
        self.inner
            .find_children(SqliteTreeKind::IndexedColList)
            .flat_map(IndexedColList::cast)
            .next()
    }
    pub fn where_clause(&self) -> Option<WhereClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
}
pub struct UsingConstraint<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> UsingConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UsingConstraint) {
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
    pub fn col_name_list(&self) -> Option<ColNameList<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColNameList)
            .flat_map(ColNameList::cast)
            .next()
    }
}
pub struct ArgStar<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ArgStar<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ArgStar) {
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
}
pub struct NewTableName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> NewTableName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::NewTableName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct OpConcat<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpConcat<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpConcat) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct TableOptionsList<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableOptionsList<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableOptionsList) {
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
    pub fn items(&self) -> impl Iterator<Item = TableOptions<N>> + use<'_, 'a, N> {
        self.inner
            .find_children(SqliteTreeKind::TableOptions)
            .flat_map(|it| it.children())
            .flat_map(TableOptions::cast)
    }
}
pub struct OpExtractOne<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpExtractOne<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpExtractOne) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct File<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> File<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::File) {
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
    pub fn statements(&self) -> impl Iterator<Item = Statement<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(Statement::cast)
    }
}
pub struct FkClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FkClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkClause) {
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
    pub fn table_name(&self) -> Option<TableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
    pub fn col_name_list(&self) -> Option<ColNameList<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColNameList)
            .flat_map(ColNameList::cast)
            .next()
    }
    pub fn fk_actions(&self) -> impl Iterator<Item = FkViolateAction<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(FkViolateAction::cast)
    }
    pub fn fk_deferrable(&self) -> Option<FkDeferrable<N>> {
        self.inner
            .find_children(SqliteTreeKind::FkDeferrable)
            .flat_map(FkDeferrable::cast)
            .next()
    }
}
pub struct SignedNumber<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> SignedNumber<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SignedNumber) {
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
    pub fn plus_or_minus(&self) -> Option<PlusOrMinus<N>> {
        self.inner.children().flat_map(PlusOrMinus::cast).next()
    }
    pub fn number(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct OpBinComplement<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpBinComplement<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpBinComplement) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct OpNotLike<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpNotLike<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotLike) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
    pub fn escape(&self) -> Option<N> {
        self.inner
            .find_children(SqliteTokenKind::STR_LIT)
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct ConstraintName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ConstraintName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ConstraintName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct DefaultConstraintExpr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DefaultConstraintExpr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DefaultConstraintExpr) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct FullTableName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FullTableName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FullTableName) {
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
    pub fn schema_name(&self) -> Option<SchemaName<N>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn table(&self) -> Option<TableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
}
pub struct WindowName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> WindowName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WindowName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct PragmaValueName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> PragmaValueName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::PragmaValueName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct FrameSpecBetweenClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FrameSpecBetweenClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecBetweenClause) {
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
    pub fn frame_spec_between_left(&self) -> Option<FrameSpecBetweenLeft<N>> {
        self.inner
            .find_children(SqliteTreeKind::FrameSpecBetweenLeft)
            .flat_map(FrameSpecBetweenLeft::cast)
            .next()
    }
    pub fn frame_spec_between_right(&self) -> Option<FrameSpecBetweenRight<N>> {
        self.inner
            .find_children(SqliteTreeKind::FrameSpecBetweenRight)
            .flat_map(FrameSpecBetweenRight::cast)
            .next()
    }
}
pub struct IfNotExists<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> IfNotExists<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::IfNotExists) {
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
}
pub struct ColumnConstraint<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ColumnConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ColumnConstraint) {
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
    pub fn column_constraint_name(&self) -> Option<ColumnConstraintName<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnConstraintName)
            .flat_map(ColumnConstraintName::cast)
            .next()
    }
    pub fn constraint_type(&self) -> Option<ConstraintType<N>> {
        self.inner.children().flat_map(ConstraintType::cast).next()
    }
}
pub struct OpNotNull<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpNotNull<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotNull) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct Collation<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> Collation<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::Collation) {
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
    pub fn collation_name(&self) -> Option<CollationName<N>> {
        self.inner
            .find_children(SqliteTreeKind::CollationName)
            .flat_map(CollationName::cast)
            .next()
    }
}
pub struct DropColumn<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DropColumn<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DropColumn) {
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
    pub fn column_name(&self) -> Option<ColumnName<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnName)
            .flat_map(ColumnName::cast)
            .next()
    }
}
pub struct OrderingTerm<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OrderingTerm<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OrderingTerm) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn collation(&self) -> Option<Collation<N>> {
        self.inner
            .find_children(SqliteTreeKind::Collation)
            .flat_map(Collation::cast)
            .next()
    }
    pub fn order(&self) -> Option<Order<N>> {
        self.inner
            .find_children(SqliteTreeKind::Order)
            .flat_map(Order::cast)
            .next()
    }
    pub fn nulls_position(&self) -> Option<NullsPosition<N>> {
        self.inner.children().flat_map(NullsPosition::cast).next()
    }
}
pub struct ConflictClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ConflictClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ConflictClause) {
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
    pub fn conflict_action(&self) -> Option<ConflictAction<N>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictAction)
            .flat_map(ConflictAction::cast)
            .next()
    }
}
pub struct FkRestrict<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FkRestrict<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkRestrict) {
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
}
pub struct RollbackStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> RollbackStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RollbackStmt) {
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
    pub fn savepoint_name(&self) -> Option<SavepointName<N>> {
        self.inner
            .find_children(SqliteTreeKind::SavepointName)
            .flat_map(SavepointName::cast)
            .next()
    }
}
pub struct CteClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CteClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CteClause) {
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
    pub fn recursive(&self) -> Option<N> {
        self.inner
            .find_children(SqliteTokenKind::KW_RECURSIVE)
            .next()
            .filter(|it| it.token().is_some())
    }
    pub fn expressions(&self) -> impl Iterator<Item = CommonTableExpr<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(CommonTableExpr::cast)
    }
}
pub struct FullPragmaName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FullPragmaName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FullPragmaName) {
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
    pub fn schema_name(&self) -> Option<SchemaName<N>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn pragma(&self) -> Option<PragmaName<N>> {
        self.inner
            .find_children(SqliteTreeKind::PragmaName)
            .flat_map(PragmaName::cast)
            .next()
    }
}
pub struct CollationName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CollationName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CollationName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct CaseWhenClauseList<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CaseWhenClauseList<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CaseWhenClauseList) {
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
    pub fn items(&self) -> impl Iterator<Item = CaseWhenClause<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(CaseWhenClause::cast)
    }
}
pub struct ReIndexStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ReIndexStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ReIndexStmt) {
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
    pub fn target(&self) -> Option<Target<N>> {
        self.inner.children().flat_map(Target::cast).next()
    }
}
pub struct UniqueConstraint<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> UniqueConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UniqueConstraint) {
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
    pub fn conflict_clause(&self) -> Option<ConflictClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictClause)
            .flat_map(ConflictClause::cast)
            .next()
    }
}
pub struct SchemaName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> SchemaName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SchemaName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct OpNotMatch<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpNotMatch<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotMatch) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct ExprSelect<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ExprSelect<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprSelect) {
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
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<N>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
}
pub struct TableConstraint<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableConstraint) {
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
    pub fn constraint_name(&self) -> Option<ConstraintName<N>> {
        self.inner
            .find_children(SqliteTreeKind::ConstraintName)
            .flat_map(ConstraintName::cast)
            .next()
    }
    pub fn table_constraint_kind(&self) -> Option<TableConstraintKind<N>> {
        self.inner
            .children()
            .flat_map(TableConstraintKind::cast)
            .next()
    }
}
pub struct RaiseActionRollBack<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> RaiseActionRollBack<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RaiseActionRollBack) {
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
    pub fn raise_func_err_message(&self) -> Option<RaiseFuncErrMessage<N>> {
        self.inner
            .find_children(SqliteTreeKind::RaiseFuncErrMessage)
            .flat_map(RaiseFuncErrMessage::cast)
            .next()
    }
}
pub struct OpEscape<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpEscape<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpEscape) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct GroupByClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> GroupByClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::GroupByClause) {
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
    pub fn items(&self) -> impl Iterator<Item = Expr<N>> + use<'_, 'a, N> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(|it| it.children())
            .flat_map(Expr::cast)
    }
}
pub struct RaiseActionAbort<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> RaiseActionAbort<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RaiseActionAbort) {
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
    pub fn raise_func_err_message(&self) -> Option<RaiseFuncErrMessage<N>> {
        self.inner
            .find_children(SqliteTreeKind::RaiseFuncErrMessage)
            .flat_map(RaiseFuncErrMessage::cast)
            .next()
    }
}
pub struct ResultColumnTableAll<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ResultColumnTableAll<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ResultColumnTableAll) {
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
    pub fn table_name(&self) -> Option<TableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
}
pub struct DefaultConstraintIden<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DefaultConstraintIden<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DefaultConstraintIden) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .find_children(SqliteTokenKind::IDEN)
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct OpLT<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpLT<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpLT) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct TypeName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TypeName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TypeName) {
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
    pub fn words(&self) -> impl Iterator<Item = TypeNameWord<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(TypeNameWord::cast)
    }
    pub fn lhs_signed_number(&self) -> Option<SignedNumber<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(SignedNumber::cast)
    }
    pub fn rhs_signed_number(&self) -> Option<SignedNumber<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(SignedNumber::cast)
    }
}
pub struct CompoundSelect<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CompoundSelect<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CompoundSelect) {
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
    pub fn compound_operator(&self) -> Option<CompoundOperator<N>> {
        self.inner
            .find_children(SqliteTreeKind::CompoundOperator)
            .flat_map(CompoundOperator::cast)
            .next()
    }
    pub fn select_core(&self) -> Option<SelectCore<N>> {
        self.inner
            .find_children(SqliteTreeKind::SelectCore)
            .flat_map(SelectCore::cast)
            .next()
    }
}
pub struct FkCascade<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FkCascade<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkCascade) {
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
}
pub struct TriggerInsteadOf<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TriggerInsteadOf<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerInsteadOf) {
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
}
pub struct ModuleName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ModuleName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ModuleName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct OpMultiply<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpMultiply<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpMultiply) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct SelectStmtWithCte<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> SelectStmtWithCte<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SelectStmtWithCte) {
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
    pub fn cte_clause(&self) -> Option<CteClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::CteClause)
            .flat_map(CteClause::cast)
            .next()
    }
    pub fn select_stmt(&self) -> Option<SelectStmt<N>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmt)
            .flat_map(SelectStmt::cast)
            .next()
    }
}
pub struct ColumnGenerated<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ColumnGenerated<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ColumnGenerated) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn column_generated_kind(&self) -> Option<ColumnGeneratedKind<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnGeneratedKind)
            .flat_map(ColumnGeneratedKind::cast)
            .next()
    }
}
pub struct OverClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OverClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OverClause) {
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
    pub fn over_clause_kind(&self) -> Option<OverClauseKind<N>> {
        self.inner.children().flat_map(OverClauseKind::cast).next()
    }
}
pub struct OpNotBetweenAnd<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpNotBetweenAnd<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotBetweenAnd) {
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
    pub fn target_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Target)
            .and_then(Expr::cast)
    }
    pub fn low_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Low)
            .and_then(Expr::cast)
    }
    pub fn high_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::High)
            .and_then(Expr::cast)
    }
}
pub struct InTable<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> InTable<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InTable) {
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
    pub fn full_table_name(&self) -> Option<FullTableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
}
pub struct CommitStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CommitStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CommitStmt) {
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
    pub fn commit_start_kw(&self) -> Option<CommitStartKw<N>> {
        self.inner.children().flat_map(CommitStartKw::cast).next()
    }
}
pub struct DropViewStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DropViewStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DropViewStmt) {
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
    pub fn full_view_name(&self) -> Option<FullViewName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullViewName)
            .flat_map(FullViewName::cast)
            .next()
    }
}
pub struct LimitClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> LimitClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::LimitClause) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn offset(&self) -> Option<Offset<N>> {
        self.inner
            .find_children(SqliteTreeKind::Offset)
            .flat_map(Offset::cast)
            .next()
    }
}
pub struct ExplainClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ExplainClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExplainClause) {
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
}
pub struct ResultColumnAll<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ResultColumnAll<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ResultColumnAll) {
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
}
pub struct HavingClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> HavingClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::HavingClause) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct ReleaseStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ReleaseStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ReleaseStmt) {
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
    pub fn savepoint_name(&self) -> Option<SavepointName<N>> {
        self.inner
            .find_children(SqliteTreeKind::SavepointName)
            .flat_map(SavepointName::cast)
            .next()
    }
}
pub struct UpsertClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> UpsertClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UpsertClause) {
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
    pub fn upsert_clause_conflict_target(&self) -> Option<UpsertClauseConflictTarget<N>> {
        self.inner
            .find_children(SqliteTreeKind::UpsertClauseConflictTarget)
            .flat_map(UpsertClauseConflictTarget::cast)
            .next()
    }
    pub fn upsert_clause_action(&self) -> Option<UpsertClauseAction<N>> {
        self.inner
            .children()
            .flat_map(UpsertClauseAction::cast)
            .next()
    }
}
pub struct InsertDefaultValuesClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> InsertDefaultValuesClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InsertDefaultValuesClause) {
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
}
pub struct OpBinRShift<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpBinRShift<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpBinRShift) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct TriggerWhenExpr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TriggerWhenExpr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerWhenExpr) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct IndexedColList<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> IndexedColList<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::IndexedColList) {
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
    pub fn items(&self) -> impl Iterator<Item = IndexedCol<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(IndexedCol::cast)
    }
}
pub struct OpIsNot<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpIsNot<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpIsNot) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct TableDetails<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableDetails<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableDetails) {
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
    pub fn columns(&self) -> impl Iterator<Item = ColumnDef<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(ColumnDef::cast)
    }
    pub fn table_constraints(&self) -> impl Iterator<Item = TableConstraint<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(TableConstraint::cast)
    }
    pub fn table_options_list(&self) -> Option<TableOptionsList<N>> {
        self.inner
            .find_children(SqliteTreeKind::TableOptionsList)
            .flat_map(TableOptionsList::cast)
            .next()
    }
}
pub struct TablePkConstraint<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TablePkConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TablePkConstraint) {
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
    pub fn columns(&self) -> impl Iterator<Item = IndexedCol<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(IndexedCol::cast)
    }
    pub fn auto_increment(&self) -> Option<N> {
        self.inner
            .find_children(SqliteTokenKind::KW_AUTOINCREMENT)
            .next()
            .filter(|it| it.token().is_some())
    }
    pub fn conflict_clause(&self) -> Option<ConflictClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictClause)
            .flat_map(ConflictClause::cast)
            .next()
    }
}
pub struct UpdateStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> UpdateStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UpdateStmt) {
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
    pub fn conflict_action(&self) -> Option<ConflictAction<N>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictAction)
            .flat_map(ConflictAction::cast)
            .next()
    }
    pub fn qualified_table_name(&self) -> Option<QualifiedTableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::QualifiedTableName)
            .flat_map(QualifiedTableName::cast)
            .next()
    }
    pub fn set_expressions(&self) -> impl Iterator<Item = SetColumnExpr<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(SetColumnExpr::cast)
    }
    pub fn from_clause(&self) -> Option<FromClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::FromClause)
            .flat_map(FromClause::cast)
            .next()
    }
    pub fn where_clause(&self) -> Option<WhereClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
    pub fn returning_clause(&self) -> Option<ReturningClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::ReturningClause)
            .flat_map(ReturningClause::cast)
            .next()
    }
    pub fn update_stmt_limited(&self) -> Option<UpdateStmtLimited<N>> {
        self.inner
            .find_children(SqliteTreeKind::UpdateStmtLimited)
            .flat_map(UpdateStmtLimited::cast)
            .next()
    }
}
pub struct OpAnd<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpAnd<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpAnd) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct FunctionName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FunctionName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FunctionName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct UnionCompoundOperator<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> UnionCompoundOperator<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UnionCompoundOperator) {
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
    pub fn all(&self) -> Option<N> {
        self.inner
            .find_children(SqliteTokenKind::KW_ALL)
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct ExprExistsSelect<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ExprExistsSelect<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprExistsSelect) {
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
    pub fn not(&self) -> Option<N> {
        self.inner
            .find_children(SqliteTokenKind::KW_NOT)
            .next()
            .filter(|it| it.token().is_some())
    }
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<N>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
}
pub struct CommonTableExpr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CommonTableExpr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CommonTableExpr) {
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
    pub fn cte_name(&self) -> Option<CteName<N>> {
        self.inner
            .find_children(SqliteTreeKind::CteName)
            .flat_map(CteName::cast)
            .next()
    }
    pub fn col_name_list(&self) -> Option<ColNameList<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColNameList)
            .flat_map(ColNameList::cast)
            .next()
    }
    pub fn materialized_cte(&self) -> Option<MaterializedCte<N>> {
        self.inner
            .find_children(SqliteTreeKind::MaterializedCte)
            .flat_map(MaterializedCte::cast)
            .next()
    }
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<N>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
}
pub struct OpAdd<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpAdd<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpAdd) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct BeginStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> BeginStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::BeginStmt) {
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
    pub fn begin_stmt_kind(&self) -> Option<BeginStmtKind<N>> {
        self.inner.children().flat_map(BeginStmtKind::cast).next()
    }
}
pub struct FkDeferrable<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FkDeferrable<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkDeferrable) {
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
    pub fn not(&self) -> Option<N> {
        self.inner
            .find_children(SqliteTokenKind::KW_NOT)
            .next()
            .filter(|it| it.token().is_some())
    }
    pub fn defer_kind(&self) -> Option<DeferKind<N>> {
        self.inner.children().flat_map(DeferKind::cast).next()
    }
}
pub struct CreateViewStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CreateViewStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CreateViewStmt) {
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
    pub fn temporary(&self) -> Option<Temporary<N>> {
        self.inner.children().flat_map(Temporary::cast).next()
    }
    pub fn if_not_exists(&self) -> Option<IfNotExists<N>> {
        self.inner
            .find_children(SqliteTreeKind::IfNotExists)
            .flat_map(IfNotExists::cast)
            .next()
    }
    pub fn full_view_name(&self) -> Option<FullViewName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullViewName)
            .flat_map(FullViewName::cast)
            .next()
    }
    pub fn col_name_list(&self) -> Option<ColNameList<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColNameList)
            .flat_map(ColNameList::cast)
            .next()
    }
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<N>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
}
pub struct ExprParen<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ExprParen<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprParen) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct TableNameNotIndexed<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableNameNotIndexed<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableNameNotIndexed) {
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
}
pub struct CheckConstraint<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CheckConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CheckConstraint) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct CaseWhenClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CaseWhenClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CaseWhenClause) {
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
    pub fn when_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::When)
            .and_then(Expr::cast)
    }
    pub fn then_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Then)
            .and_then(Expr::cast)
    }
}
pub struct OpLike<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpLike<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpLike) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
    pub fn op_escape(&self) -> Option<OpEscape<N>> {
        self.inner
            .find_children(SqliteTreeKind::OpEscape)
            .flat_map(OpEscape::cast)
            .next()
    }
}
pub struct OpRegexp<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpRegexp<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpRegexp) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct ExprFunc<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ExprFunc<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprFunc) {
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
    pub fn function_name(&self) -> Option<FunctionName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FunctionName)
            .flat_map(FunctionName::cast)
            .next()
    }
    pub fn func_arguments(&self) -> Option<FuncArguments<N>> {
        self.inner
            .find_children(SqliteTreeKind::FuncArguments)
            .flat_map(FuncArguments::cast)
            .next()
    }
    pub fn filter_clause(&self) -> Option<FilterClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::FilterClause)
            .flat_map(FilterClause::cast)
            .next()
    }
    pub fn over_clause(&self) -> Option<OverClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::OverClause)
            .flat_map(OverClause::cast)
            .next()
    }
}
pub struct RaiseActionFail<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> RaiseActionFail<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RaiseActionFail) {
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
    pub fn raise_func_err_message(&self) -> Option<RaiseFuncErrMessage<N>> {
        self.inner
            .find_children(SqliteTreeKind::RaiseFuncErrMessage)
            .flat_map(RaiseFuncErrMessage::cast)
            .next()
    }
}
pub struct FullViewName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FullViewName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FullViewName) {
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
    pub fn schema_name(&self) -> Option<SchemaName<N>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn view_name(&self) -> Option<ViewName<N>> {
        self.inner
            .find_children(SqliteTreeKind::ViewName)
            .flat_map(ViewName::cast)
            .next()
    }
}
pub struct ReturningClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ReturningClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ReturningClause) {
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
    pub fn columns(&self) -> impl Iterator<Item = ReturningClauseKind<N>> + use<'_, 'a, N> {
        self.inner
            .find_children(SqliteTreeKind::ReturningClauseKind)
            .flat_map(|it| it.children())
            .flat_map(ReturningClauseKind::cast)
    }
}
pub struct OpNotRegexp<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpNotRegexp<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotRegexp) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct FrameSpecUnboundedPreceding<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FrameSpecUnboundedPreceding<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecUnboundedPreceding) {
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
}
pub struct InTableFunc<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> InTableFunc<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InTableFunc) {
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
    pub fn full_table_function_name(&self) -> Option<FullTableFunctionName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableFunctionName)
            .flat_map(FullTableFunctionName::cast)
            .next()
    }
    pub fn items(&self) -> impl Iterator<Item = Expr<N>> + use<'_, 'a, N> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(|it| it.children())
            .flat_map(Expr::cast)
    }
}
pub struct ResultColumnExpr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ResultColumnExpr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ResultColumnExpr) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn with_alias(&self) -> Option<WithAlias<N>> {
        self.inner
            .find_children(SqliteTreeKind::WithAlias)
            .flat_map(WithAlias::cast)
            .next()
    }
}
pub struct RaiseFuncErrMessage<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> RaiseFuncErrMessage<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RaiseFuncErrMessage) {
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
}
pub struct OpModulus<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpModulus<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpModulus) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct OpEq<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpEq<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpEq) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct VacuumStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> VacuumStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::VacuumStmt) {
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
    pub fn schema_name(&self) -> Option<SchemaName<N>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct OpIsNull<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpIsNull<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpIsNull) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct AttachDbStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> AttachDbStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::AttachDbStmt) {
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
    pub fn file_name_expr(&self) -> Option<FileNameExpr<N>> {
        self.inner
            .find_children(SqliteTreeKind::FileNameExpr)
            .flat_map(FileNameExpr::cast)
            .next()
    }
    pub fn schema_name_expr(&self) -> Option<SchemaNameExpr<N>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaNameExpr)
            .flat_map(SchemaNameExpr::cast)
            .next()
    }
    pub fn password_expr(&self) -> Option<PasswordExpr<N>> {
        self.inner
            .find_children(SqliteTreeKind::PasswordExpr)
            .flat_map(PasswordExpr::cast)
            .next()
    }
}
pub struct OpNotSpaceNull<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpNotSpaceNull<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotSpaceNull) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct DefaultConstraint<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DefaultConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DefaultConstraint) {
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
    pub fn default_value_kind(&self) -> Option<DefaultValueKind<N>> {
        self.inner
            .children()
            .flat_map(DefaultValueKind::cast)
            .next()
    }
}
pub struct OpNot<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpNot<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNot) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct TableOrIdxNameWithSchema<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableOrIdxNameWithSchema<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableOrIdxNameWithSchema) {
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
    pub fn schema_name(&self) -> Option<SchemaName<N>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn table_or_index_name(&self) -> Option<TableOrIndexName<N>> {
        self.inner
            .find_children(SqliteTreeKind::TableOrIndexName)
            .flat_map(TableOrIndexName::cast)
            .next()
    }
}
pub struct TableOrIdxOrCollationName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableOrIdxOrCollationName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableOrIdxOrCollationName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct CaseElseClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CaseElseClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CaseElseClause) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct OpExtractTwo<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpExtractTwo<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpExtractTwo) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct ExprBindParam<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ExprBindParam<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprBindParam) {
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
}
pub struct DropTriggerStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DropTriggerStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DropTriggerStmt) {
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
    pub fn full_trigger_name(&self) -> Option<FullTriggerName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullTriggerName)
            .flat_map(FullTriggerName::cast)
            .next()
    }
}
pub struct CreateVirtualTableStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CreateVirtualTableStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CreateVirtualTableStmt) {
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
    pub fn if_not_exists(&self) -> Option<IfNotExists<N>> {
        self.inner
            .find_children(SqliteTreeKind::IfNotExists)
            .flat_map(IfNotExists::cast)
            .next()
    }
    pub fn full_table_name(&self) -> Option<FullTableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
    pub fn module_name(&self) -> Option<ModuleName<N>> {
        self.inner
            .find_children(SqliteTreeKind::ModuleName)
            .flat_map(ModuleName::cast)
            .next()
    }
    pub fn module_arg_list(&self) -> Option<ModuleArgList<N>> {
        self.inner
            .find_children(SqliteTreeKind::ModuleArgList)
            .flat_map(ModuleArgList::cast)
            .next()
    }
}
pub struct DeleteStmtLimited<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DeleteStmtLimited<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DeleteStmtLimited) {
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
    pub fn order_by_clause(&self) -> Option<OrderByClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::OrderByClause)
            .flat_map(OrderByClause::cast)
            .next()
    }
    pub fn limit_clause(&self) -> Option<LimitClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::LimitClause)
            .flat_map(LimitClause::cast)
            .next()
    }
}
pub struct OpOr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpOr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpOr) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct OpNotIn<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpNotIn<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotIn) {
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
    pub fn lhs(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn in_expr_kind(&self) -> Option<InExprKind<N>> {
        self.inner.children().flat_map(InExprKind::cast).next()
    }
}
pub struct RenameTable<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> RenameTable<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RenameTable) {
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
    pub fn table_name(&self) -> Option<TableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
}
pub struct OpIs<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpIs<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpIs) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct OpMatch<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpMatch<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpMatch) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct FkOnAction<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FkOnAction<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkOnAction) {
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
    pub fn fk_fail_kind(&self) -> Option<FkFailKind<N>> {
        self.inner.children().flat_map(FkFailKind::cast).next()
    }
    pub fn fk_action(&self) -> Option<FkAction<N>> {
        self.inner.children().flat_map(FkAction::cast).next()
    }
}
pub struct UpdateStmtLimited<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> UpdateStmtLimited<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UpdateStmtLimited) {
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
    pub fn order_by_clause(&self) -> Option<OrderByClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::OrderByClause)
            .flat_map(OrderByClause::cast)
            .next()
    }
    pub fn limit_clause(&self) -> Option<LimitClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::LimitClause)
            .flat_map(LimitClause::cast)
            .next()
    }
}
pub struct InsertOrAction<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> InsertOrAction<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InsertOrAction) {
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
    pub fn conflict_action(&self) -> Option<ConflictAction<N>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictAction)
            .flat_map(ConflictAction::cast)
            .next()
    }
}
pub struct CteName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> CteName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CteName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct NullConstraint<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> NullConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::NullConstraint) {
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
    pub fn is_not_null(&self) -> Option<N> {
        self.inner
            .find_children(SqliteTokenKind::KW_NOT)
            .next()
            .filter(|it| it.token().is_some())
    }
    pub fn conflict_clause(&self) -> Option<ConflictClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictClause)
            .flat_map(ConflictClause::cast)
            .next()
    }
}
pub struct AddColumn<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> AddColumn<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::AddColumn) {
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
    pub fn column_def(&self) -> Option<ColumnDef<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnDef)
            .flat_map(ColumnDef::cast)
            .next()
    }
}
pub struct AliasName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> AliasName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::AliasName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct WindowDef<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> WindowDef<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WindowDef) {
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
    pub fn window_base_name(&self) -> Option<WindowBaseName<N>> {
        self.inner
            .find_children(SqliteTreeKind::WindowBaseName)
            .flat_map(WindowBaseName::cast)
            .next()
    }
    pub fn window_partition_by_clause(&self) -> Option<WindowPartitionByClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::WindowPartitionByClause)
            .flat_map(WindowPartitionByClause::cast)
            .next()
    }
    pub fn order_by_clause(&self) -> Option<OrderByClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::OrderByClause)
            .flat_map(OrderByClause::cast)
            .next()
    }
    pub fn frame_spec(&self) -> Option<FrameSpec<N>> {
        self.inner
            .find_children(SqliteTreeKind::FrameSpec)
            .flat_map(FrameSpec::cast)
            .next()
    }
}
pub struct OrderingTermList<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OrderingTermList<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OrderingTermList) {
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
    pub fn items(&self) -> impl Iterator<Item = OrderingTerm<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(OrderingTerm::cast)
    }
}
pub struct OpGlob<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpGlob<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpGlob) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct OpDivide<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpDivide<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpDivide) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct OpBinOr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpBinOr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpBinOr) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct TableUqConstraint<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TableUqConstraint<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableUqConstraint) {
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
    pub fn indexed_col_list(&self) -> Option<IndexedColList<N>> {
        self.inner
            .find_children(SqliteTreeKind::IndexedColList)
            .flat_map(IndexedColList::cast)
            .next()
    }
    pub fn conflict_clause(&self) -> Option<ConflictClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictClause)
            .flat_map(ConflictClause::cast)
            .next()
    }
}
pub struct DetachStmt<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DetachStmt<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DetachStmt) {
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
    pub fn db_name_expr(&self) -> Option<DbNameExpr<N>> {
        self.inner
            .find_children(SqliteTreeKind::DbNameExpr)
            .flat_map(DbNameExpr::cast)
            .next()
    }
}
pub struct FrameSpecPreceding<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FrameSpecPreceding<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecPreceding) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct ExprColumnName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ExprColumnName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprColumnName) {
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
    pub fn schema_name(&self) -> Option<SchemaName<N>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn table_name(&self) -> Option<TableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
    pub fn column_name(&self) -> Option<ColumnName<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnName)
            .flat_map(ColumnName::cast)
            .next()
    }
}
pub struct ResultColumnList<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ResultColumnList<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ResultColumnList) {
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
    pub fn items(&self) -> impl Iterator<Item = ResultColumn<N>> + use<'_, 'a, N> {
        self.inner
            .find_children(SqliteTreeKind::ResultColumn)
            .flat_map(|it| it.children())
            .flat_map(ResultColumn::cast)
    }
}
pub struct RaiseFunc<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> RaiseFunc<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RaiseFunc) {
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
    pub fn raise_action(&self) -> Option<RaiseAction<N>> {
        self.inner
            .find_children(SqliteTreeKind::RaiseAction)
            .flat_map(RaiseAction::cast)
            .next()
    }
}
pub struct InsertSelectClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> InsertSelectClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InsertSelectClause) {
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
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<N>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
    pub fn upsert_clauses(&self) -> impl Iterator<Item = UpsertClause<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(UpsertClause::cast)
    }
}
pub struct UpsertDoUpdate<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> UpsertDoUpdate<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UpsertDoUpdate) {
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
    pub fn set_expressions(&self) -> impl Iterator<Item = SetColumnExpr<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(SetColumnExpr::cast)
    }
    pub fn where_clause(&self) -> Option<WhereClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
}
pub struct ValuesClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ValuesClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ValuesClause) {
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
    pub fn expr_list(&self) -> Option<ExprList<N>> {
        self.inner
            .find_children(SqliteTreeKind::ExprList)
            .flat_map(ExprList::cast)
            .next()
    }
}
pub struct ArgExpr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ArgExpr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ArgExpr) {
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
    pub fn distinct(&self) -> Option<N> {
        self.inner
            .find_children(SqliteTokenKind::KW_DISTINCT)
            .next()
            .filter(|it| it.token().is_some())
    }
    pub fn items(&self) -> impl Iterator<Item = Expr<N>> + use<'_, 'a, N> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(|it| it.children())
            .flat_map(Expr::cast)
    }
    pub fn order_by_clause(&self) -> Option<OrderByClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::OrderByClause)
            .flat_map(OrderByClause::cast)
            .next()
    }
}
pub struct OpNotEq<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpNotEq<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotEq) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct FkViolateAction<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FkViolateAction<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkViolateAction) {
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
    pub fn fk_on_or_match(&self) -> Option<FkOnOrMatch<N>> {
        self.inner.children().flat_map(FkOnOrMatch::cast).next()
    }
}
pub struct FkNoAction<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FkNoAction<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkNoAction) {
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
}
pub struct OpBinLShift<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpBinLShift<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpBinLShift) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct TriggerForEachRow<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TriggerForEachRow<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerForEachRow) {
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
}
pub struct EmptyableExprList<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> EmptyableExprList<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::EmptyableExprList) {
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
    pub fn items(&self) -> impl Iterator<Item = Expr<N>> + use<'_, 'a, N> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(|it| it.children())
            .flat_map(Expr::cast)
    }
}
pub struct InsertValuesClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> InsertValuesClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InsertValuesClause) {
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
    pub fn expr_lists(&self) -> impl Iterator<Item = ExprList<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(ExprList::cast)
    }
    pub fn upsert_clauses(&self) -> impl Iterator<Item = UpsertClause<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(UpsertClause::cast)
    }
}
pub struct RenameColumn<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> RenameColumn<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RenameColumn) {
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
    pub fn old_name(&self) -> Option<ColumnName<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnName)
            .flat_map(ColumnName::cast)
            .next()
    }
    pub fn new_name(&self) -> Option<NewColumnName<N>> {
        self.inner
            .find_children(SqliteTreeKind::NewColumnName)
            .flat_map(NewColumnName::cast)
            .next()
    }
}
pub struct StatementWithCte<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> StatementWithCte<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::StatementWithCte) {
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
    pub fn cte_clause(&self) -> Option<CteClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::CteClause)
            .flat_map(CteClause::cast)
            .next()
    }
    pub fn cte_prependable(&self) -> Option<CtePrependable<N>> {
        self.inner.children().flat_map(CtePrependable::cast).next()
    }
}
pub struct FrameSpecFollowing<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FrameSpecFollowing<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecFollowing) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct DbNameExpr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> DbNameExpr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DbNameExpr) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct FullIndexName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FullIndexName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FullIndexName) {
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
    pub fn schema_name(&self) -> Option<SchemaName<N>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn index_name(&self) -> Option<IndexName<N>> {
        self.inner
            .find_children(SqliteTreeKind::IndexName)
            .flat_map(IndexName::cast)
            .next()
    }
}
pub struct QualifiedTableName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> QualifiedTableName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::QualifiedTableName) {
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
    pub fn full_table_name(&self) -> Option<FullTableName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
    pub fn with_alias(&self) -> Option<WithAlias<N>> {
        self.inner
            .find_children(SqliteTreeKind::WithAlias)
            .flat_map(WithAlias::cast)
            .next()
    }
    pub fn index_details(&self) -> Option<IndexDetails<N>> {
        self.inner.children().flat_map(IndexDetails::cast).next()
    }
}
pub struct OpGT<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpGT<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpGT) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct OpUnaryMinus<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpUnaryMinus<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpUnaryMinus) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct ColumnDef<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ColumnDef<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ColumnDef) {
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
    pub fn column_name(&self) -> Option<ColumnName<N>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnName)
            .flat_map(ColumnName::cast)
            .next()
    }
    pub fn type_name(&self) -> Option<TypeName<N>> {
        self.inner
            .find_children(SqliteTreeKind::TypeName)
            .flat_map(TypeName::cast)
            .next()
    }
    pub fn constraints(&self) -> impl Iterator<Item = ColumnConstraint<N>> + use<'_, 'a, N> {
        self.inner.valid_children().flat_map(ColumnConstraint::cast)
    }
}
pub struct SavepointName<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> SavepointName<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SavepointName) {
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
    pub fn text(&'a self) -> &'a str {
        self.inner
            .valid_children()
            .find_map(|it| it.token().map(|it| it.text.as_str()))
            .unwrap()
    }
    pub fn value(&self) -> Option<N> {
        self.inner
            .valid_children()
            .next()
            .filter(|it| it.token().is_some())
    }
}
pub struct TriggerUpdateAction<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> TriggerUpdateAction<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerUpdateAction) {
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
    pub fn trigger_update_affect_cols(&self) -> Option<TriggerUpdateAffectCols<N>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerUpdateAffectCols)
            .flat_map(TriggerUpdateAffectCols::cast)
            .next()
    }
}
pub struct OpBinAnd<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> OpBinAnd<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpBinAnd) {
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
    pub fn lhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Lhs)
            .and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_child_by_tag(SqliteTreeTag::Rhs)
            .and_then(Expr::cast)
    }
}
pub struct FileNameExpr<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FileNameExpr<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FileNameExpr) {
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
    pub fn expr(&self) -> Option<Expr<N>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct FkSetNull<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FkSetNull<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkSetNull) {
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
}
pub struct ExprCase<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ExprCase<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprCase) {
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
    pub fn case_target_expr(&self) -> Option<CaseTargetExpr<N>> {
        self.inner
            .find_children(SqliteTreeKind::CaseTargetExpr)
            .flat_map(CaseTargetExpr::cast)
            .next()
    }
    pub fn case_when_clause_list(&self) -> Option<CaseWhenClauseList<N>> {
        self.inner
            .find_children(SqliteTreeKind::CaseWhenClauseList)
            .flat_map(CaseWhenClauseList::cast)
            .next()
    }
    pub fn case_else_clause(&self) -> Option<CaseElseClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::CaseElseClause)
            .flat_map(CaseElseClause::cast)
            .next()
    }
}
pub struct ModuleArgList<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> ModuleArgList<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ModuleArgList) {
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
    pub fn items(&self) -> impl Iterator<Item = ModuleArg<N>> + use<'_, 'a, N> {
        self.inner
            .find_children(SqliteTreeKind::ModuleArg)
            .flat_map(|it| it.children())
            .flat_map(ModuleArg::cast)
    }
}
pub struct FromClauseTableValueFunction<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FromClauseTableValueFunction<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FromClauseTableValueFunction) {
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
    pub fn full_table_function_name(&self) -> Option<FullTableFunctionName<N>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableFunctionName)
            .flat_map(FullTableFunctionName::cast)
            .next()
    }
    pub fn emptyable_expr_list(&self) -> Option<EmptyableExprList<N>> {
        self.inner
            .find_children(SqliteTreeKind::EmptyableExprList)
            .flat_map(EmptyableExprList::cast)
            .next()
    }
}
pub struct FilterClause<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FilterClause<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FilterClause) {
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
    pub fn where_clause(&self) -> Option<WhereClause<N>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
}
pub struct FkSetDefault<N> {
    pub inner: N,
}
impl<'a, N: CstNodeTrait<'a>> FkSetDefault<N> {
    pub fn cast(node: N) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkSetDefault) {
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
}

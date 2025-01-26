#![allow(non_camel_case_types)]
use super::manual::*;
pub use crate::{CstNode, CstNodeData, NodeId, SqliteToken, SqliteTokenKind, SqliteTreeKind};
pub enum RaiseAction<'a> {
    KW_IGNORE(&'a SqliteToken),
    RaiseActionRollBack(RaiseActionRollBack<'a>),
    RaiseActionAbort(RaiseActionAbort<'a>),
    RaiseActionFail(RaiseActionFail<'a>),
}
impl<'a> RaiseAction<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        if node.tree() != Some(TreeKind::RaiseAction) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_IGNORE,
                        ..
                    },
                ) => return Some(Self::KW_IGNORE(&token)),
                CstNodeData::Tree(TreeKind::RaiseActionRollBack) => {
                    return Some(Self::RaiseActionRollBack(RaiseActionRollBack::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::RaiseActionAbort) => {
                    return Some(Self::RaiseActionAbort(RaiseActionAbort::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::RaiseActionFail) => {
                    return Some(Self::RaiseActionFail(RaiseActionFail::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn kw_ignore(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_IGNORE(item) => Some(item),
            _ => None,
        }
    }
    pub fn raise_action_roll_back(self) -> Option<RaiseActionRollBack<'a>> {
        match self {
            Self::RaiseActionRollBack(item) => Some(item),
            _ => None,
        }
    }
    pub fn raise_action_abort(self) -> Option<RaiseActionAbort<'a>> {
        match self {
            Self::RaiseActionAbort(item) => Some(item),
            _ => None,
        }
    }
    pub fn raise_action_fail(self) -> Option<RaiseActionFail<'a>> {
        match self {
            Self::RaiseActionFail(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FrameSpecExcludeKind<'a> {
    FrameSpecNoOthers(FrameSpecNoOthers<'a>),
    FrameSpecCurrentRow(FrameSpecCurrentRow<'a>),
    KW_GROUP(&'a SqliteToken),
    KW_TIES(&'a SqliteToken),
}
impl<'a> FrameSpecExcludeKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        match node.data {
            CstNodeData::Tree(TreeKind::FrameSpecNoOthers) => {
                return Some(Self::FrameSpecNoOthers(FrameSpecNoOthers::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::FrameSpecCurrentRow) => {
                return Some(Self::FrameSpecCurrentRow(FrameSpecCurrentRow::cast(node)?));
            }
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_GROUP,
                    ..
                },
            ) => {
                return Some(Self::KW_GROUP(&token));
            }
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_TIES,
                    ..
                },
            ) => {
                return Some(Self::KW_TIES(&token));
            }
            _ => return None,
        }
    }
    pub fn frame_spec_no_others(self) -> Option<FrameSpecNoOthers<'a>> {
        match self {
            Self::FrameSpecNoOthers(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_current_row(self) -> Option<FrameSpecCurrentRow<'a>> {
        match self {
            Self::FrameSpecCurrentRow(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_group(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_GROUP(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_ties(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_TIES(item) => Some(item),
            _ => None,
        }
    }
}
pub enum StatementKind<'a> {
    StatementNoCte(StatementNoCte<'a>),
    StatementWithCte(StatementWithCte<'a>),
}
impl<'a> StatementKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::StatementNoCte) => {
                return Some(Self::StatementNoCte(StatementNoCte::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::StatementWithCte) => {
                return Some(Self::StatementWithCte(StatementWithCte::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn statement_no_cte(self) -> Option<StatementNoCte<'a>> {
        match self {
            Self::StatementNoCte(item) => Some(item),
            _ => None,
        }
    }
    pub fn statement_with_cte(self) -> Option<StatementWithCte<'a>> {
        match self {
            Self::StatementWithCte(item) => Some(item),
            _ => None,
        }
    }
}
pub enum TableConstraintKind<'a> {
    TablePkConstraint(TablePkConstraint<'a>),
    TableUqConstraint(TableUqConstraint<'a>),
    CheckConstraint(CheckConstraint<'a>),
    TableFkConstraint(TableFkConstraint<'a>),
}
impl<'a> TableConstraintKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::TablePkConstraint) => {
                return Some(Self::TablePkConstraint(TablePkConstraint::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::TableUqConstraint) => {
                return Some(Self::TableUqConstraint(TableUqConstraint::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::CheckConstraint) => {
                return Some(Self::CheckConstraint(CheckConstraint::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::TableFkConstraint) => {
                return Some(Self::TableFkConstraint(TableFkConstraint::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn table_pk_constraint(self) -> Option<TablePkConstraint<'a>> {
        match self {
            Self::TablePkConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn table_uq_constraint(self) -> Option<TableUqConstraint<'a>> {
        match self {
            Self::TableUqConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn check_constraint(self) -> Option<CheckConstraint<'a>> {
        match self {
            Self::CheckConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn table_fk_constraint(self) -> Option<TableFkConstraint<'a>> {
        match self {
            Self::TableFkConstraint(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ColumnGeneratedKind<'a> {
    KW_STORED(&'a SqliteToken),
    KW_VIRTUAL(&'a SqliteToken),
}
impl<'a> ColumnGeneratedKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        if node.tree() != Some(TreeKind::ColumnGeneratedKind) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_STORED,
                        ..
                    },
                ) => return Some(Self::KW_STORED(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_VIRTUAL,
                        ..
                    },
                ) => return Some(Self::KW_VIRTUAL(&token)),
                _ => return None,
            })
            .next()
    }
    pub fn kw_stored(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_STORED(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_virtual(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_VIRTUAL(item) => Some(item),
            _ => None,
        }
    }
}
pub enum NonCommaJoin<'a> {
    CrossJoin(CrossJoin<'a>),
    OuterJoin(OuterJoin<'a>),
    InnerJoin(InnerJoin<'a>),
    NaturalJoin(NaturalJoin<'a>),
    Join(Join<'a>),
}
impl<'a> NonCommaJoin<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::NonCommaJoin) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::CrossJoin) => {
                    return Some(Self::CrossJoin(CrossJoin::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OuterJoin) => {
                    return Some(Self::OuterJoin(OuterJoin::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::InnerJoin) => {
                    return Some(Self::InnerJoin(InnerJoin::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::NaturalJoin) => {
                    return Some(Self::NaturalJoin(NaturalJoin::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::Join) => {
                    return Some(Self::Join(Join::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn cross_join(self) -> Option<CrossJoin<'a>> {
        match self {
            Self::CrossJoin(item) => Some(item),
            _ => None,
        }
    }
    pub fn outer_join(self) -> Option<OuterJoin<'a>> {
        match self {
            Self::OuterJoin(item) => Some(item),
            _ => None,
        }
    }
    pub fn inner_join(self) -> Option<InnerJoin<'a>> {
        match self {
            Self::InnerJoin(item) => Some(item),
            _ => None,
        }
    }
    pub fn natural_join(self) -> Option<NaturalJoin<'a>> {
        match self {
            Self::NaturalJoin(item) => Some(item),
            _ => None,
        }
    }
    pub fn join(self) -> Option<Join<'a>> {
        match self {
            Self::Join(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ExprPostfix<'a> {
    OpNotSpaceNull(OpNotSpaceNull<'a>),
    OpCollate(OpCollate<'a>),
    OpNotNull(OpNotNull<'a>),
    OpIsNull(OpIsNull<'a>),
}
impl<'a> ExprPostfix<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::ExprPostfix) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::OpNotSpaceNull) => {
                    return Some(Self::OpNotSpaceNull(OpNotSpaceNull::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpCollate) => {
                    return Some(Self::OpCollate(OpCollate::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpNotNull) => {
                    return Some(Self::OpNotNull(OpNotNull::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpIsNull) => {
                    return Some(Self::OpIsNull(OpIsNull::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn op_not_space_null(self) -> Option<OpNotSpaceNull<'a>> {
        match self {
            Self::OpNotSpaceNull(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_collate(self) -> Option<OpCollate<'a>> {
        match self {
            Self::OpCollate(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_null(self) -> Option<OpNotNull<'a>> {
        match self {
            Self::OpNotNull(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_is_null(self) -> Option<OpIsNull<'a>> {
        match self {
            Self::OpIsNull(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ResultColumn<'a> {
    ResultColumnExpr(ResultColumnExpr<'a>),
    ResultColumnAll(ResultColumnAll<'a>),
    ResultColumnTableAll(ResultColumnTableAll<'a>),
}
impl<'a> ResultColumn<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::ResultColumn) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::ResultColumnExpr) => {
                    return Some(Self::ResultColumnExpr(ResultColumnExpr::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ResultColumnAll) => {
                    return Some(Self::ResultColumnAll(ResultColumnAll::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ResultColumnTableAll) => {
                    return Some(Self::ResultColumnTableAll(ResultColumnTableAll::cast(
                        child,
                    )?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn result_column_expr(self) -> Option<ResultColumnExpr<'a>> {
        match self {
            Self::ResultColumnExpr(item) => Some(item),
            _ => None,
        }
    }
    pub fn result_column_all(self) -> Option<ResultColumnAll<'a>> {
        match self {
            Self::ResultColumnAll(item) => Some(item),
            _ => None,
        }
    }
    pub fn result_column_table_all(self) -> Option<ResultColumnTableAll<'a>> {
        match self {
            Self::ResultColumnTableAll(item) => Some(item),
            _ => None,
        }
    }
}
pub enum Temporary<'a> {
    KW_TEMP(&'a SqliteToken),
    KW_TEMPORARY(&'a SqliteToken),
}
impl<'a> Temporary<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        match node.data {
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_TEMP,
                    ..
                },
            ) => {
                return Some(Self::KW_TEMP(&token));
            }
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_TEMPORARY,
                    ..
                },
            ) => return Some(Self::KW_TEMPORARY(&token)),
            _ => return None,
        }
    }
    pub fn kw_temp(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_TEMP(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_temporary(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_TEMPORARY(item) => Some(item),
            _ => None,
        }
    }
}
pub enum Target<'a> {
    TableOrIdxOrCollationName(TableOrIdxOrCollationName<'a>),
    TableOrIdxNameWithSchema(TableOrIdxNameWithSchema<'a>),
}
impl<'a> Target<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::TableOrIdxOrCollationName) => {
                return Some(Self::TableOrIdxOrCollationName(
                    TableOrIdxOrCollationName::cast(node)?,
                ));
            }
            CstNodeData::Tree(TreeKind::TableOrIdxNameWithSchema) => {
                return Some(Self::TableOrIdxNameWithSchema(
                    TableOrIdxNameWithSchema::cast(node)?,
                ));
            }
            _ => return None,
        }
    }
    pub fn table_or_idx_or_collation_name(self) -> Option<TableOrIdxOrCollationName<'a>> {
        match self {
            Self::TableOrIdxOrCollationName(item) => Some(item),
            _ => None,
        }
    }
    pub fn table_or_idx_name_with_schema(self) -> Option<TableOrIdxNameWithSchema<'a>> {
        match self {
            Self::TableOrIdxNameWithSchema(item) => Some(item),
            _ => None,
        }
    }
}
pub enum Order<'a> {
    KW_ASC(&'a SqliteToken),
    KW_DESC(&'a SqliteToken),
}
impl<'a> Order<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        if node.tree() != Some(TreeKind::Order) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_ASC,
                        ..
                    },
                ) => return Some(Self::KW_ASC(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_DESC,
                        ..
                    },
                ) => return Some(Self::KW_DESC(&token)),
                _ => return None,
            })
            .next()
    }
    pub fn kw_asc(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_ASC(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_desc(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_DESC(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ConstraintType<'a> {
    PrimaryConstraint(PrimaryConstraint<'a>),
    NotNullConstraint(NotNullConstraint<'a>),
    UniqueConstraint(UniqueConstraint<'a>),
    CheckConstraint(CheckConstraint<'a>),
    DefaultConstraint(DefaultConstraint<'a>),
    Collation(Collation<'a>),
    ColumnGenerated(ColumnGenerated<'a>),
}
impl<'a> ConstraintType<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::PrimaryConstraint) => {
                return Some(Self::PrimaryConstraint(PrimaryConstraint::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::NotNullConstraint) => {
                return Some(Self::NotNullConstraint(NotNullConstraint::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::UniqueConstraint) => {
                return Some(Self::UniqueConstraint(UniqueConstraint::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::CheckConstraint) => {
                return Some(Self::CheckConstraint(CheckConstraint::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::DefaultConstraint) => {
                return Some(Self::DefaultConstraint(DefaultConstraint::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::Collation) => {
                return Some(Self::Collation(Collation::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::ColumnGenerated) => {
                return Some(Self::ColumnGenerated(ColumnGenerated::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn primary_constraint(self) -> Option<PrimaryConstraint<'a>> {
        match self {
            Self::PrimaryConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn not_null_constraint(self) -> Option<NotNullConstraint<'a>> {
        match self {
            Self::NotNullConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn unique_constraint(self) -> Option<UniqueConstraint<'a>> {
        match self {
            Self::UniqueConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn check_constraint(self) -> Option<CheckConstraint<'a>> {
        match self {
            Self::CheckConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn default_constraint(self) -> Option<DefaultConstraint<'a>> {
        match self {
            Self::DefaultConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn collation(self) -> Option<Collation<'a>> {
        match self {
            Self::Collation(item) => Some(item),
            _ => None,
        }
    }
    pub fn column_generated(self) -> Option<ColumnGenerated<'a>> {
        match self {
            Self::ColumnGenerated(item) => Some(item),
            _ => None,
        }
    }
}
pub enum IndexColumn<'a> {
    ColumnName(ColumnName<'a>),
    Expr(Expr<'a>),
}
impl<'a> IndexColumn<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::ColumnName) => {
                return Some(Self::ColumnName(ColumnName::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::Expr) => {
                return Some(Self::Expr(Expr::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn column_name(self) -> Option<ColumnName<'a>> {
        match self {
            Self::ColumnName(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr(self) -> Option<Expr<'a>> {
        match self {
            Self::Expr(item) => Some(item),
            _ => None,
        }
    }
}
pub enum Range<'a> {
    KW_RANGE(&'a SqliteToken),
    KW_ROWS(&'a SqliteToken),
    KW_GROUPS(&'a SqliteToken),
}
impl<'a> Range<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        match node.data {
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_RANGE,
                    ..
                },
            ) => {
                return Some(Self::KW_RANGE(&token));
            }
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_ROWS,
                    ..
                },
            ) => {
                return Some(Self::KW_ROWS(&token));
            }
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_GROUPS,
                    ..
                },
            ) => return Some(Self::KW_GROUPS(&token)),
            _ => return None,
        }
    }
    pub fn kw_range(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_RANGE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_rows(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_ROWS(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_groups(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_GROUPS(item) => Some(item),
            _ => None,
        }
    }
}
pub enum SelectDistinct<'a> {
    KW_DISTINCT(&'a SqliteToken),
    KW_ALL(&'a SqliteToken),
}
impl<'a> SelectDistinct<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        match node.data {
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_DISTINCT,
                    ..
                },
            ) => return Some(Self::KW_DISTINCT(&token)),
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_ALL,
                    ..
                },
            ) => {
                return Some(Self::KW_ALL(&token));
            }
            _ => return None,
        }
    }
    pub fn kw_distinct(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_DISTINCT(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_all(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_ALL(item) => Some(item),
            _ => None,
        }
    }
}
pub enum JoinConstraint<'a> {
    OnConstraint(OnConstraint<'a>),
    UsingConstraint(UsingConstraint<'a>),
}
impl<'a> JoinConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::JoinConstraint) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::OnConstraint) => {
                    return Some(Self::OnConstraint(OnConstraint::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::UsingConstraint) => {
                    return Some(Self::UsingConstraint(UsingConstraint::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn on_constraint(self) -> Option<OnConstraint<'a>> {
        match self {
            Self::OnConstraint(item) => Some(item),
            _ => None,
        }
    }
    pub fn using_constraint(self) -> Option<UsingConstraint<'a>> {
        match self {
            Self::UsingConstraint(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ExprInfix<'a> {
    OpConcat(OpConcat<'a>),
    OpExtractOne(OpExtractOne<'a>),
    OpExtractTwo(OpExtractTwo<'a>),
    OpMultiply(OpMultiply<'a>),
    OpDivide(OpDivide<'a>),
    OpModulus(OpModulus<'a>),
    OpAdd(OpAdd<'a>),
    OpSubtract(OpSubtract<'a>),
    OpBinAnd(OpBinAnd<'a>),
    OpBinOr(OpBinOr<'a>),
    OpBinLShift(OpBinLShift<'a>),
    OpBinRShift(OpBinRShift<'a>),
    OpLT(OpLT<'a>),
    OpGT(OpGT<'a>),
    OpLTE(OpLTE<'a>),
    OpGTE(OpGTE<'a>),
    OpEq(OpEq<'a>),
    OpNotEq(OpNotEq<'a>),
    OpAnd(OpAnd<'a>),
    OpOr(OpOr<'a>),
    OpMatch(OpMatch<'a>),
    OpLike(OpLike<'a>),
    OpRegexp(OpRegexp<'a>),
    OpGlob(OpGlob<'a>),
    OpBetweenAnd(OpBetweenAnd<'a>),
    OpNotMatch(OpNotMatch<'a>),
    OpNotLike(OpNotLike<'a>),
    OpNotRegexp(OpNotRegexp<'a>),
    OpNotGlob(OpNotGlob<'a>),
    OpNotBetweenAnd(OpNotBetweenAnd<'a>),
    OpIsNotDistinctFrom(OpIsNotDistinctFrom<'a>),
    OpIsDistinctFrom(OpIsDistinctFrom<'a>),
    OpIsNot(OpIsNot<'a>),
    OpIs(OpIs<'a>),
    OpIn(OpIn<'a>),
}
impl<'a> ExprInfix<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::ExprInfix) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::OpConcat) => {
                    return Some(Self::OpConcat(OpConcat::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpExtractOne) => {
                    return Some(Self::OpExtractOne(OpExtractOne::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpExtractTwo) => {
                    return Some(Self::OpExtractTwo(OpExtractTwo::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpMultiply) => {
                    return Some(Self::OpMultiply(OpMultiply::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpDivide) => {
                    return Some(Self::OpDivide(OpDivide::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpModulus) => {
                    return Some(Self::OpModulus(OpModulus::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpAdd) => {
                    return Some(Self::OpAdd(OpAdd::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpSubtract) => {
                    return Some(Self::OpSubtract(OpSubtract::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpBinAnd) => {
                    return Some(Self::OpBinAnd(OpBinAnd::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpBinOr) => {
                    return Some(Self::OpBinOr(OpBinOr::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpBinLShift) => {
                    return Some(Self::OpBinLShift(OpBinLShift::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpBinRShift) => {
                    return Some(Self::OpBinRShift(OpBinRShift::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpLT) => {
                    return Some(Self::OpLT(OpLT::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpGT) => {
                    return Some(Self::OpGT(OpGT::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpLTE) => {
                    return Some(Self::OpLTE(OpLTE::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpGTE) => {
                    return Some(Self::OpGTE(OpGTE::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpEq) => {
                    return Some(Self::OpEq(OpEq::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpNotEq) => {
                    return Some(Self::OpNotEq(OpNotEq::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpAnd) => {
                    return Some(Self::OpAnd(OpAnd::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpOr) => {
                    return Some(Self::OpOr(OpOr::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpMatch) => {
                    return Some(Self::OpMatch(OpMatch::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpLike) => {
                    return Some(Self::OpLike(OpLike::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpRegexp) => {
                    return Some(Self::OpRegexp(OpRegexp::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpGlob) => {
                    return Some(Self::OpGlob(OpGlob::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpBetweenAnd) => {
                    return Some(Self::OpBetweenAnd(OpBetweenAnd::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpNotMatch) => {
                    return Some(Self::OpNotMatch(OpNotMatch::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpNotLike) => {
                    return Some(Self::OpNotLike(OpNotLike::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpNotRegexp) => {
                    return Some(Self::OpNotRegexp(OpNotRegexp::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpNotGlob) => {
                    return Some(Self::OpNotGlob(OpNotGlob::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpNotBetweenAnd) => {
                    return Some(Self::OpNotBetweenAnd(OpNotBetweenAnd::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpIsNotDistinctFrom) => {
                    return Some(Self::OpIsNotDistinctFrom(OpIsNotDistinctFrom::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpIsDistinctFrom) => {
                    return Some(Self::OpIsDistinctFrom(OpIsDistinctFrom::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpIsNot) => {
                    return Some(Self::OpIsNot(OpIsNot::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpIs) => {
                    return Some(Self::OpIs(OpIs::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpIn) => {
                    return Some(Self::OpIn(OpIn::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn op_concat(self) -> Option<OpConcat<'a>> {
        match self {
            Self::OpConcat(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_extract_one(self) -> Option<OpExtractOne<'a>> {
        match self {
            Self::OpExtractOne(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_extract_two(self) -> Option<OpExtractTwo<'a>> {
        match self {
            Self::OpExtractTwo(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_multiply(self) -> Option<OpMultiply<'a>> {
        match self {
            Self::OpMultiply(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_divide(self) -> Option<OpDivide<'a>> {
        match self {
            Self::OpDivide(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_modulus(self) -> Option<OpModulus<'a>> {
        match self {
            Self::OpModulus(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_add(self) -> Option<OpAdd<'a>> {
        match self {
            Self::OpAdd(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_subtract(self) -> Option<OpSubtract<'a>> {
        match self {
            Self::OpSubtract(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_bin_and(self) -> Option<OpBinAnd<'a>> {
        match self {
            Self::OpBinAnd(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_bin_or(self) -> Option<OpBinOr<'a>> {
        match self {
            Self::OpBinOr(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_bin_l_shift(self) -> Option<OpBinLShift<'a>> {
        match self {
            Self::OpBinLShift(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_bin_r_shift(self) -> Option<OpBinRShift<'a>> {
        match self {
            Self::OpBinRShift(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_lt(self) -> Option<OpLT<'a>> {
        match self {
            Self::OpLT(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_gt(self) -> Option<OpGT<'a>> {
        match self {
            Self::OpGT(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_lte(self) -> Option<OpLTE<'a>> {
        match self {
            Self::OpLTE(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_gte(self) -> Option<OpGTE<'a>> {
        match self {
            Self::OpGTE(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_eq(self) -> Option<OpEq<'a>> {
        match self {
            Self::OpEq(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_eq(self) -> Option<OpNotEq<'a>> {
        match self {
            Self::OpNotEq(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_and(self) -> Option<OpAnd<'a>> {
        match self {
            Self::OpAnd(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_or(self) -> Option<OpOr<'a>> {
        match self {
            Self::OpOr(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_match(self) -> Option<OpMatch<'a>> {
        match self {
            Self::OpMatch(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_like(self) -> Option<OpLike<'a>> {
        match self {
            Self::OpLike(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_regexp(self) -> Option<OpRegexp<'a>> {
        match self {
            Self::OpRegexp(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_glob(self) -> Option<OpGlob<'a>> {
        match self {
            Self::OpGlob(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_between_and(self) -> Option<OpBetweenAnd<'a>> {
        match self {
            Self::OpBetweenAnd(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_match(self) -> Option<OpNotMatch<'a>> {
        match self {
            Self::OpNotMatch(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_like(self) -> Option<OpNotLike<'a>> {
        match self {
            Self::OpNotLike(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_regexp(self) -> Option<OpNotRegexp<'a>> {
        match self {
            Self::OpNotRegexp(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_glob(self) -> Option<OpNotGlob<'a>> {
        match self {
            Self::OpNotGlob(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not_between_and(self) -> Option<OpNotBetweenAnd<'a>> {
        match self {
            Self::OpNotBetweenAnd(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_is_not_distinct_from(self) -> Option<OpIsNotDistinctFrom<'a>> {
        match self {
            Self::OpIsNotDistinctFrom(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_is_distinct_from(self) -> Option<OpIsDistinctFrom<'a>> {
        match self {
            Self::OpIsDistinctFrom(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_is_not(self) -> Option<OpIsNot<'a>> {
        match self {
            Self::OpIsNot(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_is(self) -> Option<OpIs<'a>> {
        match self {
            Self::OpIs(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_in(self) -> Option<OpIn<'a>> {
        match self {
            Self::OpIn(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FrameSpecBetweenLeft<'a> {
    FrameSpecUnboundedPreceding(FrameSpecUnboundedPreceding<'a>),
    FrameSpecPreceding(FrameSpecPreceding<'a>),
    FrameSpecCurrentRow(FrameSpecCurrentRow<'a>),
    FrameSpecFollowing(FrameSpecFollowing<'a>),
}
impl<'a> FrameSpecBetweenLeft<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::FrameSpecBetweenLeft) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::FrameSpecUnboundedPreceding) => {
                    return Some(Self::FrameSpecUnboundedPreceding(
                        FrameSpecUnboundedPreceding::cast(child)?,
                    ));
                }
                CstNodeData::Tree(TreeKind::FrameSpecPreceding) => {
                    return Some(Self::FrameSpecPreceding(FrameSpecPreceding::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::FrameSpecCurrentRow) => {
                    return Some(Self::FrameSpecCurrentRow(FrameSpecCurrentRow::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::FrameSpecFollowing) => {
                    return Some(Self::FrameSpecFollowing(FrameSpecFollowing::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn frame_spec_unbounded_preceding(self) -> Option<FrameSpecUnboundedPreceding<'a>> {
        match self {
            Self::FrameSpecUnboundedPreceding(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_preceding(self) -> Option<FrameSpecPreceding<'a>> {
        match self {
            Self::FrameSpecPreceding(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_current_row(self) -> Option<FrameSpecCurrentRow<'a>> {
        match self {
            Self::FrameSpecCurrentRow(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_following(self) -> Option<FrameSpecFollowing<'a>> {
        match self {
            Self::FrameSpecFollowing(item) => Some(item),
            _ => None,
        }
    }
}
pub enum TableColumns<'a> {
    TableDetails(TableDetails<'a>),
    CreateTableSelect(CreateTableSelect<'a>),
}
impl<'a> TableColumns<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::TableDetails) => {
                return Some(Self::TableDetails(TableDetails::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::CreateTableSelect) => {
                return Some(Self::CreateTableSelect(CreateTableSelect::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn table_details(self) -> Option<TableDetails<'a>> {
        match self {
            Self::TableDetails(item) => Some(item),
            _ => None,
        }
    }
    pub fn create_table_select(self) -> Option<CreateTableSelect<'a>> {
        match self {
            Self::CreateTableSelect(item) => Some(item),
            _ => None,
        }
    }
}
pub enum OuterJoinKind<'a> {
    KW_LEFT(&'a SqliteToken),
    KW_RIGHT(&'a SqliteToken),
    KW_FULL(&'a SqliteToken),
}
impl<'a> OuterJoinKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        match node.data {
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_LEFT,
                    ..
                },
            ) => {
                return Some(Self::KW_LEFT(&token));
            }
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_RIGHT,
                    ..
                },
            ) => {
                return Some(Self::KW_RIGHT(&token));
            }
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_FULL,
                    ..
                },
            ) => {
                return Some(Self::KW_FULL(&token));
            }
            _ => return None,
        }
    }
    pub fn kw_left(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_LEFT(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_right(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_RIGHT(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_full(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_FULL(item) => Some(item),
            _ => None,
        }
    }
}
pub enum AnalyzeTarget<'a> {
    SchemaOrIdxOrTableName(SchemaOrIdxOrTableName<'a>),
    TableOrIdxNameWithSchema(TableOrIdxNameWithSchema<'a>),
}
impl<'a> AnalyzeTarget<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::SchemaOrIdxOrTableName) => {
                return Some(Self::SchemaOrIdxOrTableName(SchemaOrIdxOrTableName::cast(
                    node,
                )?));
            }
            CstNodeData::Tree(TreeKind::TableOrIdxNameWithSchema) => {
                return Some(Self::TableOrIdxNameWithSchema(
                    TableOrIdxNameWithSchema::cast(node)?,
                ));
            }
            _ => return None,
        }
    }
    pub fn schema_or_idx_or_table_name(self) -> Option<SchemaOrIdxOrTableName<'a>> {
        match self {
            Self::SchemaOrIdxOrTableName(item) => Some(item),
            _ => None,
        }
    }
    pub fn table_or_idx_name_with_schema(self) -> Option<TableOrIdxNameWithSchema<'a>> {
        match self {
            Self::TableOrIdxNameWithSchema(item) => Some(item),
            _ => None,
        }
    }
}
pub enum AlterTableKind<'a> {
    RenameTable(RenameTable<'a>),
    RenameColumn(RenameColumn<'a>),
    AddColumn(AddColumn<'a>),
    DropColumn(DropColumn<'a>),
}
impl<'a> AlterTableKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::RenameTable) => {
                return Some(Self::RenameTable(RenameTable::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::RenameColumn) => {
                return Some(Self::RenameColumn(RenameColumn::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::AddColumn) => {
                return Some(Self::AddColumn(AddColumn::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::DropColumn) => {
                return Some(Self::DropColumn(DropColumn::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn rename_table(self) -> Option<RenameTable<'a>> {
        match self {
            Self::RenameTable(item) => Some(item),
            _ => None,
        }
    }
    pub fn rename_column(self) -> Option<RenameColumn<'a>> {
        match self {
            Self::RenameColumn(item) => Some(item),
            _ => None,
        }
    }
    pub fn add_column(self) -> Option<AddColumn<'a>> {
        match self {
            Self::AddColumn(item) => Some(item),
            _ => None,
        }
    }
    pub fn drop_column(self) -> Option<DropColumn<'a>> {
        match self {
            Self::DropColumn(item) => Some(item),
            _ => None,
        }
    }
}
pub enum UpsertClauseAction<'a> {
    UpsertDoUpdate(UpsertDoUpdate<'a>),
    KW_NOTHING(&'a SqliteToken),
}
impl<'a> UpsertClauseAction<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        match node.data {
            CstNodeData::Tree(TreeKind::UpsertDoUpdate) => {
                return Some(Self::UpsertDoUpdate(UpsertDoUpdate::cast(node)?));
            }
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_NOTHING,
                    ..
                },
            ) => return Some(Self::KW_NOTHING(&token)),
            _ => return None,
        }
    }
    pub fn upsert_do_update(self) -> Option<UpsertDoUpdate<'a>> {
        match self {
            Self::UpsertDoUpdate(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_nothing(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_NOTHING(item) => Some(item),
            _ => None,
        }
    }
}
pub enum TriggerWhen<'a> {
    KW_BEFORE(&'a SqliteToken),
    KW_AFTER(&'a SqliteToken),
    TriggerInsteadOf(TriggerInsteadOf<'a>),
}
impl<'a> TriggerWhen<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        match node.data {
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_BEFORE,
                    ..
                },
            ) => return Some(Self::KW_BEFORE(&token)),
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_AFTER,
                    ..
                },
            ) => {
                return Some(Self::KW_AFTER(&token));
            }
            CstNodeData::Tree(TreeKind::TriggerInsteadOf) => {
                return Some(Self::TriggerInsteadOf(TriggerInsteadOf::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn kw_before(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_BEFORE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_after(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_AFTER(item) => Some(item),
            _ => None,
        }
    }
    pub fn trigger_instead_of(self) -> Option<TriggerInsteadOf<'a>> {
        match self {
            Self::TriggerInsteadOf(item) => Some(item),
            _ => None,
        }
    }
}
pub enum IndexDetails<'a> {
    TableNameIndexedBy(TableNameIndexedBy<'a>),
    TableNameNotIndexed(TableNameNotIndexed<'a>),
}
impl<'a> IndexDetails<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::TableNameIndexedBy) => {
                return Some(Self::TableNameIndexedBy(TableNameIndexedBy::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::TableNameNotIndexed) => {
                return Some(Self::TableNameNotIndexed(TableNameNotIndexed::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn table_name_indexed_by(self) -> Option<TableNameIndexedBy<'a>> {
        match self {
            Self::TableNameIndexedBy(item) => Some(item),
            _ => None,
        }
    }
    pub fn table_name_not_indexed(self) -> Option<TableNameNotIndexed<'a>> {
        match self {
            Self::TableNameNotIndexed(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FkAction<'a> {
    FkSetNull(FkSetNull<'a>),
    FkSetDefault(FkSetDefault<'a>),
    FkCascade(FkCascade<'a>),
    FkRestrict(FkRestrict<'a>),
    FkNoAction(FkNoAction<'a>),
}
impl<'a> FkAction<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::FkSetNull) => {
                return Some(Self::FkSetNull(FkSetNull::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::FkSetDefault) => {
                return Some(Self::FkSetDefault(FkSetDefault::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::FkCascade) => {
                return Some(Self::FkCascade(FkCascade::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::FkRestrict) => {
                return Some(Self::FkRestrict(FkRestrict::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::FkNoAction) => {
                return Some(Self::FkNoAction(FkNoAction::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn fk_set_null(self) -> Option<FkSetNull<'a>> {
        match self {
            Self::FkSetNull(item) => Some(item),
            _ => None,
        }
    }
    pub fn fk_set_default(self) -> Option<FkSetDefault<'a>> {
        match self {
            Self::FkSetDefault(item) => Some(item),
            _ => None,
        }
    }
    pub fn fk_cascade(self) -> Option<FkCascade<'a>> {
        match self {
            Self::FkCascade(item) => Some(item),
            _ => None,
        }
    }
    pub fn fk_restrict(self) -> Option<FkRestrict<'a>> {
        match self {
            Self::FkRestrict(item) => Some(item),
            _ => None,
        }
    }
    pub fn fk_no_action(self) -> Option<FkNoAction<'a>> {
        match self {
            Self::FkNoAction(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FuncArguments<'a> {
    ArgExpr(ArgExpr<'a>),
    ArgStar(ArgStar<'a>),
}
impl<'a> FuncArguments<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::FuncArguments) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::ArgExpr) => {
                    return Some(Self::ArgExpr(ArgExpr::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ArgStar) => {
                    return Some(Self::ArgStar(ArgStar::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn arg_expr(self) -> Option<ArgExpr<'a>> {
        match self {
            Self::ArgExpr(item) => Some(item),
            _ => None,
        }
    }
    pub fn arg_star(self) -> Option<ArgStar<'a>> {
        match self {
            Self::ArgStar(item) => Some(item),
            _ => None,
        }
    }
}
pub enum TableOptions<'a> {
    TableOptWithoutRowId(TableOptWithoutRowId<'a>),
    KW_STRICT(&'a SqliteToken),
}
impl<'a> TableOptions<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        if node.tree() != Some(TreeKind::TableOptions) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::TableOptWithoutRowId) => {
                    return Some(Self::TableOptWithoutRowId(TableOptWithoutRowId::cast(
                        child,
                    )?));
                }
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_STRICT,
                        ..
                    },
                ) => return Some(Self::KW_STRICT(&token)),
                _ => return None,
            })
            .next()
    }
    pub fn table_opt_without_row_id(self) -> Option<TableOptWithoutRowId<'a>> {
        match self {
            Self::TableOptWithoutRowId(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_strict(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_STRICT(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ExprPrefix<'a> {
    OpBinComplement(OpBinComplement<'a>),
    OpUnaryPlus(OpUnaryPlus<'a>),
    OpUnaryMinus(OpUnaryMinus<'a>),
    OpNot(OpNot<'a>),
}
impl<'a> ExprPrefix<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::ExprPrefix) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::OpBinComplement) => {
                    return Some(Self::OpBinComplement(OpBinComplement::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpUnaryPlus) => {
                    return Some(Self::OpUnaryPlus(OpUnaryPlus::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpUnaryMinus) => {
                    return Some(Self::OpUnaryMinus(OpUnaryMinus::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::OpNot) => {
                    return Some(Self::OpNot(OpNot::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn op_bin_complement(self) -> Option<OpBinComplement<'a>> {
        match self {
            Self::OpBinComplement(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_unary_plus(self) -> Option<OpUnaryPlus<'a>> {
        match self {
            Self::OpUnaryPlus(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_unary_minus(self) -> Option<OpUnaryMinus<'a>> {
        match self {
            Self::OpUnaryMinus(item) => Some(item),
            _ => None,
        }
    }
    pub fn op_not(self) -> Option<OpNot<'a>> {
        match self {
            Self::OpNot(item) => Some(item),
            _ => None,
        }
    }
}
pub enum DeferKind<'a> {
    KW_DEFERRED(&'a SqliteToken),
    KW_IMMEDIATE(&'a SqliteToken),
}
impl<'a> DeferKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        match node.data {
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_DEFERRED,
                    ..
                },
            ) => return Some(Self::KW_DEFERRED(&token)),
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_IMMEDIATE,
                    ..
                },
            ) => return Some(Self::KW_IMMEDIATE(&token)),
            _ => return None,
        }
    }
    pub fn kw_deferred(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_DEFERRED(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_immediate(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_IMMEDIATE(item) => Some(item),
            _ => None,
        }
    }
}
pub enum InsertValueKind<'a> {
    InsertValuesClause(InsertValuesClause<'a>),
    InsertSelectClause(InsertSelectClause<'a>),
    InsertDefaultValuesClause(InsertDefaultValuesClause<'a>),
}
impl<'a> InsertValueKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::InsertValueKind) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::InsertValuesClause) => {
                    return Some(Self::InsertValuesClause(InsertValuesClause::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::InsertSelectClause) => {
                    return Some(Self::InsertSelectClause(InsertSelectClause::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::InsertDefaultValuesClause) => {
                    return Some(Self::InsertDefaultValuesClause(
                        InsertDefaultValuesClause::cast(child)?,
                    ));
                }
                _ => return None,
            })
            .next()
    }
    pub fn insert_values_clause(self) -> Option<InsertValuesClause<'a>> {
        match self {
            Self::InsertValuesClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn insert_select_clause(self) -> Option<InsertSelectClause<'a>> {
        match self {
            Self::InsertSelectClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn insert_default_values_clause(self) -> Option<InsertDefaultValuesClause<'a>> {
        match self {
            Self::InsertDefaultValuesClause(item) => Some(item),
            _ => None,
        }
    }
}
pub enum TriggerActionKind<'a> {
    KW_DELETE(&'a SqliteToken),
    KW_INSERT(&'a SqliteToken),
    TriggerUpdateAction(TriggerUpdateAction<'a>),
}
impl<'a> TriggerActionKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        if node.tree() != Some(TreeKind::TriggerActionKind) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_DELETE,
                        ..
                    },
                ) => return Some(Self::KW_DELETE(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_INSERT,
                        ..
                    },
                ) => return Some(Self::KW_INSERT(&token)),
                CstNodeData::Tree(TreeKind::TriggerUpdateAction) => {
                    return Some(Self::TriggerUpdateAction(TriggerUpdateAction::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn kw_delete(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_DELETE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_insert(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_INSERT(item) => Some(item),
            _ => None,
        }
    }
    pub fn trigger_update_action(self) -> Option<TriggerUpdateAction<'a>> {
        match self {
            Self::TriggerUpdateAction(item) => Some(item),
            _ => None,
        }
    }
}
pub enum BeginStmtKind<'a> {
    KW_DEFERRED(&'a SqliteToken),
    KW_IMMEDIATE(&'a SqliteToken),
    KW_EXCLUSIVE(&'a SqliteToken),
}
impl<'a> BeginStmtKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        match node.data {
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_DEFERRED,
                    ..
                },
            ) => return Some(Self::KW_DEFERRED(&token)),
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_IMMEDIATE,
                    ..
                },
            ) => return Some(Self::KW_IMMEDIATE(&token)),
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_EXCLUSIVE,
                    ..
                },
            ) => return Some(Self::KW_EXCLUSIVE(&token)),
            _ => return None,
        }
    }
    pub fn kw_deferred(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_DEFERRED(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_immediate(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_IMMEDIATE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_exclusive(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_EXCLUSIVE(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FrameSpecKind<'a> {
    FrameSpecBetweenClause(FrameSpecBetweenClause<'a>),
    FrameSpecUnboundedPreceding(FrameSpecUnboundedPreceding<'a>),
    FrameSpecPreceding(FrameSpecPreceding<'a>),
    FrameSpecCurrentRow(FrameSpecCurrentRow<'a>),
}
impl<'a> FrameSpecKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::FrameSpecBetweenClause) => {
                return Some(Self::FrameSpecBetweenClause(FrameSpecBetweenClause::cast(
                    node,
                )?));
            }
            CstNodeData::Tree(TreeKind::FrameSpecUnboundedPreceding) => {
                return Some(Self::FrameSpecUnboundedPreceding(
                    FrameSpecUnboundedPreceding::cast(node)?,
                ));
            }
            CstNodeData::Tree(TreeKind::FrameSpecPreceding) => {
                return Some(Self::FrameSpecPreceding(FrameSpecPreceding::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::FrameSpecCurrentRow) => {
                return Some(Self::FrameSpecCurrentRow(FrameSpecCurrentRow::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn frame_spec_between_clause(self) -> Option<FrameSpecBetweenClause<'a>> {
        match self {
            Self::FrameSpecBetweenClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_unbounded_preceding(self) -> Option<FrameSpecUnboundedPreceding<'a>> {
        match self {
            Self::FrameSpecUnboundedPreceding(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_preceding(self) -> Option<FrameSpecPreceding<'a>> {
        match self {
            Self::FrameSpecPreceding(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_current_row(self) -> Option<FrameSpecCurrentRow<'a>> {
        match self {
            Self::FrameSpecCurrentRow(item) => Some(item),
            _ => None,
        }
    }
}
pub enum CommitStartKw<'a> {
    KW_COMMIT(&'a SqliteToken),
    KW_END(&'a SqliteToken),
}
impl<'a> CommitStartKw<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        match node.data {
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_COMMIT,
                    ..
                },
            ) => return Some(Self::KW_COMMIT(&token)),
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_END,
                    ..
                },
            ) => {
                return Some(Self::KW_END(&token));
            }
            _ => return None,
        }
    }
    pub fn kw_commit(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_COMMIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_end(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_END(item) => Some(item),
            _ => None,
        }
    }
}
pub enum Expr<'a> {
    ExprParen(ExprParen<'a>),
    ExprLit(ExprLit<'a>),
    ExprColumnName(ExprColumnName<'a>),
    ExprPrefix(ExprPrefix<'a>),
    ExprPostfix(ExprPostfix<'a>),
    ExprInfix(ExprInfix<'a>),
    ExprBindParam(ExprBindParam<'a>),
    ExprFunc(ExprFunc<'a>),
    ExprSelect(ExprSelect<'a>),
    ExprList(ExprList<'a>),
    ExprCast(ExprCast<'a>),
    ExprCase(ExprCase<'a>),
    RaiseFunc(RaiseFunc<'a>),
}
impl<'a> Expr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::Expr) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::ExprParen) => {
                    return Some(Self::ExprParen(ExprParen::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ExprLit) => {
                    return Some(Self::ExprLit(ExprLit::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ExprColumnName) => {
                    return Some(Self::ExprColumnName(ExprColumnName::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ExprPrefix) => {
                    return Some(Self::ExprPrefix(ExprPrefix::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ExprPostfix) => {
                    return Some(Self::ExprPostfix(ExprPostfix::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ExprInfix) => {
                    return Some(Self::ExprInfix(ExprInfix::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ExprBindParam) => {
                    return Some(Self::ExprBindParam(ExprBindParam::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ExprFunc) => {
                    return Some(Self::ExprFunc(ExprFunc::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ExprSelect) => {
                    return Some(Self::ExprSelect(ExprSelect::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ExprList) => {
                    return Some(Self::ExprList(ExprList::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ExprCast) => {
                    return Some(Self::ExprCast(ExprCast::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ExprCase) => {
                    return Some(Self::ExprCase(ExprCase::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::RaiseFunc) => {
                    return Some(Self::RaiseFunc(RaiseFunc::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn expr_paren(self) -> Option<ExprParen<'a>> {
        match self {
            Self::ExprParen(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_lit(self) -> Option<ExprLit<'a>> {
        match self {
            Self::ExprLit(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_column_name(self) -> Option<ExprColumnName<'a>> {
        match self {
            Self::ExprColumnName(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_prefix(self) -> Option<ExprPrefix<'a>> {
        match self {
            Self::ExprPrefix(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_postfix(self) -> Option<ExprPostfix<'a>> {
        match self {
            Self::ExprPostfix(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_infix(self) -> Option<ExprInfix<'a>> {
        match self {
            Self::ExprInfix(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_bind_param(self) -> Option<ExprBindParam<'a>> {
        match self {
            Self::ExprBindParam(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_func(self) -> Option<ExprFunc<'a>> {
        match self {
            Self::ExprFunc(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_select(self) -> Option<ExprSelect<'a>> {
        match self {
            Self::ExprSelect(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_list(self) -> Option<ExprList<'a>> {
        match self {
            Self::ExprList(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_cast(self) -> Option<ExprCast<'a>> {
        match self {
            Self::ExprCast(item) => Some(item),
            _ => None,
        }
    }
    pub fn expr_case(self) -> Option<ExprCase<'a>> {
        match self {
            Self::ExprCase(item) => Some(item),
            _ => None,
        }
    }
    pub fn raise_func(self) -> Option<RaiseFunc<'a>> {
        match self {
            Self::RaiseFunc(item) => Some(item),
            _ => None,
        }
    }
}
pub enum InsertStmtKind<'a> {
    KW_REPLACE(&'a SqliteToken),
    InsertOrAction(InsertOrAction<'a>),
}
impl<'a> InsertStmtKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        if node.tree() != Some(TreeKind::InsertStmtKind) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_REPLACE,
                        ..
                    },
                ) => return Some(Self::KW_REPLACE(&token)),
                CstNodeData::Tree(TreeKind::InsertOrAction) => {
                    return Some(Self::InsertOrAction(InsertOrAction::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn kw_replace(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_REPLACE(item) => Some(item),
            _ => None,
        }
    }
    pub fn insert_or_action(self) -> Option<InsertOrAction<'a>> {
        match self {
            Self::InsertOrAction(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ReturningClauseKind<'a> {
    STAR(&'a SqliteToken),
    ReturningClauseExpr(ReturningClauseExpr<'a>),
}
impl<'a> ReturningClauseKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        if node.tree() != Some(TreeKind::ReturningClauseKind) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::STAR,
                        ..
                    },
                ) => {
                    return Some(Self::STAR(&token));
                }
                CstNodeData::Tree(TreeKind::ReturningClauseExpr) => {
                    return Some(Self::ReturningClauseExpr(ReturningClauseExpr::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn star(self) -> Option<&'a SqliteToken> {
        match self {
            Self::STAR(item) => Some(item),
            _ => None,
        }
    }
    pub fn returning_clause_expr(self) -> Option<ReturningClauseExpr<'a>> {
        match self {
            Self::ReturningClauseExpr(item) => Some(item),
            _ => None,
        }
    }
}
pub enum JoinOperator<'a> {
    CommaJoin(CommaJoin<'a>),
    NonCommaJoin(NonCommaJoin<'a>),
}
impl<'a> JoinOperator<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::JoinOperator) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::CommaJoin) => {
                    return Some(Self::CommaJoin(CommaJoin::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::NonCommaJoin) => {
                    return Some(Self::NonCommaJoin(NonCommaJoin::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn comma_join(self) -> Option<CommaJoin<'a>> {
        match self {
            Self::CommaJoin(item) => Some(item),
            _ => None,
        }
    }
    pub fn non_comma_join(self) -> Option<NonCommaJoin<'a>> {
        match self {
            Self::NonCommaJoin(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ModuleArg<'a> {
    STR_LIT(&'a SqliteToken),
    INT_LIT(&'a SqliteToken),
    REAL_LIT(&'a SqliteToken),
    HEX_LIT(&'a SqliteToken),
}
impl<'a> ModuleArg<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        if node.tree() != Some(TreeKind::ModuleArg) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::STR_LIT,
                        ..
                    },
                ) => return Some(Self::STR_LIT(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::INT_LIT,
                        ..
                    },
                ) => return Some(Self::INT_LIT(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::REAL_LIT,
                        ..
                    },
                ) => return Some(Self::REAL_LIT(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::HEX_LIT,
                        ..
                    },
                ) => return Some(Self::HEX_LIT(&token)),
                _ => return None,
            })
            .next()
    }
    pub fn str_lit(self) -> Option<&'a SqliteToken> {
        match self {
            Self::STR_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn int_lit(self) -> Option<&'a SqliteToken> {
        match self {
            Self::INT_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn real_lit(self) -> Option<&'a SqliteToken> {
        match self {
            Self::REAL_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn hex_lit(self) -> Option<&'a SqliteToken> {
        match self {
            Self::HEX_LIT(item) => Some(item),
            _ => None,
        }
    }
}
pub enum TriggerBodyStmt<'a> {
    UpdateStmt(UpdateStmt<'a>),
    InsertStmt(InsertStmt<'a>),
    DeleteStmt(DeleteStmt<'a>),
    SelectStmt(SelectStmt<'a>),
}
impl<'a> TriggerBodyStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::TriggerBodyStmt) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::UpdateStmt) => {
                    return Some(Self::UpdateStmt(UpdateStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::InsertStmt) => {
                    return Some(Self::InsertStmt(InsertStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::DeleteStmt) => {
                    return Some(Self::DeleteStmt(DeleteStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::SelectStmt) => {
                    return Some(Self::SelectStmt(SelectStmt::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn update_stmt(self) -> Option<UpdateStmt<'a>> {
        match self {
            Self::UpdateStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn insert_stmt(self) -> Option<InsertStmt<'a>> {
        match self {
            Self::InsertStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn delete_stmt(self) -> Option<DeleteStmt<'a>> {
        match self {
            Self::DeleteStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn select_stmt(self) -> Option<SelectStmt<'a>> {
        match self {
            Self::SelectStmt(item) => Some(item),
            _ => None,
        }
    }
}
pub enum OverClauseKind<'a> {
    WindowName(WindowName<'a>),
    WindowDef(WindowDef<'a>),
}
impl<'a> OverClauseKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::WindowName) => {
                return Some(Self::WindowName(WindowName::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::WindowDef) => {
                return Some(Self::WindowDef(WindowDef::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn window_name(self) -> Option<WindowName<'a>> {
        match self {
            Self::WindowName(item) => Some(item),
            _ => None,
        }
    }
    pub fn window_def(self) -> Option<WindowDef<'a>> {
        match self {
            Self::WindowDef(item) => Some(item),
            _ => None,
        }
    }
}
pub enum CompoundOperator<'a> {
    UnionCompoundOperator(UnionCompoundOperator<'a>),
    KW_INTERSECT(&'a SqliteToken),
    KW_EXCEPT(&'a SqliteToken),
}
impl<'a> CompoundOperator<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        if node.tree() != Some(TreeKind::CompoundOperator) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::UnionCompoundOperator) => {
                    return Some(Self::UnionCompoundOperator(UnionCompoundOperator::cast(
                        child,
                    )?));
                }
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_INTERSECT,
                        ..
                    },
                ) => return Some(Self::KW_INTERSECT(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_EXCEPT,
                        ..
                    },
                ) => return Some(Self::KW_EXCEPT(&token)),
                _ => return None,
            })
            .next()
    }
    pub fn union_compound_operator(self) -> Option<UnionCompoundOperator<'a>> {
        match self {
            Self::UnionCompoundOperator(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_intersect(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_INTERSECT(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_except(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_EXCEPT(item) => Some(item),
            _ => None,
        }
    }
}
pub enum CtePrependable<'a> {
    SelectStmt(SelectStmt<'a>),
    InsertStmt(InsertStmt<'a>),
    UpdateStmt(UpdateStmt<'a>),
    DeleteStmt(DeleteStmt<'a>),
}
impl<'a> CtePrependable<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::SelectStmt) => {
                return Some(Self::SelectStmt(SelectStmt::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::InsertStmt) => {
                return Some(Self::InsertStmt(InsertStmt::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::UpdateStmt) => {
                return Some(Self::UpdateStmt(UpdateStmt::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::DeleteStmt) => {
                return Some(Self::DeleteStmt(DeleteStmt::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn select_stmt(self) -> Option<SelectStmt<'a>> {
        match self {
            Self::SelectStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn insert_stmt(self) -> Option<InsertStmt<'a>> {
        match self {
            Self::InsertStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn update_stmt(self) -> Option<UpdateStmt<'a>> {
        match self {
            Self::UpdateStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn delete_stmt(self) -> Option<DeleteStmt<'a>> {
        match self {
            Self::DeleteStmt(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ConflictAction<'a> {
    KW_ROLLBACK(&'a SqliteToken),
    KW_ABORT(&'a SqliteToken),
    KW_FAIL(&'a SqliteToken),
    KW_IGNORE(&'a SqliteToken),
    KW_REPLACE(&'a SqliteToken),
}
impl<'a> ConflictAction<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        if node.tree() != Some(TreeKind::ConflictAction) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_ROLLBACK,
                        ..
                    },
                ) => return Some(Self::KW_ROLLBACK(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_ABORT,
                        ..
                    },
                ) => return Some(Self::KW_ABORT(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_FAIL,
                        ..
                    },
                ) => return Some(Self::KW_FAIL(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_IGNORE,
                        ..
                    },
                ) => return Some(Self::KW_IGNORE(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_REPLACE,
                        ..
                    },
                ) => return Some(Self::KW_REPLACE(&token)),
                _ => return None,
            })
            .next()
    }
    pub fn kw_rollback(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_ROLLBACK(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_abort(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_ABORT(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_fail(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_FAIL(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_ignore(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_IGNORE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_replace(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_REPLACE(item) => Some(item),
            _ => None,
        }
    }
}
pub enum PragmaValue<'a> {
    SignedNumber(SignedNumber<'a>),
    PragmaValueName(PragmaValueName<'a>),
    STR_LIT(&'a SqliteToken),
}
impl<'a> PragmaValue<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        if node.tree() != Some(TreeKind::PragmaValue) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::SignedNumber) => {
                    return Some(Self::SignedNumber(SignedNumber::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::PragmaValueName) => {
                    return Some(Self::PragmaValueName(PragmaValueName::cast(child)?));
                }
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::STR_LIT,
                        ..
                    },
                ) => return Some(Self::STR_LIT(&token)),
                _ => return None,
            })
            .next()
    }
    pub fn signed_number(self) -> Option<SignedNumber<'a>> {
        match self {
            Self::SignedNumber(item) => Some(item),
            _ => None,
        }
    }
    pub fn pragma_value_name(self) -> Option<PragmaValueName<'a>> {
        match self {
            Self::PragmaValueName(item) => Some(item),
            _ => None,
        }
    }
    pub fn str_lit(self) -> Option<&'a SqliteToken> {
        match self {
            Self::STR_LIT(item) => Some(item),
            _ => None,
        }
    }
}
pub enum PrimaryConstraintOrder<'a> {
    KW_ASC(&'a SqliteToken),
    KW_DESC(&'a SqliteToken),
}
impl<'a> PrimaryConstraintOrder<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        match node.data {
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_ASC,
                    ..
                },
            ) => {
                return Some(Self::KW_ASC(&token));
            }
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_DESC,
                    ..
                },
            ) => {
                return Some(Self::KW_DESC(&token));
            }
            _ => return None,
        }
    }
    pub fn kw_asc(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_ASC(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_desc(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_DESC(item) => Some(item),
            _ => None,
        }
    }
}
pub enum SetColumnKind<'a> {
    ColumnName(ColumnName<'a>),
    ColNameList(ColNameList<'a>),
}
impl<'a> SetColumnKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::ColumnName) => {
                return Some(Self::ColumnName(ColumnName::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::ColNameList) => {
                return Some(Self::ColNameList(ColNameList::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn column_name(self) -> Option<ColumnName<'a>> {
        match self {
            Self::ColumnName(item) => Some(item),
            _ => None,
        }
    }
    pub fn col_name_list(self) -> Option<ColNameList<'a>> {
        match self {
            Self::ColNameList(item) => Some(item),
            _ => None,
        }
    }
}
pub enum NaturalJoinKind<'a> {
    InnerJoin(InnerJoin<'a>),
    OuterJoin(OuterJoin<'a>),
    Join(Join<'a>),
}
impl<'a> NaturalJoinKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::InnerJoin) => {
                return Some(Self::InnerJoin(InnerJoin::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::OuterJoin) => {
                return Some(Self::OuterJoin(OuterJoin::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::Join) => {
                return Some(Self::Join(Join::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn inner_join(self) -> Option<InnerJoin<'a>> {
        match self {
            Self::InnerJoin(item) => Some(item),
            _ => None,
        }
    }
    pub fn outer_join(self) -> Option<OuterJoin<'a>> {
        match self {
            Self::OuterJoin(item) => Some(item),
            _ => None,
        }
    }
    pub fn join(self) -> Option<Join<'a>> {
        match self {
            Self::Join(item) => Some(item),
            _ => None,
        }
    }
}
pub enum InExprKind<'a> {
    ExprList(ExprList<'a>),
    InSelect(InSelect<'a>),
    InTableFunc(InTableFunc<'a>),
    InTable(InTable<'a>),
}
impl<'a> InExprKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::ExprList) => {
                return Some(Self::ExprList(ExprList::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::InSelect) => {
                return Some(Self::InSelect(InSelect::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::InTableFunc) => {
                return Some(Self::InTableFunc(InTableFunc::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::InTable) => {
                return Some(Self::InTable(InTable::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn expr_list(self) -> Option<ExprList<'a>> {
        match self {
            Self::ExprList(item) => Some(item),
            _ => None,
        }
    }
    pub fn in_select(self) -> Option<InSelect<'a>> {
        match self {
            Self::InSelect(item) => Some(item),
            _ => None,
        }
    }
    pub fn in_table_func(self) -> Option<InTableFunc<'a>> {
        match self {
            Self::InTableFunc(item) => Some(item),
            _ => None,
        }
    }
    pub fn in_table(self) -> Option<InTable<'a>> {
        match self {
            Self::InTable(item) => Some(item),
            _ => None,
        }
    }
}
pub enum SelectCore<'a> {
    TraditionalSelect(TraditionalSelect<'a>),
    ValuesSelect(ValuesSelect<'a>),
}
impl<'a> SelectCore<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::SelectCore) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::TraditionalSelect) => {
                    return Some(Self::TraditionalSelect(TraditionalSelect::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ValuesSelect) => {
                    return Some(Self::ValuesSelect(ValuesSelect::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn traditional_select(self) -> Option<TraditionalSelect<'a>> {
        match self {
            Self::TraditionalSelect(item) => Some(item),
            _ => None,
        }
    }
    pub fn values_select(self) -> Option<ValuesSelect<'a>> {
        match self {
            Self::ValuesSelect(item) => Some(item),
            _ => None,
        }
    }
}
pub enum DefaultValueKind<'a> {
    DefaultConstraintExpr(DefaultConstraintExpr<'a>),
    DefaultConstraintLiteral(DefaultConstraintLiteral<'a>),
    SignedNumber(SignedNumber<'a>),
}
impl<'a> DefaultValueKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::DefaultConstraintExpr) => {
                return Some(Self::DefaultConstraintExpr(DefaultConstraintExpr::cast(
                    node,
                )?));
            }
            CstNodeData::Tree(TreeKind::DefaultConstraintLiteral) => {
                return Some(Self::DefaultConstraintLiteral(
                    DefaultConstraintLiteral::cast(node)?,
                ));
            }
            CstNodeData::Tree(TreeKind::SignedNumber) => {
                return Some(Self::SignedNumber(SignedNumber::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn default_constraint_expr(self) -> Option<DefaultConstraintExpr<'a>> {
        match self {
            Self::DefaultConstraintExpr(item) => Some(item),
            _ => None,
        }
    }
    pub fn default_constraint_literal(self) -> Option<DefaultConstraintLiteral<'a>> {
        match self {
            Self::DefaultConstraintLiteral(item) => Some(item),
            _ => None,
        }
    }
    pub fn signed_number(self) -> Option<SignedNumber<'a>> {
        match self {
            Self::SignedNumber(item) => Some(item),
            _ => None,
        }
    }
}
pub enum StatementNoCte<'a> {
    CreateTableStmt(CreateTableStmt<'a>),
    AlterTableStmt(AlterTableStmt<'a>),
    AnalyzeStmt(AnalyzeStmt<'a>),
    AttachDbStmt(AttachDbStmt<'a>),
    BeginStmt(BeginStmt<'a>),
    CommitStmt(CommitStmt<'a>),
    CreateIndexStmt(CreateIndexStmt<'a>),
    CreateTriggerStmt(CreateTriggerStmt<'a>),
    CreateViewStmt(CreateViewStmt<'a>),
    CreateVirtualTableStmt(CreateVirtualTableStmt<'a>),
    DetachStmt(DetachStmt<'a>),
    DropIndexStmt(DropIndexStmt<'a>),
    DropViewStmt(DropViewStmt<'a>),
    DropTableStmt(DropTableStmt<'a>),
    DropTriggerStmt(DropTriggerStmt<'a>),
    PragmaStmt(PragmaStmt<'a>),
    ReIndexStmt(ReIndexStmt<'a>),
    ReleaseStmt(ReleaseStmt<'a>),
    RollbackStmt(RollbackStmt<'a>),
    SavepointStmt(SavepointStmt<'a>),
    VacuumStmt(VacuumStmt<'a>),
}
impl<'a> StatementNoCte<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::StatementNoCte) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::CreateTableStmt) => {
                    return Some(Self::CreateTableStmt(CreateTableStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::AlterTableStmt) => {
                    return Some(Self::AlterTableStmt(AlterTableStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::AnalyzeStmt) => {
                    return Some(Self::AnalyzeStmt(AnalyzeStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::AttachDbStmt) => {
                    return Some(Self::AttachDbStmt(AttachDbStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::BeginStmt) => {
                    return Some(Self::BeginStmt(BeginStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::CommitStmt) => {
                    return Some(Self::CommitStmt(CommitStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::CreateIndexStmt) => {
                    return Some(Self::CreateIndexStmt(CreateIndexStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::CreateTriggerStmt) => {
                    return Some(Self::CreateTriggerStmt(CreateTriggerStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::CreateViewStmt) => {
                    return Some(Self::CreateViewStmt(CreateViewStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::CreateVirtualTableStmt) => {
                    return Some(Self::CreateVirtualTableStmt(CreateVirtualTableStmt::cast(
                        child,
                    )?));
                }
                CstNodeData::Tree(TreeKind::DetachStmt) => {
                    return Some(Self::DetachStmt(DetachStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::DropIndexStmt) => {
                    return Some(Self::DropIndexStmt(DropIndexStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::DropViewStmt) => {
                    return Some(Self::DropViewStmt(DropViewStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::DropTableStmt) => {
                    return Some(Self::DropTableStmt(DropTableStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::DropTriggerStmt) => {
                    return Some(Self::DropTriggerStmt(DropTriggerStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::PragmaStmt) => {
                    return Some(Self::PragmaStmt(PragmaStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ReIndexStmt) => {
                    return Some(Self::ReIndexStmt(ReIndexStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::ReleaseStmt) => {
                    return Some(Self::ReleaseStmt(ReleaseStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::RollbackStmt) => {
                    return Some(Self::RollbackStmt(RollbackStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::SavepointStmt) => {
                    return Some(Self::SavepointStmt(SavepointStmt::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::VacuumStmt) => {
                    return Some(Self::VacuumStmt(VacuumStmt::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn create_table_stmt(self) -> Option<CreateTableStmt<'a>> {
        match self {
            Self::CreateTableStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn alter_table_stmt(self) -> Option<AlterTableStmt<'a>> {
        match self {
            Self::AlterTableStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn analyze_stmt(self) -> Option<AnalyzeStmt<'a>> {
        match self {
            Self::AnalyzeStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn attach_db_stmt(self) -> Option<AttachDbStmt<'a>> {
        match self {
            Self::AttachDbStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn begin_stmt(self) -> Option<BeginStmt<'a>> {
        match self {
            Self::BeginStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn commit_stmt(self) -> Option<CommitStmt<'a>> {
        match self {
            Self::CommitStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn create_index_stmt(self) -> Option<CreateIndexStmt<'a>> {
        match self {
            Self::CreateIndexStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn create_trigger_stmt(self) -> Option<CreateTriggerStmt<'a>> {
        match self {
            Self::CreateTriggerStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn create_view_stmt(self) -> Option<CreateViewStmt<'a>> {
        match self {
            Self::CreateViewStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn create_virtual_table_stmt(self) -> Option<CreateVirtualTableStmt<'a>> {
        match self {
            Self::CreateVirtualTableStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn detach_stmt(self) -> Option<DetachStmt<'a>> {
        match self {
            Self::DetachStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn drop_index_stmt(self) -> Option<DropIndexStmt<'a>> {
        match self {
            Self::DropIndexStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn drop_view_stmt(self) -> Option<DropViewStmt<'a>> {
        match self {
            Self::DropViewStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn drop_table_stmt(self) -> Option<DropTableStmt<'a>> {
        match self {
            Self::DropTableStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn drop_trigger_stmt(self) -> Option<DropTriggerStmt<'a>> {
        match self {
            Self::DropTriggerStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn pragma_stmt(self) -> Option<PragmaStmt<'a>> {
        match self {
            Self::PragmaStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn re_index_stmt(self) -> Option<ReIndexStmt<'a>> {
        match self {
            Self::ReIndexStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn release_stmt(self) -> Option<ReleaseStmt<'a>> {
        match self {
            Self::ReleaseStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn rollback_stmt(self) -> Option<RollbackStmt<'a>> {
        match self {
            Self::RollbackStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn savepoint_stmt(self) -> Option<SavepointStmt<'a>> {
        match self {
            Self::SavepointStmt(item) => Some(item),
            _ => None,
        }
    }
    pub fn vacuum_stmt(self) -> Option<VacuumStmt<'a>> {
        match self {
            Self::VacuumStmt(item) => Some(item),
            _ => None,
        }
    }
}
pub enum NullsPosition<'a> {
    KW_FIRST(&'a SqliteToken),
    KW_LAST(&'a SqliteToken),
}
impl<'a> NullsPosition<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        match node.data {
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_FIRST,
                    ..
                },
            ) => {
                return Some(Self::KW_FIRST(&token));
            }
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_LAST,
                    ..
                },
            ) => {
                return Some(Self::KW_LAST(&token));
            }
            _ => return None,
        }
    }
    pub fn kw_first(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_FIRST(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_last(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_LAST(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FkOnOrMatch<'a> {
    FkOnAction(FkOnAction<'a>),
    FkMatchAction(FkMatchAction<'a>),
}
impl<'a> FkOnOrMatch<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        match node.data {
            CstNodeData::Tree(TreeKind::FkOnAction) => {
                return Some(Self::FkOnAction(FkOnAction::cast(node)?));
            }
            CstNodeData::Tree(TreeKind::FkMatchAction) => {
                return Some(Self::FkMatchAction(FkMatchAction::cast(node)?));
            }
            _ => return None,
        }
    }
    pub fn fk_on_action(self) -> Option<FkOnAction<'a>> {
        match self {
            Self::FkOnAction(item) => Some(item),
            _ => None,
        }
    }
    pub fn fk_match_action(self) -> Option<FkMatchAction<'a>> {
        match self {
            Self::FkMatchAction(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FrameSpecBetweenRight<'a> {
    FrameSpecUnboundedFollowing(FrameSpecUnboundedFollowing<'a>),
    FrameSpecPreceding(FrameSpecPreceding<'a>),
    FrameSpecCurrentRow(FrameSpecCurrentRow<'a>),
    FrameSpecFollowing(FrameSpecFollowing<'a>),
}
impl<'a> FrameSpecBetweenRight<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTreeKind as TreeKind;

        if node.tree() != Some(TreeKind::FrameSpecBetweenRight) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Tree(TreeKind::FrameSpecUnboundedFollowing) => {
                    return Some(Self::FrameSpecUnboundedFollowing(
                        FrameSpecUnboundedFollowing::cast(child)?,
                    ));
                }
                CstNodeData::Tree(TreeKind::FrameSpecPreceding) => {
                    return Some(Self::FrameSpecPreceding(FrameSpecPreceding::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::FrameSpecCurrentRow) => {
                    return Some(Self::FrameSpecCurrentRow(FrameSpecCurrentRow::cast(child)?));
                }
                CstNodeData::Tree(TreeKind::FrameSpecFollowing) => {
                    return Some(Self::FrameSpecFollowing(FrameSpecFollowing::cast(child)?));
                }
                _ => return None,
            })
            .next()
    }
    pub fn frame_spec_unbounded_following(self) -> Option<FrameSpecUnboundedFollowing<'a>> {
        match self {
            Self::FrameSpecUnboundedFollowing(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_preceding(self) -> Option<FrameSpecPreceding<'a>> {
        match self {
            Self::FrameSpecPreceding(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_current_row(self) -> Option<FrameSpecCurrentRow<'a>> {
        match self {
            Self::FrameSpecCurrentRow(item) => Some(item),
            _ => None,
        }
    }
    pub fn frame_spec_following(self) -> Option<FrameSpecFollowing<'a>> {
        match self {
            Self::FrameSpecFollowing(item) => Some(item),
            _ => None,
        }
    }
}
pub enum ExprLit<'a> {
    INT_LIT(&'a SqliteToken),
    HEX_LIT(&'a SqliteToken),
    STR_LIT(&'a SqliteToken),
    REAL_LIT(&'a SqliteToken),
    BLOB_LIT(&'a SqliteToken),
    KW_NULL(&'a SqliteToken),
    KW_TRUE(&'a SqliteToken),
    KW_FALSE(&'a SqliteToken),
    KW_CURRENT_TIME(&'a SqliteToken),
    KW_CURRENT_DATE(&'a SqliteToken),
    KW_CURRENT_TIMESTAMP(&'a SqliteToken),
}
impl<'a> ExprLit<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        use SqliteTreeKind as TreeKind;
        if node.tree() != Some(TreeKind::ExprLit) {
            return None;
        }
        node.valid_children()
            .flat_map(|child| match child.data {
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::INT_LIT,
                        ..
                    },
                ) => return Some(Self::INT_LIT(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::HEX_LIT,
                        ..
                    },
                ) => return Some(Self::HEX_LIT(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::STR_LIT,
                        ..
                    },
                ) => return Some(Self::STR_LIT(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::REAL_LIT,
                        ..
                    },
                ) => return Some(Self::REAL_LIT(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::BLOB_LIT,
                        ..
                    },
                ) => return Some(Self::BLOB_LIT(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_NULL,
                        ..
                    },
                ) => return Some(Self::KW_NULL(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_TRUE,
                        ..
                    },
                ) => return Some(Self::KW_TRUE(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_FALSE,
                        ..
                    },
                ) => return Some(Self::KW_FALSE(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_CURRENT_TIME,
                        ..
                    },
                ) => return Some(Self::KW_CURRENT_TIME(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_CURRENT_DATE,
                        ..
                    },
                ) => return Some(Self::KW_CURRENT_DATE(&token)),
                CstNodeData::Token(
                    token @ SqliteToken {
                        kind: TokenKind::KW_CURRENT_TIMESTAMP,
                        ..
                    },
                ) => return Some(Self::KW_CURRENT_TIMESTAMP(&token)),
                _ => return None,
            })
            .next()
    }
    pub fn int_lit(self) -> Option<&'a SqliteToken> {
        match self {
            Self::INT_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn hex_lit(self) -> Option<&'a SqliteToken> {
        match self {
            Self::HEX_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn str_lit(self) -> Option<&'a SqliteToken> {
        match self {
            Self::STR_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn real_lit(self) -> Option<&'a SqliteToken> {
        match self {
            Self::REAL_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn blob_lit(self) -> Option<&'a SqliteToken> {
        match self {
            Self::BLOB_LIT(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_null(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_NULL(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_true(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_TRUE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_false(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_FALSE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_current_time(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_CURRENT_TIME(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_current_date(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_CURRENT_DATE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_current_timestamp(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_CURRENT_TIMESTAMP(item) => Some(item),
            _ => None,
        }
    }
}
pub enum FkFailKind<'a> {
    KW_DELETE(&'a SqliteToken),
    KW_UPDATE(&'a SqliteToken),
}
impl<'a> FkFailKind<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        use SqliteTokenKind as TokenKind;
        match node.data {
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_DELETE,
                    ..
                },
            ) => return Some(Self::KW_DELETE(&token)),
            CstNodeData::Token(
                token @ SqliteToken {
                    kind: TokenKind::KW_UPDATE,
                    ..
                },
            ) => return Some(Self::KW_UPDATE(&token)),
            _ => return None,
        }
    }
    pub fn kw_delete(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_DELETE(item) => Some(item),
            _ => None,
        }
    }
    pub fn kw_update(self) -> Option<&'a SqliteToken> {
        match self {
            Self::KW_UPDATE(item) => Some(item),
            _ => None,
        }
    }
}
pub struct SelectStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> SelectStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SelectStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn select_core(&self) -> Option<SelectCore<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SelectCore)
            .flat_map(SelectCore::cast)
            .next()
    }
    pub fn compound_selects(&self) -> impl Iterator<Item = CompoundSelect<'a>> {
        self.inner.valid_children().flat_map(CompoundSelect::cast)
    }
    pub fn order_by_clause(&self) -> Option<OrderByClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::OrderByClause)
            .flat_map(OrderByClause::cast)
            .next()
    }
    pub fn limit_clause(&self) -> Option<LimitClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::LimitClause)
            .flat_map(LimitClause::cast)
            .next()
    }
}
pub struct UpdateStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> UpdateStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UpdateStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn conflict_action(&self) -> Option<ConflictAction<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictAction)
            .flat_map(ConflictAction::cast)
            .next()
    }
    pub fn qualified_table_name(&self) -> Option<QualifiedTableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::QualifiedTableName)
            .flat_map(QualifiedTableName::cast)
            .next()
    }
    pub fn set_expressions(&self) -> impl Iterator<Item = SetColumnExpr<'a>> {
        self.inner.valid_children().flat_map(SetColumnExpr::cast)
    }
    pub fn from_clause(&self) -> Option<FromClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FromClause)
            .flat_map(FromClause::cast)
            .next()
    }
    pub fn where_clause(&self) -> Option<WhereClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
    pub fn returning_clause(&self) -> Option<ReturningClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ReturningClause)
            .flat_map(ReturningClause::cast)
            .next()
    }
    pub fn update_stmt_limited(&self) -> Option<UpdateStmtLimited<'a>> {
        self.inner
            .find_children(SqliteTreeKind::UpdateStmtLimited)
            .flat_map(UpdateStmtLimited::cast)
            .next()
    }
}
pub struct BeginStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> BeginStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::BeginStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn begin_stmt_kind(&self) -> Option<BeginStmtKind<'a>> {
        self.inner.children().flat_map(BeginStmtKind::cast).next()
    }
}
pub struct QualifiedTableName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> QualifiedTableName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::QualifiedTableName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn full_table_name(&self) -> Option<FullTableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
    pub fn with_alias(&self) -> Option<WithAlias<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WithAlias)
            .flat_map(WithAlias::cast)
            .next()
    }
    pub fn index_details(&self) -> Option<IndexDetails<'a>> {
        self.inner.children().flat_map(IndexDetails::cast).next()
    }
}
pub struct FkClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FkClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn table_name(&self) -> Option<TableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
    pub fn col_name_list(&self) -> Option<ColNameList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColNameList)
            .flat_map(ColNameList::cast)
            .next()
    }
    pub fn fk_actions(&self) -> impl Iterator<Item = FkViolateAction<'a>> {
        self.inner.valid_children().flat_map(FkViolateAction::cast)
    }
    pub fn fk_deferrable(&self) -> Option<FkDeferrable<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FkDeferrable)
            .flat_map(FkDeferrable::cast)
            .next()
    }
}
pub struct FullTableFunctionName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FullTableFunctionName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FullTableFunctionName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn schema_name(&self) -> Option<SchemaName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn table_func(&self) -> Option<TableFunctionName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TableFunctionName)
            .flat_map(TableFunctionName::cast)
            .next()
    }
}
pub struct TriggerForEachRow<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TriggerForEachRow<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerForEachRow) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct CteClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CteClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CteClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn recursive(&self) -> Option<&'a SqliteToken> {
        self.inner
            .find_children(SqliteTokenKind::KW_RECURSIVE)
            .map(|it| it.token().unwrap())
            .next()
    }
    pub fn expressions(&self) -> impl Iterator<Item = CommonTableExpr<'a>> {
        self.inner.valid_children().flat_map(CommonTableExpr::cast)
    }
}
pub struct FkMatchAction<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FkMatchAction<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkMatchAction) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn any_valid_name(&self) -> Option<AnyValidName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::AnyValidName)
            .flat_map(AnyValidName::cast)
            .next()
    }
}
pub struct NewColumnName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> NewColumnName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::NewColumnName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct GroupByClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> GroupByClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::GroupByClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn items(&self) -> impl Iterator<Item = Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(|it| it.children())
            .flat_map(Expr::cast)
    }
}
pub struct FrameSpecPreceding<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FrameSpecPreceding<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecPreceding) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct ViewName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ViewName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ViewName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct FkDeferrable<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FkDeferrable<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkDeferrable) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn not(&self) -> Option<&'a SqliteToken> {
        self.inner
            .find_children(SqliteTokenKind::KW_NOT)
            .map(|it| it.token().unwrap())
            .next()
    }
    pub fn defer_kind(&self) -> Option<DeferKind<'a>> {
        self.inner.children().flat_map(DeferKind::cast).next()
    }
}
pub struct CreateViewStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CreateViewStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CreateViewStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn temporary(&self) -> Option<Temporary<'a>> {
        self.inner.children().flat_map(Temporary::cast).next()
    }
    pub fn if_not_exists(&self) -> Option<IfNotExists<'a>> {
        self.inner
            .find_children(SqliteTreeKind::IfNotExists)
            .flat_map(IfNotExists::cast)
            .next()
    }
    pub fn full_view_name(&self) -> Option<FullViewName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullViewName)
            .flat_map(FullViewName::cast)
            .next()
    }
    pub fn col_name_list(&self) -> Option<ColNameList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColNameList)
            .flat_map(ColNameList::cast)
            .next()
    }
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
}
pub struct RenameColumn<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> RenameColumn<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RenameColumn) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn old_name(&self) -> Option<ColumnName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnName)
            .flat_map(ColumnName::cast)
            .next()
    }
    pub fn new_name(&self) -> Option<NewColumnName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::NewColumnName)
            .flat_map(NewColumnName::cast)
            .next()
    }
}
pub struct TableOrIdxNameWithSchema<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableOrIdxNameWithSchema<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableOrIdxNameWithSchema) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn schema_name(&self) -> Option<SchemaName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn table_or_index_name(&self) -> Option<TableOrIndexName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TableOrIndexName)
            .flat_map(TableOrIndexName::cast)
            .next()
    }
}
pub struct ModuleName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ModuleName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ModuleName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct TableConstraint<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableConstraint) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn constraint_name(&self) -> Option<ConstraintName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ConstraintName)
            .flat_map(ConstraintName::cast)
            .next()
    }
    pub fn table_constraint_kind(&self) -> Option<TableConstraintKind<'a>> {
        self.inner
            .children()
            .flat_map(TableConstraintKind::cast)
            .next()
    }
}
pub struct ColumnName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ColumnName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ColumnName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct ConstraintName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ConstraintName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ConstraintName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct UsingConstraint<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> UsingConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UsingConstraint) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn col_name_list(&self) -> Option<ColNameList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColNameList)
            .flat_map(ColNameList::cast)
            .next()
    }
}
pub struct Join<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> Join<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::Join) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct TableNameIndexedBy<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableNameIndexedBy<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableNameIndexedBy) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn index_name(&self) -> Option<IndexName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::IndexName)
            .flat_map(IndexName::cast)
            .next()
    }
}
pub struct SetColumnExpr<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> SetColumnExpr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SetColumnExpr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn set_column_kind(&self) -> Option<SetColumnKind<'a>> {
        self.inner.children().flat_map(SetColumnKind::cast).next()
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct PrimaryConstraint<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> PrimaryConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::PrimaryConstraint) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn primary_constraint_order(&self) -> Option<PrimaryConstraintOrder<'a>> {
        self.inner
            .children()
            .flat_map(PrimaryConstraintOrder::cast)
            .next()
    }
    pub fn conflict_clause(&self) -> Option<ConflictClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictClause)
            .flat_map(ConflictClause::cast)
            .next()
    }
    pub fn auto_increment(&self) -> Option<&'a SqliteToken> {
        self.inner
            .find_children(SqliteTokenKind::KW_AUTOINCREMENT)
            .map(|it| it.token().unwrap())
            .next()
    }
}
pub struct NotNullConstraint<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> NotNullConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::NotNullConstraint) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn conflict_clause(&self) -> Option<ConflictClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictClause)
            .flat_map(ConflictClause::cast)
            .next()
    }
}
pub struct DefaultConstraint<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> DefaultConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DefaultConstraint) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn default_value_kind(&self) -> Option<DefaultValueKind<'a>> {
        self.inner
            .children()
            .flat_map(DefaultValueKind::cast)
            .next()
    }
}
pub struct FkOnAction<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FkOnAction<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkOnAction) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn fk_fail_kind(&self) -> Option<FkFailKind<'a>> {
        self.inner.children().flat_map(FkFailKind::cast).next()
    }
    pub fn fk_action(&self) -> Option<FkAction<'a>> {
        self.inner.children().flat_map(FkAction::cast).next()
    }
}
pub struct FileNameExpr<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FileNameExpr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FileNameExpr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct ExprList<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ExprList<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprList) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn items(&self) -> impl Iterator<Item = Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(|it| it.children())
            .flat_map(Expr::cast)
    }
}
pub struct DropTableStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> DropTableStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DropTableStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn full_table_name(&self) -> Option<FullTableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
}
pub struct OrderingTermList<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OrderingTermList<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OrderingTermList) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn items(&self) -> impl Iterator<Item = OrderingTerm<'a>> {
        self.inner.valid_children().flat_map(OrderingTerm::cast)
    }
}
pub struct AnyValidName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> AnyValidName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::AnyValidName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct WindowName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> WindowName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WindowName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct OnConstraint<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OnConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OnConstraint) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn on_expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct CrossJoin<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CrossJoin<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CrossJoin) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct DefaultConstraintLiteral<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> DefaultConstraintLiteral<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DefaultConstraintLiteral) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct CreateTriggerStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CreateTriggerStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CreateTriggerStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn temporary(&self) -> Option<Temporary<'a>> {
        self.inner.children().flat_map(Temporary::cast).next()
    }
    pub fn if_not_exists(&self) -> Option<IfNotExists<'a>> {
        self.inner
            .find_children(SqliteTreeKind::IfNotExists)
            .flat_map(IfNotExists::cast)
            .next()
    }
    pub fn full_trigger_name(&self) -> Option<FullTriggerName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullTriggerName)
            .flat_map(FullTriggerName::cast)
            .next()
    }
    pub fn trigger_when(&self) -> Option<TriggerWhen<'a>> {
        self.inner.children().flat_map(TriggerWhen::cast).next()
    }
    pub fn trigger_action_kind(&self) -> Option<TriggerActionKind<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerActionKind)
            .flat_map(TriggerActionKind::cast)
            .next()
    }
    pub fn table_name(&self) -> Option<TableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
    pub fn trigger_for_each_row(&self) -> Option<TriggerForEachRow<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerForEachRow)
            .flat_map(TriggerForEachRow::cast)
            .next()
    }
    pub fn trigger_when_expr(&self) -> Option<TriggerWhenExpr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerWhenExpr)
            .flat_map(TriggerWhenExpr::cast)
            .next()
    }
    pub fn trigger_body_stmt_list(&self) -> Option<TriggerBodyStmtList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerBodyStmtList)
            .flat_map(TriggerBodyStmtList::cast)
            .next()
    }
}
pub struct StatementWithCte<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> StatementWithCte<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::StatementWithCte) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn cte_clause(&self) -> Option<CteClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::CteClause)
            .flat_map(CteClause::cast)
            .next()
    }
    pub fn cte_prependable(&self) -> Option<CtePrependable<'a>> {
        self.inner.children().flat_map(CtePrependable::cast).next()
    }
}
pub struct FunctionName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FunctionName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FunctionName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct WindowBaseName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> WindowBaseName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WindowBaseName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct OrderingTerm<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OrderingTerm<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OrderingTerm) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn collation(&self) -> Option<Collation<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Collation)
            .flat_map(Collation::cast)
            .next()
    }
    pub fn order(&self) -> Option<Order<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Order)
            .flat_map(Order::cast)
            .next()
    }
    pub fn nulls_position(&self) -> Option<NullsPosition<'a>> {
        self.inner.children().flat_map(NullsPosition::cast).next()
    }
}
pub struct ExprSelect<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ExprSelect<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprSelect) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn not(&self) -> Option<&'a SqliteToken> {
        self.inner
            .find_children(SqliteTokenKind::KW_NOT)
            .map(|it| it.token().unwrap())
            .next()
    }
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
}
pub struct ExprCase<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ExprCase<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprCase) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn case_target_expr(&self) -> Option<CaseTargetExpr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::CaseTargetExpr)
            .flat_map(CaseTargetExpr::cast)
            .next()
    }
    pub fn case_when_clause_list(&self) -> Option<CaseWhenClauseList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::CaseWhenClauseList)
            .flat_map(CaseWhenClauseList::cast)
            .next()
    }
    pub fn case_else_clause(&self) -> Option<CaseElseClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::CaseElseClause)
            .flat_map(CaseElseClause::cast)
            .next()
    }
}
pub struct WindowClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> WindowClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WindowClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn functions(&self) -> impl Iterator<Item = WindowFunction<'a>> {
        self.inner.valid_children().flat_map(WindowFunction::cast)
    }
}
pub struct TableDetails<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableDetails<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableDetails) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn columns(&self) -> impl Iterator<Item = ColumnDef<'a>> {
        self.inner.valid_children().flat_map(ColumnDef::cast)
    }
    pub fn table_constraints(&self) -> impl Iterator<Item = TableConstraint<'a>> {
        self.inner.valid_children().flat_map(TableConstraint::cast)
    }
    pub fn table_options_list(&self) -> Option<TableOptionsList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TableOptionsList)
            .flat_map(TableOptionsList::cast)
            .next()
    }
}
pub struct RaiseActionFail<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> RaiseActionFail<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RaiseActionFail) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn raise_func_err_message(&self) -> Option<RaiseFuncErrMessage<'a>> {
        self.inner
            .find_children(SqliteTreeKind::RaiseFuncErrMessage)
            .flat_map(RaiseFuncErrMessage::cast)
            .next()
    }
}
pub struct CreateIndexStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CreateIndexStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CreateIndexStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn unique(&self) -> Option<&'a SqliteToken> {
        self.inner
            .find_children(SqliteTokenKind::KW_UNIQUE)
            .map(|it| it.token().unwrap())
            .next()
    }
    pub fn if_not_exists(&self) -> Option<IfNotExists<'a>> {
        self.inner
            .find_children(SqliteTreeKind::IfNotExists)
            .flat_map(IfNotExists::cast)
            .next()
    }
    pub fn full_index_name(&self) -> Option<FullIndexName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullIndexName)
            .flat_map(FullIndexName::cast)
            .next()
    }
    pub fn table_name(&self) -> Option<TableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
    pub fn indexed_col_list(&self) -> Option<IndexedColList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::IndexedColList)
            .flat_map(IndexedColList::cast)
            .next()
    }
    pub fn where_clause(&self) -> Option<WhereClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
}
pub struct SchemaNameExpr<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> SchemaNameExpr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SchemaNameExpr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct SchemaName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> SchemaName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SchemaName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct CommaJoin<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CommaJoin<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CommaJoin) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct FrameSpecExcludeClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FrameSpecExcludeClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecExcludeClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn frame_spec_exclude_kind(&self) -> Option<FrameSpecExcludeKind<'a>> {
        self.inner
            .children()
            .flat_map(FrameSpecExcludeKind::cast)
            .next()
    }
}
pub struct InTableFunc<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> InTableFunc<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InTableFunc) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn full_table_function_name(&self) -> Option<FullTableFunctionName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableFunctionName)
            .flat_map(FullTableFunctionName::cast)
            .next()
    }
    pub fn items(&self) -> impl Iterator<Item = Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(|it| it.children())
            .flat_map(Expr::cast)
    }
}
pub struct ColNameList<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ColNameList<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ColNameList) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn items(&self) -> impl Iterator<Item = ColumnName<'a>> {
        self.inner.valid_children().flat_map(ColumnName::cast)
    }
}
pub struct CreateVirtualTableStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CreateVirtualTableStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CreateVirtualTableStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn if_not_exists(&self) -> Option<IfNotExists<'a>> {
        self.inner
            .find_children(SqliteTreeKind::IfNotExists)
            .flat_map(IfNotExists::cast)
            .next()
    }
    pub fn full_table_name(&self) -> Option<FullTableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
    pub fn module_name(&self) -> Option<ModuleName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ModuleName)
            .flat_map(ModuleName::cast)
            .next()
    }
    pub fn module_arg_list(&self) -> Option<ModuleArgList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ModuleArgList)
            .flat_map(ModuleArgList::cast)
            .next()
    }
}
pub struct TableOptWithoutRowId<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableOptWithoutRowId<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableOptWithoutRowId) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct TriggerName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TriggerName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct FullPragmaName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FullPragmaName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FullPragmaName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn schema_name(&self) -> Option<SchemaName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn pragma(&self) -> Option<PragmaName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::PragmaName)
            .flat_map(PragmaName::cast)
            .next()
    }
}
pub struct ValuesClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ValuesClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ValuesClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr_list(&self) -> Option<ExprList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ExprList)
            .flat_map(ExprList::cast)
            .next()
    }
}
pub struct File<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> File<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::File) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn statements(&self) -> impl Iterator<Item = Statement<'a>> {
        self.inner.valid_children().flat_map(Statement::cast)
    }
}
pub struct TableUqConstraint<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableUqConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableUqConstraint) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn indexed_col_list(&self) -> Option<IndexedColList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::IndexedColList)
            .flat_map(IndexedColList::cast)
            .next()
    }
    pub fn conflict_clause(&self) -> Option<ConflictClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictClause)
            .flat_map(ConflictClause::cast)
            .next()
    }
}
pub struct FkRestrict<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FkRestrict<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkRestrict) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct OpNotNull<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OpNotNull<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotNull) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct OpIn<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OpIn<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpIn) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn in_expr_kind(&self) -> Option<InExprKind<'a>> {
        self.inner.children().flat_map(InExprKind::cast).next()
    }
}
pub struct FrameSpec<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FrameSpec<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpec) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn range(&self) -> Option<Range<'a>> {
        self.inner.children().flat_map(Range::cast).next()
    }
    pub fn frame_spec_kind(&self) -> Option<FrameSpecKind<'a>> {
        self.inner.children().flat_map(FrameSpecKind::cast).next()
    }
    pub fn frame_spec_exclude_clause(&self) -> Option<FrameSpecExcludeClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FrameSpecExcludeClause)
            .flat_map(FrameSpecExcludeClause::cast)
            .next()
    }
}
pub struct OpNotIn<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OpNotIn<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotIn) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn not(&self) -> Option<&'a SqliteToken> {
        self.inner
            .find_children(SqliteTokenKind::KW_NOT)
            .map(|it| it.token().unwrap())
            .next()
    }
    pub fn in_expr_kind(&self) -> Option<InExprKind<'a>> {
        self.inner.children().flat_map(InExprKind::cast).next()
    }
}
pub struct DropTriggerStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> DropTriggerStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DropTriggerStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn full_trigger_name(&self) -> Option<FullTriggerName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullTriggerName)
            .flat_map(FullTriggerName::cast)
            .next()
    }
}
pub struct OpCollate<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OpCollate<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpCollate) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn collation(&self) -> Option<Collation<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Collation)
            .flat_map(Collation::cast)
            .next()
    }
}
pub struct IndexedColList<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> IndexedColList<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::IndexedColList) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn items(&self) -> impl Iterator<Item = IndexedCol<'a>> {
        self.inner.valid_children().flat_map(IndexedCol::cast)
    }
}
pub struct TableOrIdxOrCollationName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableOrIdxOrCollationName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableOrIdxOrCollationName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct FkSetNull<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FkSetNull<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkSetNull) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct SelectStmtWithCte<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> SelectStmtWithCte<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SelectStmtWithCte) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn cte_clause(&self) -> Option<CteClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::CteClause)
            .flat_map(CteClause::cast)
            .next()
    }
    pub fn select_stmt(&self) -> Option<SelectStmt<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmt)
            .flat_map(SelectStmt::cast)
            .next()
    }
}
pub struct RaiseActionRollBack<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> RaiseActionRollBack<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RaiseActionRollBack) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn raise_func_err_message(&self) -> Option<RaiseFuncErrMessage<'a>> {
        self.inner
            .find_children(SqliteTreeKind::RaiseFuncErrMessage)
            .flat_map(RaiseFuncErrMessage::cast)
            .next()
    }
}
pub struct ArgExpr<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ArgExpr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ArgExpr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn distinct(&self) -> Option<&'a SqliteToken> {
        self.inner
            .find_children(SqliteTokenKind::KW_DISTINCT)
            .map(|it| it.token().unwrap())
            .next()
    }
    pub fn items(&self) -> impl Iterator<Item = Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(|it| it.children())
            .flat_map(Expr::cast)
    }
    pub fn order_by_clause(&self) -> Option<OrderByClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::OrderByClause)
            .flat_map(OrderByClause::cast)
            .next()
    }
}
pub struct AnalyzeStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> AnalyzeStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::AnalyzeStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn analyze_target(&self) -> Option<AnalyzeTarget<'a>> {
        self.inner.children().flat_map(AnalyzeTarget::cast).next()
    }
}
pub struct CaseTargetExpr<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CaseTargetExpr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CaseTargetExpr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct InsertValuesClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> InsertValuesClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InsertValuesClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr_lists(&self) -> impl Iterator<Item = ExprList<'a>> {
        self.inner.valid_children().flat_map(ExprList::cast)
    }
    pub fn upsert_clauses(&self) -> impl Iterator<Item = UpsertClause<'a>> {
        self.inner.valid_children().flat_map(UpsertClause::cast)
    }
}
pub struct ColumnConstraint<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ColumnConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ColumnConstraint) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn column_constraint_name(&self) -> Option<ColumnConstraintName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnConstraintName)
            .flat_map(ColumnConstraintName::cast)
            .next()
    }
    pub fn constraint_type(&self) -> Option<ConstraintType<'a>> {
        self.inner.children().flat_map(ConstraintType::cast).next()
    }
}
pub struct OrderByClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OrderByClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OrderByClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn ordering_term_list(&self) -> Option<OrderingTermList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::OrderingTermList)
            .flat_map(OrderingTermList::cast)
            .next()
    }
}
pub struct ExprParen<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ExprParen<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprParen) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct OpNotLike<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OpNotLike<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotLike) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn escape(&self) -> Option<&'a SqliteToken> {
        self.inner
            .find_children(SqliteTokenKind::STR_LIT)
            .map(|it| it.token().unwrap())
            .next()
    }
}
pub struct FrameSpecUnboundedPreceding<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FrameSpecUnboundedPreceding<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecUnboundedPreceding) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct FullTriggerName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FullTriggerName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FullTriggerName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn schema_name(&self) -> Option<SchemaName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn trigger(&self) -> Option<TriggerName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerName)
            .flat_map(TriggerName::cast)
            .next()
    }
}
pub struct CreateTableStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CreateTableStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CreateTableStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn temporary(&self) -> Option<Temporary<'a>> {
        self.inner.children().flat_map(Temporary::cast).next()
    }
    pub fn if_not_exists(&self) -> Option<IfNotExists<'a>> {
        self.inner
            .find_children(SqliteTreeKind::IfNotExists)
            .flat_map(IfNotExists::cast)
            .next()
    }
    pub fn full_table_name(&self) -> Option<FullTableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
    pub fn table_columns(&self) -> Option<TableColumns<'a>> {
        self.inner.children().flat_map(TableColumns::cast).next()
    }
}
pub struct FullTableName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FullTableName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FullTableName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn schema_name(&self) -> Option<SchemaName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn table(&self) -> Option<TableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
}
pub struct LimitClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> LimitClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::LimitClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn offset(&self) -> Option<Offset<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Offset)
            .flat_map(Offset::cast)
            .next()
    }
}
pub struct TableNameNotIndexed<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableNameNotIndexed<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableNameNotIndexed) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct ConflictClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ConflictClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ConflictClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn conflict_action(&self) -> Option<ConflictAction<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictAction)
            .flat_map(ConflictAction::cast)
            .next()
    }
}
pub struct TableFunctionName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableFunctionName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableFunctionName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct TriggerUpdateAction<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TriggerUpdateAction<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerUpdateAction) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn trigger_update_affect_cols(&self) -> Option<TriggerUpdateAffectCols<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerUpdateAffectCols)
            .flat_map(TriggerUpdateAffectCols::cast)
            .next()
    }
}
pub struct UpdateStmtLimited<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> UpdateStmtLimited<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UpdateStmtLimited) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn order_by_clause(&self) -> Option<OrderByClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::OrderByClause)
            .flat_map(OrderByClause::cast)
            .next()
    }
    pub fn limit_clause(&self) -> Option<LimitClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::LimitClause)
            .flat_map(LimitClause::cast)
            .next()
    }
}
pub struct CreateTableSelect<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CreateTableSelect<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CreateTableSelect) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
}
pub struct FkSetDefault<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FkSetDefault<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkSetDefault) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct UnionCompoundOperator<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> UnionCompoundOperator<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UnionCompoundOperator) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn all(&self) -> Option<&'a SqliteToken> {
        self.inner
            .find_children(SqliteTokenKind::KW_ALL)
            .map(|it| it.token().unwrap())
            .next()
    }
}
pub struct FrameSpecUnboundedFollowing<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FrameSpecUnboundedFollowing<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecUnboundedFollowing) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct TriggerBodyStmtList<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TriggerBodyStmtList<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerBodyStmtList) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn items(&self) -> impl Iterator<Item = TriggerBodyStmt<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TriggerBodyStmt)
            .flat_map(|it| it.children())
            .flat_map(TriggerBodyStmt::cast)
    }
}
pub struct CompoundSelect<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CompoundSelect<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CompoundSelect) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn compound_operator(&self) -> Option<CompoundOperator<'a>> {
        self.inner
            .find_children(SqliteTreeKind::CompoundOperator)
            .flat_map(CompoundOperator::cast)
            .next()
    }
    pub fn select_core(&self) -> Option<SelectCore<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SelectCore)
            .flat_map(SelectCore::cast)
            .next()
    }
}
pub struct DbNameExpr<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> DbNameExpr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DbNameExpr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct ColumnConstraintName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ColumnConstraintName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ColumnConstraintName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn constraint_name(&self) -> Option<ConstraintName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ConstraintName)
            .flat_map(ConstraintName::cast)
            .next()
    }
}
pub struct ExprBindParam<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ExprBindParam<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprBindParam) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct FilterClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FilterClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FilterClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn where_clause(&self) -> Option<WhereClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
}
pub struct OpUnaryPlus<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OpUnaryPlus<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpUnaryPlus) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct DeleteStmtLimited<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> DeleteStmtLimited<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DeleteStmtLimited) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn order_by_clause(&self) -> Option<OrderByClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::OrderByClause)
            .flat_map(OrderByClause::cast)
            .next()
    }
    pub fn limit_clause(&self) -> Option<LimitClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::LimitClause)
            .flat_map(LimitClause::cast)
            .next()
    }
}
pub struct SchemaOrIdxOrTableName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> SchemaOrIdxOrTableName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SchemaOrIdxOrTableName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct DefaultConstraintExpr<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> DefaultConstraintExpr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DefaultConstraintExpr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct TriggerInsteadOf<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TriggerInsteadOf<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerInsteadOf) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct FullIndexName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FullIndexName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FullIndexName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn schema_name(&self) -> Option<SchemaName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn index_name(&self) -> Option<IndexName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::IndexName)
            .flat_map(IndexName::cast)
            .next()
    }
}
pub struct InsertOrAction<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> InsertOrAction<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InsertOrAction) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn conflict_action(&self) -> Option<ConflictAction<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictAction)
            .flat_map(ConflictAction::cast)
            .next()
    }
}
pub struct UpsertClauseConflictTarget<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> UpsertClauseConflictTarget<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UpsertClauseConflictTarget) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn indexed_col_list(&self) -> Option<IndexedColList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::IndexedColList)
            .flat_map(IndexedColList::cast)
            .next()
    }
    pub fn where_clause(&self) -> Option<WhereClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
}
pub struct ExprFunc<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ExprFunc<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprFunc) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn function_name(&self) -> Option<FunctionName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FunctionName)
            .flat_map(FunctionName::cast)
            .next()
    }
    pub fn func_arguments(&self) -> Option<FuncArguments<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FuncArguments)
            .flat_map(FuncArguments::cast)
            .next()
    }
    pub fn filter_clause(&self) -> Option<FilterClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FilterClause)
            .flat_map(FilterClause::cast)
            .next()
    }
    pub fn over_clause(&self) -> Option<OverClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::OverClause)
            .flat_map(OverClause::cast)
            .next()
    }
}
pub struct ArgStar<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ArgStar<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ArgStar) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct ResultColumnList<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ResultColumnList<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ResultColumnList) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn items(&self) -> impl Iterator<Item = ResultColumn<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ResultColumn)
            .flat_map(|it| it.children())
            .flat_map(ResultColumn::cast)
    }
}
pub struct OpNotSpaceNull<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OpNotSpaceNull<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotSpaceNull) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct SavepointStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> SavepointStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SavepointStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn savepoint_name(&self) -> Option<SavepointName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SavepointName)
            .flat_map(SavepointName::cast)
            .next()
    }
}
pub struct NewTableName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> NewTableName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::NewTableName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct NaturalJoin<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> NaturalJoin<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::NaturalJoin) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn natural_join_kind(&self) -> Option<NaturalJoinKind<'a>> {
        self.inner.children().flat_map(NaturalJoinKind::cast).next()
    }
}
pub struct CteName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CteName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CteName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct FkViolateAction<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FkViolateAction<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkViolateAction) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn fk_on_or_match(&self) -> Option<FkOnOrMatch<'a>> {
        self.inner.children().flat_map(FkOnOrMatch::cast).next()
    }
}
pub struct DropViewStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> DropViewStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DropViewStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn full_view_name(&self) -> Option<FullViewName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullViewName)
            .flat_map(FullViewName::cast)
            .next()
    }
}
pub struct TableName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct InTable<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> InTable<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InTable) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn full_table_name(&self) -> Option<FullTableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
}
pub struct ResultColumnAll<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ResultColumnAll<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ResultColumnAll) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct DeleteStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> DeleteStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DeleteStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn qualified_table_name(&self) -> Option<QualifiedTableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::QualifiedTableName)
            .flat_map(QualifiedTableName::cast)
            .next()
    }
    pub fn where_clause(&self) -> Option<WhereClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
    pub fn returning_clause(&self) -> Option<ReturningClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ReturningClause)
            .flat_map(ReturningClause::cast)
            .next()
    }
    pub fn delete_stmt_limited(&self) -> Option<DeleteStmtLimited<'a>> {
        self.inner
            .find_children(SqliteTreeKind::DeleteStmtLimited)
            .flat_map(DeleteStmtLimited::cast)
            .next()
    }
}
pub struct TablePkConstraint<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TablePkConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TablePkConstraint) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn indexed_col_list(&self) -> Option<IndexedColList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::IndexedColList)
            .flat_map(IndexedColList::cast)
            .next()
    }
    pub fn conflict_clause(&self) -> Option<ConflictClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictClause)
            .flat_map(ConflictClause::cast)
            .next()
    }
}
pub struct RaiseFunc<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> RaiseFunc<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RaiseFunc) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn raise_action(&self) -> Option<RaiseAction<'a>> {
        self.inner
            .find_children(SqliteTreeKind::RaiseAction)
            .flat_map(RaiseAction::cast)
            .next()
    }
}
pub struct RenameTable<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> RenameTable<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RenameTable) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn table_name(&self) -> Option<TableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
}
pub struct CommitStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CommitStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CommitStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn commit_start_kw(&self) -> Option<CommitStartKw<'a>> {
        self.inner.children().flat_map(CommitStartKw::cast).next()
    }
}
pub struct CheckConstraint<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CheckConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CheckConstraint) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct DropIndexStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> DropIndexStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DropIndexStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn full_index_name(&self) -> Option<FullIndexName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullIndexName)
            .flat_map(FullIndexName::cast)
            .next()
    }
}
pub struct ColumnGenerated<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ColumnGenerated<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ColumnGenerated) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn column_generated_kind(&self) -> Option<ColumnGeneratedKind<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnGeneratedKind)
            .flat_map(ColumnGeneratedKind::cast)
            .next()
    }
}
pub struct ExprCast<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ExprCast<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprCast) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn type_name(&self) -> Option<TypeName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TypeName)
            .flat_map(TypeName::cast)
            .next()
    }
}
pub struct AttachDbStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> AttachDbStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::AttachDbStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn file_name_expr(&self) -> Option<FileNameExpr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FileNameExpr)
            .flat_map(FileNameExpr::cast)
            .next()
    }
    pub fn schema_name_expr(&self) -> Option<SchemaNameExpr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaNameExpr)
            .flat_map(SchemaNameExpr::cast)
            .next()
    }
    pub fn password_expr(&self) -> Option<PasswordExpr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::PasswordExpr)
            .flat_map(PasswordExpr::cast)
            .next()
    }
}
pub struct WhereClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> WhereClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WhereClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct InsertStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> InsertStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InsertStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn insert_stmt_kind(&self) -> Option<InsertStmtKind<'a>> {
        self.inner
            .find_children(SqliteTreeKind::InsertStmtKind)
            .flat_map(InsertStmtKind::cast)
            .next()
    }
    pub fn full_table_name(&self) -> Option<FullTableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
    pub fn with_alias(&self) -> Option<WithAlias<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WithAlias)
            .flat_map(WithAlias::cast)
            .next()
    }
    pub fn col_name_list(&self) -> Option<ColNameList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColNameList)
            .flat_map(ColNameList::cast)
            .next()
    }
    pub fn insert_value_kind(&self) -> Option<InsertValueKind<'a>> {
        self.inner
            .find_children(SqliteTreeKind::InsertValueKind)
            .flat_map(InsertValueKind::cast)
            .next()
    }
    pub fn returning_clause(&self) -> Option<ReturningClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ReturningClause)
            .flat_map(ReturningClause::cast)
            .next()
    }
}
pub struct InsertDefaultValuesClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> InsertDefaultValuesClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InsertDefaultValuesClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct FromClauseSelectStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FromClauseSelectStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FromClauseSelectStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
    pub fn with_alias(&self) -> Option<WithAlias<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WithAlias)
            .flat_map(WithAlias::cast)
            .next()
    }
}
pub struct IndexName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> IndexName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::IndexName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct ReIndexStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ReIndexStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ReIndexStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn target(&self) -> Option<Target<'a>> {
        self.inner.children().flat_map(Target::cast).next()
    }
}
pub struct OverClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OverClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OverClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn over_clause_kind(&self) -> Option<OverClauseKind<'a>> {
        self.inner.children().flat_map(OverClauseKind::cast).next()
    }
}
pub struct RaiseActionAbort<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> RaiseActionAbort<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RaiseActionAbort) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn raise_func_err_message(&self) -> Option<RaiseFuncErrMessage<'a>> {
        self.inner
            .find_children(SqliteTreeKind::RaiseFuncErrMessage)
            .flat_map(RaiseFuncErrMessage::cast)
            .next()
    }
}
pub struct DropColumn<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> DropColumn<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DropColumn) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn column_name(&self) -> Option<ColumnName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnName)
            .flat_map(ColumnName::cast)
            .next()
    }
}
pub struct TypeNameWord<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TypeNameWord<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TypeNameWord) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct FrameSpecBetweenClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FrameSpecBetweenClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecBetweenClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn frame_spec_between_left(&self) -> Option<FrameSpecBetweenLeft<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FrameSpecBetweenLeft)
            .flat_map(FrameSpecBetweenLeft::cast)
            .next()
    }
    pub fn frame_spec_between_right(&self) -> Option<FrameSpecBetweenRight<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FrameSpecBetweenRight)
            .flat_map(FrameSpecBetweenRight::cast)
            .next()
    }
}
pub struct CollationName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CollationName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CollationName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct TraditionalSelect<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TraditionalSelect<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TraditionalSelect) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn select_distinct(&self) -> Option<SelectDistinct<'a>> {
        self.inner.children().flat_map(SelectDistinct::cast).next()
    }
    pub fn result_column_list(&self) -> Option<ResultColumnList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ResultColumnList)
            .flat_map(ResultColumnList::cast)
            .next()
    }
    pub fn from_clause(&self) -> Option<FromClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FromClause)
            .flat_map(FromClause::cast)
            .next()
    }
    pub fn where_clause(&self) -> Option<WhereClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
    pub fn group_by_clause(&self) -> Option<GroupByClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::GroupByClause)
            .flat_map(GroupByClause::cast)
            .next()
    }
    pub fn having_clause(&self) -> Option<HavingClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::HavingClause)
            .flat_map(HavingClause::cast)
            .next()
    }
    pub fn window_clause(&self) -> Option<WindowClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WindowClause)
            .flat_map(WindowClause::cast)
            .next()
    }
}
pub struct PasswordExpr<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> PasswordExpr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::PasswordExpr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct InsertSelectClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> InsertSelectClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InsertSelectClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
    pub fn upsert_clause(&self) -> Option<UpsertClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::UpsertClause)
            .flat_map(UpsertClause::cast)
            .next()
    }
}
pub struct OpUnaryMinus<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OpUnaryMinus<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpUnaryMinus) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct FullViewName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FullViewName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FullViewName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn schema_name(&self) -> Option<SchemaName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn view_name(&self) -> Option<ViewName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ViewName)
            .flat_map(ViewName::cast)
            .next()
    }
}
pub struct HavingClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> HavingClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::HavingClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct ResultColumnExpr<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ResultColumnExpr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ResultColumnExpr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn with_alias(&self) -> Option<WithAlias<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WithAlias)
            .flat_map(WithAlias::cast)
            .next()
    }
}
pub struct OpNot<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OpNot<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNot) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct OpIsNull<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OpIsNull<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpIsNull) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct IndexedCol<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> IndexedCol<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::IndexedCol) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn index_column(&self) -> Option<IndexColumn<'a>> {
        self.inner.children().flat_map(IndexColumn::cast).next()
    }
    pub fn collation(&self) -> Option<Collation<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Collation)
            .flat_map(Collation::cast)
            .next()
    }
    pub fn order(&self) -> Option<Order<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Order)
            .flat_map(Order::cast)
            .next()
    }
}
pub struct WithAlias<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> WithAlias<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WithAlias) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn alias_name(&self) -> Option<AliasName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::AliasName)
            .flat_map(AliasName::cast)
            .next()
    }
}
pub struct MaterializedCte<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> MaterializedCte<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::MaterializedCte) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn not(&self) -> Option<&'a SqliteToken> {
        self.inner
            .find_children(SqliteTokenKind::KW_NOT)
            .map(|it| it.token().unwrap())
            .next()
    }
}
pub struct FromClauseTableValueFunction<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FromClauseTableValueFunction<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FromClauseTableValueFunction) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn full_table_function_name(&self) -> Option<FullTableFunctionName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableFunctionName)
            .flat_map(FullTableFunctionName::cast)
            .next()
    }
    pub fn expr_list(&self) -> Option<ExprList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ExprList)
            .flat_map(ExprList::cast)
            .next()
    }
    pub fn with_alias(&self) -> Option<WithAlias<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WithAlias)
            .flat_map(WithAlias::cast)
            .next()
    }
}
pub struct FrameSpecFollowing<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FrameSpecFollowing<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecFollowing) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct InnerJoin<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> InnerJoin<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InnerJoin) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn natural(&self) -> Option<&'a SqliteToken> {
        self.inner
            .find_children(SqliteTokenKind::KW_INNER)
            .map(|it| it.token().unwrap())
            .next()
    }
}
pub struct ModuleArgList<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ModuleArgList<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ModuleArgList) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn items(&self) -> impl Iterator<Item = ModuleArg<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ModuleArg)
            .flat_map(|it| it.children())
            .flat_map(ModuleArg::cast)
    }
}
pub struct ColumnDef<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ColumnDef<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ColumnDef) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn column_name(&self) -> Option<ColumnName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnName)
            .flat_map(ColumnName::cast)
            .next()
    }
    pub fn type_name(&self) -> Option<TypeName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TypeName)
            .flat_map(TypeName::cast)
            .next()
    }
    pub fn constraints(&self) -> impl Iterator<Item = ColumnConstraint<'a>> {
        self.inner.valid_children().flat_map(ColumnConstraint::cast)
    }
}
pub struct TriggerUpdateAffectCols<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TriggerUpdateAffectCols<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerUpdateAffectCols) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn columns(&self) -> impl Iterator<Item = ColumnName<'a>> {
        self.inner.valid_children().flat_map(ColumnName::cast)
    }
}
pub struct AliasName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> AliasName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::AliasName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct CommonTableExpr<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CommonTableExpr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CommonTableExpr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn cte_name(&self) -> Option<CteName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::CteName)
            .flat_map(CteName::cast)
            .next()
    }
    pub fn col_name_list(&self) -> Option<ColNameList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColNameList)
            .flat_map(ColNameList::cast)
            .next()
    }
    pub fn materialized_cte(&self) -> Option<MaterializedCte<'a>> {
        self.inner
            .find_children(SqliteTreeKind::MaterializedCte)
            .flat_map(MaterializedCte::cast)
            .next()
    }
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
}
pub struct VacuumStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> VacuumStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::VacuumStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn schema_name(&self) -> Option<SchemaName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct TableFkConstraint<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableFkConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableFkConstraint) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn col_name_list(&self) -> Option<ColNameList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColNameList)
            .flat_map(ColNameList::cast)
            .next()
    }
    pub fn fk_clause(&self) -> Option<FkClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FkClause)
            .flat_map(FkClause::cast)
            .next()
    }
}
pub struct FkCascade<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FkCascade<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkCascade) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct WindowFunction<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> WindowFunction<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WindowFunction) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn window_name(&self) -> Option<WindowName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WindowName)
            .flat_map(WindowName::cast)
            .next()
    }
    pub fn window_def(&self) -> Option<WindowDef<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WindowDef)
            .flat_map(WindowDef::cast)
            .next()
    }
}
pub struct ReturningClauseExpr<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ReturningClauseExpr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ReturningClauseExpr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
    pub fn col_alias(&self) -> Option<AliasName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::AliasName)
            .flat_map(AliasName::cast)
            .next()
    }
}
pub struct UpsertClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> UpsertClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UpsertClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn upsert_clause_conflict_target(&self) -> Option<UpsertClauseConflictTarget<'a>> {
        self.inner
            .find_children(SqliteTreeKind::UpsertClauseConflictTarget)
            .flat_map(UpsertClauseConflictTarget::cast)
            .next()
    }
    pub fn upsert_clause_action(&self) -> Option<UpsertClauseAction<'a>> {
        self.inner
            .children()
            .flat_map(UpsertClauseAction::cast)
            .next()
    }
}
pub struct WindowDef<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> WindowDef<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WindowDef) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn window_base_name(&self) -> Option<WindowBaseName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WindowBaseName)
            .flat_map(WindowBaseName::cast)
            .next()
    }
    pub fn window_partition_by_clause(&self) -> Option<WindowPartitionByClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WindowPartitionByClause)
            .flat_map(WindowPartitionByClause::cast)
            .next()
    }
    pub fn order_by_clause(&self) -> Option<OrderByClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::OrderByClause)
            .flat_map(OrderByClause::cast)
            .next()
    }
    pub fn frame_spec(&self) -> Option<FrameSpec<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FrameSpec)
            .flat_map(FrameSpec::cast)
            .next()
    }
}
pub struct FkNoAction<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FkNoAction<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FkNoAction) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct WindowPartitionByClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> WindowPartitionByClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::WindowPartitionByClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn items(&self) -> impl Iterator<Item = Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(|it| it.children())
            .flat_map(Expr::cast)
    }
}
pub struct CaseWhenClauseList<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CaseWhenClauseList<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CaseWhenClauseList) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn items(&self) -> impl Iterator<Item = CaseWhenClause<'a>> {
        self.inner.valid_children().flat_map(CaseWhenClause::cast)
    }
}
pub struct RollbackStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> RollbackStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RollbackStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn savepoint_name(&self) -> Option<SavepointName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SavepointName)
            .flat_map(SavepointName::cast)
            .next()
    }
}
pub struct PragmaName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> PragmaName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::PragmaName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct ExplainClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ExplainClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExplainClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct TriggerWhenExpr<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TriggerWhenExpr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TriggerWhenExpr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct ReturningClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ReturningClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ReturningClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn columns(&self) -> impl Iterator<Item = ReturningClauseKind<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ReturningClauseKind)
            .flat_map(|it| it.children())
            .flat_map(ReturningClauseKind::cast)
    }
}
pub struct IfNotExists<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> IfNotExists<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::IfNotExists) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct UpsertDoUpdate<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> UpsertDoUpdate<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UpsertDoUpdate) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn set_expressions(&self) -> impl Iterator<Item = SetColumnExpr<'a>> {
        self.inner.valid_children().flat_map(SetColumnExpr::cast)
    }
    pub fn where_clause(&self) -> Option<WhereClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::WhereClause)
            .flat_map(WhereClause::cast)
            .next()
    }
}
pub struct FrameSpecCurrentRow<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FrameSpecCurrentRow<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecCurrentRow) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct ReleaseStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ReleaseStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ReleaseStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn savepoint_name(&self) -> Option<SavepointName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SavepointName)
            .flat_map(SavepointName::cast)
            .next()
    }
}
pub struct AlterTableStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> AlterTableStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::AlterTableStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn full_table_name(&self) -> Option<FullTableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::FullTableName)
            .flat_map(FullTableName::cast)
            .next()
    }
    pub fn alter_table_kind(&self) -> Option<AlterTableKind<'a>> {
        self.inner.children().flat_map(AlterTableKind::cast).next()
    }
}
pub struct ExprColumnName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ExprColumnName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ExprColumnName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn schema_name(&self) -> Option<SchemaName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SchemaName)
            .flat_map(SchemaName::cast)
            .next()
    }
    pub fn table_name(&self) -> Option<TableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
    pub fn column_name(&self) -> Option<ColumnName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnName)
            .flat_map(ColumnName::cast)
            .next()
    }
}
pub struct CaseElseClause<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> CaseElseClause<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::CaseElseClause) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct UniqueConstraint<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> UniqueConstraint<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::UniqueConstraint) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn conflict_clause(&self) -> Option<ConflictClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ConflictClause)
            .flat_map(ConflictClause::cast)
            .next()
    }
}
pub struct TableOptionsList<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableOptionsList<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableOptionsList) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn items(&self) -> impl Iterator<Item = TableOptions<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TableOptions)
            .flat_map(|it| it.children())
            .flat_map(TableOptions::cast)
    }
}
pub struct FrameSpecNoOthers<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> FrameSpecNoOthers<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::FrameSpecNoOthers) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct PragmaValueName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> PragmaValueName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::PragmaValueName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct DetachStmt<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> DetachStmt<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::DetachStmt) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn db_name_expr(&self) -> Option<DbNameExpr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::DbNameExpr)
            .flat_map(DbNameExpr::cast)
            .next()
    }
}
pub struct Statement<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> Statement<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::Statement) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn explain_clause(&self) -> Option<ExplainClause<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ExplainClause)
            .flat_map(ExplainClause::cast)
            .next()
    }
    pub fn statement_kind(&self) -> Option<StatementKind<'a>> {
        self.inner.children().flat_map(StatementKind::cast).next()
    }
}
pub struct SavepointName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> SavepointName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::SavepointName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct Collation<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> Collation<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::Collation) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn collation_name(&self) -> Option<CollationName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::CollationName)
            .flat_map(CollationName::cast)
            .next()
    }
}
pub struct OuterJoin<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OuterJoin<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OuterJoin) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn outer_join_kind(&self) -> Option<OuterJoinKind<'a>> {
        self.inner.children().flat_map(OuterJoinKind::cast).next()
    }
}
pub struct AddColumn<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> AddColumn<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::AddColumn) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn column_def(&self) -> Option<ColumnDef<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ColumnDef)
            .flat_map(ColumnDef::cast)
            .next()
    }
}
pub struct TableOrIndexName<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> TableOrIndexName<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::TableOrIndexName) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn value(&self) -> Option<&'a SqliteToken> {
        self.inner.valid_children().next().and_then(|it| it.token())
    }
}
pub struct RaiseFuncErrMessage<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> RaiseFuncErrMessage<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::RaiseFuncErrMessage) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
}
pub struct OpEscape<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OpEscape<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpEscape) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn escape(&self) -> Option<&'a SqliteToken> {
        self.inner
            .find_children(SqliteTokenKind::STR_LIT)
            .map(|it| it.token().unwrap())
            .next()
    }
}
pub struct ResultColumnTableAll<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ResultColumnTableAll<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ResultColumnTableAll) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn table_name(&self) -> Option<TableName<'a>> {
        self.inner
            .find_children(SqliteTreeKind::TableName)
            .flat_map(TableName::cast)
            .next()
    }
}
pub struct ValuesSelect<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> ValuesSelect<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::ValuesSelect) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr_list(&self) -> Option<ExprList<'a>> {
        self.inner
            .find_children(SqliteTreeKind::ExprList)
            .flat_map(ExprList::cast)
            .next()
    }
}
pub struct InSelect<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> InSelect<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::InSelect) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn select_stmt_with_cte(&self) -> Option<SelectStmtWithCte<'a>> {
        self.inner
            .find_children(SqliteTreeKind::SelectStmtWithCte)
            .flat_map(SelectStmtWithCte::cast)
            .next()
    }
}
pub struct OpBinComplement<'a> {
    pub inner: CstNode<'a>,
}
impl<'a> OpBinComplement<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpBinComplement) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn expr(&self) -> Option<Expr<'a>> {
        self.inner
            .find_children(SqliteTreeKind::Expr)
            .flat_map(Expr::cast)
            .next()
    }
}
pub struct OpBinLShift<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpBinLShift<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpBinLShift) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpMatch<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpMatch<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpMatch) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpRegexp<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpRegexp<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpRegexp) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpExtractTwo<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpExtractTwo<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpExtractTwo) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpIsDistinctFrom<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpIsDistinctFrom<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpIsDistinctFrom) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpGlob<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpGlob<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpGlob) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpModulus<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpModulus<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpModulus) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpExtractOne<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpExtractOne<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpExtractOne) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpIsNot<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpIsNot<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpIsNot) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpLike<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpLike<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpLike) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpGTE<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpGTE<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpGTE) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpOr<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpOr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpOr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpDivide<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpDivide<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpDivide) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpNotMatch<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpNotMatch<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotMatch) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpBinOr<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpBinOr<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpBinOr) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpLTE<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpLTE<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpLTE) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpIs<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpIs<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpIs) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpEq<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpEq<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpEq) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpBinAnd<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpBinAnd<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpBinAnd) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpConcat<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpConcat<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpConcat) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpNotRegexp<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpNotRegexp<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotRegexp) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpMultiply<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpMultiply<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpMultiply) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpBinRShift<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpBinRShift<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpBinRShift) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpGT<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpGT<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpGT) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpIsNotDistinctFrom<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpIsNotDistinctFrom<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpIsNotDistinctFrom) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpAdd<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpAdd<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpAdd) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpNotGlob<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpNotGlob<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotGlob) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpSubtract<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpSubtract<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpSubtract) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpNotEq<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpNotEq<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpNotEq) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpLT<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpLT<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpLT) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}
pub struct OpAnd<'a> {
    inner: CstNode<'a>,
}
impl<'a> OpAnd<'a> {
    pub fn cast(node: CstNode<'a>) -> Option<Self> {
        if node.tree() == Some(SqliteTreeKind::OpAnd) {
            Some(Self { inner: node })
        } else {
            None
        }
    }
    pub fn lhs_expr(&self) -> Option<Expr<'a>> {
        self.inner.valid_children().next().and_then(Expr::cast)
    }
    pub fn rhs_expr(&self) -> Option<Expr<'a>> {
        let mut child_iter = self.inner.valid_children();
        child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));
        child_iter.find_map(Expr::cast)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SqliteTreeKind {
    ErrorTree,
    File,
    SelectStmt,
    ResultColumn,
    ResultColumnList,
    Name,
    Operator,
    ExprPrefix,
    ExprPostfix,
    ExprInfix,
    ExprParen,
    Expr,
    ColumnAlias,
    JoinOperator,
    FromClause,
    JoinConstraint,
    TableOrSubquery,
    TableValueFunction,
    TableName,
    JoinClause,

    // Different types of expressions
    ExprLit,
    ExprColumnName,

    // Prefix Operators
    OpBinComplement,
    OpUnaryMinus,
    OpUnaryPlus,
    OpNot,

    // Prefix Operators (Binary Operators)
    OpConcat,
    OpExtractOne,
    OpExtractTwo,
    OpMultiply,
    OpDivide,
    OpModulus,
    OpAdd,
    OpSubtract,
    OpBinAnd,
    OpBinOr,
    OpBinLShift,
    OpBinRShift,
    OpLT,
    OpGT,
    OpLTE,
    OpGTE,
    OpEq,
    OpNotEq,
    OpIs,
    OpIsNot,
    OpIsDistinctFrom,
    OpIsNotDistinctFrom,
    OpAnd,
    OpOr,

    OpIn,
    OpNotIn,

    OpNotMatch,
    OpMatch,

    OpNotLike,
    OpLike,

    OpNotRegexp, // Regular Expression
    OpRegexp,    // Regular Expression

    OpNotGlob,
    OpGlob,

    // Postfix Operators
    OpCollate,
    OpEscape,
    OpIsNull,

    /// `NOTNULL` - one word. There is also `NOT NULL` which different
    OpNotNull,
    /// `NOT NULL` - Two words, literal space in the middle. There is also `NOTNULL` which is different
    OpNotSpaceNull,

    // Complexfix Operators
    OpBetweenAnd,
    OpNotBetweenAnd,
}

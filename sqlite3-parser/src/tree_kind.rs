#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, strum :: IntoStaticStr)]
pub enum SqliteTreeKind {
    File,
    Statement,
    ExplainClause,
    StatementNoCte,
    StatementWithCte,
    CreateTableStmt,
    AlterTableStmt,
    AnalyzeStmt,
    AttachDbStmt,
    BeginStmt,
    CommitStmt,
    CreateIndexStmt,
    CreateTriggerStmt,
    CreateViewStmt,
    CreateVirtualTableStmt,
    DetachStmt,
    DropIndexStmt,
    DropViewStmt,
    DropTableStmt,
    DropTriggerStmt,
    PragmaStmt,
    ReIndexStmt,
    ReleaseStmt,
    RollbackStmt,
    SavepointStmt,
    VacuumStmt,
    CteClause,
    SelectStmt,
    InsertStmt,
    UpdateStmt,
    DeleteStmt,
    FullTableName,
    RenameTable,
    RenameColumn,
    AddColumn,
    DropColumn,
    TableName,
    ColumnName,
    NewColumnName,
    ColumnDef,
    SchemaOrIdxOrTableName,
    TableOrIdxNameWithSchema,
    SchemaName,
    TableOrIndexName,
    FileNameExpr,
    SchemaNameExpr,
    PasswordExpr,
    IfNotExists,
    FullIndexName,
    IndexedColList,
    WhereClause,
    IndexName,
    Expr,
    FullPragmaName,
    PragmaValue,
    PragmaName,
    SignedNumber,
    PragmaValueName,
    TableOrIdxOrCollationName,
    SavepointName,
    FullTriggerName,
    TriggerInsteadOf,
    TriggerActionKind,
    TriggerForEachRow,
    TriggerWhenExpr,
    TriggerBodyStmtList,
    TriggerName,
    TriggerUpdateAction,
    TriggerUpdateAffectCols,
    TriggerBodyStmt,
    FullViewName,
    ColNameList,
    SelectStmtWithCte,
    ModuleName,
    ModuleArgList,
    ModuleArg,
    QualifiedTableName,
    ReturningClause,
    DeleteStmtLimited,
    DbNameExpr,
    ConflictAction,
    SetColumnExpr,
    FromClause,
    UpdateStmtLimited,
    OrderByClause,
    LimitClause,
    InsertStmtKind,
    WithAlias,
    InsertValueKind,
    InsertOrAction,
    InsertValuesClause,
    InsertSelectClause,
    InsertDefaultValuesClause,
    ExprList,
    UpsertClause,
    UpsertClauseConflictTarget,
    UpsertDoUpdate,
    OrderingTermList,
    OrderingTerm,
    Collation,
    Order,
    Offset,
    ReturningClauseKind,
    ReturningClauseExpr,
    AliasName,
    CommonTableExpr,
    CteName,
    MaterializedCte,
    ViewName,
    TableNameIndexedBy,
    TableNameNotIndexed,
    TableDetails,
    CreateTableSelect,
    TableConstraint,
    TableOptionsList,
    TableOptions,
    TableOptWithoutRowId,
    ConstraintName,
    TablePkConstraint,
    TableUqConstraint,
    CheckConstraint,
    TableFkConstraint,
    ConflictClause,
    FkClause,
    FkViolateAction,
    FkDeferrable,
    FkOnAction,
    FkMatchAction,
    AnyValidName,
    FkSetNull,
    FkSetDefault,
    FkCascade,
    FkRestrict,
    FkNoAction,
    IndexedCol,
    CollationName,
    FullTableFunctionName,
    TableFunctionName,
    SelectCore,
    TraditionalSelect,
    ValuesSelect,
    ResultColumnList,
    GroupByClause,
    HavingClause,
    WindowClause,
    CompoundSelect,
    CompoundOperator,
    UnionCompoundOperator,
    ResultColumn,
    TableOrSubquery,
    JoinClause,
    ValuesClause,
    WindowFunction,
    WindowName,
    WindowDef,
    WindowBaseName,
    WindowPartitionByClause,
    FrameSpec,
    FrameSpecBetweenClause,
    FrameSpecUnboundedPreceding,
    FrameSpecPreceding,
    FrameSpecCurrentRow,
    FrameSpecExcludeClause,
    FrameSpecNoOthers,
    FrameSpecBetweenLeft,
    FrameSpecBetweenRight,
    FrameSpecFollowing,
    FrameSpecUnboundedFollowing,
    FromClauseTableValueFunction,
    FromClauseSelectStmt,
    JoinOperator,
    JoinConstraint,
    OnConstraint,
    UsingConstraint,
    CommaJoin,
    NonCommaJoin,
    CrossJoin,
    OuterJoin,
    InnerJoin,
    NaturalJoin,
    Join,
    ResultColumnExpr,
    ResultColumnAll,
    ResultColumnTableAll,
    TypeName,
    ColumnConstraint,
    ColumnConstraintName,
    PrimaryConstraint,
    NotNullConstraint,
    UniqueConstraint,
    DefaultConstraint,
    ColumnGenerated,
    DefaultConstraintExpr,
    DefaultConstraintLiteral,
    ColumnGeneratedKind,
    ExprParen,
    ExprLit,
    ExprColumnName,
    ExprPrefix,
    ExprPostfix,
    ExprInfix,
    ExprBindParam,
    ExprFunc,
    ExprSelect,
    ExprCast,
    ExprCase,
    RaiseFunc,
    InSelect,
    InTable,
    InTableFunc,
    CaseTargetExpr,
    CaseWhenClauseList,
    CaseElseClause,
    CaseWhenClause,
    FunctionName,
    FuncArguments,
    FilterClause,
    OverClause,
    RaiseAction,
    RaiseActionRollBack,
    RaiseActionAbort,
    RaiseActionFail,
    RaiseFuncErrMessage,
    ArgExpr,
    ArgStar,
    OpBinComplement,
    OpUnaryPlus,
    OpUnaryMinus,
    OpNot,
    OpNotSpaceNull,
    OpCollate,
    OpNotNull,
    OpIsNull,
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
    OpAnd,
    OpOr,
    OpMatch,
    OpLike,
    OpRegexp,
    OpGlob,
    OpBetweenAnd,
    OpNotMatch,
    OpNotLike,
    OpNotRegexp,
    OpNotGlob,
    OpNotBetweenAnd,
    OpIsNotDistinctFrom,
    OpIsDistinctFrom,
    OpIsNot,
    OpIs,
    OpIn,
    OpEscape,
    OpNotIn,
    TypeNameWord,
    NewTableName,
}

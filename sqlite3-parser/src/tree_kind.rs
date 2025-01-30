#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

impl SqliteTreeKind {
    pub fn as_str(&self) -> &'static str {
        use SqliteTreeKind::*;
        match self {
            File => "File",
            Statement => "Statement",
            ExplainClause => "ExplainClause",
            StatementNoCte => "StatementNoCte",
            StatementWithCte => "StatementWithCte",
            CreateTableStmt => "CreateTableStmt",
            AlterTableStmt => "AlterTableStmt",
            AnalyzeStmt => "AnalyzeStmt",
            AttachDbStmt => "AttachDbStmt",
            BeginStmt => "BeginStmt",
            CommitStmt => "CommitStmt",
            CreateIndexStmt => "CreateIndexStmt",
            CreateTriggerStmt => "CreateTriggerStmt",
            CreateViewStmt => "CreateViewStmt",
            CreateVirtualTableStmt => "CreateVirtualTableStmt",
            DetachStmt => "DetachStmt",
            DropIndexStmt => "DropIndexStmt",
            DropViewStmt => "DropViewStmt",
            DropTableStmt => "DropTableStmt",
            DropTriggerStmt => "DropTriggerStmt",
            PragmaStmt => "PragmaStmt",
            ReIndexStmt => "ReIndexStmt",
            ReleaseStmt => "ReleaseStmt",
            RollbackStmt => "RollbackStmt",
            SavepointStmt => "SavepointStmt",
            VacuumStmt => "VacuumStmt",
            CteClause => "CteClause",
            SelectStmt => "SelectStmt",
            InsertStmt => "InsertStmt",
            UpdateStmt => "UpdateStmt",
            DeleteStmt => "DeleteStmt",
            FullTableName => "FullTableName",
            RenameTable => "RenameTable",
            RenameColumn => "RenameColumn",
            AddColumn => "AddColumn",
            DropColumn => "DropColumn",
            TableName => "TableName",
            ColumnName => "ColumnName",
            NewColumnName => "NewColumnName",
            ColumnDef => "ColumnDef",
            SchemaOrIdxOrTableName => "SchemaOrIdxOrTableName",
            TableOrIdxNameWithSchema => "TableOrIdxNameWithSchema",
            SchemaName => "SchemaName",
            TableOrIndexName => "TableOrIndexName",
            FileNameExpr => "FileNameExpr",
            SchemaNameExpr => "SchemaNameExpr",
            PasswordExpr => "PasswordExpr",
            IfNotExists => "IfNotExists",
            FullIndexName => "FullIndexName",
            IndexedColList => "IndexedColList",
            WhereClause => "WhereClause",
            IndexName => "IndexName",
            Expr => "Expr",
            FullPragmaName => "FullPragmaName",
            PragmaValue => "PragmaValue",
            PragmaName => "PragmaName",
            SignedNumber => "SignedNumber",
            PragmaValueName => "PragmaValueName",
            TableOrIdxOrCollationName => "TableOrIdxOrCollationName",
            SavepointName => "SavepointName",
            FullTriggerName => "FullTriggerName",
            TriggerInsteadOf => "TriggerInsteadOf",
            TriggerActionKind => "TriggerActionKind",
            TriggerForEachRow => "TriggerForEachRow",
            TriggerWhenExpr => "TriggerWhenExpr",
            TriggerBodyStmtList => "TriggerBodyStmtList",
            TriggerName => "TriggerName",
            TriggerUpdateAction => "TriggerUpdateAction",
            TriggerUpdateAffectCols => "TriggerUpdateAffectCols",
            TriggerBodyStmt => "TriggerBodyStmt",
            FullViewName => "FullViewName",
            ColNameList => "ColNameList",
            SelectStmtWithCte => "SelectStmtWithCte",
            ModuleName => "ModuleName",
            ModuleArgList => "ModuleArgList",
            ModuleArg => "ModuleArg",
            QualifiedTableName => "QualifiedTableName",
            ReturningClause => "ReturningClause",
            DeleteStmtLimited => "DeleteStmtLimited",
            DbNameExpr => "DbNameExpr",
            ConflictAction => "ConflictAction",
            SetColumnExpr => "SetColumnExpr",
            FromClause => "FromClause",
            UpdateStmtLimited => "UpdateStmtLimited",
            OrderByClause => "OrderByClause",
            LimitClause => "LimitClause",
            InsertStmtKind => "InsertStmtKind",
            WithAlias => "WithAlias",
            InsertValueKind => "InsertValueKind",
            InsertOrAction => "InsertOrAction",
            InsertValuesClause => "InsertValuesClause",
            InsertSelectClause => "InsertSelectClause",
            InsertDefaultValuesClause => "InsertDefaultValuesClause",
            ExprList => "ExprList",
            UpsertClause => "UpsertClause",
            UpsertClauseConflictTarget => "UpsertClauseConflictTarget",
            UpsertDoUpdate => "UpsertDoUpdate",
            OrderingTermList => "OrderingTermList",
            OrderingTerm => "OrderingTerm",
            Collation => "Collation",
            Order => "Order",
            Offset => "Offset",
            ReturningClauseKind => "ReturningClauseKind",
            ReturningClauseExpr => "ReturningClauseExpr",
            AliasName => "AliasName",
            CommonTableExpr => "CommonTableExpr",
            CteName => "CteName",
            MaterializedCte => "MaterializedCte",
            ViewName => "ViewName",
            TableNameIndexedBy => "TableNameIndexedBy",
            TableNameNotIndexed => "TableNameNotIndexed",
            TableDetails => "TableDetails",
            CreateTableSelect => "CreateTableSelect",
            TableConstraint => "TableConstraint",
            TableOptionsList => "TableOptionsList",
            TableOptions => "TableOptions",
            TableOptWithoutRowId => "TableOptWithoutRowId",
            ConstraintName => "ConstraintName",
            TablePkConstraint => "TablePkConstraint",
            TableUqConstraint => "TableUqConstraint",
            CheckConstraint => "CheckConstraint",
            TableFkConstraint => "TableFkConstraint",
            ConflictClause => "ConflictClause",
            FkClause => "FkClause",
            FkViolateAction => "FkViolateAction",
            FkDeferrable => "FkDeferrable",
            FkOnAction => "FkOnAction",
            FkMatchAction => "FkMatchAction",
            AnyValidName => "AnyValidName",
            FkSetNull => "FkSetNull",
            FkSetDefault => "FkSetDefault",
            FkCascade => "FkCascade",
            FkRestrict => "FkRestrict",
            FkNoAction => "FkNoAction",
            IndexedCol => "IndexedCol",
            CollationName => "CollationName",
            FullTableFunctionName => "FullTableFunctionName",
            TableFunctionName => "TableFunctionName",
            SelectCore => "SelectCore",
            TraditionalSelect => "TraditionalSelect",
            ValuesSelect => "ValuesSelect",
            ResultColumnList => "ResultColumnList",
            GroupByClause => "GroupByClause",
            HavingClause => "HavingClause",
            WindowClause => "WindowClause",
            CompoundSelect => "CompoundSelect",
            CompoundOperator => "CompoundOperator",
            UnionCompoundOperator => "UnionCompoundOperator",
            ResultColumn => "ResultColumn",
            TableOrSubquery => "TableOrSubquery",
            JoinClause => "JoinClause",
            ValuesClause => "ValuesClause",
            WindowFunction => "WindowFunction",
            WindowName => "WindowName",
            WindowDef => "WindowDef",
            WindowBaseName => "WindowBaseName",
            WindowPartitionByClause => "WindowPartitionByClause",
            FrameSpec => "FrameSpec",
            FrameSpecBetweenClause => "FrameSpecBetweenClause",
            FrameSpecUnboundedPreceding => "FrameSpecUnboundedPreceding",
            FrameSpecPreceding => "FrameSpecPreceding",
            FrameSpecCurrentRow => "FrameSpecCurrentRow",
            FrameSpecExcludeClause => "FrameSpecExcludeClause",
            FrameSpecNoOthers => "FrameSpecNoOthers",
            FrameSpecBetweenLeft => "FrameSpecBetweenLeft",
            FrameSpecBetweenRight => "FrameSpecBetweenRight",
            FrameSpecFollowing => "FrameSpecFollowing",
            FrameSpecUnboundedFollowing => "FrameSpecUnboundedFollowing",
            FromClauseTableValueFunction => "FromClauseTableValueFunction",
            FromClauseSelectStmt => "FromClauseSelectStmt",
            JoinOperator => "JoinOperator",
            JoinConstraint => "JoinConstraint",
            OnConstraint => "OnConstraint",
            UsingConstraint => "UsingConstraint",
            CommaJoin => "CommaJoin",
            NonCommaJoin => "NonCommaJoin",
            CrossJoin => "CrossJoin",
            OuterJoin => "OuterJoin",
            InnerJoin => "InnerJoin",
            NaturalJoin => "NaturalJoin",
            Join => "Join",
            ResultColumnExpr => "ResultColumnExpr",
            ResultColumnAll => "ResultColumnAll",
            ResultColumnTableAll => "ResultColumnTableAll",
            TypeName => "TypeName",
            ColumnConstraint => "ColumnConstraint",
            ColumnConstraintName => "ColumnConstraintName",
            PrimaryConstraint => "PrimaryConstraint",
            NotNullConstraint => "NotNullConstraint",
            UniqueConstraint => "UniqueConstraint",
            DefaultConstraint => "DefaultConstraint",
            ColumnGenerated => "ColumnGenerated",
            DefaultConstraintExpr => "DefaultConstraintExpr",
            DefaultConstraintLiteral => "DefaultConstraintLiteral",
            ColumnGeneratedKind => "ColumnGeneratedKind",
            ExprParen => "ExprParen",
            ExprLit => "ExprLit",
            ExprColumnName => "ExprColumnName",
            ExprPrefix => "ExprPrefix",
            ExprPostfix => "ExprPostfix",
            ExprInfix => "ExprInfix",
            ExprBindParam => "ExprBindParam",
            ExprFunc => "ExprFunc",
            ExprSelect => "ExprSelect",
            ExprCast => "ExprCast",
            ExprCase => "ExprCase",
            RaiseFunc => "RaiseFunc",
            InSelect => "InSelect",
            InTable => "InTable",
            InTableFunc => "InTableFunc",
            CaseTargetExpr => "CaseTargetExpr",
            CaseWhenClauseList => "CaseWhenClauseList",
            CaseElseClause => "CaseElseClause",
            CaseWhenClause => "CaseWhenClause",
            FunctionName => "FunctionName",
            FuncArguments => "FuncArguments",
            FilterClause => "FilterClause",
            OverClause => "OverClause",
            RaiseAction => "RaiseAction",
            RaiseActionRollBack => "RaiseActionRollBack",
            RaiseActionAbort => "RaiseActionAbort",
            RaiseActionFail => "RaiseActionFail",
            RaiseFuncErrMessage => "RaiseFuncErrMessage",
            ArgExpr => "ArgExpr",
            ArgStar => "ArgStar",
            OpBinComplement => "OpBinComplement",
            OpUnaryPlus => "OpUnaryPlus",
            OpUnaryMinus => "OpUnaryMinus",
            OpNot => "OpNot",
            OpNotSpaceNull => "OpNotSpaceNull",
            OpCollate => "OpCollate",
            OpNotNull => "OpNotNull",
            OpIsNull => "OpIsNull",
            OpConcat => "OpConcat",
            OpExtractOne => "OpExtractOne",
            OpExtractTwo => "OpExtractTwo",
            OpMultiply => "OpMultiply",
            OpDivide => "OpDivide",
            OpModulus => "OpModulus",
            OpAdd => "OpAdd",
            OpSubtract => "OpSubtract",
            OpBinAnd => "OpBinAnd",
            OpBinOr => "OpBinOr",
            OpBinLShift => "OpBinLShift",
            OpBinRShift => "OpBinRShift",
            OpLT => "OpLT",
            OpGT => "OpGT",
            OpLTE => "OpLTE",
            OpGTE => "OpGTE",
            OpEq => "OpEq",
            OpNotEq => "OpNotEq",
            OpAnd => "OpAnd",
            OpOr => "OpOr",
            OpMatch => "OpMatch",
            OpLike => "OpLike",
            OpRegexp => "OpRegexp",
            OpGlob => "OpGlob",
            OpBetweenAnd => "OpBetweenAnd",
            OpNotMatch => "OpNotMatch",
            OpNotLike => "OpNotLike",
            OpNotRegexp => "OpNotRegexp",
            OpNotGlob => "OpNotGlob",
            OpNotBetweenAnd => "OpNotBetweenAnd",
            OpIsNotDistinctFrom => "OpIsNotDistinctFrom",
            OpIsDistinctFrom => "OpIsDistinctFrom",
            OpIsNot => "OpIsNot",
            OpIs => "OpIs",
            OpIn => "OpIn",
            OpEscape => "OpEscape",
            OpNotIn => "OpNotIn",
            TypeNameWord => "TypeNameWord",
            NewTableName => "NewTableName",
        }
    }
}

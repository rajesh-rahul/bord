use crate::SqliteTokenKind;
use enumset::{enum_set, EnumSet};
use SqliteTokenKind::*;

/// Same as `ID` Terminal token in SQLite parse.y. Some of these tokens are only
/// considered `ID` in specific builds of SQLite and therefore in the parser, IDEN_SET will be
/// dynamic
pub(crate) const IDEN_SET: EnumSet<SqliteTokenKind> = enum_set!(
    IDEN | KW_ABORT
        | KW_ACTION
        | KW_AFTER
        | KW_ANALYZE
        | KW_ASC
        | KW_ATTACH
        | KW_BEFORE
        | KW_BEGIN
        | KW_BY
        | KW_CASCADE
        | KW_CAST
        | KW_COLUMN
        | KW_CONFLICT
        | KW_DATABASE
        | KW_DEFERRED
        | KW_DESC
        | KW_DETACH
        | KW_DO
        | KW_EACH
        | KW_END
        | KW_EXCLUSIVE
        | KW_EXPLAIN
        | KW_FAIL
        | KW_FOR
        | KW_IGNORE
        | KW_IMMEDIATE
        | KW_INITIALLY
        | KW_INSTEAD
        | KW_LIKE
        | KW_MATCH
        | KW_NO
        | KW_PLAN
        | KW_QUERY
        | KW_KEY
        | KW_OF
        | KW_OFFSET
        | KW_PRAGMA
        | KW_RAISE
        | KW_RECURSIVE
        | KW_RELEASE
        | KW_REPLACE
        | KW_RESTRICT
        | KW_ROW
        | KW_ROWS
        | KW_ROLLBACK
        | KW_SAVEPOINT
        | KW_TEMP
        | KW_TRIGGER
        | KW_VACUUM
        | KW_VIEW
        | KW_VIRTUAL
        | KW_WITH
        | KW_WITHOUT
        | KW_NULLS
        | KW_FIRST
        | KW_LAST
        | KW_EXCEPT
        | KW_INTERSECT
        | KW_UNION
        | KW_CURRENT
        | KW_FOLLOWING
        | KW_PARTITION
        | KW_PRECEDING
        | KW_RANGE
        | KW_UNBOUNDED
        | KW_EXCLUDE
        | KW_GROUPS
        | KW_OTHERS
        | KW_TIES
        | KW_GENERATED
        | KW_ALWAYS
        | KW_MATERIALIZED
        | KW_REINDEX
        | KW_RENAME
        | KW_CURRENT_TIME
        | KW_CURRENT_DATE
        | KW_CURRENT_TIME
        | KW_IF

        // The following are not keywords and just regular words in SQLite, but to make parsing
        // easier we considered them keywords and therefore we must add them to this set here
        | KW_TRUE
        | KW_FALSE
        | KW_STORED
        | KW_ROWID
        | KW_STRICT
);

pub(crate) const JOIN_KEYWORDS: EnumSet<SqliteTokenKind> =
    enum_set!(KW_CROSS | KW_FULL | KW_INNER | KW_LEFT | KW_NATURAL | KW_OUTER | KW_RIGHT);

pub(crate) const EXPR_LIT_START: EnumSet<SqliteTokenKind> = enum_set!(
    HEX_LIT
        | INT_LIT
        | STR_LIT
        | KW_TRUE
        | KW_FALSE
        | KW_CURRENT_TIME
        | REAL_LIT
        | KW_NULL
        | KW_CURRENT_DATE
        | KW_CURRENT_TIMESTAMP
        | BLOB_LIT
);

pub(crate) const EXPR_PREFIX_START: EnumSet<SqliteTokenKind> =
    enum_set!(PLUS | MINUS | KW_NOT | TILDA);

pub(crate) const EXPR_BIND_PARAM_START: EnumSet<SqliteTokenKind> = enum_set!(Q_MARK);

pub(crate) const NUMERIC_LIT: EnumSet<SqliteTokenKind> = enum_set!(INT_LIT | REAL_LIT | HEX_LIT);

pub(crate) const LITERAL_VALUE: EnumSet<SqliteTokenKind> = enum_set!(
    NUMERIC_LIT
        | STR_LIT
        | BLOB_LIT
        | KW_NULL
        | KW_TRUE
        | KW_FALSE
        | KW_CURRENT_TIME
        | KW_CURRENT_DATE
);

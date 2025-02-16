#![allow(non_snake_case, non_camel_case_types)]

use enumset::EnumSetType;

use crate::ExpectedItem;

pub const MAX_KEYWORD_LEN: usize = 17;

#[derive(Debug, PartialOrd, Ord, Hash, EnumSetType)]
pub enum SqliteTokenKind {
    KW_ABORT,
    KW_ACTION,
    KW_ADD,
    KW_AFTER,
    KW_ALL,
    KW_ALTER,
    KW_ALWAYS,
    KW_ANALYZE,
    KW_AND,
    KW_AS,
    KW_ASC,
    KW_ATTACH,
    KW_AUTOINCREMENT,
    KW_BEFORE,
    KW_BEGIN,
    KW_BETWEEN,
    KW_BY,
    KW_CASCADE,
    KW_CASE,
    KW_CAST,
    KW_CHECK,
    KW_COLLATE,
    KW_COLUMN,
    KW_COMMIT,
    KW_CONFLICT,
    KW_CONSTRAINT,
    KW_CREATE,
    KW_CROSS,
    KW_CURRENT,
    KW_CURRENT_DATE,
    KW_CURRENT_TIME,
    KW_CURRENT_TIMESTAMP,
    KW_DATABASE,
    KW_DEFAULT,
    KW_DEFERRABLE,
    KW_DEFERRED,
    KW_DELETE,
    KW_DESC,
    KW_DETACH,
    KW_DISTINCT,
    KW_DO,
    KW_DROP,
    KW_EACH,
    KW_ELSE,
    KW_END,
    KW_ESCAPE,
    KW_EXCEPT,
    KW_EXCLUDE,
    KW_EXCLUSIVE,
    KW_EXISTS,
    KW_EXPLAIN,
    KW_FAIL,
    KW_FILTER,
    KW_FIRST,
    KW_FOLLOWING,
    KW_FOR,
    KW_FOREIGN,
    KW_FROM,
    KW_FULL,
    KW_GENERATED,
    KW_GLOB,
    KW_GROUP,
    KW_GROUPS,
    KW_HAVING,
    KW_IF,
    KW_IGNORE,
    KW_IMMEDIATE,
    KW_IN,
    KW_INDEX,
    KW_INDEXED,
    KW_INITIALLY,
    KW_INNER,
    KW_INSERT,
    KW_INSTEAD,
    KW_INTERSECT,
    KW_INTO,
    KW_IS,
    KW_ISNULL,
    KW_JOIN,
    KW_KEY,
    KW_LAST,
    KW_LEFT,
    KW_LIKE,
    KW_LIMIT,
    KW_MATCH,
    KW_MATERIALIZED,
    KW_NATURAL,
    KW_NO,
    KW_NOT,
    KW_NOTHING,
    KW_NOTNULL,
    KW_NULL,
    KW_NULLS,
    KW_OF,
    KW_OFFSET,
    KW_ON,
    KW_OR,
    KW_ORDER,
    KW_OTHERS,
    KW_OUTER,
    KW_OVER,
    KW_PARTITION,
    KW_PLAN,
    KW_PRAGMA,
    KW_PRECEDING,
    KW_PRIMARY,
    KW_QUERY,
    KW_RAISE,
    KW_RANGE,
    KW_RECURSIVE,
    KW_REFERENCES,
    KW_REGEXP,
    KW_REINDEX,
    KW_RELEASE,
    KW_RENAME,
    KW_REPLACE,
    KW_RESTRICT,
    KW_RETURNING,
    KW_RIGHT,
    KW_ROLLBACK,
    KW_ROW,
    KW_ROWS,
    KW_SAVEPOINT,
    KW_SELECT,
    KW_SET,
    KW_TABLE,
    KW_TEMP,
    KW_TEMPORARY,
    KW_THEN,
    KW_TIES,
    KW_TO,
    KW_TRANSACTION,
    KW_TRIGGER,
    KW_UNBOUNDED,
    KW_UNION,
    KW_UNIQUE,
    KW_UPDATE,
    KW_USING,
    KW_VACUUM,
    KW_VALUES,
    KW_VIEW,
    KW_VIRTUAL,
    KW_WHEN,
    KW_WHERE,
    KW_WINDOW,
    KW_WITH,
    KW_WITHOUT,

    WHITESPACE,
    /// Single Line Comment
    S_LINE_COMMENT,

    // Multi-line comment
    M_LINE_COMMENT,
    STR_LIT,
    REAL_LIT,
    IDEN,
    DOT,
    /// `*`. Ex: SELECT `*` FROM table;
    STAR,
    /// `(`
    L_PAREN,
    /// `)`
    R_PAREN,
    /// `,`
    COMMA,
    /// `;`
    SEMICOLON,
    /// ":"
    COLON,
    /// `=`
    EQ_SQL,
    /// `==`
    EQ,
    /// `<>`
    NOT_EQ_SQL,
    /// `!=`
    NOT_EQ,
    /// `+`
    PLUS,
    /// `-`
    MINUS,
    /// `/`
    F_SLASH,
    /// `%`
    PERCENT,
    EOF,
    /// `->>`
    EXTRACT_TWO,
    /// `->`
    EXTRACT_ONE,
    /// `<`
    L_CHEV,
    /// `>`
    R_CHEV,
    /// `<=`
    L_CHEV_EQ,
    /// `>=`
    R_CHEV_EQ,

    /// `||`
    DOUBLE_PIPE,

    /// `~`
    TILDA,
    /// `<<`
    L_CHEV_TWO,
    /// `>>`
    R_CHEV_TWO,
    /// `|`
    PIPE,
    /// `&`
    AMPERSAND,
    /// `@`
    AT_MARK,
    ///  `?`
    Q_MARK,
    INT_LIT,
    HEX_LIT,
    BLOB_LIT,
    ERROR,

    // Not actually keywords but we treat them as such to make things easy in the parser
    KW_TRUE,
    KW_FALSE,
    KW_STORED,
    KW_ROWID,
    KW_STRICT,
}

impl SqliteTokenKind {
    pub const fn to_expected_item(&self) -> ExpectedItem {
        ExpectedItem::Token(*self)
    }

    pub const fn size(&self) -> Option<u8> {
        use SqliteTokenKind::*;

        match self {
            DOT | STAR | COMMA | SEMICOLON | PLUS | MINUS | PERCENT | L_PAREN | R_PAREN
            | EQ_SQL | F_SLASH | L_CHEV | R_CHEV | TILDA | PIPE | AMPERSAND | COLON | Q_MARK => {
                Some(1)
            }

            EQ | NOT_EQ | DOUBLE_PIPE | EXTRACT_ONE | L_CHEV_EQ | R_CHEV_EQ | L_CHEV_TWO
            | R_CHEV_TWO | NOT_EQ_SQL => Some(2),

            EXTRACT_TWO => Some(3),

            _ => None,
        }
    }

    pub const fn is_trivia(&self) -> bool {
        matches!(
            self,
            SqliteTokenKind::WHITESPACE
                | SqliteTokenKind::S_LINE_COMMENT
                | SqliteTokenKind::M_LINE_COMMENT
        )
    }
}

#[macro_export]
// Source: rust-analyzer
macro_rules! T {
    (*) => {
        SqliteTokenKind::STAR
    };
    (;) => {
        SqliteTokenKind::SEMICOLON
    };
    (,) => {
        SqliteTokenKind::COMMA
    };
    (.) => {
        SqliteTokenKind::DOT
    };
    (+) => {
        SqliteTokenKind::PLUS
    };
    (-) => {
        SqliteTokenKind::MINUS
    };
    (/) => {
        SqliteTokenKind::F_SLASH
    };
    (||) => {
        SqliteTokenKind::DOUBLE_PIPE
    };
    (->) => {
        SqliteTokenKind::EXTRACT_ONE
    };
    (->>) => {
        SqliteTokenKind::EXTRACT_TWO
    };
    (<>) => {
        SqliteTokenKind::NOT_EQ_SQL
    };
    (%) => {
        SqliteTokenKind::PERCENT
    };
    (&) => {
        SqliteTokenKind::AMPERSAND
    };
    (|) => {
        SqliteTokenKind::PIPE
    };
    (<<) => {
        SqliteTokenKind::L_CHEV_TWO
    };
    (>>) => {
        SqliteTokenKind::R_CHEV_TWO
    };
    (<) => {
        SqliteTokenKind::L_CHEV
    };
    (>) => {
        SqliteTokenKind::R_CHEV
    };
    (<=) => {
        SqliteTokenKind::L_CHEV_EQ
    };
    (>=) => {
        SqliteTokenKind::R_CHEV_EQ
    };
    (==) => {
        SqliteTokenKind::EQ
    };
    (=) => {
        SqliteTokenKind::EQ_SQL
    };
    (!=) => {
        SqliteTokenKind::NOT_EQ
    };
    (~) => {
        SqliteTokenKind::TILDA
    };
    (?) => {
        SqliteTokenKind::Q_MARK
    };
    (@) => {
        SqliteTokenKind::AT_MARK
    };
    (:) => {
        SqliteTokenKind::COLON
    };
    ('(') => {
        SqliteTokenKind::L_PAREN
    };
    (')') => {
        SqliteTokenKind::R_PAREN
    };
}

impl SqliteTokenKind {
    pub const fn as_str(&self) -> &'static str {
        use SqliteTokenKind::*;

        match self {
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
            KW_TRUE => "TRUE",
            KW_FALSE => "FALSE",
            WHITESPACE => "WHITESPACE",
            S_LINE_COMMENT => "S_LINE_COMMENT",
            M_LINE_COMMENT => "M_LINE_COMMENT",
            STR_LIT => "STR_LIT",
            REAL_LIT => "REAL_LIT",
            IDEN => "IDEN",
            BLOB_LIT => "BLOB_LIT",
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
            INT_LIT => "INT_LIT",
            HEX_LIT => "HEX_LIT",
            ERROR => "ERROR",
            KW_STORED => "STORED",
            KW_ROWID => "ROWID",
            KW_STRICT => "STRICT",
        }
    }
}

// NOTE: We tried phf Map and AhashMap here previously, because we thought a simple match
// statement like the current impl was slower (prolly even tested it), but re-examining
// this shows 10% percent better compared to phf Map and on par performance with Ahash.
// (maybe Rust got faster?)
pub fn sqlite_keywords(keyword: &[u8]) -> Option<SqliteTokenKind> {
    let result = match keyword {
        b"ABORT" => SqliteTokenKind::KW_ABORT,
        b"ACTION" => SqliteTokenKind::KW_ACTION,
        b"ADD" => SqliteTokenKind::KW_ADD,
        b"AFTER" => SqliteTokenKind::KW_AFTER,
        b"ALL" => SqliteTokenKind::KW_ALL,
        b"ALTER" => SqliteTokenKind::KW_ALTER,
        b"ALWAYS" => SqliteTokenKind::KW_ALWAYS,
        b"ANALYZE" => SqliteTokenKind::KW_ANALYZE,
        b"AND" => SqliteTokenKind::KW_AND,
        b"AS" => SqliteTokenKind::KW_AS,
        b"ASC" => SqliteTokenKind::KW_ASC,
        b"ATTACH" => SqliteTokenKind::KW_ATTACH,
        b"AUTOINCREMENT" => SqliteTokenKind::KW_AUTOINCREMENT,
        b"BEFORE" => SqliteTokenKind::KW_BEFORE,
        b"BEGIN" => SqliteTokenKind::KW_BEGIN,
        b"BETWEEN" => SqliteTokenKind::KW_BETWEEN,
        b"BY" => SqliteTokenKind::KW_BY,
        b"CASCADE" => SqliteTokenKind::KW_CASCADE,
        b"CASE" => SqliteTokenKind::KW_CASE,
        b"CAST" => SqliteTokenKind::KW_CAST,
        b"CHECK" => SqliteTokenKind::KW_CHECK,
        b"COLLATE" => SqliteTokenKind::KW_COLLATE,
        b"COLUMN" => SqliteTokenKind::KW_COLUMN,
        b"COMMIT" => SqliteTokenKind::KW_COMMIT,
        b"CONFLICT" => SqliteTokenKind::KW_CONFLICT,
        b"CONSTRAINT" => SqliteTokenKind::KW_CONSTRAINT,
        b"CREATE" => SqliteTokenKind::KW_CREATE,
        b"CROSS" => SqliteTokenKind::KW_CROSS,
        b"CURRENT" => SqliteTokenKind::KW_CURRENT,
        b"CURRENT_DATE" => SqliteTokenKind::KW_CURRENT_DATE,
        b"CURRENT_TIME" => SqliteTokenKind::KW_CURRENT_TIME,
        b"CURRENT_TIMESTAMP" => SqliteTokenKind::KW_CURRENT_TIMESTAMP,
        b"DATABASE" => SqliteTokenKind::KW_DATABASE,
        b"DEFAULT" => SqliteTokenKind::KW_DEFAULT,
        b"DEFERRABLE" => SqliteTokenKind::KW_DEFERRABLE,
        b"DEFERRED" => SqliteTokenKind::KW_DEFERRED,
        b"DELETE" => SqliteTokenKind::KW_DELETE,
        b"DESC" => SqliteTokenKind::KW_DESC,
        b"DETACH" => SqliteTokenKind::KW_DETACH,
        b"DISTINCT" => SqliteTokenKind::KW_DISTINCT,
        b"DO" => SqliteTokenKind::KW_DO,
        b"DROP" => SqliteTokenKind::KW_DROP,
        b"EACH" => SqliteTokenKind::KW_EACH,
        b"ELSE" => SqliteTokenKind::KW_ELSE,
        b"END" => SqliteTokenKind::KW_END,
        b"ESCAPE" => SqliteTokenKind::KW_ESCAPE,
        b"EXCEPT" => SqliteTokenKind::KW_EXCEPT,
        b"EXCLUDE" => SqliteTokenKind::KW_EXCLUDE,
        b"EXCLUSIVE" => SqliteTokenKind::KW_EXCLUSIVE,
        b"EXISTS" => SqliteTokenKind::KW_EXISTS,
        b"EXPLAIN" => SqliteTokenKind::KW_EXPLAIN,
        b"FAIL" => SqliteTokenKind::KW_FAIL,
        b"FILTER" => SqliteTokenKind::KW_FILTER,
        b"FIRST" => SqliteTokenKind::KW_FIRST,
        b"FOLLOWING" => SqliteTokenKind::KW_FOLLOWING,
        b"FOR" => SqliteTokenKind::KW_FOR,
        b"FOREIGN" => SqliteTokenKind::KW_FOREIGN,
        b"FROM" => SqliteTokenKind::KW_FROM,
        b"FULL" => SqliteTokenKind::KW_FULL,
        b"GENERATED" => SqliteTokenKind::KW_GENERATED,
        b"GLOB" => SqliteTokenKind::KW_GLOB,
        b"GROUP" => SqliteTokenKind::KW_GROUP,
        b"GROUPS" => SqliteTokenKind::KW_GROUPS,
        b"HAVING" => SqliteTokenKind::KW_HAVING,
        b"IF" => SqliteTokenKind::KW_IF,
        b"IGNORE" => SqliteTokenKind::KW_IGNORE,
        b"IMMEDIATE" => SqliteTokenKind::KW_IMMEDIATE,
        b"IN" => SqliteTokenKind::KW_IN,
        b"INDEX" => SqliteTokenKind::KW_INDEX,
        b"INDEXED" => SqliteTokenKind::KW_INDEXED,
        b"INITIALLY" => SqliteTokenKind::KW_INITIALLY,
        b"INNER" => SqliteTokenKind::KW_INNER,
        b"INSERT" => SqliteTokenKind::KW_INSERT,
        b"INSTEAD" => SqliteTokenKind::KW_INSTEAD,
        b"INTERSECT" => SqliteTokenKind::KW_INTERSECT,
        b"INTO" => SqliteTokenKind::KW_INTO,
        b"IS" => SqliteTokenKind::KW_IS,
        b"ISNULL" => SqliteTokenKind::KW_ISNULL,
        b"JOIN" => SqliteTokenKind::KW_JOIN,
        b"KEY" => SqliteTokenKind::KW_KEY,
        b"LAST" => SqliteTokenKind::KW_LAST,
        b"LEFT" => SqliteTokenKind::KW_LEFT,
        b"LIKE" => SqliteTokenKind::KW_LIKE,
        b"LIMIT" => SqliteTokenKind::KW_LIMIT,
        b"MATCH" => SqliteTokenKind::KW_MATCH,
        b"MATERIALIZED" => SqliteTokenKind::KW_MATERIALIZED,
        b"NATURAL" => SqliteTokenKind::KW_NATURAL,
        b"NO" => SqliteTokenKind::KW_NO,
        b"NOT" => SqliteTokenKind::KW_NOT,
        b"NOTHING" => SqliteTokenKind::KW_NOTHING,
        b"NOTNULL" => SqliteTokenKind::KW_NOTNULL,
        b"NULL" => SqliteTokenKind::KW_NULL,
        b"NULLS" => SqliteTokenKind::KW_NULLS,
        b"OF" => SqliteTokenKind::KW_OF,
        b"OFFSET" => SqliteTokenKind::KW_OFFSET,
        b"ON" => SqliteTokenKind::KW_ON,
        b"OR" => SqliteTokenKind::KW_OR,
        b"ORDER" => SqliteTokenKind::KW_ORDER,
        b"OTHERS" => SqliteTokenKind::KW_OTHERS,
        b"OUTER" => SqliteTokenKind::KW_OUTER,
        b"OVER" => SqliteTokenKind::KW_OVER,
        b"PARTITION" => SqliteTokenKind::KW_PARTITION,
        b"PLAN" => SqliteTokenKind::KW_PLAN,
        b"PRAGMA" => SqliteTokenKind::KW_PRAGMA,
        b"PRECEDING" => SqliteTokenKind::KW_PRECEDING,
        b"PRIMARY" => SqliteTokenKind::KW_PRIMARY,
        b"QUERY" => SqliteTokenKind::KW_QUERY,
        b"RAISE" => SqliteTokenKind::KW_RAISE,
        b"RANGE" => SqliteTokenKind::KW_RANGE,
        b"RECURSIVE" => SqliteTokenKind::KW_RECURSIVE,
        b"REFERENCES" => SqliteTokenKind::KW_REFERENCES,
        b"REGEXP" => SqliteTokenKind::KW_REGEXP,
        b"REINDEX" => SqliteTokenKind::KW_REINDEX,
        b"RELEASE" => SqliteTokenKind::KW_RELEASE,
        b"RENAME" => SqliteTokenKind::KW_RENAME,
        b"REPLACE" => SqliteTokenKind::KW_REPLACE,
        b"RESTRICT" => SqliteTokenKind::KW_RESTRICT,
        b"RETURNING" => SqliteTokenKind::KW_RETURNING,
        b"RIGHT" => SqliteTokenKind::KW_RIGHT,
        b"ROLLBACK" => SqliteTokenKind::KW_ROLLBACK,
        b"ROW" => SqliteTokenKind::KW_ROW,
        b"ROWS" => SqliteTokenKind::KW_ROWS,
        b"SAVEPOINT" => SqliteTokenKind::KW_SAVEPOINT,
        b"SELECT" => SqliteTokenKind::KW_SELECT,
        b"SET" => SqliteTokenKind::KW_SET,
        b"TABLE" => SqliteTokenKind::KW_TABLE,
        b"TEMP" => SqliteTokenKind::KW_TEMP,
        b"TEMPORARY" => SqliteTokenKind::KW_TEMPORARY,
        b"THEN" => SqliteTokenKind::KW_THEN,
        b"TIES" => SqliteTokenKind::KW_TIES,
        b"TO" => SqliteTokenKind::KW_TO,
        b"TRANSACTION" => SqliteTokenKind::KW_TRANSACTION,
        b"TRIGGER" => SqliteTokenKind::KW_TRIGGER,
        b"UNBOUNDED" => SqliteTokenKind::KW_UNBOUNDED,
        b"UNION" => SqliteTokenKind::KW_UNION,
        b"UNIQUE" => SqliteTokenKind::KW_UNIQUE,
        b"UPDATE" => SqliteTokenKind::KW_UPDATE,
        b"USING" => SqliteTokenKind::KW_USING,
        b"VACUUM" => SqliteTokenKind::KW_VACUUM,
        b"VALUES" => SqliteTokenKind::KW_VALUES,
        b"VIEW" => SqliteTokenKind::KW_VIEW,
        b"VIRTUAL" => SqliteTokenKind::KW_VIRTUAL,
        b"WHEN" => SqliteTokenKind::KW_WHEN,
        b"WHERE" => SqliteTokenKind::KW_WHERE,
        b"WINDOW" => SqliteTokenKind::KW_WINDOW,
        b"WITH" => SqliteTokenKind::KW_WITH,
        b"WITHOUT" => SqliteTokenKind::KW_WITHOUT,

        // These doesn't seem to be keywords but we will treat them as such
        b"TRUE" => SqliteTokenKind::KW_TRUE,
        b"FALSE" => SqliteTokenKind::KW_FALSE,
        b"STORED" => SqliteTokenKind::KW_STORED,
        b"STRICT" => SqliteTokenKind::KW_STRICT,
        b"ROWID" => SqliteTokenKind::KW_ROWID,
        _ => return None,
    };

    Some(result)
}

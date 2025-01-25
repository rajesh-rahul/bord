#![allow(non_snake_case, non_camel_case_types)]

use std::sync::OnceLock;

use ahash::AHashMap;
use enumset::EnumSetType;

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
    pub fn as_str(&self) -> &'static str {
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

pub fn sqlite_keywords() -> &'static AHashMap<&'static [u8], SqliteTokenKind> {
    static MAP: OnceLock<AHashMap<&'static [u8], SqliteTokenKind>> = OnceLock::new();

    MAP.get_or_init(|| {
        let mut map = AHashMap::new();
        map.insert("ABORT".as_bytes(), SqliteTokenKind::KW_ABORT);
        map.insert("ACTION".as_bytes(), SqliteTokenKind::KW_ACTION);
        map.insert("ADD".as_bytes(), SqliteTokenKind::KW_ADD);
        map.insert("AFTER".as_bytes(), SqliteTokenKind::KW_AFTER);
        map.insert("ALL".as_bytes(), SqliteTokenKind::KW_ALL);
        map.insert("ALTER".as_bytes(), SqliteTokenKind::KW_ALTER);
        map.insert("ALWAYS".as_bytes(), SqliteTokenKind::KW_ALWAYS);
        map.insert("ANALYZE".as_bytes(), SqliteTokenKind::KW_ANALYZE);
        map.insert("AND".as_bytes(), SqliteTokenKind::KW_AND);
        map.insert("AS".as_bytes(), SqliteTokenKind::KW_AS);
        map.insert("ASC".as_bytes(), SqliteTokenKind::KW_ASC);
        map.insert("ATTACH".as_bytes(), SqliteTokenKind::KW_ATTACH);
        map.insert(
            "AUTOINCREMENT".as_bytes(),
            SqliteTokenKind::KW_AUTOINCREMENT,
        );
        map.insert("BEFORE".as_bytes(), SqliteTokenKind::KW_BEFORE);
        map.insert("BEGIN".as_bytes(), SqliteTokenKind::KW_BEGIN);
        map.insert("BETWEEN".as_bytes(), SqliteTokenKind::KW_BETWEEN);
        map.insert("BY".as_bytes(), SqliteTokenKind::KW_BY);
        map.insert("CASCADE".as_bytes(), SqliteTokenKind::KW_CASCADE);
        map.insert("CASE".as_bytes(), SqliteTokenKind::KW_CASE);
        map.insert("CAST".as_bytes(), SqliteTokenKind::KW_CAST);
        map.insert("CHECK".as_bytes(), SqliteTokenKind::KW_CHECK);
        map.insert("COLLATE".as_bytes(), SqliteTokenKind::KW_COLLATE);
        map.insert("COLUMN".as_bytes(), SqliteTokenKind::KW_COLUMN);
        map.insert("COMMIT".as_bytes(), SqliteTokenKind::KW_COMMIT);
        map.insert("CONFLICT".as_bytes(), SqliteTokenKind::KW_CONFLICT);
        map.insert("CONSTRAINT".as_bytes(), SqliteTokenKind::KW_CONSTRAINT);
        map.insert("CREATE".as_bytes(), SqliteTokenKind::KW_CREATE);
        map.insert("CROSS".as_bytes(), SqliteTokenKind::KW_CROSS);
        map.insert("CURRENT".as_bytes(), SqliteTokenKind::KW_CURRENT);
        map.insert("CURRENT_DATE".as_bytes(), SqliteTokenKind::KW_CURRENT_DATE);
        map.insert("CURRENT_TIME".as_bytes(), SqliteTokenKind::KW_CURRENT_TIME);
        map.insert(
            "CURRENT_TIMESTAMP".as_bytes(),
            SqliteTokenKind::KW_CURRENT_TIMESTAMP,
        );
        map.insert("DATABASE".as_bytes(), SqliteTokenKind::KW_DATABASE);
        map.insert("DEFAULT".as_bytes(), SqliteTokenKind::KW_DEFAULT);
        map.insert("DEFERRABLE".as_bytes(), SqliteTokenKind::KW_DEFERRABLE);
        map.insert("DEFERRED".as_bytes(), SqliteTokenKind::KW_DEFERRED);
        map.insert("DELETE".as_bytes(), SqliteTokenKind::KW_DELETE);
        map.insert("DESC".as_bytes(), SqliteTokenKind::KW_DESC);
        map.insert("DETACH".as_bytes(), SqliteTokenKind::KW_DETACH);
        map.insert("DISTINCT".as_bytes(), SqliteTokenKind::KW_DISTINCT);
        map.insert("DO".as_bytes(), SqliteTokenKind::KW_DO);
        map.insert("DROP".as_bytes(), SqliteTokenKind::KW_DROP);
        map.insert("EACH".as_bytes(), SqliteTokenKind::KW_EACH);
        map.insert("ELSE".as_bytes(), SqliteTokenKind::KW_ELSE);
        map.insert("END".as_bytes(), SqliteTokenKind::KW_END);
        map.insert("ESCAPE".as_bytes(), SqliteTokenKind::KW_ESCAPE);
        map.insert("EXCEPT".as_bytes(), SqliteTokenKind::KW_EXCEPT);
        map.insert("EXCLUDE".as_bytes(), SqliteTokenKind::KW_EXCLUDE);
        map.insert("EXCLUSIVE".as_bytes(), SqliteTokenKind::KW_EXCLUSIVE);
        map.insert("EXISTS".as_bytes(), SqliteTokenKind::KW_EXISTS);
        map.insert("EXPLAIN".as_bytes(), SqliteTokenKind::KW_EXPLAIN);
        map.insert("FAIL".as_bytes(), SqliteTokenKind::KW_FAIL);
        map.insert("FILTER".as_bytes(), SqliteTokenKind::KW_FILTER);
        map.insert("FIRST".as_bytes(), SqliteTokenKind::KW_FIRST);
        map.insert("FOLLOWING".as_bytes(), SqliteTokenKind::KW_FOLLOWING);
        map.insert("FOR".as_bytes(), SqliteTokenKind::KW_FOR);
        map.insert("FOREIGN".as_bytes(), SqliteTokenKind::KW_FOREIGN);
        map.insert("FROM".as_bytes(), SqliteTokenKind::KW_FROM);
        map.insert("FULL".as_bytes(), SqliteTokenKind::KW_FULL);
        map.insert("GENERATED".as_bytes(), SqliteTokenKind::KW_GENERATED);
        map.insert("GLOB".as_bytes(), SqliteTokenKind::KW_GLOB);
        map.insert("GROUP".as_bytes(), SqliteTokenKind::KW_GROUP);
        map.insert("GROUPS".as_bytes(), SqliteTokenKind::KW_GROUPS);
        map.insert("HAVING".as_bytes(), SqliteTokenKind::KW_HAVING);
        map.insert("IF".as_bytes(), SqliteTokenKind::KW_IF);
        map.insert("IGNORE".as_bytes(), SqliteTokenKind::KW_IGNORE);
        map.insert("IMMEDIATE".as_bytes(), SqliteTokenKind::KW_IMMEDIATE);
        map.insert("IN".as_bytes(), SqliteTokenKind::KW_IN);
        map.insert("INDEX".as_bytes(), SqliteTokenKind::KW_INDEX);
        map.insert("INDEXED".as_bytes(), SqliteTokenKind::KW_INDEXED);
        map.insert("INITIALLY".as_bytes(), SqliteTokenKind::KW_INITIALLY);
        map.insert("INNER".as_bytes(), SqliteTokenKind::KW_INNER);
        map.insert("INSERT".as_bytes(), SqliteTokenKind::KW_INSERT);
        map.insert("INSTEAD".as_bytes(), SqliteTokenKind::KW_INSTEAD);
        map.insert("INTERSECT".as_bytes(), SqliteTokenKind::KW_INTERSECT);
        map.insert("INTO".as_bytes(), SqliteTokenKind::KW_INTO);
        map.insert("IS".as_bytes(), SqliteTokenKind::KW_IS);
        map.insert("ISNULL".as_bytes(), SqliteTokenKind::KW_ISNULL);
        map.insert("JOIN".as_bytes(), SqliteTokenKind::KW_JOIN);
        map.insert("KEY".as_bytes(), SqliteTokenKind::KW_KEY);
        map.insert("LAST".as_bytes(), SqliteTokenKind::KW_LAST);
        map.insert("LEFT".as_bytes(), SqliteTokenKind::KW_LEFT);
        map.insert("LIKE".as_bytes(), SqliteTokenKind::KW_LIKE);
        map.insert("LIMIT".as_bytes(), SqliteTokenKind::KW_LIMIT);
        map.insert("MATCH".as_bytes(), SqliteTokenKind::KW_MATCH);
        map.insert("MATERIALIZED".as_bytes(), SqliteTokenKind::KW_MATERIALIZED);
        map.insert("NATURAL".as_bytes(), SqliteTokenKind::KW_NATURAL);
        map.insert("NO".as_bytes(), SqliteTokenKind::KW_NO);
        map.insert("NOT".as_bytes(), SqliteTokenKind::KW_NOT);
        map.insert("NOTHING".as_bytes(), SqliteTokenKind::KW_NOTHING);
        map.insert("NOTNULL".as_bytes(), SqliteTokenKind::KW_NOTNULL);
        map.insert("NULL".as_bytes(), SqliteTokenKind::KW_NULL);
        map.insert("NULLS".as_bytes(), SqliteTokenKind::KW_NULLS);
        map.insert("OF".as_bytes(), SqliteTokenKind::KW_OF);
        map.insert("OFFSET".as_bytes(), SqliteTokenKind::KW_OFFSET);
        map.insert("ON".as_bytes(), SqliteTokenKind::KW_ON);
        map.insert("OR".as_bytes(), SqliteTokenKind::KW_OR);
        map.insert("ORDER".as_bytes(), SqliteTokenKind::KW_ORDER);
        map.insert("OTHERS".as_bytes(), SqliteTokenKind::KW_OTHERS);
        map.insert("OUTER".as_bytes(), SqliteTokenKind::KW_OUTER);
        map.insert("OVER".as_bytes(), SqliteTokenKind::KW_OVER);
        map.insert("PARTITION".as_bytes(), SqliteTokenKind::KW_PARTITION);
        map.insert("PLAN".as_bytes(), SqliteTokenKind::KW_PLAN);
        map.insert("PRAGMA".as_bytes(), SqliteTokenKind::KW_PRAGMA);
        map.insert("PRECEDING".as_bytes(), SqliteTokenKind::KW_PRECEDING);
        map.insert("PRIMARY".as_bytes(), SqliteTokenKind::KW_PRIMARY);
        map.insert("QUERY".as_bytes(), SqliteTokenKind::KW_QUERY);
        map.insert("RAISE".as_bytes(), SqliteTokenKind::KW_RAISE);
        map.insert("RANGE".as_bytes(), SqliteTokenKind::KW_RANGE);
        map.insert("RECURSIVE".as_bytes(), SqliteTokenKind::KW_RECURSIVE);
        map.insert("REFERENCES".as_bytes(), SqliteTokenKind::KW_REFERENCES);
        map.insert("REGEXP".as_bytes(), SqliteTokenKind::KW_REGEXP);
        map.insert("REINDEX".as_bytes(), SqliteTokenKind::KW_REINDEX);
        map.insert("RELEASE".as_bytes(), SqliteTokenKind::KW_RELEASE);
        map.insert("RENAME".as_bytes(), SqliteTokenKind::KW_RENAME);
        map.insert("REPLACE".as_bytes(), SqliteTokenKind::KW_REPLACE);
        map.insert("RESTRICT".as_bytes(), SqliteTokenKind::KW_RESTRICT);
        map.insert("RETURNING".as_bytes(), SqliteTokenKind::KW_RETURNING);
        map.insert("RIGHT".as_bytes(), SqliteTokenKind::KW_RIGHT);
        map.insert("ROLLBACK".as_bytes(), SqliteTokenKind::KW_ROLLBACK);
        map.insert("ROW".as_bytes(), SqliteTokenKind::KW_ROW);
        map.insert("ROWS".as_bytes(), SqliteTokenKind::KW_ROWS);
        map.insert("SAVEPOINT".as_bytes(), SqliteTokenKind::KW_SAVEPOINT);
        map.insert("SELECT".as_bytes(), SqliteTokenKind::KW_SELECT);
        map.insert("SET".as_bytes(), SqliteTokenKind::KW_SET);
        map.insert("TABLE".as_bytes(), SqliteTokenKind::KW_TABLE);
        map.insert("TEMP".as_bytes(), SqliteTokenKind::KW_TEMP);
        map.insert("TEMPORARY".as_bytes(), SqliteTokenKind::KW_TEMPORARY);
        map.insert("THEN".as_bytes(), SqliteTokenKind::KW_THEN);
        map.insert("TIES".as_bytes(), SqliteTokenKind::KW_TIES);
        map.insert("TO".as_bytes(), SqliteTokenKind::KW_TO);
        map.insert("TRANSACTION".as_bytes(), SqliteTokenKind::KW_TRANSACTION);
        map.insert("TRIGGER".as_bytes(), SqliteTokenKind::KW_TRIGGER);
        map.insert("UNBOUNDED".as_bytes(), SqliteTokenKind::KW_UNBOUNDED);
        map.insert("UNION".as_bytes(), SqliteTokenKind::KW_UNION);
        map.insert("UNIQUE".as_bytes(), SqliteTokenKind::KW_UNIQUE);
        map.insert("UPDATE".as_bytes(), SqliteTokenKind::KW_UPDATE);
        map.insert("USING".as_bytes(), SqliteTokenKind::KW_USING);
        map.insert("VACUUM".as_bytes(), SqliteTokenKind::KW_VACUUM);
        map.insert("VALUES".as_bytes(), SqliteTokenKind::KW_VALUES);
        map.insert("VIEW".as_bytes(), SqliteTokenKind::KW_VIEW);
        map.insert("VIRTUAL".as_bytes(), SqliteTokenKind::KW_VIRTUAL);
        map.insert("WHEN".as_bytes(), SqliteTokenKind::KW_WHEN);
        map.insert("WHERE".as_bytes(), SqliteTokenKind::KW_WHERE);
        map.insert("WINDOW".as_bytes(), SqliteTokenKind::KW_WINDOW);
        map.insert("WITH".as_bytes(), SqliteTokenKind::KW_WITH);
        map.insert("WITHOUT".as_bytes(), SqliteTokenKind::KW_WITHOUT);

        // These doesn't seem to be keywords but we will treat them as such
        map.insert("TRUE".as_bytes(), SqliteTokenKind::KW_TRUE);
        map.insert("FALSE".as_bytes(), SqliteTokenKind::KW_FALSE);
        map.insert("STORED".as_bytes(), SqliteTokenKind::KW_STORED);
        map.insert("STRICT".as_bytes(), SqliteTokenKind::KW_STRICT);
        map.insert("ROWID".as_bytes(), SqliteTokenKind::KW_ROWID);

        map
    })
}

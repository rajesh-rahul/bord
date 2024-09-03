//! This is a parser to parse SQLite3(v.3.46.0 onwards). It follows the excellent error 
//! resillient parsing [guide]((https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html)) 
//! by matklad

mod cst;
mod parser;
mod token_kind;
mod tree_kind;
mod version;
mod grammar;
mod cursor;
mod lexer;

pub use cst::{ChildNodeKey, NodeId, SqliteNode, SqliteToken, SqliteUntypedAst};

pub use token_kind::{sqlite_keywords, SqliteTokenKind, MAX_KEYWORD_LEN};

pub use tree_kind::SqliteTreeKind;

pub use version::SqliteVersion;

pub use parser::{SqliteParseError, SqliteParser};

pub use lexer::{SqliteLexer, LexError};

pub use grammar::parse;
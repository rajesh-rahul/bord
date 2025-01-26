//! This is a parser to parse SQLite3(v.3.46.0 onwards). It follows the excellent error
//! resillient parsing [guide]((https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html))
//! by matklad

// mod cst;
mod cst;
mod cursor;
mod grammar;
mod lexer;
mod parser;
mod token_kind;
mod tree_kind;
pub mod ungram;
mod version;

pub use cst::{ChildNodeKey, CstNode, CstNodeData, NodeId, SqliteToken, SqliteUntypedCst};

use enumset::EnumSet;
pub use token_kind::{sqlite_keywords, SqliteTokenKind, MAX_KEYWORD_LEN};

pub use tree_kind::SqliteTreeKind;

pub use version::SqliteVersion;

pub use parser::{SqliteParseError, SqliteParser};

pub use lexer::{LexError, SqliteLexer};

pub mod ast;

#[cfg(feature = "test_utils")]
pub mod test_utils;

pub fn parse(text: &str) -> SqliteUntypedCst {
    let (tokens, _) = SqliteLexer::new(text, SqliteVersion([3, 46, 0])).lex();

    let mut p = SqliteParser::new(tokens);
    grammar::file(&mut p, Default::default());

    let result = p.build_tree();

    result
}

pub fn parse_any(
    text: &str,
    r: EnumSet<SqliteTokenKind>,
    parse_function: fn(&mut SqliteParser, EnumSet<SqliteTokenKind>),
) -> SqliteUntypedCst {
    let (tokens, _) = SqliteLexer::new(text, SqliteVersion([3, 46, 0])).lex();
    let mut p = SqliteParser::new(tokens);
    parse_function(&mut p, r);

    p.build_tree()
}

#[test]
fn simple_parser_test() {
    let input = "WITH derived AS (
            SELECT MAX(a) AS max_a,
                   COUNT(b) AS b_num,
                   user_id
            FROM `TABLE`
            GROUP BY user_id
        )
        SELECT * FROM `table`
        LEFT JOIN derived USING (user_id);";

    let cst = parse(input);

    println!("{cst}");
    assert!(cst.errors().is_empty());
}

//! This is a parser to parse SQLite3(v.3.46.0 onwards). It follows the excellent error
//! resillient parsing [guide]((https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html))
//! by matklad

// mod cst;
mod cst2;
mod cursor;
mod grammar;
mod lexer;
mod new_cst;
mod parser;
mod token_kind;
mod tree_kind;
pub mod ungram;
mod version;

// pub use new_cst::*;

pub use cst2::{
    ChildNodeKey, CstNode, CstNodeData, CstNodeDataKind, LexError, ModifiedBranchesInfo, NodeId,
    SqliteToken, SqliteUntypedCst, TextPatch, TextPatchKind,
};

// use cst::incr::{IncrCstNode, IncrSqlCst};
// pub use cst::*;

use enumset::EnumSet;
use grammar::utils::STATEMENT_START;
use parser::new_on_demand_lexer;
pub use token_kind::{sqlite_keywords, SqliteTokenKind, MAX_KEYWORD_LEN};

pub use tree_kind::SqliteTreeKind;

pub use version::SqliteVersion;

pub use parser::{
    ExpectedItem, NormalLexer, OnDemandLexer, ParseErrorKind, SqliteParseError, SqliteParser,
};

pub use lexer::SqliteLexer;

// pub mod ast;

#[cfg(feature = "test_utils")]
pub mod test_utils;

pub fn parse(text: &str) -> SqliteUntypedCst {
    let lexer = SqliteLexer::new(text, SqliteVersion([3, 46, 0]));

    let mut p = SqliteParser::new(NormalLexer::from(lexer));
    grammar::file(&mut p, Default::default());

    p.build_cst()
}

pub fn incremental_parse(text: &str, text_patch: TextPatch<usize, usize>) -> SqliteUntypedCst {
    let on_demand_lexer = new_on_demand_lexer(text, SqliteVersion([3, 46, 0]));
    let mut p = SqliteParser::with_abs_pos(on_demand_lexer, text_patch.relex_start);

    let m = p.open();
    let r = STATEMENT_START | T![;];

    let mut extra_loops = 0;
    while !p.eof() {
        p.eat_trivia();

        if p.at_any(STATEMENT_START) {
            grammar::statement(&mut p, r);
        } else {
            expected_one_of!(p, r, SqliteTreeKind::Statement);
            // p.expected(SqliteTreeKind::Statement, STATEMENT_START);
        }

        p.must_eat(T![;], r);

        // NOTE: It seems that affected_node_by_len is an overestimate (prolly not worth fixing though)
        let expected_len = text_patch.start - text_patch.relex_start
            + text_patch.size
            + text_patch.affected_node_byte_len;
        let actual_len = p.curr_byte_len();

        let spillover = actual_len as i64 - expected_len as i64;

        // If we consumed all tokens that were added during this text patch, go through the loop
        // the loop one more time to deal with edge cases like dangling semicolons
        if spillover > 0 {
            if extra_loops > 0 {
                break;
            } else {
                extra_loops += 1;
            }
        }

        p.eat_trivia();
    }

    p.eat_trivia();

    p.close(m, SqliteTreeKind::File);

    p.build_cst()
}

pub fn parse_any(
    text: &str,
    r: EnumSet<SqliteTokenKind>,
    parse_function: fn(&mut SqliteParser<NormalLexer>, EnumSet<SqliteTokenKind>),
) -> SqliteUntypedCst {
    let lexer = SqliteLexer::new(text, SqliteVersion([3, 46, 0]));

    let mut p = SqliteParser::new(NormalLexer::from(lexer));
    parse_function(&mut p, r);

    p.build_cst()
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

    let clause = cst
        .root()
        .me_and_descendants()
        .find(|it| it.tree() == Some(SqliteTreeKind::CteClause))
        .unwrap();

    println!("{}`", clause.to_text());
    assert!(cst.errors().next().is_none());
}

#[test]
fn simple_parser_test2() {
    let cst = parse("\n\n\n");

    assert_eq!("\n\n\n", cst.root().to_text());
}

#[test]
fn simple_parser_test3() {
    let cst = parse(
        "SELECT * FROM users;


S",
    );

    println!("{cst}");
}

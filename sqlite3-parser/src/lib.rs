//! This is a parser to parse SQLite3(v.3.46.0 onwards). It follows the excellent error
//! resillient parsing [guide]((https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html))
//! by matklad

// mod cst;
// mod cst2;
mod cst;
mod cursor;
mod grammar;
mod lexer;
mod parser;
mod token_kind;
mod tree_kind;
pub mod ungram;
mod version;

pub use cst::*;

// pub use cst2::{
//     ChildNodeKey, CstNode, CstNodeData, CstNodeDataKind, LexError, ModifiedBranchesInfo, NodeId,
//     SqliteToken, SqliteUntypedCst, TextPatch, TextPatchKind,
// };

// use cst::incr::{IncrCstNode, IncrSqlCst};
// pub use cst::*;

use enumset::EnumSet;
use grammar::utils::STATEMENT_START;
use parser::{new_on_demand_lexer, Event};
use text_size::TextSize;
pub use token_kind::{sqlite_keywords, SqliteTokenKind, MAX_KEYWORD_LEN};

pub use tree_kind::SqliteTreeKind;

pub use version::SqliteVersion;

pub use parser::{
    ExpectedItem, NormalLexer, OnDemandLexer, ParseErrorKind, SqliteParseError, SqliteParser,
};

pub use lexer::SqliteLexer;
pub use text_size;
// pub mod ast;

#[cfg(feature = "test_utils")]
pub mod test_utils;

pub fn parse_with_abs_pos<CST: CstTrait>(abs_pos: TextSize, text: &str) -> CST {
    let lexer = SqliteLexer::new(text, SqliteVersion([3, 46, 0]));

    let mut p = SqliteParser::with_abs_pos(NormalLexer::from(lexer), abs_pos);
    grammar::file(&mut p, Default::default());

    p.build_cst()
}

pub fn parse<CST: CstTrait>(text: &str) -> CST {
    parse_with_abs_pos(TextSize::new(0), text)
}

pub fn parse_events_and_tokens(text: &str) -> (Vec<Event>, Vec<SqliteToken>) {
    let lexer = SqliteLexer::new(text, SqliteVersion([3, 46, 0]));

    let mut p = SqliteParser::with_abs_pos(NormalLexer::from(lexer), TextSize::new(0));
    grammar::file(&mut p, Default::default());

    p.to_events_and_tokens()
}

pub fn incremental_parse(
    text: &str,
    text_patch: TextPatch<TextSize, TextSize>,
) -> (Vec<Event>, Vec<SqliteToken>) {
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

        let was_at_semicolon = p.at(T![;]);
        p.must_eat(T![;], r);

        if !was_at_semicolon {
            continue;
        }

        // NOTE: It seems that affected_node_by_len is an overestimate (prolly not worth fixing though)
        let expected_len = text_patch.start - text_patch.relex_start
            + text_patch.size
            + text_patch.affected_node_byte_len;
        let actual_len = p.curr_byte_len();

        let spillover = actual_len.checked_add(expected_len);

        // If we consumed all tokens that were added during this text patch, go through the loop
        // the loop one more time to deal with edge cases like dangling semicolons
        if spillover.is_some() {
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

    p.to_events_and_tokens()
}

pub fn incremental_parse2<Cst: CstTrait>(
    text: &str,
    text_patch: TextPatch<TextSize, TextSize>,
) -> Cst {
    let on_demand_lexer = new_on_demand_lexer(text, SqliteVersion([3, 46, 0]));
    let mut p = SqliteParser::with_abs_pos(on_demand_lexer, text_patch.relex_start);

    let m = p.open();
    let r = STATEMENT_START | T![;];

    let extra_loops = 0;
    while !p.eof() {
        p.eat_trivia();

        if p.at_any(STATEMENT_START) {
            grammar::statement(&mut p, r);
        } else {
            expected_one_of!(p, r, SqliteTreeKind::Statement);
            // p.expected(SqliteTreeKind::Statement, STATEMENT_START);
        }

        let was_at_semicolon = p.at(T![;]);
        p.must_eat(T![;], r);

        if !was_at_semicolon {
            continue;
        }

        // NOTE: It seems that affected_node_by_len is an overestimate (prolly not worth fixing though)
        // let expected_len = text_patch.start - text_patch.relex_start
        //     + text_patch.size
        //     + text_patch.affected_node_byte_len;
        // let actual_len = p.curr_byte_len();

        // let spillover = actual_len as i64 - expected_len as i64;

        // // If we consumed all tokens that were added during this text patch, go through the loop
        // // the loop one more time to deal with edge cases like dangling semicolons
        // if spillover > 0 {
        //     if extra_loops > 0 {
        //         break;
        //     } else {
        //         extra_loops += 1;
        //     }
        // }

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
) -> incr::IncrSqlCst {
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

    let cst: batch::SqlCst = parse(input);

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
    let cst: incr::IncrSqlCst = parse("\n\n\n");

    assert_eq!("\n\n\n", cst.root().to_text());
}

#[test]
fn simple_parser_test3() {
    let cst: incr::IncrSqlCst = parse("CREATE TABLE f     ");

    println!("{:#?}", cst.root().comparable());
}

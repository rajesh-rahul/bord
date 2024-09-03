use std::time::Instant;

use enumset::{enum_set, EnumSet};

use crate::cst::SqliteUntypedAst;
use crate::{SqliteLexer, SqliteTokenKind, SqliteTreeKind, SqliteVersion, T};
use crate::parser::*;

use SqliteTokenKind::*;
use SqliteTreeKind::*;
use utils::*;

const STMT_RECOV: EnumSet<SqliteTokenKind> = enum_set!(KW_SELECT);
const SELECT_STMT_RECOV: EnumSet<SqliteTokenKind> =
    enum_set!(KW_FROM | KW_WHERE | KW_ORDER | KW_LIMIT | KW_HAVING | KW_GROUP | SEMICOLON);


pub fn parse(text: &str) -> (SqliteUntypedAst, Vec<SqliteParseError>) {
        let start = Instant::now();
        let (tokens, _) = SqliteLexer::new(text, SqliteVersion([3, 46, 0])).lex();
    
        let mut p = SqliteParser::new(tokens);
        file(&mut p);
    
        let result = p.build_tree();
        let duration = start.elapsed();
        eprintln!("{:?}", result.0);
        eprintln!("Tree build time: {duration:?}");
    
        result
}

    
fn file(p: &mut SqliteParser) {
    let m = p.open();
    while !p.eof() {
        if p.at(KW_SELECT) {
            select_stmt(p, STMT_RECOV);
        } else {
            p.try_or_adv("expected a statement", STMT_RECOV);
        }
    }
    p.eat_whitespace();
    p.close(m, File);
}

fn select_stmt(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) {
    assert!(p.at(KW_SELECT));
    let m = p.open();

    p.expect(KW_SELECT);
    p.eat_any(KW_DISTINCT | KW_ALL);

    result_column_list(p, r | SELECT_STMT_RECOV);

    if p.at(KW_FROM) {
        from_clause(p, r | SELECT_STMT_RECOV);
    }

    p.expect_or_advance(T![;], STMT_RECOV);

    p.close(m, SelectStmt);
}

fn from_clause(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) {
    assert!(p.at(KW_FROM));

    let m = p.open();
    p.expect(KW_FROM);

    if utils::at_table_or_subquery(p) {
        join_clause(p, r);
    } else {
        p.try_or_adv("expected a table or subquery", r);
    }

    p.close(m, FromClause);
}

fn join_clause(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) {
    assert!(utils::at_table_or_subquery(p));
    // let m = p.open();
    let mut lhs_marker = table_or_subquery(p, r);

    loop {
        if !utils::at_join_operator(p) {
            break;
        }

        let m = p.open_before(lhs_marker);
        join_operator(p, r);
        if utils::at_table_or_subquery(p) {
            table_or_subquery(p, r);
        } else {
            p.try_or_adv("expected a table or subquery", r);
        }

        // TODO: Verify this is valid for all type of joins
        // NOTE: Join constraint is optional
        if utils::at_join_constraint(p) {
            join_constraint(p, r);
        }
        lhs_marker = p.close(m, JoinClause);
    }
}

// NOTE: join constraint is allowed to match nothing according to SQLite railroad diagrams.
// We won't do that here (yet)
fn join_constraint(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) {
    assert!(utils::at_join_constraint(p));

    let m = p.open();

    if p.eat(KW_ON) {
        utils::expect_expr(p, r);
    } else if p.eat(KW_USING) {
        p.expect_or_advance(T!['('], r);
        p.expect_or_advance(IDEN, r);
        while p.at(T![,]) {
            p.expect(T![,]);
            p.expect_or_advance(IDEN, r);
        }
        p.expect_or_advance(T![')'], r);
    } else {
        unreachable!("JOIN_CONSTRAINT_START is wrong")
    }

    p.close(m, JoinConstraint);
}

fn table_or_subquery(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) -> MarkClosed {
    assert!(utils::at_table_or_subquery(p));

    let m = p.open();
    if p.at(T!['(']) {
        p.expect(T!['(']);
        if p.at(KW_SELECT) {
            select_stmt(p, r);
            p.expect_or_advance(T![')'], r);
            return p.close(m, TableOrSubquery);
        } else if at_table_or_subquery(p) {
            join_clause(p, r);
            p.expect_or_advance(T![')'], r);
            return p.close(m, TableOrSubquery);
        } else {
            p.try_or_adv("expected a subquery or table", r);
        }
    } else if p.at(IDEN) {
        let m = p.open();
        // Has schema specifier
        if p.nth(1) == T![.] {
            p.expect(IDEN);
            p.expect(T![.]);
            p.expect_or_advance(IDEN, r);
        } else {
            p.expect(IDEN);
        }

        // TODO: Table valued functions
        if p.eat(T!['(']) {
            utils::expect_expr(p, r | T![,]);
            while p.eat(T![,]) {
                utils::expect_expr(p, r | T![,]);
            }

            p.expect_or_advance(T![')'], r);
            p.close(m, TableValueFunction);
        } else {
            p.close(m, TableName);
        }

        // TODO: INDEXED BY ...
    } else {
        unreachable!("TABLE_OR_SUBQUERY start is incorrect")
    }

    return p.close(m, TableOrSubquery);
}

/// ``
fn table_name_or_table_valued_func_name(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) {
    assert!(p.at(IDEN));
    let m = p.open();

    // Table Valued Function
    if p.eat(T!['(']) {
        if !at_expr(p) {
            p.try_or_adv("expected expression", r);
        }

        expr(p, r);
        while p.eat(T![,]) {
            if !at_expr(p) {
                p.try_or_adv("expected expression", r);
            }
        }
    }
}

fn join_operator(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) {
    assert!(utils::at_join_operator(p));

    let m = p.open();

    if p.at(T![,]) {
        p.expect(T![,]);
        p.close(m, JoinOperator);
        return;
    }

    if p.eat(KW_CROSS) {
        p.expect_or_advance(KW_JOIN, r);
        p.close(m, JoinOperator);
        return;
    }

    let ate_kw_natural = p.eat(KW_NATURAL);

    if p.eat(KW_JOIN) {
        p.close(m, JoinOperator);
        return;
    }

    if p.eat_any(KW_LEFT | KW_RIGHT | KW_FULL) {
        p.eat(KW_OUTER);
        p.expect_or_advance(KW_JOIN, r);
        p.close(m, JoinOperator);
        return;
    }

    if p.eat(KW_INNER) {
        p.expect_or_advance(KW_JOIN, r);
        p.close(m, JoinOperator);
        return;
    }

    if ate_kw_natural {
        p.try_or_adv("expected one of: LEFT, RIGHT, FULL, INNER, or JOIN", r);
        p.close(m, JoinOperator);
        return;
    }

    unreachable!("JOIN_OPERATOR_START is incorrect")
}

// pub const TABLE_OR_SUBQUERY_RECOV

// pub const RESULT_COLUMN_LIST_RECOV: RecoveryDef = recovery![KW_FROM | KW_WHEN, KW_ORDER];

fn result_column_list(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) {
    // At least one result column is expected
    if !utils::at_result_column(p) {
        p.try_or_adv("Expected result column", r);
        return;
    }

    let m = p.open();

    result_column(p, r);

    while p.at(T![,]) {
        p.expect(T![,]);

        if utils::at_result_column(p) {
            result_column(p, r);
        } else {
            p.try_or_adv("Expected result column", r);
        }
    }

    p.close(m, ResultColumnList);
}

fn result_column(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) {
    assert!(utils::at_result_column(p));
    let m = p.open();

    // CASE 1: SELECT *
    if p.eat(T![*]) {
        p.close(m, ResultColumn);
        return;
    }

    // CASE 2: SELECT country.*
    // This case needs to be handled before expr because expr can also match this
    if matches!(p.tokens(), [IDEN, T![.], T![*], ..]) {
        p.advance_by(3);
        p.close(m, ResultColumn);
        return;
    }

    // CASE 3: Any expr
    assert!(
        utils::at_expr(p),
        "DEV ERROR: at_result_column is incorrect"
    );

    expr(p, r);

    // Expressions may be given an alias:
    // Example 1: SELECT 1 as number;
    // Example 2: SELECT 1 number;
    if p.at(KW_AS) {
        p.wrap(ColumnAlias, |p| {
            p.expect(KW_AS);
            p.expect_or_advance(IDEN, r);
        });
    } else if p.at(IDEN) {
        p.wrap(ColumnAlias, |p| {
            p.expect(IDEN);
        });
    }

    p.close(m, ResultColumn);
    return;
}

/// skipped_op
fn expr(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) -> MarkClosed {
    assert!(utils::at_expr(p));

    expr_bp(p, r, 0)
}

fn expr_lit(p: &mut SqliteParser) -> MarkClosed {
    assert!(utils::at_expr_lit(p));

    p.wrap(Expr, |p| {
        p.wrap(ExprLit, |p| p.advance());
    })
}

fn expr_bracketed(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) -> MarkClosed {
    assert!(utils::at_expr_bracketed(p));
    p.wrap(Expr, |p| {
        p.wrap(ExprParen, |p| {
            p.expect(T!['(']);
            utils::expect_expr(p, r);
            p.expect_or_advance(T![')'], r);
        });
    })
}

fn expr_column_name(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) -> MarkClosed {
    assert!(utils::at_expr_column_name(p));

    let m = p.open();
    p.wrap(ExprColumnName, |p| {
        p.eat(IDEN);

        if p.eat(T![.]) {
            p.expect_or_advance(IDEN, r);

            if p.eat(T![.]) {
                p.expect_or_advance(IDEN, r);
            }
        }
    });

    p.close(m, Expr)
}

fn expr_prefix(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) -> MarkClosed {
    assert!(utils::at_prefix_op(p));

    let m = p.open();

    p.wrap(ExprPrefix, |p| {
        let op = parse_prefix_op(p);
        let (None, Some(r_bp)) = precedence_table(op) else {
            unreachable!("DEV ERROR: Operator should be prefix")
        };

        // NOTE: We use expr_bp here so cannot use `utils::expect_expr`
        if utils::at_expr(p) {
            expr_bp(p, r, r_bp);
        } else {
            p.try_or_adv("expected an expression", r);
        }
    });

    p.close(m, Expr)
}

/// Parsing expressions involve parsing the "left hand side" and optionally
/// parsing a postfix operator or infix operator followed by the right hand side.
/// "left hand side", even if incomplete, always exists for expressions. The most complicated
/// piece of code in this parser by a long shot.
fn expr_bp(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>, min_bp: u8) -> MarkClosed {
    assert!(at_expr(p));
    // let expr_m = p.open();
    // Section 1: This large if block Handles left hand side of the expr. `MarkClosed` from the freshly
    // parsed SqliteTreeNode maybe used later if there is a postfix/infix operator. In that case,
    // this lhs SqliteTreeNode will become the child of a larger expression and MarkClosed will
    // be used with the parser's `open_before` API to facilitate this. If there is no postfix
    // or infix operator, the lhs part of the expression is taken to be the complete expression.
    let mut lhs_marker = if utils::at_expr_lit(p) {
        expr_lit(p)
    } else if utils::at_expr_bracketed(p) {
        expr_bracketed(p, r)
    } else if utils::at_expr_bind_param(p) {
        todo!()
    } else if utils::at_expr_column_name(p) {
        expr_column_name(p, r)
    } else if utils::at_prefix_op(p) {
        expr_prefix(p, r)
    } else if utils::at_expr_function(p) {
        todo!()
    } else if utils::at_expr_cast(p) {
        todo!()
    } else if utils::at_expr_select(p) {
        todo!()
    } else if utils::at_expr_case(p) {
        todo!()
    } else {
        unreachable!("DEV ERROR: at_expr is incorrect")
    };

    // Section 2: The loop has two main cases:
    // - postfix operators
    // - infix operators
    loop {
        // Section 2.1: Handle postfix operators
        if let Some(postfix_op) = utils::which_postfix_op(p) {
            // If we think we are also at an infix_op, there is ambiguity and usually means
            // our at_infix_op is not specific enough (we only check start tokens after all)
            assert!(!utils::which_infix_op(p).is_none());

            let (Some(l_bp), None) = precedence_table(postfix_op) else {
                unreachable!("DEV ERROR: Operator should be postfix")
            };

            if l_bp < min_bp {
                break;
            }

            let m = p.open_before(lhs_marker);
            parse_postfix_op(p, r);
            let child_close_m = p.close(m, ExprPostfix);

            // Wrap in Expr. For example, ExprPostfix{...} -> Expr { ExprPostfix {...} }
            lhs_marker = p.wrap_as_parent(child_close_m, Expr);

            continue;
        }

        // Section 2.2: Handle infix operators
        if let Some(infix_op) = utils::which_infix_op(p) {
            // Even though KW_AND is binary op on its own, it also appears as part of the
            // `[expr] BETWEEN [expr] AND [expr]` operator. Therefore, we have to ignore
            // occurnces of KW_AND when we are already trying to parse a between and operator
            if infix_op == OpAnd && utils::is_processing_between_and_op(p) {
                break;
            }

            let (Some(l_bp), Some(r_bp)) = precedence_table(infix_op) else {
                unreachable!("DEV ERROR: Operator should be infix")
            };

            if l_bp < min_bp {
                break;
            }
            // Wrap our LHS in Expr. For example, ExprLit{...} -> Expr { ExprLit {...} }
            let m = p.open_before(lhs_marker);

            // Section 2.2.1: LIKE | NOT LIKE are special case
            if matches!(infix_op, OpLike | OpNotLike) {
                if infix_op == OpNotLike {
                    p.expect(KW_NOT);
                }
                p.expect(KW_LIKE);

                // Parse RHS
                if at_expr(p) {
                    expr_bp(p, r, r_bp);
                } else {
                    p.try_or_adv("expected a right hand side for the expression", r);
                }

                if p.eat(KW_ESCAPE) {
                    // TODO: SQLite spec actually allow an expr here but we will only support
                    // string literals for now
                    p.expect_or_advance(STR_LIT, r);
                }
            }
            // Section 2.2.2: BETWEEN [expr] AND [expr] is yet another special case
            else if matches!(infix_op, OpBetweenAnd | OpNotBetweenAnd) {
                let m = p.open();

                if infix_op == OpNotBetweenAnd {
                    p.expect(KW_NOT);
                }
                p.expect(KW_BETWEEN);
                utils::expect_expr(p, r);

                p.expect_or_advance(KW_AND, r);
                p.close(m, infix_op);

                // Parse RHS
                if at_expr(p) {
                    expr_bp(p, r, r_bp);
                } else {
                    p.try_or_adv("expected a right hand side for the expression", r);
                }
            }
            // Section 2.2.3: Handle normal infix operators
            else {
                parse_infix_op(p, r);

                // Parse RHS
                if at_expr(p) {
                    expr_bp(p, r, r_bp);
                } else {
                    p.try_or_adv("expected a right hand side for the expression", r);
                }
            }

            let child_close_m = p.close(m, ExprInfix);

            // Wrap in Expr. For example, ExprInfix{...} -> Expr { ExprInfix {...} }
            lhs_marker = p.wrap_as_parent(child_close_m, Expr);

            continue;
        }

        break;
    }

    // p.close(expr_m, Expr)
    lhs_marker
}

fn parse_prefix_op(p: &mut SqliteParser) -> SqliteTreeKind {
    assert!(utils::at_prefix_op(p));

    let m = p.open();

    let op_node = match p.nth(0) {
        T![~] => OpBinComplement,
        T![+] => OpUnaryPlus,
        T![-] => OpUnaryMinus,
        KW_NOT => OpNot,
        _ => unreachable!("DEV ERROR: at_prefix_op is incorrect"),
    };

    p.advance();
    p.close(m, op_node);

    op_node
}

fn parse_postfix_op(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) {
    let Some(op_node) = utils::which_postfix_op(p) else {
        unreachable!("DEV ERROR: parse_postfix_op called incorrectly");
    };

    let m = p.open();
    match op_node {
        OpNotSpaceNull => {
            p.advance_by(2);
        }
        OpCollate => {
            p.expect(KW_COLLATE);

            // SQLite supports collation names specified as string literals or identifiers.
            // For ex: 'NOCASE' v/s NOCASE respectively
            if p.at_any(IDEN | STR_LIT) {
                p.advance();
            } else {
                p.try_or_adv("expected a collation name, one of: [BINARY, NO_CASE]", r)
            }
        }
        _ => p.advance(),
    }

    p.close(m, op_node);
}

fn parse_infix_op(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) {
    let Some(op_node) = utils::which_infix_op(p) else {
        unreachable!("DEV ERROR: parse_infix_op called incorrectly");
    };

    debug_assert!(
        !matches!(op_node, OpNotLike | OpLike | OpNotBetweenAnd | OpBetweenAnd),
        "While these are infix ops, they are meant to handled specially and not here"
    );

    match op_node {
        // Operators that are two tokens wide
        OpNotIn | OpNotMatch | OpNotRegexp | OpNotGlob | OpNotSpaceNull | OpIsNot => {
            p.wrap(op_node, |p| p.advance_by(2));
        }

        // IS DISTINCT FROM / IS NOT DISTINCT FROM is unusually wide.
        OpIsDistinctFrom | OpIsNotDistinctFrom => {
            p.wrap(op_node, |p| {
                p.expect(KW_IS);
                p.eat(KW_NOT);
                p.expect(KW_DISTINCT);

                // To be more intelligent, `which_infix_op`, did not check if there is a `KW_FROM`
                // token at the end since KW_IS, KW_NOT, and KW_DISTINCT is enough to deterimine
                // the operator. Thus, we use `expect_or_advance` instead of `expect`
                p.expect_or_advance(KW_FROM, r);
            });
        }

        // The rest are just one token wide
        _ => {
            p.wrap(op_node, |p| p.advance());
        }
    }
}

#[rustfmt::skip]
fn precedence_table(op: SqliteTreeKind) -> (Option<u8>, Option<u8>) {
    match op {
        OpBinComplement | OpUnaryMinus | OpUnaryPlus             => (None, Some(23)),
        OpCollate                                                => (Some(21), None),
        OpConcat | OpExtractOne | OpExtractTwo                   => (Some(19), Some(20)),
        OpMultiply | OpDivide | OpModulus                        => (Some(17), Some(18)),
        OpAdd | OpSubtract                                       => (Some(15), Some(16)),
        OpBinAnd | OpBinOr | OpBinLShift | OpBinRShift           => (Some(13), Some(12)),
        OpEscape                                                 => (Some(11), None),
        OpLT | OpGT | OpLTE | OpGTE                              => (Some(9) , Some(10)),

        OpEq | OpNotEq | OpIs | OpIsNot | OpIsDistinctFrom
          | OpIsNotDistinctFrom | OpBetweenAnd | OpNotBetweenAnd
          | OpIn | OpNotIn | OpMatch | OpNotMatch | OpLike
          | OpNotLike| OpRegexp | OpNotRegexp | OpGlob
          | OpNotGlob | OpIsNull | OpNotNull | OpNotSpaceNull    => (Some(7), Some(8)),

        OpNot                                                    => (None, Some(5)),
        OpAnd                                                    => (Some(3), Some(4)),
        OpOr                                                     => (Some(1), Some(2)),
        _ => unreachable!("DEV ERROR: Did not cover all operator nodes")
    }
}



/// Note: rust-analyzer tip: You can do `Ctrl/Cmd + Hover` to see the function definition (instead of `Ctrl/Cmd + Click`)
mod utils {
    use crate::{SqliteToken, SqliteTreeKind};

    use super::*;

    // TODO: Add inlines

    pub(super) fn at_expr(p: &SqliteParser) -> bool {
        at_expr_bind_param(p)
            || at_expr_bracketed(p)
            || at_expr_case(p)
            || at_expr_cast(p)
            || at_expr_lit(p)
            || at_expr_column_name(p)
            || at_expr_select(p)
            || at_expr_function(p)
    }

    pub(super) fn at_expr_lit(p: &SqliteParser) -> bool {
        p.at_any(INT_LIT | HEX_LIT | STR_LIT | REAL_LIT)
    }

    /// Bracketed means enclosed in round brackets
    pub(super) fn at_expr_bracketed(p: &SqliteParser) -> bool {
        // It is not enough just check we are at L_PAREN because expr_select is also like that
        p.at(T!['(']) && !at_expr_select(p)
    }

    pub(super) fn at_expr_column_name(p: &SqliteParser) -> bool {
        p.at(IDEN) && !at_expr_function(p)
    }

    pub(super) fn at_expr_function(p: &SqliteParser) -> bool {
        matches!(p.tokens(), [IDEN, T!['('], ..])
    }

    #[inline]
    pub(super) fn at_expr_cast(p: &SqliteParser) -> bool {
        p.at(KW_CAST)
    }

    pub(super) fn at_expr_bind_param(p: &SqliteParser) -> bool {
        p.at_any(T![?] | T![:] | T![@])
    }

    pub(super) fn at_expr_select(p: &SqliteParser) -> bool {
        p.at_any(KW_NOT | KW_EXISTS) // Yep, no other expr type can begin with `NOT` or `EXISTS`
            || matches!(p.tokens(), [T!['('], KW_SELECT, ..] )
    }

    #[inline]
    pub(super) fn at_expr_case(p: &SqliteParser) -> bool {
        p.at(KW_CASE)
    }

    pub(super) fn at_table_or_subquery(p: &SqliteParser) -> bool {
        p.at_any(IDEN | T!['('])
    }

    pub(super) fn at_result_column(p: &SqliteParser) -> bool {
        p.at_any(IDEN | T![*]) || at_expr(p)
    }

    pub(super) fn at_join_operator(p: &SqliteParser) -> bool {
        p.at_any(T![,] | KW_NATURAL | KW_JOIN | KW_LEFT | KW_RIGHT | KW_FULL | KW_INNER | KW_CROSS)
    }

    pub(super) fn at_prefix_op(p: &SqliteParser) -> bool {
        p.at_any(T![~] | T![+] | T![-] | KW_NOT)
    }

    pub(super) fn which_infix_op(p: &SqliteParser) -> Option<SqliteTreeKind> {
        let op = match p.tokens() {
            [T![||], ..] => OpConcat,
            [T![->], ..] => OpExtractOne,
            [T![->>], ..] => OpExtractTwo,
            [T![*], ..] => OpMultiply,
            [T![/], ..] => OpDivide,
            [T![%], ..] => OpModulus,
            [T![+], ..] => OpAdd,
            [T![-], ..] => OpSubtract,
            [T![&], ..] => OpBinAnd,
            [T![|], ..] => OpBinOr,
            [T![<<], ..] => OpBinLShift,
            [T![>>], ..] => OpBinRShift,
            [T![<], ..] => OpLT,
            [T![>], ..] => OpGT,
            [T![<=], ..] => OpLTE,
            [T![>=], ..] => OpGTE,
            [T![=], ..] | [T![==], ..] => OpEq,
            [T![!=], ..] | [T![<>], ..] => OpNotEq,

            // Because of the [expr] BETWEEN [expr]  AND [expr], we currently have the following
            // hack to check if we are actually parsing OpBetweenAnd operator and if so
            // we hide the existence of the AND operator even if we match it
            [KW_AND, ..] => {
                // // [EVENT_ADVANCE(BETWEEN), EVENT_OPEN(EXPR), ... EVENT_CLOSE(EXPR), AND]
                // // We are trying to match an event stream similar to above.
                // assert!(!p.events.is_empty());
                // let mut event_diff = 0;
                // let mut tk_count = 0; // Tokens in the event that we are trying to skip
                // for e in p.events.iter().rev() {
                //     match e {
                //         Event::Open { .. } => event_diff += 1,
                //         Event::Close => event_diff -= 1,
                //         Event::Advance => tk_count += 1,
                //     }

                //     // when event_diff is zero, Open and close events match
                //     if event_diff == 0 {
                //         break;
                //     }
                // }

                // if let Some(SqliteToken {
                //     kind: KW_BETWEEN, ..
                // }) = p.all_tokens.get(p.all_tokens_pos - tk_count - 1)
                // {
                //     return None;
                // }

                OpAnd
            }
            [KW_OR, ..] => OpOr,
            [KW_IN, ..] => OpIn,
            [KW_MATCH, ..] => OpMatch,
            [KW_LIKE, ..] => OpLike,
            [KW_REGEXP, ..] => OpRegexp,
            [KW_GLOB, ..] => OpGlob,
            // We don't match the full operator but this is enough to narrow down to one operator
            [KW_BETWEEN, ..] => OpBetweenAnd,
            [KW_NOT, KW_IN, ..] => OpNotIn,
            [KW_NOT, KW_MATCH, ..] => OpNotMatch,
            [KW_NOT, KW_LIKE, ..] => OpNotLike,
            [KW_NOT, KW_REGEXP, ..] => OpNotRegexp,
            [KW_NOT, KW_GLOB, ..] => OpNotGlob,
            // For the following three three cases, We don't match the full operator but it
            // is enough to narrow down to one operator
            [KW_NOT, KW_BETWEEN, ..] => OpNotBetweenAnd,
            [KW_IS, KW_NOT, KW_DISTINCT, ..] => OpIsNotDistinctFrom,
            [KW_IS, KW_DISTINCT, ..] => OpIsDistinctFrom,

            // Its very important these operators are matched last or we will never match tokens
            // like `IS DISTINCT FROM`
            [KW_IS, KW_NOT, ..] => OpIsNot,
            [KW_IS, ..] => OpIs,
            [KW_NOT, ..] => OpNot,
            _ => return None,
        };

        Some(op)
    }

    pub(super) fn at_join_constraint(p: &SqliteParser) -> bool {
        p.at_any(KW_ON | KW_USING)
    }

    pub(super) fn which_postfix_op(p: &SqliteParser) -> Option<SqliteTreeKind> {
        // Technically, ESCAPE is also a postfix op but it can only appear in a particular
        // type of expr so we handle it separately
        let op = match p.nth(0) {
            KW_COLLATE => OpCollate,
            KW_ISNULL => OpIsNull,
            KW_NOTNULL => OpNotNull,
            KW_NOT if p.nth(1) == KW_NULL => OpNotSpaceNull,
            _ => return None,
        };

        Some(op)
    }

    pub(super) fn expect_expr(p: &mut SqliteParser, r: EnumSet<SqliteTokenKind>) {
        if !at_expr(p) {
            p.try_or_adv("expected an expression", r);
        } else {
            expr(p, r);
        }
    }

    // A helper function that lets us figure out if we are in the middle of parsing the
    // BETWEEN AND operator.
    pub(super) fn is_processing_between_and_op(p: &SqliteParser) -> bool {
        assert!(p.at(KW_AND));

        // [EVENT_ADVANCE(BETWEEN), EVENT_OPEN(EXPR), ... EVENT_CLOSE(EXPR), AND]
        // We are trying to match an event stream similar to above.
        assert!(!p.events.is_empty());
        let mut event_diff = 0;
        let mut tk_count = 0; // Tokens in the event that we are trying to skip
        for e in p.events.iter().rev() {
            match e {
                Event::Open { .. } => event_diff += 1,
                Event::Close => event_diff -= 1,
                Event::Advance => tk_count += 1,
            }

            // when event_diff is zero, Open and close events match
            if event_diff == 0 {
                break;
            }
        }

        if let Some(SqliteToken {
            kind: KW_BETWEEN, ..
        }) = p.go_back_all_tokens_by(tk_count)
        {
            return true;
        } else {
            return false;
        }
    }
}

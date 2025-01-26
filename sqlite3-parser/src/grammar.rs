pub(crate) mod common;

use crate::parser::{Event, ExpectedItem, MarkClosed, SqliteParser};
use crate::SqliteTokenKind::*;
use crate::SqliteTreeKind::*;
use crate::T;
use crate::{SqliteTokenKind, SqliteTreeKind};
use common::*;
use enumset::{enum_set, EnumSet};

type TokenSet = enumset::EnumSet<SqliteTokenKind>;
use utils::*;

macro_rules! expected_one_of {
    ($parser:expr, $recover:expr, $($ident:ident $(|)? )*) => {
        $parser.expected_one_of([$(ExpectedItem::from($ident),)*].into_iter(), $recover)
    };
}

macro_rules! must_eat_one_of {
    ($parser:expr, $recover:expr, $ident:ident | $($ident2:ident $(|)? )*) => {
        if !$parser.eat_any($($ident2 |)* $ident) {
            expected_one_of!($parser, $recover, $($ident2 |)* $ident);
        }
    };
}

macro_rules! bail_if_not_at {
    ($parser:expr, $r:expr, $token_set:expr, $expected:expr) => {
        if !$parser.at_any_now_or_later(($token_set | enum_set!()), $r) {
            $parser.expected($expected, $r);
            return;
        }
    };
}

macro_rules! expected_items {
    ($($ident:ident $(|)? )*) => {
        [$(ExpectedItem::from($ident),)*]
    };
}

pub fn file(p: &mut SqliteParser, _r: TokenSet) {
    let m = p.open();
    let r = STATEMENT_START | T![;];

    while !p.eof() {
        if p.at_any(STATEMENT_START) {
            statement(p, r);
            p.must_eat(T![;], r);
        } else {
            p.expected(Statement, STATEMENT_START);
        }
    }
    p.eat_whitespace();

    p.close(m, File);
}

pub fn explain_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_EXPLAIN);

    if p.eat(KW_QUERY) {
        p.must_eat(KW_PLAN, r);
    }

    p.close(m, ExplainClause);
}

pub fn statement(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, STATEMENT_START, Statement);

    let m = p.open();

    if p.at(KW_EXPLAIN) {
        explain_clause(p, r);
    }

    if p.at_any(STATEMENT_NO_CTE_START) {
        statement_no_cte(p, r);
    } else if p.at_any(STATEMENT_WITH_CTE_START) {
        statement_with_cte(p, r);
    } else {
        expected_one_of!(p, r, StatementNoCte | StatementWithCte);
    }

    p.close(m, Statement);
}

pub fn statement_no_cte(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, STATEMENT_NO_CTE_START, StatementNoCte);

    let m = p.open();

    if p.nth(0) == KW_CREATE {}

    let _ = match (p.nth(0), p.nth(1), p.nth(2)) {
        (KW_CREATE, KW_TEMPORARY | KW_TEMP, KW_TABLE) => create_table_stmt(p, r),
        (KW_CREATE, KW_TABLE, _) => create_table_stmt(p, r),

        (KW_CREATE, KW_TEMPORARY | KW_TEMP, KW_TRIGGER) => create_trigger_stmt(p, r),
        (KW_CREATE, KW_TRIGGER, _) => create_trigger_stmt(p, r),

        (KW_CREATE, KW_TEMPORARY | KW_TEMP, KW_VIEW) => create_view_stmt(p, r),
        (KW_CREATE, KW_VIEW, _) => create_view_stmt(p, r),
        (KW_CREATE, KW_TEMPORARY | KW_TEMP, _) => {
            let items = expected_items!(CreateTableStmt | CreateTriggerStmt | CreateViewStmt);
            p.wrap_err(items.as_slice(), r, |p| {
                p.guaranteed(KW_CREATE);
                p.guaranteed_any(KW_TEMP | KW_TEMPORARY);
            });
        }

        (KW_CREATE, KW_UNIQUE, KW_INDEX) => create_index_stmt(p, r),
        (KW_CREATE, KW_INDEX, _) => create_index_stmt(p, r),
        (KW_CREATE, KW_VIRTUAL, _) => create_virtual_table_stmt(p, r),
        (KW_CREATE, _, _) => {
            let items = expected_items!(
                CreateTableStmt
                    | CreateTriggerStmt
                    | CreateIndexStmt
                    | CreateViewStmt
                    | CreateVirtualTableStmt
            );
            p.wrap_err(items.as_slice(), r, |p| {
                p.guaranteed(KW_CREATE);
            });
        }
        (KW_DROP, KW_TABLE, _) => drop_table_stmt(p, r),
        (KW_DROP, KW_VIEW, _) => drop_view_stmt(p, r),
        (KW_DROP, KW_INDEX, _) => drop_index_stmt(p, r),
        (KW_DROP, KW_TRIGGER, _) => drop_trigger_stmt(p, r),
        (KW_DROP, _, _) => {
            let items =
                expected_items!(DropTableStmt | DropViewStmt | DropTriggerStmt | DropIndexStmt);
            p.wrap_err(items.as_slice(), r, |p| {
                p.guaranteed(KW_DROP);
            });
        }
        (KW_ALTER, _, _) => alter_table_stmt(p, r),
        (KW_ANALYZE, _, _) => analyze_stmt(p, r),
        (KW_ATTACH, _, _) => attach_db_stmt(p, r),
        (KW_BEGIN, _, _) => begin_stmt(p, r),
        (KW_COMMIT | KW_END, _, _) => commit_stmt(p, r),
        (KW_DETACH, _, _) => detach_stmt(p, r),
        (KW_PRAGMA, _, _) => pragma_stmt(p, r),
        (KW_REINDEX, _, _) => re_index_stmt(p, r),
        (KW_RELEASE, _, _) => release_stmt(p, r),
        (KW_ROLLBACK, _, _) => rollback_stmt(p, r),
        (KW_SAVEPOINT, _, _) => savepoint_stmt(p, r),
        (KW_VACUUM, _, _) => vacuum_stmt(p, r),
        _ => {
            expected_one_of!(
                p,
                r,
                CreateTableStmt
                    | AlterTableStmt
                    | AnalyzeStmt
                    | AttachDbStmt
                    | BeginStmt
                    | CommitStmt
                    | CreateIndexStmt
                    | CreateTriggerStmt
                    | CreateViewStmt
                    | CreateVirtualTableStmt
                    | DetachStmt
                    | DropIndexStmt
                    | DropViewStmt
                    | DropTableStmt
                    | DropTriggerStmt
                    | PragmaStmt
                    | ReIndexStmt
                    | ReleaseStmt
                    | RollbackStmt
                    | SavepointStmt
                    | VacuumStmt
            );
        }
    };

    p.close(m, StatementNoCte);
}

pub fn statement_with_cte(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, STATEMENT_WITH_CTE_START, StatementWithCte);

    let m = p.open();

    if p.at(KW_WITH) {
        cte_clause(p, r);
    }

    if p.at_any(SELECT_STMT_START) {
        select_stmt(p, r);
    } else if p.at_any(INSERT_STMT_START) {
        insert_stmt(p, r);
    } else if p.at(KW_UPDATE) {
        update_stmt(p, r);
    } else if p.at(KW_DELETE) {
        delete_stmt(p, r);
    } else {
        expected_one_of!(p, r, SelectStmt | InsertStmt | UpdateStmt | DeleteStmt);
    }

    p.close(m, StatementWithCte);
}

pub fn vacuum_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_VACUUM);

    if p.at_any(p.name_token) {
        must_eat_name(p, r, SchemaName);
    }

    if p.eat(KW_INTO) {
        if p.at_any(p.expr_start) {
            expr(p, r);
        } else {
            p.expected(Expr, r);
        }
    }

    p.close(m, VacuumStmt);
}

pub fn savepoint_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_SAVEPOINT);
    must_eat_name(p, r, SavepointName);

    p.close(m, SavepointStmt);
}

pub fn rollback_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_ROLLBACK);
    p.eat(KW_TRANSACTION);

    if p.eat(KW_TO) {
        p.eat(KW_SAVEPOINT);
        must_eat_name(p, r, SavepointName);
    }

    p.close(m, RollbackStmt);
}

pub fn release_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_RELEASE);
    p.eat(KW_SAVEPOINT);
    must_eat_name(p, r, SavepointName);

    p.close(m, ReleaseStmt);
}

pub fn re_index_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_REINDEX);

    if p.nth(1) == T![.] {
        p.wrap(TableOrIdxNameWithSchema, |p| {
            must_eat_name(p, r, SchemaName);
            p.must_eat(T![.], r);
            must_eat_name(p, r, TableOrIndexName);
        });
    } else if p.at_any(p.name_token) {
        p.wrap(TableOrIdxOrCollationName, |p| p.advance());
    }

    p.close(m, ReIndexStmt);
}

pub fn pragma_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_PRAGMA);

    if p.at_any(p.name_token) {
        full_pragma_name(p, r);
    } else {
        p.expected(PragmaName, r);
    }

    if p.eat(T![=]) {
        if p.at_any(T![+] | T![-] | NUMERIC_LIT | p.name_token) {
            pragma_value(p, r);
        } else {
            p.expected(PragmaValue, r);
        }
    } else if p.at(T!['(']) {
        if p.at_any(T![+] | T![-] | NUMERIC_LIT | p.name_token) {
            pragma_value(p, r);
        } else {
            p.expected(PragmaValue, r);
        }
        p.must_eat(T![')'], r);
    }

    p.close(m, PragmaStmt);
}

pub fn pragma_value(p: &mut SqliteParser, r: TokenSet) {
    let pragma_value_start = T![+] | T![-] | NUMERIC_LIT | p.name_token | STR_LIT;
    bail_if_not_at!(p, r, pragma_value_start, PragmaValue);

    let m = p.open();

    if p.at_any(T![+] | T![-] | NUMERIC_LIT) {
        signed_number(p, r);
    } else if p.at_any(p.name_token) {
        must_eat_name(p, r, PragmaValueName);
    } else if p.at(STR_LIT) {
        p.must_eat(STR_LIT, r);
    } else {
        expected_one_of!(p, r, SignedNumber | PragmaValueName | STR_LIT);
    }

    p.close(m, PragmaValue);
}

pub fn full_pragma_name(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, FullPragmaName);

    let m = p.open();

    if p.nth(1) == T![.] {
        must_eat_name(p, r, SchemaName);
        p.must_eat(T![.], r);
    }
    must_eat_name(p, r, PragmaName);

    p.close(m, FullPragmaName);
}

pub fn drop_trigger_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_DROP);
    p.guaranteed(KW_TRIGGER);

    if p.eat(KW_IF) {
        p.must_eat(KW_EXISTS, r);
    }

    if p.at_any(p.name_token) {
        full_trigger_name(p, r);
    } else {
        p.expected(FullTriggerName, r);
    }

    p.close(m, DropTriggerStmt);
}

pub fn drop_table_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_DROP);
    p.guaranteed(KW_TABLE);

    if p.eat(KW_IF) {
        p.must_eat(KW_EXISTS, r);
    }

    if p.at_any(p.name_token) {
        full_table_name(p, r);
    } else {
        p.expected(FullTableName, r);
    }

    p.close(m, DropTableStmt);
}

pub fn drop_view_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_DROP);
    p.guaranteed(KW_VIEW);

    if p.eat(KW_IF) {
        p.must_eat(KW_EXISTS, r);
    }

    if p.at_any(p.name_token) {
        full_view_name(p, r);
    } else {
        p.expected(FullViewName, r);
    }

    p.close(m, DropViewStmt);
}

pub fn drop_index_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_DROP);
    p.guaranteed(KW_INDEX);

    if p.eat(KW_IF) {
        p.must_eat(KW_EXISTS, r);
    }

    if p.at_any(p.name_token) {
        full_index_name(p, r);
    } else {
        p.expected(FullIndexName, r);
    }

    p.close(m, DropIndexStmt);
}

pub fn detach_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_DETACH);
    p.eat(KW_DATABASE);

    expr(p, r);

    p.close(m, DetachStmt);
}

/// TODO: Support ModuleArg properly, it is much more expressive than just these
pub fn module_arg_list(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(T!['(']);

    if p.at_any(MODULE_ARG_START) {
        p.wrap(ModuleArg, |p| p.advance());
    } else {
        p.expected(ModuleArg, r);
    }
    while p.at(T![,]) {
        p.guaranteed(T![,]);

        if p.at_any(MODULE_ARG_START) {
            p.wrap(ModuleArg, |p| p.advance());
        } else {
            p.expected(ModuleArg, r);
        }
    }
    p.must_eat(T![')'], r);

    p.close(m, ModuleArgList);
}

pub fn create_virtual_table_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_CREATE);
    p.guaranteed(KW_VIRTUAL);
    p.guaranteed(KW_TABLE);

    if p.at(KW_IF) {
        if_not_exists(p, r);
    }

    if p.at_any(p.name_token) {
        full_table_name(p, r);
    } else {
        p.expected(FullTableName, r);
    }
    p.must_eat(KW_USING, r);
    must_eat_name(p, r, ModuleName);

    if p.at(T!['(']) {
        module_arg_list(p, r);
    }

    p.close(m, CreateVirtualTableStmt);
}

pub fn full_view_name(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, FullViewName);

    let m = p.open();

    if p.nth(1) == T![.] {
        must_eat_name(p, r, SchemaName);
        p.must_eat(T![.], r);
    }
    must_eat_name(p, r, ViewName);

    p.close(m, FullViewName);
}

pub fn create_view_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_CREATE);
    p.eat_any(KW_TEMP | KW_TEMPORARY);
    p.guaranteed(KW_VIEW);

    if p.at(KW_IF) {
        if_not_exists(p, r);
    }

    if p.at_any(p.name_token) {
        full_view_name(p, r);
    } else {
        p.expected(FullViewName, r);
    }

    if p.at(T!['(']) {
        col_name_list(p, r);
    }
    p.must_eat(KW_AS, r);

    if p.at_any(KW_SELECT | KW_VALUES | KW_WITH) {
        select_stmt_with_cte(p, r);
    } else {
        p.expected(SelectStmtWithCte, r);
    }

    p.close(m, CreateViewStmt);
}

pub fn delete_stmt_limited(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, DELETE_STMT_LIMITED_START, DeleteStmtLimited);

    let m = p.open();

    if p.at(KW_ORDER) {
        order_by_clause(p, r);
    }
    limit_clause(p, r);

    p.close(m, DeleteStmtLimited);
}

pub fn delete_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_DELETE);
    p.must_eat(KW_FROM, r);

    if p.at_any(p.name_token) {
        qualified_table_name(p, r);
    } else {
        p.expected(QualifiedTableName, r);
    }

    if p.at(KW_WHERE) {
        where_clause(p, r);
    }

    if p.at(KW_RETURNING) {
        returning_clause(p, r);
    }

    if p.at_any(KW_ORDER | KW_LIMIT) {
        delete_stmt_limited(p, r);
    }

    p.close(m, DeleteStmt);
}

pub fn insert_stmt(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, INSERT_STMT_START, InsertStmt);

    let m = p.open();

    insert_stmt_kind(p, r);
    p.must_eat(KW_INTO, r);

    full_table_name(p, r);

    // NOTE: Usually `AS` is not required before alias name but in this case it is
    if p.at(KW_AS) {
        with_alias(p, r);
    }

    if p.at(T!['(']) {
        col_name_list(p, r);
    }
    insert_value_kind(p, r);

    if p.at(KW_RETURNING) {
        returning_clause(p, r);
    }

    p.close(m, InsertStmt);
}

pub fn insert_stmt_kind(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, INSERT_STMT_KIND_START, InsertStmtKind);

    let m = p.open();

    if p.at(KW_INSERT) {
        p.wrap(InsertOrAction, |p| {
            p.guaranteed(KW_INSERT);

            if p.eat(KW_OR) {
                conflict_action(p, r);
            }
        });
    } else {
        p.guaranteed(KW_REPLACE);
    }

    p.close(m, InsertStmtKind);
}

pub fn insert_default_values_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_DEFAULT);
    p.must_eat(KW_VALUES, r);

    p.close(m, InsertDefaultValuesClause);
}

pub fn insert_value_kind(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, INSERT_VALUE_KIND_START, InsertValueKind);

    let m = p.open();

    if p.at(KW_VALUES) {
        insert_values_clause(p, r);
    } else if p.at_any(KW_WITH | KW_SELECT | KW_VALUES) {
        insert_select_clause(p, r);
    } else if p.at(KW_DEFAULT) {
        insert_default_values_clause(p, r);
    } else {
        expected_one_of!(
            p,
            r,
            InsertValuesClause | InsertSelectClause | InsertDefaultValuesClause
        );
    }

    p.close(m, InsertValueKind);
}

pub fn insert_select_clause(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, INSERT_SELECT_CLAUSE_START, InsertSelectClause);

    let m = p.open();

    select_stmt_with_cte(p, r);

    if p.at(KW_ON) {
        upsert_clause(p, r);
    }

    p.close(m, InsertSelectClause);
}

pub fn upsert_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_ON);
    p.must_eat(KW_CONFLICT, r);

    if p.at(T!['(']) {
        upsert_clause_conflict_target(p, r);
    }
    p.must_eat(KW_DO, r);

    if p.at(KW_UPDATE) {
        upsert_do_update(p, r);
    } else if p.at(KW_NOTHING) {
        p.guaranteed(KW_NOTHING);
    } else {
        expected_one_of!(p, r, KW_NOTHING | UpsertDoUpdate);
    }

    p.close(m, UpsertClause);
}

pub fn upsert_do_update(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_UPDATE);
    p.must_eat(KW_SET, r);
    set_column_expr(p, r);
    while p.at(T![,]) {
        p.must_eat(T![,], r);
        set_column_expr(p, r);
    }

    if p.at(KW_WHERE) {
        where_clause(p, r);
    }

    p.close(m, UpsertDoUpdate);
}

pub fn upsert_clause_conflict_target(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    indexed_col_list(p, r);

    if p.at(KW_WHERE) {
        where_clause(p, r);
    }

    p.close(m, UpsertClauseConflictTarget);
}

pub fn insert_values_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_VALUES);

    expr_list(p, r);
    while p.at(T![,]) {
        p.must_eat(T![,], r);
        expr_list(p, r);
    }

    if p.at(KW_ON) {
        upsert_clause(p, r);
    }

    p.close(m, InsertValuesClause);
}

pub fn update_stmt_limited(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, UPDATE_STMT_LIMITED_START, UpdateStmtLimited);

    let m = p.open();

    if p.at(KW_ORDER) {
        order_by_clause(p, r);
    }

    if p.at(KW_LIMIT) {
        limit_clause(p, r);
    }

    p.close(m, UpdateStmtLimited);
}

pub fn returning_clause_expr(p: &mut SqliteParser, r: TokenSet) {
    assert!(p.at_any(p.expr_start)); // TODO: Add these asserts when its guaranteed for other functions

    let m = p.open();

    expr(p, r);

    if p.eat(KW_AS) {
        must_eat_name(p, r, AliasName);
    } else {
        must_eat_name(p, r, AliasName);
    }

    p.close(m, ReturningClauseExpr);
}

pub fn returning_clause_kind(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.expr_start | T![*], ReturningClauseKind);

    let m = p.open();

    if p.at(T![*]) {
        p.must_eat(T![*], r);
    } else if p.at_any(p.expr_start) {
        returning_clause_expr(p, r);
    } else {
        unreachable!("DEV ERROR: returning_clause_kind start check is wrong")
    }

    p.close(m, ReturningClauseKind);
}

pub fn returning_clause(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, RETURNING_CLAUSE_START, ReturningClause);

    let m = p.open();

    p.must_eat(KW_RETURNING, r);
    returning_clause_kind(p, r);
    while p.eat(T![,]) {
        returning_clause_kind(p, r);
    }

    p.close(m, ReturningClause);
}

pub fn set_column_expr(p: &mut SqliteParser, r: TokenSet) {
    if !p.at_any(p.name_token | T!['(']) {
        p.expected(SetColumnExpr, r);
    }

    let m = p.open();

    if p.at_any(p.name_token) {
        must_eat_name(p, r, ColumnName);
    } else if p.at(T!['(']) {
        col_name_list(p, r);
    } else {
        unreachable!("DEV ERROR: set_column_expr start check is wrong");
    }

    p.must_eat(T![=], r);
    expr(p, r);

    p.close(m, SetColumnExpr);
}

pub fn update_stmt(p: &mut SqliteParser, r: TokenSet) {
    if !p.at(KW_UPDATE) {
        p.expected(SetColumnExpr, r);
    }

    let m = p.open();

    p.must_eat(KW_UPDATE, r);

    if p.eat(KW_OR) {
        conflict_action(p, r);
    }
    qualified_table_name(p, r);
    p.must_eat(KW_SET, r);
    set_column_expr(p, r);
    while p.eat(T![,]) {
        set_column_expr(p, r);
    }

    if p.at(KW_FROM) {
        from_clause(p, r);
    }

    if p.at(KW_WHERE) {
        where_clause(p, r);
    }

    if p.at(KW_RETURNING) {
        returning_clause(p, r);
    }

    if p.at(KW_LIMIT) {
        update_stmt_limited(p, r);
    }

    p.close(m, UpdateStmt);
}

pub fn trigger_body_stmt(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, TRIGGER_BODY_STMT_START, TriggerBodyStmt);

    let m = p.open();

    if p.at(KW_UPDATE) {
        update_stmt(p, r);
    } else if p.at(KW_INSERT) {
        insert_stmt(p, r);
    } else if p.at(KW_DELETE) {
        delete_stmt(p, r);
    } else if p.at_any(SELECT_STMT_START) {
        select_stmt(p, r);
    } else {
        unreachable!("DEV ERROR: TRIGGER_BODY_STMT_START is wrong");
    }

    p.close(m, TriggerBodyStmt);
}

pub fn trigger_body_stmt_list(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, TRIGGER_BODY_STMT_START, TriggerBodyStmtList);

    let m = p.open();

    trigger_body_stmt(p, r);
    while p.at(T![;]) && TRIGGER_BODY_STMT_START.contains(p.nth(1)) {
        p.guaranteed(T![;]);
        trigger_body_stmt(p, r);
    }
    p.must_eat(T![;], r);

    p.close(m, TriggerBodyStmtList);
}

pub fn trigger_when_expr(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_WHEN);
    expr(p, r);

    p.close(m, TriggerWhenExpr);
}

pub fn trigger_for_each_row(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_FOR);
    p.must_eat(KW_EACH, r);
    p.must_eat(KW_ROW, r);

    p.close(m, TriggerForEachRow);
}

pub fn trigger_update_affect_cols(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_OF);
    must_eat_name(p, r, ColumnName);
    while p.eat(T![,]) {
        must_eat_name(p, r, ColumnName);
    }

    p.close(m, TriggerUpdateAffectCols);
}

pub fn trigger_update_action(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_UPDATE);

    if p.at(KW_OF) {
        trigger_update_affect_cols(p, r);
    }

    p.close(m, TriggerUpdateAction);
}

pub fn trigger_action_kind(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, TRIGGER_ACTION_KIND_START, TriggerActionKind);

    let m = p.open();

    if p.at(KW_DELETE) {
        p.must_eat(KW_DELETE, r);
    } else if p.at(KW_INSERT) {
        p.must_eat(KW_INSERT, r);
    } else if p.at(KW_UPDATE) {
        trigger_update_action(p, r);
    } else {
        expected_one_of!(p, r, KW_DELETE | KW_INSERT | TriggerUpdateAction);
    }

    p.close(m, TriggerActionKind);
}

pub fn trigger_instead_of(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_INSTEAD);
    p.must_eat(KW_OF, r);

    p.close(m, TriggerInsteadOf);
}

pub fn full_trigger_name(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, FullTriggerName);

    let m = p.open();

    if p.nth(1) == T![.] {
        must_eat_name(p, r, SchemaName);
        p.must_eat(T![.], r);
    }
    must_eat_name(p, r, TriggerName);

    p.close(m, FullTriggerName);
}

pub fn create_trigger_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_CREATE);
    p.eat_any(KW_TEMP | KW_TEMPORARY);
    p.guaranteed(KW_TRIGGER);

    if p.at(KW_IF) {
        if_not_exists(p, r);
    }
    full_trigger_name(p, r);

    if p.at_any(KW_BEFORE | KW_AFTER) {
        p.eat_any(KW_BEFORE | KW_AFTER);
    } else if p.at(KW_INSTEAD) {
        trigger_instead_of(p, r);
    } else {
        expected_one_of!(p, r, KW_BEFORE | KW_AFTER | TriggerInsteadOf);
    }

    trigger_action_kind(p, r);
    p.must_eat(KW_ON, r);
    must_eat_name(p, r, TableName);

    if p.at(KW_FOR) {
        trigger_for_each_row(p, r);
    }

    if p.at(KW_WHEN) {
        trigger_when_expr(p, r);
    }
    p.must_eat(KW_BEGIN, r);
    trigger_body_stmt_list(p, r);
    p.must_eat(KW_END, r);

    p.close(m, CreateTriggerStmt);
}

pub fn full_index_name(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, FullIndexName);

    let m = p.open();

    if p.nth(1) == T![.] {
        must_eat_name(p, r, SchemaName);
        p.must_eat(T![.], r);
    }
    must_eat_name(p, r, IndexName);

    p.close(m, FullIndexName);
}

pub fn create_index_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_CREATE);
    p.eat(KW_UNIQUE);
    p.guaranteed(KW_INDEX);

    if p.at(KW_IF) {
        if_not_exists(p, r);
    }
    full_index_name(p, r);
    p.must_eat(KW_ON, r);
    must_eat_name(p, r, TableName);
    indexed_col_list(p, r);

    if p.at(KW_WHERE) {
        where_clause(p, r);
    }

    p.close(m, CreateIndexStmt);
}

pub fn commit_stmt(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, COMMIT_STMT_START, CommitStmt);

    let m = p.open();

    p.guaranteed_any(COMMIT_STMT_START);
    p.eat(KW_TRANSACTION);

    p.close(m, CommitStmt);
}

pub fn begin_stmt(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_BEGIN, BeginStmt);

    let m = p.open();

    p.guaranteed(KW_BEGIN);
    must_eat_one_of!(p, r, KW_DEFERRED | KW_IMMEDIATE | KW_EXCLUSIVE);
    p.eat(KW_TRANSACTION);

    p.close(m, BeginStmt);
}

pub fn attach_db_stmt(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_ATTACH, AttachDbStmt);

    let m = p.open();

    p.guaranteed(KW_ATTACH);
    p.eat(KW_DATABASE);

    if p.at_any(p.expr_start) {
        p.wrap(FileNameExpr, |p| {
            expr(p, r);
        });
    } else {
        p.expected(FileNameExpr, r);
    }
    p.must_eat(KW_AS, r);

    if p.at_any(p.expr_start) {
        p.wrap(SchemaNameExpr, |p| {
            expr(p, r);
        });
    } else {
        p.expected(SchemaNameExpr, r);
    }

    if p.eat(KW_KEY) {
        if p.at_any(p.expr_start) {
            p.wrap(PasswordExpr, |p| {
                expr(p, r);
            });
        } else {
            p.expected(PasswordExpr, r);
        }
    }

    p.close(m, AttachDbStmt);
}

pub fn analyze_stmt(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_ANALYZE, AnalyzeStmt);

    let m = p.open();

    p.guaranteed(KW_ANALYZE);

    if p.nth(1) == T![.] {
        p.wrap(TableOrIdxNameWithSchema, |p| {
            must_eat_name(p, r, SchemaName);
            p.must_eat(T![.], r);
            must_eat_name(p, r, TableOrIndexName);
        });
    } else if p.at_any(p.name_token) {
        must_eat_name(p, r, SchemaOrIdxOrTableName);
    }

    p.close(m, AnalyzeStmt);
}

pub fn drop_column(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_DROP);
    p.eat(KW_COLUMN);
    must_eat_name(p, r, ColumnName);

    p.close(m, DropColumn);
}

pub fn add_column(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_ADD);
    p.eat(KW_COLUMN);
    column_def(p, r);

    p.close(m, AddColumn);
}

pub fn rename_column(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.must_eat(KW_RENAME, r);
    p.eat(KW_COLUMN);
    must_eat_name(p, r, ColumnName);
    p.must_eat(KW_TO, r);
    must_eat_name(p, r, NewColumnName);

    p.close(m, RenameColumn);
}

pub fn rename_table(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.must_eat(KW_RENAME, r);
    p.must_eat(KW_TO, r);
    must_eat_name(p, r, TableName);

    p.close(m, RenameTable);
}

pub fn alter_table_stmt(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_ALTER);
    p.must_eat(KW_TABLE, r);
    full_table_name(p, r);

    if p.at(KW_RENAME) {
        if p.nth(1) == KW_TO {
            rename_table(p, r);
        } else if p.nth(1) == KW_COLUMN {
            rename_column(p, r);
        } else {
            let items = expected_items!(RenameTable | RenameColumn);
            p.wrap_err(items.as_slice(), r, |p| p.guaranteed(KW_RENAME));
        }
    } else if p.at(KW_ADD) {
        add_column(p, r);
    } else if p.at(KW_DROP) {
        drop_column(p, r);
    } else {
        expected_one_of!(p, r, RenameTable | RenameColumn | AddColumn | DropColumn);
    }

    p.close(m, AlterTableStmt);
}

pub fn create_table_select(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_AS);
    select_stmt_with_cte(p, r);

    p.close(m, CreateTableSelect);
}

pub fn table_opt_without_row_id(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_WITHOUT);

    p.must_eat(KW_ROWID, r);

    p.close(m, TableOptWithoutRowId);
}

pub fn table_options(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, TABLE_OPTIONS_START, TableOptions);

    let m = p.open();

    if p.at(KW_WITHOUT) {
        table_opt_without_row_id(p, r);
    } else if p.at(KW_STRICT) {
        p.advance();
    } else {
        expected_one_of!(p, r, TableOptWithoutRowId | KW_STRICT);
    }

    p.close(m, TableOptions);
}

pub fn table_options_list(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    table_options(p, r);
    while p.eat(T![,]) {
        table_options(p, r);
    }

    p.close(m, TableOptionsList);
}

pub fn fk_clause(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_REFERENCES, FkClause);

    let m = p.open();

    p.guaranteed(KW_REFERENCES);
    must_eat_name(p, r, TableName);

    if p.at(T!['(']) {
        col_name_list(p, r);
    }

    while p.at_any(KW_ON | KW_MATCH) {
        fk_action(p, r);
    }

    if p.at(KW_DEFERRABLE) {
        fk_deferrable(p, r);
    }

    p.close(m, FkClause);
}

pub fn fk_on_action(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_ON);
    must_eat_one_of!(p, r, KW_DELETE | KW_UPDATE);

    if p.at(KW_SET) {
        match p.nth(1) {
            KW_NULL => fk_set_null(p, r),
            KW_DEFAULT => fk_set_default(p, r),
            _ => {
                let items = expected_items!(FkSetNull | FkSetDefault);
                p.wrap_err(items.as_slice(), r, |p| p.advance());
            }
        }
    } else if p.at(KW_CASCADE) {
        p.wrap(FkCascade, |p| p.guaranteed(KW_CASCADE));
    } else if p.at(KW_RESTRICT) {
        p.wrap(FkRestrict, |p| p.guaranteed(KW_RESTRICT));
    } else if p.at(KW_NO) {
        fk_no_action(p, r);
    } else {
        expected_one_of!(
            p,
            r,
            FkSetNull | FkSetDefault | FkCascade | FkRestrict | FkNoAction
        );
    }

    p.close(m, FkOnAction);
}

pub fn fk_set_null(p: &mut SqliteParser, _r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_SET);
    p.guaranteed(KW_NULL);

    p.close(m, FkSetNull);
}

pub fn fk_match_action(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_MATCH);
    must_eat_name(p, r, AnyValidName);

    p.close(m, FkMatchAction);
}

pub fn fk_no_action(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_NO);
    p.must_eat(KW_ACTION, r);

    p.close(m, FkNoAction);
}

pub fn fk_set_default(p: &mut SqliteParser, _r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_SET);
    p.guaranteed(KW_DEFAULT);

    p.close(m, FkSetDefault);
}

pub fn fk_action(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    if p.at(KW_ON) {
        fk_on_action(p, r);
    } else if p.at(KW_MATCH) {
        fk_match_action(p, r);
    } else {
        unreachable!("DEV ERROR: fk_action is called wrong")
    }

    p.close(m, FkViolateAction);
}

pub fn fk_deferrable(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, FK_DEFERRABLE_START, FkDeferrable);

    let m = p.open();

    p.eat(KW_NOT);
    p.must_eat(KW_DEFERRABLE, r);

    if p.eat(KW_INITIALLY) {
        must_eat_one_of!(p, r, KW_DEFERRED | KW_IMMEDIATE);
    }

    p.close(m, FkDeferrable);
}

pub fn table_constraint(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, TABLE_CONSTRAINT_START, TableConstraint);

    let m = p.open();

    if p.eat(KW_CONSTRAINT) {
        must_eat_name(p, r, ConstraintName);
    }

    if p.at(KW_PRIMARY) {
        table_pk_constraint(p, r);
    } else if p.at(KW_UNIQUE) {
        table_uq_constraint(p, r);
    } else if p.at(KW_CHECK) {
        check_constraint(p, r);
    } else if p.at(KW_FOREIGN) {
        table_fk_constraint(p, r);
    } else {
        expected_one_of!(
            p,
            r,
            TablePkConstraint | TableUqConstraint | CheckConstraint | TableFkConstraint
        );
    }

    p.close(m, TableConstraint);
}

pub fn table_fk_constraint(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_FOREIGN);
    p.must_eat(KW_KEY, r);
    col_name_list(p, r);
    fk_clause(p, r);

    p.close(m, TableFkConstraint);
}

pub fn table_uq_constraint(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_UNIQUE);
    indexed_col_list(p, r);

    if p.at(KW_ON) {
        conflict_clause(p, r);
    }

    p.close(m, TableUqConstraint);
}

pub fn table_pk_constraint(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_PRIMARY);
    p.must_eat(KW_KEY, r);
    indexed_col_list(p, r);

    if p.at(KW_ON) {
        conflict_clause(p, r);
    }

    p.close(m, TablePkConstraint);
}

pub fn indexed_col_list(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, T!['('], IndexedColList);

    let m = p.open();

    p.guaranteed(T!['(']);
    indexed_col(p, r);
    while p.eat(T![,]) {
        indexed_col(p, r);
    }
    p.must_eat(T![')'], r);

    p.close(m, IndexedColList);
}

pub fn indexed_col(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token | p.expr_start, IndexedCol);

    let m = p.open();

    if p.at_any(p.name_token) {
        must_eat_name(p, r, ColumnName);
    } else if p.at_any(p.expr_start) {
        expr(p, r);
    } else {
        unreachable!("DEV ERROR: indexed_col start check is wrong");
    }

    if p.at(KW_COLLATE) {
        collation(p, r);
    }

    if p.at_any(KW_ASC | KW_DESC) {
        order(p, r);
    }

    p.close(m, IndexedCol);
}

pub fn column_generated(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    if p.eat(KW_GENERATED) {
        p.must_eat(KW_ALWAYS, r);
    }
    p.must_eat(KW_AS, r);
    p.must_eat(T!['('], r);
    expr(p, r);
    p.must_eat(T![')'], r);

    if p.at_any(KW_VIRTUAL | KW_STORED) {
        p.wrap(ColumnGeneratedKind, |p| {
            p.guaranteed_any(KW_VIRTUAL | KW_STORED)
        });
    }

    p.close(m, ColumnGenerated);
}

pub fn default_constraint_expr(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(T!['(']);
    expr(p, r);
    p.must_eat(T![')'], r);

    p.close(m, DefaultConstraintExpr);
}

pub fn default_constraint(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_DEFAULT);

    // NOTE: SQLite seems to also accept `iden` (see SqliteParser for what this means)
    // but its not clear we should do it so we don't
    if p.at(T!['(']) {
        default_constraint_expr(p, r);
    } else if p.at_any(LITERAL_VALUE) {
        p.wrap(DefaultConstraintLiteral, |p| p.advance());
    } else if p.at_any(SIGNED_NUMBER_START) {
        signed_number(p, r);
    } else {
        expected_one_of!(
            p,
            r,
            DefaultConstraintExpr | DefaultConstraintLiteral | SignedNumber
        );
    }

    p.close(m, DefaultConstraint);
}

pub fn raise_action_fail(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_FAIL);
    p.must_eat(T![,], r);
    raise_func_err_message(p, r);

    p.close(m, RaiseActionFail);
}

pub fn raise_action_abort(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_ABORT);
    p.must_eat(T![,], r);
    raise_func_err_message(p, r);

    p.close(m, RaiseActionAbort);
}

pub fn raise_func_err_message(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, STR_LIT, RaiseFuncErrMessage);

    let m = p.open();

    p.guaranteed(STR_LIT);

    p.close(m, RaiseFuncErrMessage);
}

pub fn raise_action_roll_back(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_ROLLBACK);
    p.must_eat(T![,], r);
    raise_func_err_message(p, r);

    p.close(m, RaiseActionRollBack);
}

pub fn raise_action(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(
        p,
        r,
        KW_IGNORE | KW_ROLLBACK | KW_ABORT | KW_FAIL,
        RaiseAction
    );

    let m = p.open();

    if p.eat(KW_IGNORE) {
    } else if p.at(KW_ROLLBACK) {
        raise_action_roll_back(p, r);
    } else if p.at(KW_ABORT) {
        raise_action_abort(p, r);
    } else if p.at(KW_FAIL) {
        raise_action_fail(p, r);
    } else {
        unreachable!("DEV ERROR: raise_action start check is wrong");
    }

    p.close(m, RaiseAction);
}

pub fn raise_func(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_RAISE);

    p.must_eat(T!['('], r);
    raise_action(p, r);
    p.must_eat(T![')'], r);

    p.close(m, RaiseFunc);
}

pub fn case_else_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_ELSE);
    expr(p, r);

    p.close(m, CaseElseClause);
}

pub fn case_when_clause(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_WHEN, CaseWhenClause);

    let m = p.open();

    p.guaranteed(KW_WHEN);
    expr(p, r);
    p.must_eat(KW_THEN, r);
    expr(p, r);

    p.close(m, CaseWhenClause);
}

pub fn case_when_clause_list(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_WHEN, CaseWhenClauseList);

    let m = p.open();

    case_when_clause(p, r);
    while p.at(KW_WHEN) {
        case_when_clause(p, r);
    }

    p.close(m, CaseWhenClauseList);
}

pub fn case_target_expr(p: &mut SqliteParser, r: TokenSet) {
    assert!(p.at_any(p.expr_start));

    let m = p.open();

    expr(p, r);

    p.close(m, CaseTargetExpr);
}

pub fn expr_case(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_CASE);

    if p.at_any(p.expr_start) {
        case_target_expr(p, r);
    }

    case_when_clause_list(p, r);

    if p.at(KW_ELSE) {
        case_else_clause(p, r);
    }

    p.must_eat(KW_END, r);

    p.close(m, ExprCase);
}

pub fn expr_cast(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_CAST);

    p.must_eat(T!['('], r);
    expr(p, r);
    p.must_eat(KW_AS, r);
    type_name(p, r);
    p.must_eat(T![')'], r);

    p.close(m, ExprCast);
}

pub fn expr_select(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, EXPR_EXISTS_SELECT_START, ExprSelect);

    let m = p.open();

    p.eat(KW_NOT);
    p.eat(KW_EXISTS);

    p.must_eat(T!['('], r);
    select_stmt_with_cte(p, r);
    p.must_eat(T![')'], r);

    p.close(m, ExprSelect);
}

pub fn over_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_OVER);

    if p.at_any(p.name_token) {
        must_eat_name(p, r, WindowName);
    } else if p.at(T!['(']) {
        window_def(p, r);
    } else {
        expected_one_of!(p, r, WindowName | WindowDef);
    }

    p.close(m, OverClause);
}

pub fn filter_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_FILTER);
    p.must_eat(T!['('], r);

    where_clause(p, r);

    p.must_eat(T![')'], r);

    p.close(m, FilterClause);
}

pub fn arg_star(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.must_eat(T![*], r);

    p.close(m, ArgStar);
}

pub fn arg_expr(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_DISTINCT | p.expr_start, ArgExpr);

    let m = p.open();

    p.eat(KW_DISTINCT);

    expr(p, r);
    while p.eat(T![,]) {
        expr(p, r);
    }

    if p.at(KW_ORDER) {
        order_by_clause(p, r);
    }

    p.close(m, ArgExpr);
}

pub fn func_arguments(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    if p.at_any(p.expr_start | KW_DISTINCT) {
        arg_expr(p, r);
    } else if p.at(T![*]) {
        arg_star(p, r);
    } else {
        unreachable!("DEV ERROR: func_arguments start check is wrong")
    }

    p.close(m, FuncArguments);
}

pub fn expr_func(p: &mut SqliteParser, r: TokenSet) {
    assert!(p.at_any(p.iden_or_join));

    let m = p.open();

    p.must_eat_any(p.iden_or_join, FunctionName, r);
    p.must_eat(T!['('], r);

    if p.at_any(p.expr_start | KW_DISTINCT | T![*]) {
        func_arguments(p, r);
    }

    p.must_eat(T![')'], r);

    if p.at(KW_FILTER) {
        filter_clause(p, r);
    }

    if p.at(KW_OVER) {
        over_clause(p, r);
    }

    p.close(m, ExprFunc);
}

pub fn expr_bind_param(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, EXPR_BIND_PARAM_START, ExprBindParam);

    let m = p.open();

    todo!();

    p.close(m, ExprBindParam);
}

pub fn in_table(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, InTable);

    let m = p.open();

    full_table_name(p, r);

    p.close(m, InTable);
}

pub fn in_table_func(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, InTableFunc);

    let m = p.open();

    full_table_function_name(p, r);
    p.must_eat(T!['('], r);

    if p.at_any(p.expr_start) {
        expr(p, r);
    }

    while p.eat(T![,]) {
        expr(p, r);
    }

    p.must_eat(T![')'], r);

    p.close(m, InTableFunc);
}

pub fn offset(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, OFFSET_START, Offset);

    let m = p.open();

    if p.eat(KW_OFFSET) {
        expr(p, r);
    } else if p.eat(T![,]) {
        expr(p, r);
    } else {
        unreachable!("DEV ERROR: OFFSET_START is incorrect")
    }

    p.close(m, Offset);
}

pub fn limit_clause(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_LIMIT, LimitClause);

    let m = p.open();

    p.must_eat(KW_LIMIT, r);
    expr(p, r);

    if p.at_any(OFFSET_START) {
        offset(p, r);
    }

    p.close(m, LimitClause);
}

pub fn compound_operator(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, COMPOUND_OPERATOR_START, CompoundOperator);

    let m = p.open();

    if p.at(KW_UNION) {
        p.wrap(UnionCompoundOperator, |p| {
            p.guaranteed(KW_UNION);
            p.eat(KW_ALL);
        });
    } else if p.eat(KW_INTERSECT) {
    } else if p.eat(KW_EXCEPT) {
    } else {
        unreachable!()
    }

    p.close(m, CompoundOperator);
}

pub fn compound_select(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, COMPOUND_SELECT_START, CompoundSelect);

    let m = p.open();

    compound_operator(p, r);
    select_core(p, r);

    p.close(m, CompoundSelect);
}

pub fn values_select(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_VALUES);
    expr_list(p, r);

    p.close(m, ValuesSelect);
}

pub fn frame_spec(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, FRAME_SPEC_START, FrameSpec);

    let m = p.open();

    must_eat_one_of!(p, r, KW_RANGE | KW_ROWS | KW_GROUPS);

    if p.at(KW_BETWEEN) {
        frame_spec_between_clause(p, r);
    } else if p.at(KW_UNBOUNDED) {
        frame_spec_unbounded_preceding(p, r);
    } else if p.at_any(p.expr_start) {
        frame_spec_preceding(p, r);
    } else if p.at(KW_CURRENT) {
        frame_spec_current_row(p, r);
    } else {
        expected_one_of!(
            p,
            r,
            FrameSpecBetweenClause
                | FrameSpecUnboundedPreceding
                | FrameSpecPreceding
                | FrameSpecCurrentRow
        );
    }

    if p.at(KW_EXCLUDE) {
        frame_spec_exclude_clause(p, r);
    }

    p.close(m, FrameSpec);
}

pub fn frame_spec_between_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_BETWEEN);
    frame_spec_between_left(p, r);

    p.must_eat(KW_AND, r);
    frame_spec_between_right(p, r);

    p.close(m, FrameSpecBetweenClause);
}

pub fn frame_spec_between_left(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(
        p,
        r,
        p.expr_start | KW_CURRENT | KW_UNBOUNDED,
        FrameSpecBetweenLeft
    );

    let m = p.open();

    if p.at(KW_UNBOUNDED) {
        frame_spec_unbounded_preceding(p, r);
    } else if p.at(KW_CURRENT) {
        frame_spec_current_row(p, r);
    } else if p.at_any(p.expr_start) {
        let Some(close_m) = expr(p, r) else {
            unreachable!(
                "DEV ERROR: expr should only return null if parser is not at p.expr_start"
            );
        };
        let m1 = p.open_before(close_m);

        if p.eat(KW_PRECEDING) {
            p.close(m1, FrameSpecPreceding);
        } else if p.eat(KW_FOLLOWING) {
            p.close(m1, FrameSpecFollowing);
        } else {
            expected_one_of!(p, r, KW_PRECEDING | KW_FOLLOWING)
        }
    } else {
        unreachable!("DEV ERROR: frame_spec_between_right start check is wrong");
    }

    p.close(m, FrameSpecBetweenLeft);
}

pub fn frame_spec_between_right(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(
        p,
        r,
        p.expr_start | KW_CURRENT | KW_UNBOUNDED,
        FrameSpecBetweenRight
    );

    let m = p.open();

    if p.at(KW_UNBOUNDED) {
        frame_spec_unbounded_following(p, r);
    } else if p.at(KW_CURRENT) {
        frame_spec_current_row(p, r);
    } else if p.at_any(p.expr_start) {
        let Some(close_m) = expr(p, r) else {
            unreachable!(
                "DEV ERROR: expr should only return null if parser is not at p.expr_start"
            );
        };
        let m1 = p.open_before(close_m);

        if p.eat(KW_PRECEDING) {
            p.close(m1, FrameSpecPreceding);
        } else if p.eat(KW_FOLLOWING) {
            p.close(m1, FrameSpecFollowing);
        } else {
            expected_one_of!(p, r, KW_PRECEDING | KW_FOLLOWING)
        }
    } else {
        unreachable!("DEV ERROR: frame_spec_between_right start check is wrong");
    }

    p.close(m, FrameSpecBetweenRight);
}

pub fn frame_spec_exclude_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_EXCLUDE);

    if p.at(KW_NO) {
        frame_spec_exclude_no_others(p, r);
    } else if p.at(KW_CURRENT) {
        frame_spec_current_row(p, r);
    } else if p.at(KW_GROUP) {
        p.must_eat(KW_GROUP, r);
    } else if p.at(KW_TIES) {
        p.must_eat(KW_TIES, r);
    } else {
        expected_one_of!(
            p,
            r,
            FrameSpecNoOthers | FrameSpecCurrentRow | KW_GROUP | KW_TIES
        );
    }

    p.close(m, FrameSpecExcludeClause);
}

pub fn frame_spec_exclude_no_others(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_NO);
    p.must_eat(KW_OTHERS, r);

    p.close(m, FrameSpecNoOthers);
}

pub fn frame_spec_unbounded_following(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_UNBOUNDED);
    p.must_eat(KW_FOLLOWING, r);

    p.close(m, FrameSpecUnboundedFollowing);
}
pub fn frame_spec_current_row(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_CURRENT);
    p.must_eat(KW_ROW, r);

    p.close(m, FrameSpecCurrentRow);
}

pub fn frame_spec_preceding(p: &mut SqliteParser, r: TokenSet) {
    assert!(p.at_any(p.expr_start));

    let m = p.open();

    expr(p, r);
    p.must_eat(KW_PRECEDING, r);

    p.close(m, FrameSpecPreceding);
}

pub fn frame_spec_unbounded_preceding(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_UNBOUNDED);
    p.must_eat(KW_PRECEDING, r);

    p.close(m, FrameSpecUnboundedPreceding);
}

pub fn order(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, ORDER_START, Order);

    let m = p.open();

    p.guaranteed_any(KW_ASC | KW_DESC);

    p.close(m, Order);
}

pub fn ordering_term_list(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.expr_start, OrderingTermList);

    let m = p.open();

    ordering_term(p, r);
    while p.eat(T![,]) {
        ordering_term(p, r);
    }

    p.close(m, OrderingTermList);
}

pub fn ordering_term(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.expr_start, OrderingTerm);

    let m = p.open();

    expr(p, r);

    if p.at(KW_COLLATE) {
        collation(p, r);
    }

    if p.at_any(ORDER_START) {
        order(p, r);
    }

    if p.eat(KW_NULLS) {
        must_eat_one_of!(p, r, KW_FIRST | KW_LAST);
    }

    p.close(m, OrderingTerm);
}

pub fn order_by_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_ORDER);
    p.must_eat(KW_BY, r);
    ordering_term_list(p, r);

    p.close(m, OrderByClause);
}

pub fn window_def(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, T!['('], WindowDef);

    let m = p.open();

    p.must_eat(T!['('], r);

    if !p.at(KW_PARTITION) && p.at_any(p.name_token) {
        must_eat_name(p, r, WindowBaseName);
    }

    if p.at(KW_PARTITION) {
        window_partition_by_clause(p, r);
    }

    if p.at(KW_ORDER) {
        order_by_clause(p, r);
    }

    if p.at_any(FRAME_SPEC_START) {
        frame_spec(p, r);
    }

    p.must_eat(T![')'], r);

    p.close(m, WindowDef);
}

pub fn window_partition_by_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_PARTITION);
    p.must_eat(KW_BY, r);

    expr(p, r);
    while p.eat(T![,]) {
        expr(p, r);
    }

    p.close(m, WindowPartitionByClause);
}

pub fn window_function(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, WindowFunction);

    let m = p.open();

    must_eat_name(p, r, WindowName);
    p.must_eat(KW_AS, r);
    window_def(p, r);

    p.close(m, WindowFunction);
}

pub fn window_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_WINDOW);

    window_function(p, r);
    while p.eat(T![,]) {
        window_function(p, r);
    }

    p.close(m, WindowClause);
}

pub fn having_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_HAVING);
    expr(p, r);

    p.close(m, HavingClause);
}

pub fn group_by_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_GROUP);
    p.must_eat(KW_BY, r);

    expr(p, r);
    while p.eat(T![,]) {
        expr(p, r);
    }

    p.close(m, GroupByClause);
}

pub fn where_clause(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_WHERE, WhereClause);

    let m = p.open();

    p.must_eat(KW_WHERE, r);
    expr(p, r);

    p.close(m, WhereClause);
}

pub fn join_constraint(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, JOIN_CONSTRAINT_START, JoinConstraint);

    let m = p.open();

    if p.at(KW_ON) {
        p.wrap(OnConstraint, |p| {
            p.guaranteed(KW_ON);
            expr(p, r);
        });
    } else if p.at(KW_USING) {
        p.wrap(UsingConstraint, |p| {
            p.guaranteed(KW_USING);
            col_name_list(p, r);
        });
    } else {
        unreachable!()
    }

    p.close(m, JoinConstraint);
}

pub fn natural_join(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_NATURAL, NaturalJoin);

    let m = p.open();

    p.must_eat(KW_NATURAL, r);

    if p.at(KW_INNER) {
        inner_join(p, r);
    } else if p.at_any(OUTER_JOIN_START) {
        outer_join(p, r);
    } else if p.at(KW_JOIN) {
        p.wrap(Join, |p| p.guaranteed(KW_JOIN));
    } else {
        expected_one_of!(p, r, InnerJoin | OuterJoin | Join);
    }
    p.close(m, NaturalJoin);
}

pub fn inner_join(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_INNER, InnerJoin);

    let m = p.open();

    p.must_eat(KW_INNER, r);
    p.must_eat(KW_JOIN, r);

    p.close(m, InnerJoin);
}

pub fn outer_join(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, OUTER_JOIN_START, OuterJoin);

    let m = p.open();

    must_eat_one_of!(p, r, KW_LEFT | KW_RIGHT | KW_FULL);
    p.eat(KW_OUTER);
    p.must_eat(KW_JOIN, r);

    p.close(m, OuterJoin);
}

pub fn cross_join(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_CROSS, CrossJoin);

    let m = p.open();

    p.must_eat(KW_CROSS, r);
    p.must_eat(KW_JOIN, r);

    p.close(m, CrossJoin);
}

pub fn non_comma_join(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, NON_COMMA_JOIN_START, NonCommaJoin);

    let m = p.open();

    if p.at(KW_CROSS) {
        cross_join(p, r);
    } else if p.at_any(OUTER_JOIN_START) {
        outer_join(p, r);
    } else if p.at(KW_INNER) {
        inner_join(p, r);
    } else if p.at(KW_NATURAL) {
        natural_join(p, r);
    } else if p.at(KW_JOIN) {
        p.wrap(Join, |p| p.guaranteed(KW_JOIN));
    } else {
        unreachable!("DEV ERROR: NON_COMMA_JOIN_START is incorrect")
    }

    p.close(m, NonCommaJoin);
}

pub fn join_operator(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, JOIN_OPERATOR_START, JoinOperator);

    let m = p.open();

    if p.at(T![,]) {
        p.wrap(CommaJoin, |p| p.advance());
    } else {
        non_comma_join(p, r);
    }

    p.close(m, JoinOperator);
}

pub fn join_clause(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token | T!['('], JoinClause);

    let mut lhs_marker = table_or_subquery(p, r);

    loop {
        if !p.at_any(JOIN_OPERATOR_START) {
            break;
        }

        let m = p.open_before(lhs_marker);
        join_operator(p, r);
        table_or_subquery(p, r);

        // TODO: Verify this is valid for all type of joins
        // NOTE: Join constraint is optional
        if p.at_any(JOIN_CONSTRAINT_START) {
            join_constraint(p, r);
        }
        lhs_marker = p.close(m, JoinClause);
    }
}

pub fn expr_list(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, T!['('], ExprList);

    let m = p.open();

    p.guaranteed(T!['(']);

    expr(p, r);
    while p.eat(T![,]) {
        expr(p, r);
    }

    p.must_eat(T![')'], r);

    p.close(m, ExprList);
}

pub fn full_table_function_name(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, FullTableFunctionName);

    let m = p.open();

    if p.nth(1) == T![.] {
        must_eat_name(p, r, SchemaName);
        p.must_eat(T![.], r);
    }
    must_eat_name(p, r, TableFunctionName);

    p.close(m, FullTableFunctionName);
}

pub fn from_clause_table_value_function(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    full_table_function_name(p, r);
    expr_list(p, r);

    if p.at_any(p.with_alias_start) {
        with_alias(p, r);
    }

    p.close(m, FromClauseTableValueFunction);
}

pub fn table_name_not_indexed(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_NOT);
    p.must_eat(KW_INDEXED, r);

    p.close(m, TableNameNotIndexed);
}

pub fn table_name_indexed_by(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_INDEXED);
    p.must_eat(KW_BY, r);
    must_eat_name(p, r, IndexName);

    p.close(m, TableNameIndexedBy);
}

pub fn with_alias(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.with_alias_start, WithAlias);

    let m = p.open();

    if p.eat(KW_AS) {
        must_eat_name(p, r, AliasName);
    } else {
        // The reason the else case do not allow name_token is because other there will be
        // ambiguity related to join keywords
        p.must_eat_any(p.iden_or_str, AliasName, r);
    }

    p.close(m, WithAlias);
}

pub fn qualified_table_name(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, QualifiedTableName);

    let m = p.open();

    full_table_name(p, r);

    if p.at_any(p.with_alias_start) {
        with_alias(p, r);
    }

    if p.at(KW_INDEXED) {
        table_name_indexed_by(p, r);
    } else if p.at(KW_NOT) {
        table_name_not_indexed(p, r);
    }

    p.close(m, QualifiedTableName);
}

pub fn table_or_subquery(p: &mut SqliteParser, r: TokenSet) -> MarkClosed {
    let m = p.open();

    let table_or_subquery_start = p.name_token | T!['('];
    // Table Value function must come first or it would be parsed as qualified table name
    if p.at_any(p.name_token) && (p.nth(1) == T!['('] || p.nth(3) == T![')']) {
        from_clause_table_value_function(p, r);
    } else if p.at_any(p.name_token) {
        qualified_table_name(p, r);
    } else if p.eat(T!['(']) {
        if SELECT_STMT_WITH_CTE_START.contains(p.nth(1)) {
            p.wrap(FromClauseSelectStmt, |p| {
                select_stmt_with_cte(p, r | T![')']);
                p.must_eat(T![')'], r);

                if p.at_any(p.with_alias_start) {
                    with_alias(p, r);
                }
            });
        } else if table_or_subquery_start.contains(p.nth(1)) {
            join_clause(p, r | T![')']);
            p.must_eat(T![')'], r);
        } else {
            expected_one_of!(p, r | T![')'], SelectStmtWithCte | TableOrSubquery);
            p.must_eat(T![')'], r);
        }
    }

    p.close(m, TableOrSubquery)
}

pub fn from_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_FROM);
    join_clause(p, r);

    p.close(m, FromClause);
}

pub fn result_column_table_all(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    must_eat_name(p, r, TableName);
    p.guaranteed(T![.]);
    p.guaranteed(T![*]);

    p.close(m, ResultColumnTableAll);
}

pub fn result_column_expr(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.expr_start, ResultColumnExpr);

    let m = p.open();

    expr(p, r);

    if p.at_any(p.with_alias_start) {
        with_alias(p, r);
    }

    p.close(m, ResultColumnExpr);
}

pub fn result_column_list(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, T![*] | p.name_token | p.expr_start, ResultColumnList);

    let m = p.open();

    result_column(p, r);
    while p.eat(T![,]) {
        result_column(p, r);
    }

    p.close(m, ResultColumnList);
}

pub fn result_column(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, T![*] | p.name_token | p.expr_start, ResultColumn);

    let m = p.open();

    if p.at(T![*]) {
        p.wrap(ResultColumnAll, |p| p.advance());
    } else if p.at_any(p.name_token) && p.nth(1) == T![.] && p.nth(2) == T![*] {
        result_column_table_all(p, r);
    } else if p.at_any(p.expr_start) {
        result_column_expr(p, r);
    } else {
        // NOTE: p.expr_start is a superset of p.name_token, this is why we are sure we won't hit
        // this case
        unreachable!("DEV ERROR: result_column start check is wrong")
    }

    p.close(m, ResultColumn);
}

pub fn traditional_select(p: &mut SqliteParser, r: TokenSet) {
    let r = r | KW_FROM | KW_GROUP | KW_HAVING | KW_WINDOW;
    let m = p.open();

    p.must_eat(KW_SELECT, r);
    p.eat_any(KW_DISTINCT | KW_ALL);

    result_column_list(p, r);

    if p.at(KW_FROM) {
        from_clause(p, r);
    }

    if p.at(KW_WHERE) {
        where_clause(p, r);
    }

    if p.at(KW_GROUP) {
        group_by_clause(p, r);
    }

    if p.at(KW_HAVING) {
        having_clause(p, r);
    }

    if p.at(KW_WINDOW) {
        window_clause(p, r);
    }

    p.close(m, TraditionalSelect);
}

pub fn select_core(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, SELECT_STMT_START, SelectCore);

    let m = p.open();

    if p.at(KW_SELECT) {
        traditional_select(p, r);
    } else if p.at(KW_VALUES) {
        values_select(p, r);
    } else {
        unreachable!("DEV ERROR: SELECT_STMT_START is incorrect")
    }

    p.close(m, SelectCore);
}

pub fn select_stmt(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, SELECT_STMT_START, SelectStmt);
    let r = r | SELECT_STMT_START | COMPOUND_SELECT_START | KW_ORDER | KW_LIMIT;

    let m = p.open();

    select_core(p, r);
    while p.at_any(COMPOUND_SELECT_START) {
        compound_select(p, r);
    }

    if p.at(KW_ORDER) {
        order_by_clause(p, r);
    }

    if p.at(KW_LIMIT) {
        limit_clause(p, r);
    }

    p.close(m, SelectStmt);
}

pub fn materialized_cte(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, MATERIALIZED_CTE_START, MaterializedCte);

    let m = p.open();

    p.eat(KW_NOT);
    p.must_eat(KW_MATERIALIZED, r);

    p.close(m, MaterializedCte);
}

pub fn col_name_list(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, T!['('], ColNameList);

    let m = p.open();

    p.guaranteed(T!['(']);

    must_eat_name(p, r, ColumnName);
    while p.eat(T![,]) {
        must_eat_name(p, r, ColumnName);
    }

    p.must_eat(T![')'], r);

    p.close(m, ColNameList);
}

pub fn common_table_expr(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, CommonTableExpr);

    let m = p.open();

    must_eat_name(p, r, CteName);

    if p.at(T!['(']) {
        col_name_list(p, r);
    }

    p.must_eat(KW_AS, r);

    if p.at(KW_NOT) {
        materialized_cte(p, r);
    }

    p.must_eat(T!['('], r);
    select_stmt_with_cte(p, r);
    p.must_eat(T![')'], r);

    p.close(m, CommonTableExpr);
}

pub fn cte_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_WITH);

    p.eat(KW_RECURSIVE);

    common_table_expr(p, r);
    while p.eat(T![,]) {
        common_table_expr(p, r);
    }

    p.close(m, CteClause);
}

pub fn select_stmt_with_cte(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, SELECT_STMT_WITH_CTE_START, SelectStmtWithCte);

    let m = p.open();

    if p.at(KW_WITH) {
        cte_clause(p, r);
    }
    select_stmt(p, r);

    p.close(m, SelectStmtWithCte);
}

pub fn in_select(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(T!['(']);
    select_stmt_with_cte(p, r);
    p.must_eat(T![')'], r);

    p.close(m, InSelect);
}

pub fn collation(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, KW_COLLATE, Collation);

    let m = p.open();

    p.guaranteed(KW_COLLATE);
    p.must_eat_any(p.iden_or_str, CollationName, r);

    p.close(m, Collation);
}

pub fn expr_prefix(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, EXPR_PREFIX_START, ExprPrefix);

    let m = p.open();

    let op_node = match p.nth(0) {
        T![~] => OpBinComplement,
        T![+] => OpUnaryPlus,
        T![-] => OpUnaryMinus,
        KW_NOT => OpNot,
        _ => unreachable!("DEV ERROR: EXPR_PREFIX_START mismatch"),
    };

    let (None, Some(r_bp)) = precedence_table(op_node) else {
        unreachable!("DEV ERROR: Operator Node mismatch")
    };

    p.wrap(op_node, |p| {
        p.advance(); // consume operator token
        expr_bp(p, r, r_bp);
    });

    p.close(m, ExprPrefix);
}

pub fn expr_column_name(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, ExprColumnName);

    let m = p.open();

    if p.nth(1) == T![.] && p.nth(3) == T![.] {
        must_eat_name(p, r, SchemaName);
        p.must_eat(T![.], r);
        must_eat_name(p, r, TableName);
        p.must_eat(T![.], r);
        must_eat_name(p, r, ColumnName);
    } else if p.nth(1) == T![.] {
        must_eat_name(p, r, TableName);
        p.must_eat(T![.], r);
        must_eat_name(p, r, ColumnName);
    } else {
        // iden_or_join here is intended
        p.must_eat_any(p.iden_or_join, ColumnName, r);
    }

    p.close(m, ExprColumnName);
}

pub fn expr_lit(p: &mut SqliteParser, _r: TokenSet) {
    let m = p.open();

    p.guaranteed_any(EXPR_LIT_START);

    p.close(m, ExprLit);
}

pub fn check_constraint(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_CHECK);
    p.must_eat(T!['('], r);
    expr(p, r);
    p.must_eat(T![')'], r);

    p.close(m, CheckConstraint);
}

pub fn unique_constraint(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_UNIQUE);

    if p.at(KW_ON) {
        conflict_clause(p, r);
    }

    p.close(m, UniqueConstraint);
}

pub fn not_null_constraint(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_NOT);
    p.must_eat(KW_NULL, r);

    if p.at(KW_ON) {
        conflict_clause(p, r);
    }

    p.close(m, NotNullConstraint);
}

pub fn conflict_action(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, CONFLICT_ACTION_START, ConflictAction);

    let m = p.open();

    must_eat_one_of!(
        p,
        r,
        KW_ROLLBACK | KW_ABORT | KW_FAIL | KW_IGNORE | KW_REPLACE
    );

    p.close(m, ConflictAction);
}

pub fn conflict_clause(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_ON);
    p.must_eat(KW_CONFLICT, r);
    conflict_action(p, r);

    p.close(m, ConflictClause);
}

pub fn primary_constraint(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_PRIMARY);
    p.must_eat(KW_KEY, r);

    if p.at_any(ORDER_START) {
        order(p, r);
    }

    if p.at(KW_ON) {
        conflict_clause(p, r);
    }

    p.eat(KW_AUTOINCREMENT);

    p.close(m, PrimaryConstraint);
}

pub fn column_constraint_name(p: &mut SqliteParser, r: TokenSet) {
    let m = p.open();

    p.guaranteed(KW_CONSTRAINT);

    must_eat_name(p, r, ConstraintName);

    p.close(m, ColumnConstraintName);
}

pub fn column_constraint(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, COLUMN_CONSTRAINT_START, ColumnConstraint);

    let m = p.open();

    if p.at(KW_CONSTRAINT) {
        column_constraint_name(p, r);
    }

    if p.at(KW_PRIMARY) {
        primary_constraint(p, r);
    } else if p.at(KW_NOT) {
        not_null_constraint(p, r);
    } else if p.at(KW_UNIQUE) {
        unique_constraint(p, r);
    } else if p.at(KW_CHECK) {
        check_constraint(p, r);
    } else if p.at(KW_DEFAULT) {
        default_constraint(p, r);
    } else if p.at(KW_COLLATE) {
        collation(p, r);
    } else if p.at_any(COLUMN_GENERATED_START) {
        column_generated(p, r);
    } else {
        unreachable!("DEV ERROR: COLUMN_CONSTRAINT_START is incorrect");
    }

    p.close(m, ColumnConstraint);
}

pub fn signed_number(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, SIGNED_NUMBER_START, SignedNumber);

    let m = p.open();

    p.eat_any(PLUS | MINUS);
    if !p.eat_any(NUMERIC_LIT) {
        expected_one_of!(p, r, INT_LIT | REAL_LIT | HEX_LIT);
    }

    p.close(m, SignedNumber);
}

pub fn type_name(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.iden_or_str, TypeName);

    let m = p.open();

    // NOTE: While SQLite standard is to allow STR_LIT, does it have any drawbacks?
    p.must_eat_any(p.iden_or_str, TypeNameWord, r);

    while p.at_any(p.iden_or_str) {
        p.must_eat_any(p.iden_or_str, TypeNameWord, r);
    }

    if p.eat(T!['(']) {
        signed_number(p, r);

        if p.eat(T![,]) {
            signed_number(p, r);
        }
        p.must_eat(T![')'], r);
    }

    p.close(m, TypeName);
}

pub fn column_def(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, ColumnDef);

    let m = p.open();

    must_eat_name(p, r, ColumnName);

    if p.at_any(p.iden_or_str) {
        type_name(p, r);
    }
    while p.at_any(COLUMN_CONSTRAINT_START) {
        column_constraint(p, r);
    }

    p.close(m, ColumnDef);
}

pub fn table_details(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, T!['('], TableDetails);
    let r = r | p.name_token | T![,] | TABLE_CONSTRAINT_START | T![')'] | TABLE_OPTIONS_START;
    let m = p.open();

    p.must_eat(T!['('], r);

    column_def(p, r);
    while p.eat(T![,]) {
        if p.at_any(TABLE_CONSTRAINT_START) {
            break;
        } else {
            column_def(p, r);
        }
    }

    if p.at_any(TABLE_CONSTRAINT_START) {
        table_constraint(p, r);
        while p.eat(T![,]) {
            table_constraint(p, r);
        }
    }

    p.must_eat(T![')'], r);

    if p.at_any(TABLE_OPTIONS_START) {
        table_options_list(p, r);
    }

    p.close(m, TableDetails);
}

pub fn full_table_name(p: &mut SqliteParser, r: TokenSet) {
    bail_if_not_at!(p, r, p.name_token, FullTableName);

    let m = p.open();

    if p.nth(1) == T![.] {
        must_eat_name(p, r, SchemaName);
        p.must_eat(T![.], r);
    }
    must_eat_name(p, r, TableName);

    p.close(m, FullTableName);
}

pub fn if_not_exists(p: &mut SqliteParser, r: TokenSet) {
    let r = r | KW_NOT | KW_EXISTS;
    let m = p.open();

    p.guaranteed(KW_IF);
    p.must_eat(KW_NOT, r);
    p.must_eat(KW_EXISTS, r);

    p.close(m, IfNotExists);
}

pub fn create_table_stmt(p: &mut SqliteParser, r: TokenSet) {
    let r = r | p.name_token | KW_AS | T!['('];
    let m = p.open();

    p.guaranteed(KW_CREATE);
    p.eat_any(KW_TEMP | KW_TEMPORARY);
    p.guaranteed(KW_TABLE);

    if p.at(KW_IF) {
        if_not_exists(p, r);
    }
    full_table_name(p, r);

    if p.at(KW_AS) {
        create_table_select(p, r);
    } else if p.at(T!['(']) {
        table_details(p, r);
    } else {
        expected_one_of!(p, r, CreateTableSelect | TableDetails)
    }

    p.close(m, CreateTableStmt);
}

pub(crate) fn expr(p: &mut SqliteParser, r: TokenSet) -> Option<MarkClosed> {
    expr_bp(p, r, 0)
}

/// Parsing expressions involve parsing the "left hand side" and optionally parsing a
/// right hand side expression if the left hand side is followed by a postfix/infix operator.
/// If there is no right hand side expression, then the left hand side becomes the whole
/// expression
fn expr_bp(p: &mut SqliteParser, r: TokenSet, min_bp: u8) -> Option<MarkClosed> {
    if !p.at_any_now_or_later(p.expr_start, r) {
        p.expected(Expr, r);
        return None;
    }

    // This large if block Handles left hand side of the expr. `MarkClosed` from the freshly
    // parsed SqliteTreeNode maybe used later if there is a postfix/infix operator. In that case,
    // this lhs SqliteTreeNode will become the child of a larger expression and MarkClosed will
    // be used with the parser's `open_before` API to facilitate this. If there is no postfix
    // or infix operator, the lhs part of the expression is taken to be the complete expression.
    let lhs_open = p.open();
    if p.at_any(EXPR_LIT_START) {
        expr_lit(p, r)
    } else if p.at(T!['(']) {
        if p.nth(1) == KW_SELECT {
            expr_select(p, r)
        } else if p.expr_start.contains(p.nth(1)) {
            expr_list(p, r)
        } else {
            let items = expected_items!(ExprSelect | ExprList);
            p.wrap_err(items.as_slice(), r, |p| p.advance());
        }
    } else if p.at_any(EXPR_BIND_PARAM_START) {
        expr_bind_param(p, r);
    } else if p.at_any(p.name_token) {
        if p.nth(1) == T!['('] {
            expr_func(p, r);
        } else {
            expr_column_name(p, r);
        }
    } else if p.at_any(EXPR_PREFIX_START) {
        expr_prefix(p, r)
    } else if p.at(KW_CAST) {
        expr_cast(p, r)
    } else if p.at(KW_CASE) {
        expr_case(p, r)
    } else if p.at(KW_RAISE) {
        raise_func(p, r);
    } else if p.at_any(KW_NOT | KW_EXISTS) {
        expr_select(p, r);
    } else {
        unreachable!("DEV ERROR: Bug in this if statement")
    };

    let mut lhs_marker = p.close(lhs_open, Expr);

    // The loop has two main cases:
    // - postfix operators
    // - infix operators
    loop {
        // Section 2.1: Handle postfix operators
        if let Some(postfix_op) = utils::which_postfix_op(p) {
            // If we think we are also at an infix_op, there is ambiguity and means
            // our at_infix_op is not specific enough (we only check start tokens after all)
            assert!(!utils::which_infix_op(p).is_none());

            let (Some(l_bp), None) = precedence_table(postfix_op) else {
                unreachable!("DEV ERROR: Operator should be postfix")
            };

            if l_bp < min_bp {
                break;
            }

            parse_postfix_op(p, r, postfix_op);

            // { Expr postfix_token } -> Expr { ExprPostfix { PostfixOp { Expr postfix_token }}}
            lhs_marker = p.wrap_parent(lhs_marker, postfix_op);
            lhs_marker = p.wrap_parent(lhs_marker, ExprPostfix);
            lhs_marker = p.wrap_parent(lhs_marker, Expr);

            continue;
        }

        // Section 2.2: Handle infix operators
        if let Some(infix_op) = utils::which_infix_op(p) {
            // Even though KW_AND is binary op on its own, it also appears as part of the
            // `[expr] BETWEEN [expr] AND [expr]` operator. Therefore, we have to ignore
            // occurences of KW_AND when we are already trying to parse a `between and` operator
            if infix_op == OpAnd && utils::is_processing_between_and_op(p) {
                break;
            }

            let (Some(l_bp), Some(r_bp)) = precedence_table(infix_op) else {
                unreachable!("DEV ERROR: Operator should be infix")
            };

            if l_bp < min_bp {
                break;
            }

            // LIKE | NOT LIKE are special cases
            if matches!(infix_op, OpLike | OpNotLike) {
                if infix_op == OpNotLike {
                    p.guaranteed(KW_NOT);
                }
                p.guaranteed(KW_LIKE);

                expr_bp(p, r, r_bp);

                if p.eat(KW_ESCAPE) {
                    // TODO: SQLite spec actually allow an expr here but we will only support
                    // string literals for now
                    p.must_eat(STR_LIT, r);
                }
            }
            // IN () and NOT IN () are special cases
            else if matches!(infix_op, OpIn | OpNotIn) {
                if infix_op == OpNotIn {
                    p.guaranteed(KW_NOT);
                }
                p.guaranteed(KW_IN);

                if p.at(T!['(']) {
                    if p.nth(1) == KW_SELECT {
                        in_select(p, r)
                    } else if p.expr_start.contains(p.nth(1)) {
                        expr_list(p, r)
                    } else {
                        let items = expected_items!(ExprSelect | ExprList);
                        p.wrap_err(items.as_slice(), r, |p| p.advance());
                    }
                } else if p.at_any(p.name_token) {
                    // Case where a schema name exists:
                    // SchemaName '.' TableName | SchemaName '.' TableFunctionName '('
                    if p.nth(1) == T![.] {
                        if p.nth(3) == T!['('] {
                            in_table_func(p, r);
                        } else {
                            in_table(p, r);
                        }
                    }
                    // Usual Case
                    // TableName | TableFunctionName '('
                    else {
                        if p.nth(1) == T!['('] {
                            in_table_func(p, r);
                        } else {
                            in_table(p, r);
                        }
                    }
                }
            }
            // BETWEEN [expr] AND [expr] is yet another special case
            else if matches!(infix_op, OpBetweenAnd | OpNotBetweenAnd) {
                if infix_op == OpNotBetweenAnd {
                    p.guaranteed(KW_NOT);
                }
                p.guaranteed(KW_BETWEEN);
                expr(p, r);
                p.must_eat(KW_AND, r);

                expr_bp(p, r, r_bp);
            }
            // Handle normal infix operators
            else {
                parse_infix_op(p, r, infix_op);
                expr_bp(p, r, r_bp);
            }

            // { Expr infix_token Expr } -> Expr { ExprInfix { InfixOp { Expr infix_token Expr }}}
            lhs_marker = p.wrap_parent(lhs_marker, infix_op);
            lhs_marker = p.wrap_parent(lhs_marker, ExprInfix);
            lhs_marker = p.wrap_parent(lhs_marker, Expr);

            continue;
        }

        break;
    }

    // p.close(expr_m, Expr)
    Some(lhs_marker)
}

fn parse_postfix_op(p: &mut SqliteParser, r: TokenSet, postfix_op: SqliteTreeKind) {
    match postfix_op {
        OpNotSpaceNull => {
            p.advance_by(2);
        }
        OpCollate => {
            collation(p, r);
        }
        OpIsNull | OpNotNull => p.advance(),
        _ => unreachable!("DEV ERROR: Unknown Postfix Operator"),
    }
}

fn parse_infix_op(p: &mut SqliteParser, r: TokenSet, infix_op: SqliteTreeKind) {
    assert!(
        !matches!(
            infix_op,
            OpNotLike | OpLike | OpNotBetweenAnd | OpBetweenAnd | OpIn | OpNotIn
        ),
        "While these are infix ops, they are meant to handled specially and not here"
    );

    match infix_op {
        // Operators that are two tokens wide
        OpNotMatch | OpNotRegexp | OpNotGlob | OpNotSpaceNull | OpIsNot => {
            p.advance_by(2);
        }

        // IS DISTINCT FROM / IS NOT DISTINCT FROM is unusually wide.
        OpIsDistinctFrom | OpIsNotDistinctFrom => {
            p.guaranteed(KW_IS);
            p.eat(KW_NOT);
            p.guaranteed(KW_DISTINCT);

            // To be more intelligent, `which_infix_op`, did not check if there is a `KW_FROM`
            // token at the end since KW_IS, KW_NOT, and KW_DISTINCT is enough to deterimine
            // the operator. Thus, we use `expect_or_advance` instead of `expect`
            p.must_eat(KW_FROM, r);
        }

        OpConcat | OpExtractOne | OpExtractTwo | OpMultiply | OpDivide | OpModulus | OpAdd
        | OpSubtract | OpBinAnd | OpBinOr | OpBinLShift | OpBinRShift | OpLT | OpGT | OpLTE
        | OpGTE | OpEq | OpNotEq | OpAnd | OpOr | OpMatch | OpRegexp | OpGlob | OpIs => {
            p.advance();
        }

        _ => unreachable!("DEV ERROR: Unknown Infix Operator"),
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

pub fn must_eat_name(p: &mut SqliteParser, r: TokenSet, name_kind: SqliteTreeKind) {
    p.must_eat_any(p.name_token, name_kind, r);
}

pub(crate) mod utils {
    use crate::SqliteToken;

    use super::*;

    pub(crate) fn which_infix_op(p: &SqliteParser) -> Option<SqliteTreeKind> {
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
            [KW_AND, ..] => OpAnd,
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
            // [KW_NOT, ..] => OpNot, ??
            _ => return None,
        };

        Some(op)
    }

    pub(crate) fn which_postfix_op(p: &SqliteParser) -> Option<SqliteTreeKind> {
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

    // A helper function that lets us figure out if we are in the middle of parsing the
    // BETWEEN AND operator.
    pub(crate) fn is_processing_between_and_op(p: &SqliteParser) -> bool {
        assert!(p.at(KW_AND));

        // [EVENT_ADVANCE(BETWEEN), EVENT_OPEN(EXPR), ... EVENT_CLOSE(EXPR), AND]
        // We are trying to match an event stream similar to above.
        assert!(!p.events.is_empty());
        let mut event_diff = 0;
        let mut tk_count = 0; // Tokens in the event that we are trying to skip
        for e in p.events.iter().rev() {
            match e {
                Event::Open { .. } | Event::Error { .. } => event_diff += 1,
                Event::Close => event_diff -= 1,
                Event::Advance { .. } => tk_count += 1,
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

    pub(super) const STATEMENT_START: TokenSet =
        enum_set!(KW_EXPLAIN | STATEMENT_NO_CTE_START | STATEMENT_WITH_CTE_START);

    pub(super) const STATEMENT_NO_CTE_START: TokenSet = enum_set!(
        KW_CREATE
            | KW_ALTER
            | KW_DETACH
            | KW_DROP
            | KW_ATTACH
            | KW_SAVEPOINT
            | KW_ROLLBACK
            | KW_PRAGMA
            | KW_ANALYZE
            | KW_RELEASE
            | KW_VACUUM
            | KW_BEGIN
            | COMMIT_STMT_START
            | KW_REINDEX
    );

    pub(super) const STATEMENT_WITH_CTE_START: TokenSet =
        enum_set!(KW_UPDATE | INSERT_STMT_START | SELECT_STMT_START | KW_DELETE | KW_WITH);

    pub(super) const COMMIT_STMT_START: TokenSet = enum_set!(KW_END | KW_COMMIT);

    pub(super) const SELECT_STMT_START: TokenSet = enum_set!(KW_SELECT | KW_VALUES);

    pub(super) const INSERT_STMT_START: TokenSet = enum_set!(KW_REPLACE | KW_INSERT);

    pub(super) const FK_DEFERRABLE_START: TokenSet = enum_set!(KW_NOT | KW_DEFERRABLE);

    pub(super) const SIGNED_NUMBER_START: TokenSet = enum_set!(NUMERIC_LIT | PLUS | MINUS);

    pub(super) const TRIGGER_ACTION_KIND_START: TokenSet =
        enum_set!(KW_DELETE | KW_INSERT | KW_UPDATE);

    pub(super) const TRIGGER_BODY_STMT_START: TokenSet =
        enum_set!(KW_INSERT | KW_UPDATE | KW_DELETE | SELECT_STMT_START);

    pub(super) const SELECT_STMT_WITH_CTE_START: TokenSet = enum_set!(KW_WITH | SELECT_STMT_START);

    pub(super) const MODULE_ARG_START: TokenSet = enum_set!(NUMERIC_LIT | STR_LIT);

    pub(super) const RETURNING_CLAUSE_START: TokenSet = enum_set!(KW_RETURNING);

    pub(super) const DELETE_STMT_LIMITED_START: TokenSet = enum_set!(KW_ORDER | KW_LIMIT);

    pub(super) const CONFLICT_ACTION_START: TokenSet =
        enum_set!(KW_ROLLBACK | KW_ABORT | KW_FAIL | KW_REPLACE | KW_IGNORE);

    pub(super) const UPDATE_STMT_LIMITED_START: TokenSet = enum_set!(KW_ORDER | KW_LIMIT);

    pub(super) const INSERT_STMT_KIND_START: TokenSet = enum_set!(KW_REPLACE | KW_INSERT);

    pub(super) const INSERT_VALUE_KIND_START: TokenSet =
        enum_set!(INSERT_SELECT_CLAUSE_START | KW_DEFAULT | KW_VALUES);

    pub(super) const INSERT_SELECT_CLAUSE_START: TokenSet = enum_set!(SELECT_STMT_WITH_CTE_START);

    pub(super) const ORDER_START: TokenSet = enum_set!(KW_DESC | KW_ASC);

    pub(super) const OFFSET_START: TokenSet = enum_set!(COMMA | KW_OFFSET);

    pub(super) const MATERIALIZED_CTE_START: TokenSet = enum_set!(KW_MATERIALIZED | KW_NOT);

    pub(super) const TABLE_CONSTRAINT_START: TokenSet =
        enum_set!(KW_PRIMARY | KW_UNIQUE | KW_FOREIGN | KW_CHECK | KW_CONSTRAINT);

    pub(super) const TABLE_OPTIONS_START: TokenSet = enum_set!(KW_STRICT | KW_WITHOUT);

    pub(super) const COMPOUND_SELECT_START: TokenSet = enum_set!(COMPOUND_OPERATOR_START);

    pub(super) const COMPOUND_OPERATOR_START: TokenSet =
        enum_set!(KW_INTERSECT | KW_UNION | KW_EXCEPT);

    pub(super) const FRAME_SPEC_START: TokenSet = enum_set!(KW_GROUPS | KW_ROWS | KW_RANGE);

    pub(super) const JOIN_OPERATOR_START: TokenSet = enum_set!(NON_COMMA_JOIN_START | COMMA);

    pub(super) const JOIN_CONSTRAINT_START: TokenSet = enum_set!(KW_USING | KW_ON);

    pub(super) const NON_COMMA_JOIN_START: TokenSet =
        enum_set!(KW_INNER | KW_NATURAL | OUTER_JOIN_START | KW_CROSS | KW_JOIN);

    pub(super) const OUTER_JOIN_START: TokenSet = enum_set!(KW_RIGHT | KW_LEFT | KW_FULL);

    pub(super) const COLUMN_CONSTRAINT_START: TokenSet = enum_set!(
        KW_UNIQUE
            | COLUMN_GENERATED_START
            | KW_PRIMARY
            | KW_NOT
            | KW_CONSTRAINT
            | KW_CHECK
            | KW_COLLATE
            | KW_DEFAULT
    );

    pub(super) const COLUMN_GENERATED_START: TokenSet = enum_set!(KW_AS | KW_GENERATED);

    pub(super) const EXPR_EXISTS_SELECT_START: TokenSet = enum_set!(KW_NOT | KW_EXISTS | L_PAREN);
}

use ahash::{HashMap, HashSet};


use yukon_sqlite3_parser::*;

use SqliteTokenKind::*;

pub struct Context;

pub enum QueryType {
    Scalar(ColumnType),
    Many(Vec<ColumnType>),
}

#[derive(Clone, Copy)]
pub enum PrimitiveType {
    Integer,
    Real,
    String,
    Optional(&'static PrimitiveType),
}

#[derive(Clone, Copy)]
pub struct ColumnType {
    pub schema: &'static str,
    pub table: &'static str,
    pub column_name: &'static str,
    pub column_type: PrimitiveType,
}

pub enum ExprType {
    PrimitiveType(PrimitiveType),
    ColumnType(ColumnType),
}

use SqliteTreeKind::*;

// We are sure that the query is valid at this point, we just need to return types
fn resolve_type_select_stmt(
    node: &SqliteNode,
    ast: &SqliteUntypedAst,
    ctx: &Context,
) -> Option<Vec<ColumnType>> {
    assert!(matches!(node.tree_kind(), Some(SelectStmt)));

    // let mut result = Vec::new();

    // No from clause: simple query
    if node.find_child(ast, FromClause).is_none() {
        let ty = node
            .find_child(ast, ResultColumnList)?
            .find_children(ast, ResultColumn)
            .flat_map(|it| resolve_type_result_column(it, ast, &Vec::new()))
            .collect();

        return Some(ty);
    } else {
    }

    todo!()
}

fn resolve_type_result_column(
    node: &SqliteNode,
    ast: &SqliteUntypedAst,
    from_clause: &Vec<ColumnType>,
) -> Vec<ColumnType> {
    assert!(matches!(node.tree_kind(), Some(ResultColumn)));

    match node.children(ast).next() {
        // Select user.first_name ...
        Some(SqliteNode::Token { token, .. }) if token.kind == IDEN => {
            let cols = from_clause
                .iter()
                // TODO: This comparison doesn't cover all cases
                .filter(|col_type| col_type.table == token.text)
                .copied()
                .collect::<Vec<_>>();

            cols
        }
        // SELECT * ...
        Some(SqliteNode::Token { token, .. }) if token.kind == T![*] => from_clause.to_vec(),
        // SELECT 1 ...
        Some(SqliteNode::Tree { kind: Expr, .. }) => {
            todo!()
        }
        _ => unreachable!("DEV ERROR: Unexpected ResultColumn"),
    }
}
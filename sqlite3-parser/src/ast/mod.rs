mod generated;
mod manual;

pub use generated::*;
pub use manual::*;

#[test]
fn test_create_table_ast() {
    let cst = crate::parse("CREATE TABLE IF NOT EXISTS users(name)");

    println!("{cst}");
    let create_table_stmt = cst
        .typed_ast()
        .statements()
        .find_map(|it| match it.statement_kind().unwrap() {
            StatementKind::StatementNoCte(StatementNoCte::CreateTableStmt(stmt)) => Some(stmt),
            _ => None,
        })
        .unwrap();

    assert!(create_table_stmt.if_not_exists().is_some());
}

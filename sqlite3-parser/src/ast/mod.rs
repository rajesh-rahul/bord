mod compare;
mod generated;
mod manual;

pub use generated::*;

#[test]
fn test_create_table_ast() {
    use crate::CstTrait;
    let cst: crate::incr::IncrSqlCst = crate::parse("CREATE TABLE IF NOT EXISTS users(name)");

    let create_table_stmt = cst
        .typed_ast()
        .statements()
        .find_map(|it| match it.statement_kind().unwrap() {
            StatementKind::StatementNoCte(StatementNoCte::CreateTableStmt(stmt)) => Some(stmt),
            _ => None,
        })
        .unwrap();

    println!(
        "{}",
        create_table_stmt
            .full_table_name()
            .unwrap()
            .table()
            .unwrap()
            .text()
    );
}

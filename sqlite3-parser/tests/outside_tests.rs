use bord_sqlite3_parser::*;

// Read each file in the corpus directory parse it and compare the result to the expected output
#[test]
fn test_from_corpus() {
    use std::fs;
    use std::path::Path;

    let corpus_dir = Path::new("tests/resources");

    let file_paths = fs::read_dir("tests/resources/official-suite")
        .unwrap()
        // .chain(fs::read_dir("tests/resources/official-suite").unwrap())
        .flatten()
        .filter(|e| e.metadata().is_ok_and(|m| m.is_file()));

    for entry in file_paths {
        eprintln!("Parsing {:?}", entry.path().canonicalize().unwrap());
        let sql = &fs::read_to_string(entry.path()).unwrap();

        // Test Incremental CST
        // let incr_ast: incr::IncrSqlCst = bord_sqlite3_parser::parse(sql);
        let batch_ast: batch::SqlCst = bord_sqlite3_parser::parse(sql);
        // let slot_ast: slot::SlotIncrSqlCst = bord_sqlite3_parser::parse(sql);

        // assert!(incr_ast.errors().next().is_none());
        if batch_ast
            .errors()
            .filter(|it| {
                let err = it.error().unwrap();
                if err.is_missing_semicolon_err() {
                    return false;
                } else {
                    match err {
                        ParseErrorKind::ExpectedItems(items) => {
                            if items.as_slice() == [ExpectedItem::Tree(SqliteTreeKind::Statement)]
                                && matches!(
                                    it.right_siblings().next().and_then(|it| it.token_kind()),
                                    Some(SqliteTokenKind::SEMICOLON)
                                )
                            {
                                false
                            } else {
                                true
                            }
                        }
                        _ => true,
                    }
                }
            })
            .next()
            .is_some()
        {
            eprintln!("{batch_ast}");
            panic!("Found errors");
        }
        crate::test_utils::ensure_ast_conforms_to_ungram(&batch_ast);
        // assert!(slot_ast.errors().next().is_none());
    }
}

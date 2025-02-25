use bord_sqlite3_parser::*;

#[test]
// Read each file in the corpus directory parse it and compare the result to the expected output
fn test_from_corpus() {
    use std::fs;
    use std::path::Path;

    let corpus_dir = Path::new("tests/test_input");

    let file_paths = fs::read_dir(corpus_dir)
        .unwrap()
        .flatten()
        .filter(|e| e.metadata().is_ok_and(|m| m.is_file()));

    for entry in file_paths {
        let input_string = fs::read_to_string(entry.path()).unwrap();

        input_string.split("%%").for_each(|test_data| {
            println!("data: {test_data}");
            let (sql, expected_ast_as_str) = test_data.split_once("\n\n").unwrap();

            let events_and_tokens = bord_sqlite3_parser::parse_events_and_tokens(sql);

            // Test Incremental CST
            let ast: incr::IncrSqlCst = bord_sqlite3_parser::parse(sql);
            assert_eq!(events_and_tokens, ast.to_events_and_tokens());

            crate::test_utils::check_input(&ast, expected_ast_as_str);
            crate::test_utils::ensure_ast_conforms_to_ungram(&ast);

            // Test Batch CST
            let ast: batch::SqlCst = bord_sqlite3_parser::parse(sql);
            assert_eq!(events_and_tokens, ast.to_events_and_tokens());

            crate::test_utils::check_input(&ast, expected_ast_as_str);
            crate::test_utils::ensure_ast_conforms_to_ungram(&ast);

            // Test Batch CST
            let ast: slot::SlotIncrSqlCst = bord_sqlite3_parser::parse(sql);
            assert_eq!(events_and_tokens, ast.to_events_and_tokens());

            crate::test_utils::check_input(&ast, expected_ast_as_str);
            crate::test_utils::ensure_ast_conforms_to_ungram(&ast);
        });
    }
}

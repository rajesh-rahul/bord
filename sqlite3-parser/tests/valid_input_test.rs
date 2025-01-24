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
            let ast = bord_sqlite3_parser::parse(sql);

            crate::test_utils::check_input(&ast, expected_ast_as_str);
            crate::test_utils::ensure_ast_conforms_to_ungram(&ast);
        });
    }
}

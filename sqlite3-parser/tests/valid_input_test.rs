#[cfg(test)]
mod common;

#[cfg(test)]
use pretty_assertions::assert_eq;

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
        let mut input = input_string.as_str();

        while !input.is_empty() {
            let (i, data) = common::test_data(&input).unwrap();

            let (actual_tree, _) = yukon_sqlite3_parser::parse(&data.input);
            let actual = common::convert_tree_to_simple_node(&actual_tree);
            let expected = data.expected;

            if actual != expected {
                println!("Failed to parse {:?}:", entry.path());
                let mut actual_str = String::new();
                for node in &actual {
                    common::print_simple_node(&node, 0, &mut actual_str, true);
                }
                let mut expected_str = String::new();
                for node in &expected {
                    common::print_simple_node(&node, 0, &mut expected_str, true);
                }

                let mut actual_str_wide = String::new();
                for node in &actual {
                    common::print_simple_node(&node, 0, &mut actual_str_wide, false);
                }

                println!("Actual str:\n{}", actual_str_wide);
                assert_eq!(expected_str, actual_str);
            }
            input = i;
        }
    }
}

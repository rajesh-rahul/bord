use yukon_sqlite3_parser::parse;

#[test]
fn test_parse() {
    let (tree, errors) = parse("SELECT 1 BETWEEN 1 BETWEEN (6 + 9) + 3 AND 0 + 3 AND 0;");

    println!("{:?}", tree);
    println!("{:?}", errors);
}
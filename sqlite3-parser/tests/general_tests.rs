use bord_sqlite3_parser::parse;

#[test]
fn test_parse() {
    let ast = parse("CREATE TABLE f");

    println!("{:?}", ast);
}

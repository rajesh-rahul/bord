use bord_sqlite3_parser::parse;

#[test]
fn test_parse() {
    let cst = parse("CREATE TABLE f");

    println!("{}", cst);
}

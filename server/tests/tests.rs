use regex::Regex;

#[test]
fn test_regex() {
    let re =
        Regex::new(r#"sql!\(\s*(?:"?(?<n1>(?:\\.|[^"\\])*)"|r#"(?<n2>(?:\\.|[^"\\])*)"\#)\s*\)"#)
            .unwrap();
    //  r#"ql!\(\s*(?:"([^"]*)"|r#*"([^"]*)"#*)\s*\)"#
    let input = r##"

    fn main() {
        let conn = Conn(rusqlite::Connection::open_in_memory().unwrap());
        let q = sql!("SELECT ");
    
        
        dbg!(q);
    }
"##;

    let names = ["n1", "n2"];

    for m in re
        .captures_iter(input)
        .flat_map(|cap| names.into_iter().flat_map(move |nm| cap.name(nm)))
    {
        let start = m.start();
        let end = m.end();
        println!("SQL: {}, Position: {}-{}", m.as_str(), start, end);
    }
}

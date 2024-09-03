use std::time::{Duration, Instant};

use yukon_sqlite3_parser::{parse, SqliteParseError};

pub fn publish_diagnosics(input: &str) -> Vec<SqliteParseError> {
    let start = Instant::now();
    let (tree, errors) = yukon_sqlite3_parser::parse(input);
    let duration = start.elapsed();
    eprintln!("{:?}", tree);
    eprintln!("TREEEEEEEEEEEEEEEE TIME: {duration:?}");

    errors
}

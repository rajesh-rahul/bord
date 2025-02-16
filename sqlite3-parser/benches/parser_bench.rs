use criterion::{criterion_group, criterion_main, Criterion};
use fallible_iterator::FallibleIterator;
use sqlite3_parser::lexer::sql::Parser as LemonParser;
/// Some notes:
/// Using tiny vec for storing children nodes in the cst resulted in 36% improvement
/// 13% improvement when allocating the cst up front
fn parser_benchmark(c: &mut Criterion) {
    // Create a benchmark group
    let mut group = c.benchmark_group("Parser");

    let input_sql = include_str!("../../schema.sql");

    group.bench_function("Bord Parser", |b| {
        b.iter(|| {
            let cst = bord_sqlite3_parser::parse(input_sql);
            assert!(cst.errors().next().is_none());
        })
    });

    group.bench_function("Lemon-rs Parser", |b| {
        b.iter(|| {
            let mut parser = LemonParser::new(input_sql.as_bytes());
            loop {
                let cmd = parser.next().unwrap();

                if cmd.is_none() {
                    break;
                }
            }
        })
    });

    // Finish the benchmark group
    group.finish();
}

// Taken from: https://github.com/gwenn/lemon-rs/blob/master/sqlparser_bench/benches/sqlparser_bench.rs
// Added semicolon for statements
fn basic_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("sqlite3_parser parsing benchmark");

    let string = "SELECT * FROM `table` WHERE 1 = 1;";
    group.bench_with_input("sqlite3_parser::select", &string.as_bytes(), |b, &s| {
        b.iter(|| {
            let mut parser = LemonParser::new(s);
            assert!(parser.next().unwrap().unwrap().readonly());
        });
    });

    group.bench_with_input("bord_parser::select", &string, |b, &s| {
        b.iter(|| {
            let cst = bord_sqlite3_parser::parse(s);
            assert!(cst.errors().next().is_none());
        });
    });

    let with_query = "
        WITH derived AS (
            SELECT MAX(a) AS max_a,
                   COUNT(b) AS b_num,
                   user_id
            FROM `TABLE`
            GROUP BY user_id
        )
        SELECT * FROM `table`
        LEFT JOIN derived USING (user_id);
    ";
    group.bench_with_input(
        "sqlite3_parser::with_select",
        &with_query.as_bytes(),
        |b, &s| {
            b.iter(|| {
                let mut parser = LemonParser::new(s);
                assert!(parser.next().unwrap().unwrap().readonly())
            });
        },
    );

    group.bench_with_input("bord_parser::with_select", &with_query, |b, &s| {
        b.iter(|| {
            let cst = bord_sqlite3_parser::parse(s);
            assert!(cst.errors().next().is_none());
        });
    });
}

criterion_group!(benches, parser_benchmark, basic_queries);
criterion_main!(benches);

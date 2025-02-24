use bord_sqlite3_parser::{batch, incr};
use criterion::{criterion_group, criterion_main, Criterion};
use regex::Regex;

fn regex_benchmark(c: &mut Criterion) {
    // Create a benchmark group
    let mut group = c.benchmark_group("Regex matching bench");

    let input = r#"
        fn main() {
            hello;
            let x = sql!("SELECT * FROM \"users\"");
            some ohter stuff
            let k = sql!("SELECT * FROM countries");
        }
    "#;

    group.bench_with_input("Regex SQL String match", input, |b, s| {
        let re = Regex::new(
            r#"sql!\(\s*(?:"?(?<n1>(?:\\.|[^"\\])*)"|r#"(?<n2>(?:\\.|[^"\\])*)"\#)\s*\)"#,
        )
        .unwrap();
        let names = ["n1", "n2"];

        b.iter(|| {
            for m in re
                .captures_iter(input)
                .flat_map(|cap| names.into_iter().flat_map(move |nm| cap.name(nm)))
            {
                let start = m.start();
                let end = m.end();
                // println!("SQL: {}, Position: {}-{}", m.as_str(), start, end);
            }
        })
    });

    // Finish the benchmark group
    group.finish();
}

criterion_group!(benches, regex_benchmark);
criterion_main!(benches);

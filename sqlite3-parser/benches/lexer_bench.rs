use criterion::{criterion_group, criterion_main, Criterion};
use yukon_sqlite3_parser::{SqliteLexer, SqliteVersion};
use sqlite3_parser::lexer::{sql::Tokenizer, Scanner};

fn lexer_benchmark(c: &mut Criterion) {
    // Create a benchmark group
    let mut group = c.benchmark_group("Lexer");

    // Define the input SQL query
    let input_sql = "SELECT 1e1";

    let lexer = SqliteLexer::new(input_sql, SqliteVersion([3, 46, 0]));
    let tokens = lexer.lex();

    println!("{:?}", tokens);

    // Benchmark the lexer in your project
    group.bench_function("Yksql Lexer", |b| {
        b.iter(|| {
            let lexer = SqliteLexer::new(input_sql, SqliteVersion([3, 46, 0]));
            let _ = lexer.lex();
        })
    });

    let tokenizer = Tokenizer::new();
    let mut s = Scanner::new(tokenizer);

    let data = input_sql.as_bytes();
    let mut tokens = vec![];
    loop {
        match s.scan(data) {
            Ok((_, None, _)) => break,
            Err(err) => println!("Error: {err}"),
            Ok((start, Some((_, token_type)), end)) => {
                tokens.push((start, token_type, end));
            }
        }
    }

    println!("{:?}", tokens);

    // Benchmark the lemon-rs lexer
    group.bench_function("Lemon-rs Lexer", |b| {
        b.iter(|| {
            let tokenizer = Tokenizer::new();
            let mut s = Scanner::new(tokenizer);

            let mut tokens = vec![];

            loop {
                match s.scan(data) {
                    Ok((_, None, _)) => break,
                    Err(_) => todo!(),
                    Ok((_, Some((token, token_type)), _)) => {
                        tokens.push((token, token_type));
                    }
                }
            }
        })
    });

    // Finish the benchmark group
    group.finish();
}

criterion_group!(benches, lexer_benchmark);
criterion_main!(benches);

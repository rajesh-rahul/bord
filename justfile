build:
    cargo build --bin bord-server
    mkdir -p ../bordsql-vscode/client/out
    mv target/debug/bord-server ../bordsql-vscode/client/out/bord-server

gen_ast:
    cargo run --bin gen_ast > sqlite3-parser/src/ast/temporary_generated_file.rs
    mv sqlite3-parser/src/ast/temporary_generated_file.rs sqlite3-parser/src/ast/generated.rs 
    cargo +nightly fix --allow-dirty --all-features
    just fmt

fmt:
    cargo +nightly fmt

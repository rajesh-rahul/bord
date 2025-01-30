build:
    cargo build --bin bord-cli
    mkdir -p ../bord-vscode/client/out
    mv target/debug/bord-cli ../bord-vscode/client/out/bord-cli

gen_ast:
    cargo run --bin gen_ast > sqlite3-parser/src/ast/temporary_generated_file.rs
    mv sqlite3-parser/src/ast/temporary_generated_file.rs sqlite3-parser/src/ast/generated.rs 
    cargo +nightly fix --allow-dirty --all-features
    just fmt

fmt:
    cargo +nightly fmt

fix:
    cargo +nightly fix --allow-dirty --all-features
    just fmt
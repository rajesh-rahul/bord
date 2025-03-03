# Build the bord cli in debug mode and move it to bord-vscode folder to use in extension development environment
build:
    cargo build --bin bord-cli
    mkdir -p ../bord-vscode/client/out
    mv target/debug/bord-cli ../bord-vscode/client/out/bord-cli

# Build the binary that saves all traces to a file (which can then be used for tests). Also moved to bord-vscode folder to use in extension development environment
build-incr-parser-test:
    cargo build --bin server-test
    mkdir -p ../bord-vscode/client/out
    mv target/debug/server-test ../bord-vscode/client/out/bord-cli

# Build the bord cli in release mode and move it to bord-vscode folder to use in extension development environment
build-release:
    cargo build --release --bin bord-cli
    mkdir -p ../bord-vscode/client/out
    mv target/release/bord-cli ../bord-vscode/client/out/bord-cli

# Generate the ast. TODO: Consider a build.rs script?
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

test:
    cargo test --all-features

flamegraph:
    cargo flamegraph --root --bench parser_bench -- --bench

official_test:
    cargo test --package bord-sqlite3-parser --test outside_tests --all-features -- test_from_corpus --exact --show-output
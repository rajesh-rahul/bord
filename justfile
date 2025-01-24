build:
    cargo build
    mkdir -p ../bordsql-vscode/client/out
    mv target/debug/bord-server ../bordsql-vscode/client/out/bord-server

gen_ast:
    ungrammar2json < sqlite.ungram > sqlite.ungram.json
    python gen_ast.py > sqlite3-parser/src/ast/generated.rs
    just fmt

fmt:
    cargo +nightly fmt

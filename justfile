build:
    cargo build
    mkdir -p ../yukonsql-vscode/client/out
    mv target/debug/yukon-server ../yukonsql-vscode/client/out/yukon-server

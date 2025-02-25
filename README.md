# Tic-Tac-Toe Game Rust

1. Install the CLI tool
    ```
    cargo install cargo-generate
    ```
2. Install the WebAssembly CLI tool
    ```
    cargo install wasm-pack
    ```
3. Build with cargo
    ```
    cargo build
    ```
4. Run the native application binary
     ```
     cargo run
     ```
5. To build with WebAssembly browser version
    In PowerShell:
     ```     
     $env:RUSTFLAGS = "--cfg getrandom_backend=""wasm_js"""
     cargo build --lib --release --target wasm32-unknown-unknown
     wasm-pack build --release --target web
     ```
6. To access the web version
     ```     
     python -m http.server
     ```
The browser version here:
http://localhost:8000

Or here:
https://d17lzqqtmm6hs0.cloudfront.net

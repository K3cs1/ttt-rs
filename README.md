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
     ```     
     wasm-pack build --release --target web
     ```
6. To access the web version
     ```     
     python3 -m http.server
     ```
The Web version can be accessed:
http://localhost:8000

Or here:
https://d17lzqqtmm6hs0.cloudfront.net

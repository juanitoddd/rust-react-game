cargo build --target wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
wasm-bindgen --target web --out-dir pkg ./target/wasm32-unknown-unknown/debug/game.wasm
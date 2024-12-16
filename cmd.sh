cargo build --target wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli

npm run build:cargo
npm run build:wasm # ---> wasm-pack build --target web --out-dir pkg
npm run update:pkg # ---> npm install ./lib/pkg
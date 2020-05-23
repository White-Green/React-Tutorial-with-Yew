 rustup target add wasm32-unknown-unknown
 cargo build --target wasm32-unknown-unknown
 wasm-bindgen target/wasm32-unknown-unknown/debug/react_tutorial.wasm --out-dir node/src
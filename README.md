Build wasm component
```
cargo b -p rust-component --target wasm32-unknown-unknown 
```
with WASI support
```
cargo b -p rust-component --target wasm32-wasi
```
Run wasm runtime with the loaded `app` wasm component
```
cargo run -p app
```
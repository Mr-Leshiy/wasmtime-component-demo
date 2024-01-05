Build wasm component
```
cargo b -p rust-component --target wasm32-unknown-unknown 
```
Run wasm runtime with the loaded `app` wasm component
```
cargo run -p app
```
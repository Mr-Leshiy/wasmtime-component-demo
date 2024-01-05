use wasmtime::{component::{Component, Linker}, Config, Engine, Store};
use wit_component::ComponentEncoder;

wasmtime::component::bindgen!();

fn main() -> Result<(), wasmtime::Error> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let linker = Linker::<u64>::new(&engine);
    let mut store = Store::new(&engine, 0_u64);

    // we first read the bytes of the wasm module.
    let module = std::fs::read("./target/wasm32-unknown-unknown/debug/rust_component.wasm")?;
    let component = ComponentEncoder::default()
        .module(module.as_slice())?
        .validate(true)
        .encode()?;
    let component = Component::from_binary(&engine, &component)?;

    let (demo_component, _) = Demo::instantiate(&mut store, &component, &linker)?;
    
    let res = demo_component.my_demo_host().call_hello(&mut store, "Alex")?;
    println!("{}", res);

    Ok(())
}

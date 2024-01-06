use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use wit_component::ComponentEncoder;

wasmtime::component::bindgen!();

struct HostRuntime;

impl my::demo::bar::Host for HostRuntime {
    fn say(&mut self, name: String) -> wasmtime::Result<()> {
        println!("{}", name);
        Ok(())
    }
}

fn main() -> Result<(), wasmtime::Error> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut linker = Linker::<HostRuntime>::new(&engine);
    let mut store = Store::new(&engine, HostRuntime);

    my::demo::bar::add_to_linker(&mut linker, |state: &mut HostRuntime| state)?;

    // we first read the bytes of the wasm module.
    let module = std::fs::read("./target/wasm32-unknown-unknown/debug/rust_component.wasm")?;
    let component = ComponentEncoder::default()
        .module(module.as_slice())?
        .validate(true)
        .encode()?;
    let component = Component::from_binary(&engine, &component)?;

    let (demo_component, _) = Demo::instantiate(&mut store, &component, &linker)?;

    let res = demo_component
        .my_demo_foo()
        .call_hello(&mut store, "Alex")?;

    Ok(())
}

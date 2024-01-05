wit_bindgen::generate!({
    exports: {
       "my:demo/host": MyHost
    },
    path: "../app/wit/example.wit",
});

use exports::my::demo::host::Guest;
struct MyHost;

impl Guest for MyHost {
    fn hello(name: wit_bindgen::rt::string::String) -> wit_bindgen::rt::string::String {
        let res = format!("Hello dear, {}!", name);
        println!("{res}");
        res
    }
}

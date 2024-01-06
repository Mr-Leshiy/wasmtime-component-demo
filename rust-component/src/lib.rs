wit_bindgen::generate!({
    exports: {
       "my:demo/foo": MyComponent
    },
    path: "../app/wit/example.wit",
});

use exports::my::demo::foo::Guest;
struct MyComponent;

impl Guest for MyComponent {
    fn hello(name: wit_bindgen::rt::string::String) -> wit_bindgen::rt::string::String {
        let res = format!("Hello dear, {}!", name);
        my::demo::bar::say(&res);
        res
    }
}

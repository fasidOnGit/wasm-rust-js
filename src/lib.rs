mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name=log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name=log)]
    fn  log_many(a: &str, b:&str);
}

#[wasm_bindgen(module = "/www/src/person.ts")]
extern "C" {
    fn what() -> String;

    type Person;
    #[wasm_bindgen(constructor)]
    fn new(name: &str) -> Person;
    #[wasm_bindgen(method, getter)]
    fn name(this: &Person) -> String;
    #[wasm_bindgen(method, setter)]
    fn set_name(this: &Person, name: &str) -> Person;
    #[wasm_bindgen(method)]
    fn talk(this: &Person) -> String;
}
fn bare_bones() {
    log("Hello from Rust!");
    log_u32(42);
    log_many("Logging", "Many values!");
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


fn using_web_sys() {
    use web_sys::console;

    console::log_1(&"Hello using web-sys".into());

    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values look like".into(), &js);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, Kader!");
}

#[wasm_bindgen(start)]
pub fn run() {
    bare_bones();
    using_a_macro();
    using_web_sys();
    using_imported_js();
}

fn using_imported_js() {
    log(&format!("Hello from {}", what()));
    let x = Person::new("Fasid");
    log(&format!("{}", x.name()));
    x.set_name("Yasmine");
    log(&format!("{}", x.name()));
    log(&x.talk());
}

fn using_a_macro() {
    console_log!("Hello {}!", "World!");
    console_log!("11 +3 = {}", 11+3);
}

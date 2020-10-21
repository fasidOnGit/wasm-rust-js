mod utils;
mod char_counter;
mod js_sys_example;
mod web_sys_dom;
mod web_sys_closures;
mod performance_now;
mod fetch_json;
mod arr_of_obj_to_rust;

use wasm_bindgen::prelude::*;
use crate::js_sys_example::js_sys_async;

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name=log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name=log)]
    fn  log_many(a: &str, b:&str);
}

#[wasm_bindgen(module = "/frontend/src/person.ts")]
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
    #[wasm_bindgen(method, getter)]
    fn age(this: &Person) -> usize;
    #[wasm_bindgen(method, getter)]
    fn work(this: &Person) -> JsValue;
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
    // alert("Hello, Kader!");
    println!("Hello! Kader!");
}

#[wasm_bindgen]
pub fn take_a_closure(f: &js_sys::Function) -> Result<usize, JsValue> {
    let this = JsValue::NULL;
    // alert(
    //     f.call0(&this)?.as_string().unwrap().as_str()
    // );
    Ok(10)
}

#[wasm_bindgen(start)]
pub fn run() {
    femme::start(log::LevelFilter::Trace);
    log::info!("deii");
    bare_bones();
    using_a_macro();
    using_web_sys();
    using_imported_js();
    js_sys_async();
}

fn using_imported_js() {
    log(&format!("Hello froms {}", what()));
    let x = Person::new("Mohideen");
    log(&format!("{}", x.name()));
    x.set_name("Yasmine");
    log(&format!("{}", x.name()));
    log(&format!("My age is {}", x.age()));
    let work: Work = JsValue::into_serde(&x.work()).unwrap();
    log(&format!("My age is {}", work.company));
    log(&x.talk());
}

fn using_a_macro() {
    console_log!("Hello {}!", "World!");
    console_log!("11 +3 = {}", 11+3);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Work {
    pub salary: usize,
    pub company: String
}
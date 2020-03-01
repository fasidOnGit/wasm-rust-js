use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn web_sys_dom() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("where the hell is the body!");
    let div = document.get_element_by_id("webSys".into()).expect("Something wrong");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Web sys!");
    div.append_child(&val).expect("Cannot Append!");
    Ok(())
}

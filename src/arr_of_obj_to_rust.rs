use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Reverse;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Somes {
    name: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Element {
    name: String,
    id: usize,
    parent_id: String,
    some: Somes
}

#[wasm_bindgen]
pub fn mighty_fn(js_obj: &JsValue) -> JsValue {
    log::warn!("Unauthorized access attempt on /login");
    log::info!("Listening on port 8080");
    let mut elements: Vec<Element> = js_obj.into_serde().unwrap();
    for e in 0..elements.len() {
        elements[e].name = elements[e].name.to_owned() + " Mighty".into();
    }
    log(&format!("{:?}", elements));
    JsValue::from_serde(&elements).unwrap()
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TryUndefined {
    a: Option<String>,
    b: Option<f64>
}

/*
* `JsValue::into_serde` and `from_serde` pretty much converts the undefined to null.
*/
#[wasm_bindgen]
pub fn try_undefined(js_obj: &JsValue) -> JsValue {
    let elem: TryUndefined = JsValue::into_serde(js_obj).unwrap();
    log::warn!("Deii {:?}", serde_wasm_bindgen::to_value(&elem)); // Works as JSON.parse(JSON.stringify)
    JsValue::from_serde(&elem).unwrap()
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Student {
    name: String,
    class: String,
    subject: Subject
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Subject {
    name: String,
    key: String
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct School {
    students: Vec<Student>,
    unit: Option<String>,
    offset: usize,
    limit: usize,
    filter: Option<String>,
    sort_property: Option<String>,
    sort_dir: Option<String>
}

#[wasm_bindgen]
pub fn data_filter(filter_obj: &JsValue) -> JsValue {
    let elem: RefCell<School> = RefCell::new(
        JsValue::into_serde(filter_obj).unwrap()
    );
    let el = elem.clone();
    let mut result = el.borrow_mut().students.clone();
    if is_empty_none(el.borrow().filter.borrow()) && is_empty_none(el.borrow().sort_property.borrow()) {
        JsValue::from_serde(&result).unwrap()
    } else {
        let filter_lower_case = el.borrow_mut().filter.as_ref().unwrap().to_lowercase().to_string();
        result = result
            .into_iter()
            .filter(|e| e.name.to_lowercase().contains(&filter_lower_case) || e.class.to_lowercase().contains(&filter_lower_case))
            .collect();
        if !is_empty_none(el.borrow().unit.borrow()) {
            result = elem.borrow().students.clone()
                .into_iter()
                .filter(|e| e.subject.key == elem.borrow().unit.as_ref().unwrap().to_string())
                .collect();
        }
        if !is_empty_none(el.borrow().sort_property.borrow()) {
            let sort = elem.clone().into_inner().sort_property;
            if is_empty_none(elem.borrow().sort_dir.borrow()) || elem.borrow().sort_dir.as_ref().unwrap().eq("asc") { // Missed asc
                result.sort_by_key(|e| match &sort {
                    Some(s) => match s.as_str() {
                        "name" => e.clone().name,
                        "class" => e.clone().class,
                        _ => e.clone().class
                    },
                    None => e.clone().class
                });
            } else {
                result.sort_by_key(
                    |_e| Reverse(elem.clone().into_inner().sort_property)
                );
            }
        }
        JsValue::from_serde(&result).unwrap()
    }
}

fn is_empty_none(obj: &Option<String>) -> bool {
    (obj.is_none() || obj.clone().unwrap().is_empty())
}

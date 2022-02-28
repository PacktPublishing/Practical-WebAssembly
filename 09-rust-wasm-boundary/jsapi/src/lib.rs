use wasm_bindgen::prelude::*;

use js_sys::Map;

#[wasm_bindgen]
pub fn new_js_map() -> Map {
    Map::new()
}

#[wasm_bindgen]
pub fn set_get_js_map() -> JsValue {
    let map = Map::new();
    map.set(&"foo".into(), &"bar".into());
    map.get(&"foo".into())
}

#[wasm_bindgen]
pub fn run_through_map() -> f64 {
    let map = Map::new();
    map.set(&1.into(), &1.into());
    map.set(&2.into(), &2.into());
    map.set(&3.into(), &3.into());
    map.set(&4.into(), &4.into());
    map.set(&5.into(), &5.into());
    let mut res: f64 = 0.0;

    map.for_each(&mut |value, _| {
        res = res + value.as_f64().unwrap();
    });

    res
}
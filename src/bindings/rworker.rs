use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type RObjectBase;

    #[wasm_bindgen(constructor, js_namespace = "RWorker")]
    pub fn new(ptr: u32) -> RObjectBase;

    #[wasm_bindgen(method, getter, js_namespace = "RWorker")]
    pub fn ptr(this: &RObjectBase) -> u32;

    #[wasm_bindgen(method, js_namespace = "RWorker", js_name = "type")]
    pub fn type_(this: &RObjectBase) -> String;
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type WebR;

    #[wasm_bindgen(constructor, js_namespace = ["WebR"])]
    pub fn new() -> WebR;

    #[wasm_bindgen(method)]
    pub async fn close(this: &WebR);

    // TODO need to define RObject in RMain module
    // #[wasm_bindgen(method)]
    // pub async fn destroy(x: RObject) ->

    #[wasm_bindgen(method, js_name = "evalR")]
    pub async fn eval_r(this: &WebR, code: String) -> JsValue;
}

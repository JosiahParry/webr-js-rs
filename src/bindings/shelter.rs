use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "https://webr.r-wasm.org/latest/webr.mjs")]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(js_name = Shelter)]
    pub type Shelter;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Shelter;

    // methods are
    // captureR destroy evalR purge and size
    #[wasm_bindgen(method, catch, js_name = "captureR")]
    pub async fn capture_r(this: &Shelter, code: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn destroy(this: &Shelter, x: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalR")]
    pub async fn eval_r(this: &Shelter, code: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn purge(this: &Shelter) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn size(this: &Shelter) -> Result<JsValue, JsValue>;

}

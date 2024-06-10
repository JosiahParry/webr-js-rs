use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "https://webr.r-wasm.org/latest/webr.mjs")]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(js_name = WebR)]
    pub type WebR;

    #[wasm_bindgen(constructor)]
    pub fn new() -> WebR;

    #[wasm_bindgen(method, catch)]
    pub async fn init(this: &WebR) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "installPackages")]
    pub async fn install_package(this: &WebR, packages: Vec<String>) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalR")]
    pub async fn eval_r(this: &WebR, code: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalRBoolean")]
    pub async fn eval_r_boolean(this: &WebR, code: String) -> Result<bool, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalRNumber")]
    pub async fn eval_r_number(this: &WebR, code: String) -> Result<f64, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalRRaw")]
    pub async fn eval_r_raw(this: &WebR, code: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalRString")]
    pub async fn eval_r_string(this: &WebR, code: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalRVoid")]
    pub async fn eval_r_void(this: &WebR, code: String) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
    pub type FS;

    #[wasm_bindgen(method, getter, js_name = FS)]
    pub fn fs(_: &WebR) -> FS;

    #[wasm_bindgen(method, catch, js_class = "FS")]
    pub async fn mount(this: &FS, type_: FSType, mountpoint: String) -> Result<(), JsValue>;
}

/// Note that NODEFS only works under Node JS so it most likely will not work for you
/// WORKERFS is read only, however? not sure on how you would write to it
#[wasm_bindgen]
pub enum FSType {
    NODEFS = "NODEFS",
    WORKERFS = "WORKERFS",
}

// #[derive(Debug, Clone)]
// #[wasm_bindgen]
// pub struct FSMountOptions {
//     root: String,
//     // Ommiting these for simplicity right now
//     // blobs: Option<Vec<BlobData>>,
//     // files: Option<Vec<JsValue>>, // File or FileList
//     // packages: Option<Vec<PackageData>>,
// }

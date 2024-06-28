use wasm_bindgen::prelude::*;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebROptions {
    #[serde(rename = "RArgs")]
    pub r_args: Option<Vec<String>>,
    // pub base_url: Option<String>,
    // pub channel_type: Option<String>,
    // pub create_lazy_filesystem: Option<bool>,
    // pub interactive: Option<bool>,
    // pub repo_url: Option<String>,
    // pub service_worker_url: Option<String>,
}

#[wasm_bindgen(module = "https://webr.r-wasm.org/v0.3.3/webr.mjs")]
extern "C" {
    #[derive(Debug, Clone, PartialEq)]
    #[wasm_bindgen(js_name = WebR)]
    pub type WebR;

    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue) -> WebR;

    #[wasm_bindgen(method, catch)]
    pub async fn close(this: &WebR) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn destroy(this: &WebR, x: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn init(this: &WebR) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "installPackages")]
    pub async fn install_package(this: &WebR, packages: Vec<String>) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalR")]
    pub async fn eval_r(this: &WebR, code: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalRBoolean")]
    pub async fn eval_r_boolean(this: &WebR, code: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalRNumber")]
    pub async fn eval_r_number(this: &WebR, code: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalRRaw")]
    pub async fn eval_r_raw(this: &WebR, code: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalRString")]
    pub async fn eval_r_string(this: &WebR, code: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "evalRVoid")]
    pub async fn eval_r_void(this: &WebR, code: String) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "writeConsole")]
    pub fn write_console(this: &WebR, input: String) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = "read")]
    pub async fn read(this: &WebR) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "flush")]
    pub async fn flush(this: &WebR) -> Result<JsValue, JsValue>;

    // Missing are:
    // interrupt -- doesn't work anyways b/c postmessage
    // write
    // invokeWasmFunction
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Message {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: String,
}

impl WebR {
    // Create a new WebR instance with default options
    pub fn default() -> WebR {
        let opts = serde_wasm_bindgen::to_value(&WebROptions::default()).unwrap();
        WebR::new(opts)
    }

    /// Starts an instance of WebR with the `--quiet` flag
    pub fn shhh() -> WebR {
        let opts = serde_wasm_bindgen::to_value(&WebROptions {
            r_args: Some(vec!["--quiet".into()]),
            ..Default::default()
        })
        .unwrap();
        WebR::new(opts)
    }

    /// Reads all stdout messages until it reaches the prompt
    /// The results are stored as a vector of strings.
    pub async fn read_all_stdout(&self) -> Vec<String> {
        let mut res = vec![];
        loop {
            let msg = self.read().await.unwrap();
            let msg: Message = serde_wasm_bindgen::from_value(msg).unwrap();

            if msg.data == "> " {
                break;
            } else {
                res.push(msg.data);
            }
        }
        res
    }

    /// Flushes the stdout buffer and reads all messages until it
    /// reaches the prompt
    pub async fn flush_and_read(&self) -> Vec<String> {
        let _ = self.flush().await.unwrap();
        self.read_all_stdout().await
    }
}

#[wasm_bindgen]
extern "C" {
    pub type FS;

    #[wasm_bindgen(method, getter, js_name = FS)]
    pub fn fs(_: &WebR) -> FS;

    #[wasm_bindgen(method, catch, js_class = "FS")]
    pub async fn mount(this: &FS, type_: FSType, mountpoint: String) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_class = "FS", js_name = "readFile")]
    pub async fn read_file(this: &FS) -> JsValue;
}
/// Note that NODEFS only works under Node JS so it most likely will not work for you
/// WORKERFS is read only, however? not sure on how you would write to it
#[wasm_bindgen]
pub enum FSType {
    NODEFS = "NODEFS",
    WORKERFS = "WORKERFS",
}

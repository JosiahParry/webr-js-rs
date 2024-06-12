use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use webr_js_rs::*;

// Run tests with
// wasm-pack test --chrome
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn eval_bool() {
    let webr = crate::webr::WebR::new();
    let res = webr.eval_r_boolean("TRUE".into()).await.unwrap();
    println!("{:?}", res);
    assert_eq!(true, res.as_bool().unwrap());
}

#[wasm_bindgen_test]
async fn eval_r_string() {
    let webr = crate::webr::WebR::new();
    let res = webr
        .eval_r_string(r#"paste0("hello", "world")"#.into())
        .await
        .unwrap();

    assert_eq!("helloworld".to_string(), res.as_string().unwrap());
}

#[wasm_bindgen_test]
async fn eval_r_number() {
    let webr = crate::webr::WebR::new();
    let res = webr.eval_r_number("1+1".into()).await.unwrap();
    assert_eq!(2, res.as_f64().unwrap() as i32);
}

// This causes a WebR error
// TypeError: Cannot read properties of undefined (reading 'evalR')
// #[wasm_bindgen_test]
// async fn eval_r() {
//     let webr = crate::webr::WebR::new();
//     let res = webr.eval_r("1+1".into()).await.unwrap();
//     web_sys::console::log_1(&res);
// }

#[wasm_bindgen_test]
async fn test() {
    let webr = crate::webr::WebR::new();
    let _ = webr.init().await;
    let _ = webr.write_console("rnorm(100)".into());
    let _ = webr.flush().await.unwrap();

    loop {
        let res = webr.read().await;

        let res = serde_wasm_bindgen::from_value::<Message>(res.unwrap()).unwrap();
        web_sys::console::log_1(&JsValue::from_str(&res.data));

        if &res.data == "> " {
            break;
        }
    }
}

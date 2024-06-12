# webr-js-sys

Unofficial wasm-bindgen bindings for the [`WebR` JavaScript library](https://docs.r-wasm.org/) written in Rust. 

This library crate only work ons wasm32-unknown-unknown target.

```rs
// create a new instance
let webr = crate::webr::WebR::new();

// initialize webR
let _ = webr.init().await;

// write to the console
let _ = webr.write_console("rnorm(100)".into());

// flush the console and read the output
let res = webr.flush_and_read().await;
res.iter().for_each(|x| {
    web_sys::console::log_1(&JsValue::from_str(x));
});
```

use js_sys::JsString;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "fs", "BaseDirectory"], js_name = "AppLocalData", thread_local)]
    pub static APP_LOCAL_DATA: JsString;
}

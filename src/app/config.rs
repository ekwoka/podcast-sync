use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "fs"], catch)]
    async fn exists(token: &str, args: JsValue) -> Result<JsValue,JsValue>;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "fs"], catch)]
    async fn create(token: &str, args: JsValue) -> Result<JsValue,JsValue>;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "fs"], catch, js_name="writeTextFile")]
    async fn write_text_file(token: &str, content: &str, args: JsValue) -> Result<JsValue,JsValue>;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "fs"], catch, js_name="readTextFile")]
    async fn read_text_file(token: &str, args: JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "fs"], catch)]
    async fn mkdir(token: &str, args: JsValue) -> Result<JsValue,JsValue>;
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FsArgs {
    base_dir: u16,
}

pub struct Config();

impl Config {
  pub async fn setup() {
    let fs_args = serde_wasm_bindgen::to_value(&FsArgs {base_dir: 15 }).unwrap();
    if exists("", fs_args.clone()).await.expect("should have permissions").is_falsy() {
      mkdir("", fs_args).await.expect("should have permissions");
            };
  }
  pub async fn exists() -> bool {
    let fs_args = serde_wasm_bindgen::to_value(&FsArgs {base_dir: 15 }).unwrap();
    exists("config", fs_args).await.expect("should have permissions").is_falsy()
  }
  pub async fn create() -> Result<JsValue, JsValue> {
    let fs_args = serde_wasm_bindgen::to_value(&FsArgs {base_dir: 15 }).unwrap();
    create("config.json", fs_args).await
  }
  pub async fn update(content: &str) -> Result<JsValue, JsValue> {
    let fs_args = serde_wasm_bindgen::to_value(&FsArgs {base_dir: 15 }).unwrap();
                  write_text_file("config.json",
              content, fs_args).await
  }
  pub async fn load() -> Result<JsValue, JsValue> {
    let fs_args = serde_wasm_bindgen::to_value(&FsArgs {base_dir: 15 }).unwrap();
                 read_text_file("config.json",
               fs_args).await
  }
}

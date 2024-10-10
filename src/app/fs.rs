use js_sys::JsString;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use wasm_bindgen::prelude::*;
use api_types::config::Config;

#[derive(Serialize_repr, Deserialize_repr, Clone, Default, Debug)]
#[repr(u16)]
pub enum BaseDirectory {
    Audio = 1,
    Cache,
    Config,
    Data,
    LocalData,
    Document,
    Download,
    Picture,
    Public,
    Video,
    Resource,
    Temp,
    AppConfig,
    #[default]
    AppData,
    AppLocalData,
    AppCache,
    AppLog,
    Desktop,
    Executable,
    Font,
    Home,
    Runtime,
    Template,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_PLUGIN_FS__"], catch)]
    async fn exists(token: &str, args: JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_PLUGIN_FS__"], catch)]
    async fn create(token: &str, args: JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_PLUGIN_FS__"], catch, js_name="writeTextFile")]
    async fn write_text_file(
        token: &str,
        content: JsString,
        args: JsValue,
    ) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_PLUGIN_FS__"], catch, js_name="readTextFile")]
    async fn read_text_file(token: &str, args: JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_PLUGIN_FS__"], catch)]
    async fn mkdir(token: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FsArgs {
    base_dir: BaseDirectory,
}

pub trait File: Sized {
    async fn create_if_missing(self) -> Result<Self, JsValue>;
    async fn exists(&self) -> bool;
    async fn create(self) -> Result<Self, JsValue>;
    fn update(self, content: &str) -> Self;
    async fn load(self) -> Result<Self, JsValue>;
    async fn save(self) -> Result<Self, JsValue>;
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigFile {
    pub content: Config,

    #[serde(skip_serializing, default = "default_config_dir")]
    pub directory: BaseDirectory,
}

fn default_config_dir() -> BaseDirectory {
    BaseDirectory::AppConfig
}

impl File for Config {
    async fn create_if_missing(self) -> Result<Self, JsValue> {
        let fs_args = serde_wasm_bindgen::to_value(&FsArgs {
            base_dir: self.directory.clone(),
        })
        .unwrap();
        if exists("", fs_args.clone()).await?.is_falsy() {
            mkdir("", fs_args).await?;
        };
        Ok(self)
    }
    async fn exists(&self) -> bool {
        let fs_args = serde_wasm_bindgen::to_value(&FsArgs {
            base_dir: self.directory.clone(),
        })
        .unwrap();
        exists("config", fs_args)
            .await
            .expect("should have permissions")
            .is_falsy()
    }
    async fn create(self) -> Result<Self, JsValue> {
        let fs_args = serde_wasm_bindgen::to_value(&FsArgs {
            base_dir: self.directory.clone(),
        })
        .unwrap();
        create("config.ron", fs_args).await?;
        Ok(self)
    }
    fn update(mut self, content: &str) -> Self {
        self.latest_message = content.to_owned();
        self
    }
    async fn load(mut self) -> Result<Self, JsValue> {
        let fs_args = serde_wasm_bindgen::to_value(&FsArgs {
            base_dir: self.directory.clone(),
        })
        .unwrap();
        let content: Config = ron::from_str(
            &read_text_file("config.ron", fs_args)
                .await?
                .as_string()
                .unwrap(),
        )
        .unwrap();
        self.latest_message = content.latest_message.clone();
        Ok(self)
    }
    async fn save(self) -> Result<Self, JsValue> {
        let fs_args = serde_wasm_bindgen::to_value(&FsArgs {
            base_dir: self.directory.clone(),
        })
        .unwrap();
        write_text_file(
            "config.ron",
            serde_wasm_bindgen::to_value(&self.to_string().unwrap())
                .unwrap()
                .into(),
            fs_args,
        )
        .await?;
        Ok(self)
    }
}

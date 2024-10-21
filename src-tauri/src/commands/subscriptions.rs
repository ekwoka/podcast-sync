use std::{path::PathBuf, str::FromStr};

use api_types::subscriptions::{Subscription, Subscriptions};
use tauri::{path::BaseDirectory, Manager};
use tauri_plugin_fs::SafeFilePath;

pub fn resolve_path<R: tauri::Runtime>(
    webview: &tauri::Webview<R>,
    path: SafeFilePath,
    base_dir: Option<BaseDirectory>,
) -> Option<PathBuf> {
    let path = path.into_path().ok()?;
    if let Some(base_dir) = base_dir {
        webview.path().resolve(&path, base_dir).ok()
    } else {
        Some(path)
    }
}

#[tauri::command]
pub fn subscribe<R: tauri::Runtime>(
    webview: tauri::Webview<R>,
    subscription: Subscription,
) -> Subscription {
    let subscriptions = retrieve_subscriptions(&webview);
    let exists = subscriptions
        .subscriptions
        .iter()
        .any(|s| s.id == subscription.id);
    if !exists {
        let subscriptions = Subscriptions {
            subscriptions: subscriptions
                .subscriptions
                .into_iter()
                .chain(std::iter::once(subscription.clone()))
                .collect(),
        };
        save_subscriptions(&webview, subscriptions);
    }
    subscription
}

#[tauri::command]
pub fn unsubscribe<R: tauri::Runtime>(
    webview: tauri::Webview<R>,
    subscription_id: i32,
) -> Subscription {
    let subscriptions = retrieve_subscriptions(&webview);
    let subscription = subscriptions
        .subscriptions
        .iter()
        .find(|s| s.id == subscription_id)
        .unwrap()
        .clone();
    let subscriptions = Subscriptions {
        subscriptions: subscriptions
            .subscriptions
            .into_iter()
            .filter(|s| s.id != subscription_id)
            .collect(),
    };
    save_subscriptions(&webview, subscriptions);
    subscription
}

#[tauri::command]
pub fn load_subscriptions<R: tauri::Runtime>(webview: tauri::Webview<R>) -> Subscriptions {
    retrieve_subscriptions(&webview)
}

#[tauri::command]
pub fn load_subscription<R: tauri::Runtime>(
    webview: tauri::Webview<R>,
    subscription_id: i32,
) -> Option<Subscription> {
    retrieve_subscriptions(&webview)
        .subscriptions
        .into_iter()
        .find(|s| s.id == subscription_id)
}

fn retrieve_subscriptions<R: tauri::Runtime>(webview: &tauri::Webview<R>) -> Subscriptions {
    SafeFilePath::from_str("subscriptions.ron")
        .ok()
        .and_then(|path| resolve_path(webview, path, Some(BaseDirectory::AppData)))
        .and_then(|path| {
            if path.exists() {
                ron::de::from_reader(std::fs::File::open(path).ok()?).ok()
            } else {
                None
            }
        })
        .unwrap_or_else(|| Subscriptions {
            subscriptions: vec![],
        })
}

fn save_subscriptions<R: tauri::Runtime>(
    webview: &tauri::Webview<R>,
    subscriptions: Subscriptions,
) {
    let path = resolve_path(
        webview,
        SafeFilePath::from_str("subscriptions.ron").unwrap(),
        Some(BaseDirectory::AppData),
    );
    subscriptions
        .to_writer(&mut std::fs::File::create(path.unwrap()).unwrap())
        .unwrap();
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod services;

use tauri::async_runtime::Mutex;
use tauri::Manager;
use tauri_plugin_deep_link::DeepLinkExt;

use crate::services::{db::lib::setup_db, types::MitraServices};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, argv, _cwd| {
                  println!("a new app instance was opened with {argv:?} and the deep link event was already triggered");
                }))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let _ = app.deep_link().on_open_url(|event| {
                println!("deep link URLs: {:?}", event.urls());
            });

            let handler = app.handle();
            tauri::async_runtime::spawn(async move {});
                   tauri::async_runtime::block_on(async move {
            let db = setup_db(&handler).await.unwrap();

            let state = MitraServices {db: Mutex::new(db)};
            handler.manage(state)

                   });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

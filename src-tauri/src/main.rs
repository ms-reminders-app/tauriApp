// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reminders_subscribe::{RemindersSubscribe, Subscriber};
use std::{borrow::BorrowMut, sync::Arc};
use tokio::sync::Mutex;

use crate::db::models::*;
use db::{database, queries::Queries};
use sqlx::{Pool, Sqlite};

mod background_service;
mod db;
mod reminders_subscribe;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_reminders(
    queries_state: tauri::State<'_, Arc<Mutex<Queries>>>,
) -> Result<Vec<Reminder>, String> {
    let queries = queries_state.lock().await;
    queries.get_reminders().await.map_err(|e| e.to_string())
}

fn main() {
    let db_pool: Pool<Sqlite> = tauri::async_runtime::block_on(database::create_connection_pool())
        .expect("error while creating connection pool");

    let queries = Arc::new(Mutex::new(Queries::new(db_pool)));

    let queries_clone = queries.clone();

    let reminders_subscriber = Arc::new(std::sync::Mutex::new(RemindersSubscribe::new()));

    let reminders_subscriber_clone = reminders_subscriber.clone();

    tauri::async_runtime::spawn(async move {});

    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_reminders])
        .manage(queries)
        .setup(|app| {
            let handle = app.handle();

            tauri::async_runtime::spawn(async move {
                let reminders = queries_clone.lock().await.get_reminders().await.unwrap();
                reminders_subscriber_clone
                    .lock()
                    .unwrap()
                    .set_value(Arc::new(std::sync::Mutex::new(reminders)));
                background_service::create_service(reminders_subscriber_clone.clone(), &handle)
                    .await;
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_, _| {});
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};

use db::database;
use crate::db::models::*;

mod db;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_reminders(db_connection: tauri::State<Arc<Mutex<SqliteConnection>>>) -> Result<Vec<Reminder>, String> {
    use db::schema::reminders::dsl::*;

    let db_connection_clone = db_connection.clone();
    let mut connection = match db_connection_clone.lock() {
        Ok(val) => {
            Ok(val)
        }
        Err(_) => {
            Err(String::from("Unable to connect to the database"))
        }
    }?;

    let reminders_result = reminders
        .select(Reminder::as_select())
        .load(&mut *connection);

    match reminders_result {
        Ok(val) => {
            Ok(val)
        }
        Err(_) => {
            Err(String::from("Unable to select reminders"))
        }
    }

    // Ok(results)
}

fn main() {
    let db_connection: Arc<Mutex<SqliteConnection>> = Arc::new(Mutex::new(
        database::establish_connection()
    ));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_reminders])
        .manage(db_connection)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

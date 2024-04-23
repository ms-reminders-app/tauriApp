use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use tauri::Manager;
use tokio::time::{self};

use crate::{
    reminders_subscribe::{RemindersSubscribe, Subscriber},
    Reminder,
};

pub fn check_for_due_reminders(reminders: Vec<Reminder>) -> Option<Vec<Reminder>> {
    let due_reminders = reminders
        .iter()
        .filter(|reminder| {
            if reminder.completed == true {
                return false;
            }
            let date_from_snoozed_due = chrono::NaiveDateTime::parse_from_str(
                reminder.snoozed_due.as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap();

            println!("Snoozed due: {:?}", date_from_snoozed_due);
            let now = chrono::Local::now().naive_local();
            println!("Now: {:?}", now);
            if date_from_snoozed_due <= now {
                return true;
            }
            false
        })
        .cloned()
        .collect();
    Some(due_reminders)
}

pub async fn create_service(
    reminders_subscriber: Arc<Mutex<RemindersSubscribe>>,
    app: &tauri::AppHandle,
) {
    let reminders: Arc<Mutex<Vec<Reminder>>> = Arc::new(Mutex::new(vec![]));

    let reminders_clone = reminders.clone();

    reminders_subscriber
        .lock()
        .unwrap()
        .subscribe(Box::new(move |value: Vec<Reminder>| {
            *reminders_clone.lock().unwrap() = value;
            println!("Value set: {:?}", reminders_clone.lock().unwrap().clone());
        }));

    let mut current_due_reminders: Vec<Reminder> = vec![];

    loop {
        let due_reminders = check_for_due_reminders(reminders.lock().unwrap().clone());
        println!("Checking for due reminders");
        println!("reminders: {:?}", reminders);

        let delay = time::sleep(Duration::from_secs(30));
        if let Some(due_reminders) = due_reminders {
            if check_if_equal(&current_due_reminders, &due_reminders) {
                println!("No new reminders");
                delay.await;
                continue;
            }
            current_due_reminders = due_reminders.clone();

            match app.get_window("Notifications") {
                Some(w) if w.is_visible().unwrap() => {
                    w.emit_all("refresh", due_reminders).unwrap();
                    println!("Emitting refresh reminders");
                    delay.await;
                    continue;
                }
                Some(w) if !w.is_visible().unwrap() => {
                    println!("Window isn't visible, reopening");
                    w.close().unwrap();
                    time::sleep(Duration::from_millis(1000)).await;
                }
                _ => (),
            };

            tauri::WindowBuilder::new(
                app,
                "Notifications",
                tauri::WindowUrl::App("notification.html".into()),
            )
            .always_on_top(true)
            .resizable(false)
            .inner_size(500.0, 400.0)
            .title("Notifications")
            .build()
            .unwrap();
        }
        delay.await;
    }
}

fn check_if_equal(current_due_reminders: &[Reminder], due_reminders: &[Reminder]) -> bool {
    if current_due_reminders.len() != due_reminders.len() {
        return false;
    }
    for (index, reminder) in current_due_reminders.iter().enumerate() {
        if reminder != &due_reminders[index] {
            return false;
        }
    }

    true
}

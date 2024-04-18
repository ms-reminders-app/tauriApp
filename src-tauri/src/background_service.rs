use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::time::{self};

use crate::{
    reminders_subscribe::{RemindersSubscribe, Subscriber},
    Reminder,
};

pub fn check_for_due_reminders(reminders: Vec<Reminder>) -> Option<Vec<Reminder>> {
    let due_reminders = reminders
        .iter()
        .filter(|reminder| {
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

pub async fn create_service(reminders_subscriber: Arc<Mutex<RemindersSubscribe>>) -> ! {
    let mut reminders: Arc<Mutex<Vec<Reminder>>> = Arc::new(Mutex::new(vec![]));

    let mut reminders_clone = reminders.clone();

    reminders_subscriber
        .lock()
        .unwrap()
        .subscribe(Box::new(move |value: Vec<Reminder>| {
            *reminders_clone.lock().unwrap() = value;
            println!("Value set: {:?}", reminders_clone.lock().unwrap().clone());
        }));

    loop {
        let due_reminders = check_for_due_reminders(reminders.lock().unwrap().clone());
        println!("Checking for due reminders");
        println!("reminders: {:?}", reminders);
        if let Some(due_reminders) = due_reminders {
            println!("Due reminders: {:?}", due_reminders);
        }
        time::sleep(Duration::from_secs(30)).await;
    }
}

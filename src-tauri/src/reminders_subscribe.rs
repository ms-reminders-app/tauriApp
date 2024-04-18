use std::{
    borrow::Borrow,
    ptr,
    sync::{Arc, Mutex},
};

use crate::Reminder;

pub(crate) trait Subscriber<T, TObserver>
where
    TObserver: FnMut(Vec<T>),
{
    fn subscribe(&mut self, observer: TObserver);
    fn unsubscribe(&mut self, observer: TObserver);
    fn set_value(&mut self, value: Arc<Mutex<Vec<T>>>);
    fn notify(&self);
    // add code here
}

type Callback<T> = fn(value: T);

pub(crate) struct RemindersSubscribe {
    reminders: Arc<Mutex<Vec<Reminder>>>,
    subscribers: Arc<Mutex<Vec<Box<dyn Fn(Vec<Reminder>) + Send>>>>,
}

impl RemindersSubscribe {
    pub fn new() -> Self {
        Self {
            reminders: Arc::new(Mutex::new(Vec::new())),
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Subscriber<Reminder, Box<dyn Fn(Vec<Reminder>) + Send>> for RemindersSubscribe {
    fn subscribe(&mut self, observer: Box<dyn Fn(Vec<Reminder>) + Send>) {
        observer(self.reminders.lock().unwrap().clone());
        self.subscribers.lock().unwrap().push(observer);
    }

    fn unsubscribe(&mut self, observer: Box<dyn Fn(Vec<Reminder>) + Send>) {
        self.subscribers
            .lock()
            .unwrap()
            .retain(|x| !ptr::eq(&x, &observer.borrow()));
    }

    fn set_value(&mut self, value: Arc<Mutex<Vec<Reminder>>>) {
        *self.reminders.lock().unwrap() = value.lock().unwrap().clone();
        println!("Value set: {:?}", self.reminders.lock().unwrap().clone());
        self.notify();
    }

    fn notify(&self) {
        for subscriber in self.subscribers.lock().unwrap().iter() {
            subscriber(self.reminders.lock().unwrap().clone());
        }
    }
}

use serde::Serialize;
use sqlx::prelude::*;

#[derive(Serialize, FromRow, Clone, Debug, PartialEq)]
pub struct Reminder {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub due: String,
    pub reminder: f32,
    pub snoozed_due: String,
}

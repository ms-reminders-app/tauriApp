use serde::Serialize;
use sqlx::prelude::*;

#[derive(Serialize, FromRow)]
pub struct Reminder {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub due: String,
    pub reminder: f32,
}

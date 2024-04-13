use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::db::schema::reminders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Reminder {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub due: String,
    pub reminder: f32,
}
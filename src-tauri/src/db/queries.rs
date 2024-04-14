use crate::Reminder;

pub struct Queries {
    db_pool: sqlx::Pool<sqlx::Sqlite>,
}

impl Queries {
    pub fn new(db_pool: sqlx::Pool<sqlx::Sqlite>) -> Self {
        Self { db_pool }
    }

    pub async fn get_reminders(&self) -> Result<Vec<Reminder>, sqlx::Error> {
        let reminders = sqlx::query_as::<_, Reminder>(
            r#"
            SELECT * FROM reminders
            "#,
        );

        let reminders = reminders.fetch_all(&self.db_pool).await?;
        Ok(reminders)
    }
}

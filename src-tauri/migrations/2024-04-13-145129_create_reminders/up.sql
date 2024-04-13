-- Your SQL goes here
CREATE TABLE reminders (
    id integer primary key autoincrement not null,
    title text not null,
    description text not null,
    completed boolean not null default(0) check ( completed in (0, 1) ),
    due text not null,
    reminder REAL not null default 0
);
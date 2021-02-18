use crate::schema::*;

use chrono::prelude::*;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)
#[table_name="todolist"]
pub struct todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub deadline: DateTime<Utc>,
    pub done: u32
}

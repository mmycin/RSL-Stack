use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::todos)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateTodoInput {
    pub title: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateTodoInput {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
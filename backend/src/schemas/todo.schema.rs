use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoInput {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoInput {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoResponse {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}
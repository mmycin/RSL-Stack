use rocket::serde::json::Json;
use rocket::State;

use crate::models::todo::*;
use crate::services::todo::TodoService;

#[get("/todos")]
pub async fn list_todos(todo_service: &State<TodoService>) -> Json<Vec<Todo>> {
    let todos = todo_service.list_todos().unwrap_or_default();
    Json(todos)
}

#[post("/todos", data = "<input>")]
pub async fn create_todo(todo_service: &State<TodoService>, input: Json<CreateTodoInput>) -> Json<Todo> {
    let todo = todo_service.create_todo(input.into_inner()).unwrap();
    Json(todo)
}

#[put("/todos/<id>", data = "<input>")]
pub async fn update_todo(
    todo_service: &State<TodoService>,
    id: i32,
    input: Json<UpdateTodoInput>
) -> Json<Todo> {
    let todo = todo_service.update_todo(id, input.into_inner()).unwrap();
    Json(todo)
}

#[delete("/todos/<id>")]
pub async fn delete_todo(todo_service: &State<TodoService>, id: i32) -> Json<bool> {
    let success = todo_service.delete_todo(id).unwrap_or(false);
    Json(success)
}
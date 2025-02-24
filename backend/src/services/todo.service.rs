use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use chrono::Utc;

use crate::models::todo::*;
use crate::schema::todos;

pub struct TodoService {
    db: Pool<ConnectionManager<SqliteConnection>>
}

impl TodoService {
    pub fn new(db: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { db }
    }

    pub fn create_todo(&self, input: CreateTodoInput) -> Result<Todo, diesel::result::Error> {
        use crate::schema::todos::dsl::*;
        
        let now = Utc::now().naive_utc();
        let new_todo = Todo {
            id: 0, // SQLite will auto-increment
            title: input.title,
            completed: false,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(todos)
            .values(&new_todo)
            .execute(&mut self.db.get().unwrap())?;

        todos.order(id.desc())
            .first(&mut self.db.get().unwrap())
    }

    pub fn list_todos(&self) -> Result<Vec<Todo>, diesel::result::Error> {
        use crate::schema::todos::dsl::*;

        todos.order(created_at.desc())
            .load::<Todo>(&mut self.db.get().unwrap())
    }

    pub fn update_todo(&self, todo_id: i32, input: UpdateTodoInput) -> Result<Todo, diesel::result::Error> {
        use crate::schema::todos::dsl::*;

        let mut updates = todos::table.filter(id.eq(todo_id));

        if let Some(new_title) = input.title {
            updates = updates.set(title.eq(new_title));
        }

        if let Some(new_completed) = input.completed {
            updates = updates.set(completed.eq(new_completed));
        }

        updates
            .set(updated_at.eq(Utc::now().naive_utc()))
            .execute(&mut self.db.get().unwrap())?;

        todos.filter(id.eq(todo_id))
            .first(&mut self.db.get().unwrap())
    }

    pub fn delete_todo(&self, todo_id: i32) -> Result<bool, diesel::result::Error> {
        use crate::schema::todos::dsl::*;

        let deleted = diesel::delete(todos.filter(id.eq(todo_id)))
            .execute(&mut self.db.get().unwrap())?;

        Ok(deleted > 0)
    }
}
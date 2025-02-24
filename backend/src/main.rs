#[macro_use] extern crate rocket;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use rocket::fs::FileServer;
use rocket_cors::CorsOptions;

mod models;
mod routes;
mod schema;
mod services;

use crate::routes::todo::*;
use crate::services::todo::TodoService;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap_or("todos.db".to_string());
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let cors = CorsOptions::default()
        .allow_credentials(true)
        .to_cors()
        .expect("CORS configuration failed");

    rocket::build()
        .manage(TodoService::new(pool))
        .attach(cors)
        .mount("/api", routes![
            list_todos,
            create_todo,
            update_todo,
            delete_todo
        ])
        .mount("/", FileServer::from("static"))
}

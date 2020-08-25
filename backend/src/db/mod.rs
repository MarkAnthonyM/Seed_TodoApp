// Database abstraction layer(?)

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use schema::task;

pub fn establish_connection() -> PgConnection {
    // Load env file located in root directory
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// Delete task from database
pub fn delete_task(connection: &PgConnection, id: &str) {
    let id = id.parse::<i32>().expect("Invalid ID");

    let target = diesel::delete(task::table.filter(task::id.eq(id)))
        .execute(connection);
    match target {
        Ok(v) => if v == 0 {
            println!("Error: ID {} not found in database", id);
        } else {
            println!("Task successfully deleted! ID: {}", id);
        },
        Err(error) => println!("Problem deleting task. Error: {}", error),
    }
}

pub fn create_task(connection: &PgConnection, title: &str, done: &bool) {
    let task = models::NewTask { title, done };

    diesel::insert_into(task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn query_task(connection: &PgConnection) -> Vec<models::Task> {
    task::table.load::<models::Task>(connection).expect("Error loading task")
}

// Update done status of task
pub fn update_task_status(connection: &PgConnection, id: &str) {
    let id = id.parse::<i32>().expect("Invalid ID");

    // Can be refactored to use only update method call like in delete_task function
    let target = task::table.find(id).first::<models::Task>(connection);
    match target {
        Ok(task) => if task.done {
            println!("task already done! ID: {}", task.id);
        } else {
            diesel::update(task::table.find(id))
                .set(task::done.eq(true))
                .execute(connection)
                .expect("Error updating table");
            println!("Updating task status! ID: {}", task.id);
        },
        Err(error) => println!("Error: ID {} {}", id, error),
    }
}
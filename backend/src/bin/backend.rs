#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel;
use rocket_cors::{ AllowedHeaders, AllowedOrigins, Error };

use backend::db::query_task;

use seed_todoapp::{ JsonApiResponse, TaskWrapper };

// Postgres database connection pool
#[database("postgres_todo")]
struct TodoDbConn(diesel::PgConnection);

#[get("/tasks")]
fn tasks_get(conn: TodoDbConn) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![], };

    for db_task in query_task(&conn) {
        let api_task = seed_todoapp::Task {
            id: db_task.id,
            title: db_task.title,
        };

        let formatted_task = TaskWrapper {
            _type: "tasks".to_string(),
            id: api_task.id.to_string(),
            attributes: api_task,
        };
        response.data.push(formatted_task);
    }

    Json(response)
}

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()?;
    
    rocket::ignite()
        .attach(TodoDbConn::fairing())
        .attach(cors)
        .mount("/", routes![tasks_get])
        .launch();
    
    Ok(())
}
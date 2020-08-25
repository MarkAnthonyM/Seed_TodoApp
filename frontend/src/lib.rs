extern crate seed;
use seed::{ prelude::*, * };
use seed_todoapp::{JsonApiResponse, TaskWrapper};

struct Model {
    tasks: Vec<TaskWrapper>,
}

enum Msg {
    FetchedTasks(Result<JsonApiResponse, FetchError>),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTasks(Ok(mut result)) => {
            model.tasks.clear();
            model.tasks.append(&mut result.data);
        },
        Msg::FetchedTasks(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
        },
    }
}

fn view(model: &Model) -> Node<Msg> {
    h1![
        "Tasks",
        ul![
            model
                .tasks
                .iter()
                .map(|t| li![ t.attributes.title.clone() ])
        ],
    ]
}

async fn fetch_drills() -> Option<Msg> {
    let request = fetch("http://localhost:8000/tasks/").await;
    let payload: Result<JsonApiResponse, FetchError> = request.unwrap().json().await;
    Some(Msg::FetchedTasks(payload))
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(fetch_drills());
    Model { tasks: vec![] }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::start("app", init, update, view);
}


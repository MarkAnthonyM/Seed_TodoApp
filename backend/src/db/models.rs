use super::schema::task;

// Create portion of CRUD operations 
#[derive(Insertable)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub done: &'a bool,
}

// Read portion of CRUD operations
// how is Serialize being imported here?
#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

#[derive(Serialize)]
pub struct TaskWrapper {
    pub _type: String,
    pub id: String,
    pub attributes: Task,
}
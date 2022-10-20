use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TodoObject {
    id: i32,
    title: String,
    status: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NewTodoRequest {
    title: String,
    status: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UpdateTodoRequest {
    pub id: i32,
    status: String,
}

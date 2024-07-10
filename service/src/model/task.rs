use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct Task {
    pub name: Option<String>,
    pub created: String,
    pub status: String
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}


use serde::{Deserialize, Serialize};
use mongodb::bson::Binary;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub id: Binary,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoItem {
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoItem {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub done: bool,
}

pub type Db = Arc<Mutex<Vec<Todo>>>;

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            done: false,
        }
    }
}

pub fn create(db: &Db, mut todo: Todo) -> Result<Todo, String> {
    if todo.title.trim().is_empty() {
        return Err("Title cannot be empty".into());
    }
    if todo.id.is_nil() {
        todo.id = Uuid::new_v4();
    }
    db.lock().map_err(|_| "DB lock failed".to_string())?.push(todo.clone());
    Ok(todo)
}

pub fn list(db: &Db) -> Vec<Todo> {
    db.lock().unwrap().clone()
}

pub fn read(db: &Db, id: Uuid) -> Option<Todo> {
    db.lock().unwrap().iter().cloned().find(|t| t.id == id)
}

pub fn update(db: &Db, id: Uuid, title: String, done: bool) -> Result<Todo, String> {
    let mut todos = db.lock().map_err(|_| "DB lock failed".to_string())?;
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        if title.trim().is_empty() {
            return Err("Title cannot be empty".into());
        }
        todo.title = title;
        todo.done = done;
        Ok(todo.clone())
    } else {
        Err("Not Found".into())
    }
}

pub fn delete(db: &Db, id: Uuid) -> bool {
    let mut todos = db.lock().unwrap();
    let before = todos.len();
    todos.retain(|t| t.id != id);
    before != todos.len()
}

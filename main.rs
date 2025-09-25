use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: Uuid,
    title: String,
    done: bool,
}

type Db = Arc<Mutex<Vec<Todo>>>;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

enum ApiError {
    NotFound,
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        let body = Json(ErrorResponse { error: msg });
        (status, body).into_response()
    }
}

// CREATE
async fn create(State(db): State<Db>, Json(mut todo): Json<Todo>)
    -> Result<(StatusCode, Json<Todo>), ApiError>
{
    if todo.title.trim().is_empty() {
        return Err(ApiError::BadRequest("Title cannot be empty".into()));
    }
    if todo.id.is_nil() {
        todo.id = Uuid::new_v4();
    }
    db.lock().map_err(|_| ApiError::Internal("DB lock failed".into()))?
        .push(todo.clone());
    Ok((StatusCode::CREATED, Json(todo)))
}

// READ ALL
async fn list(State(db): State<Db>) -> Result<Json<Vec<Todo>>, ApiError> {
    let todos = db.lock().map_err(|_| ApiError::Internal("DB lock failed".into()))?;
    Ok(Json(todos.clone()))
}

// READ ONE
async fn read(State(db): State<Db>, Path(id): Path<Uuid>)
    -> Result<Json<Todo>, ApiError>
{
    let todos = db.lock().map_err(|_| ApiError::Internal("DB lock failed".into()))?;
    if let Some(todo) = todos.iter().cloned().find(|t| t.id == id) {
        Ok(Json(todo))
    } else {
        Err(ApiError::NotFound)
    }
}

// UPDATE
async fn update(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
    Json(new): Json<Todo>,
) -> Result<Json<Todo>, ApiError> {
    let mut todos = db.lock().map_err(|_| ApiError::Internal("DB lock failed".into()))?;
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        if new.title.trim().is_empty() {
            return Err(ApiError::BadRequest("Title cannot be empty".into()));
        }
        todo.title = new.title;
        todo.done = new.done;
        Ok(Json(todo.clone()))
    } else {
        Err(ApiError::NotFound)
    }
}

// DELETE
async fn delete(State(db): State<Db>, Path(id): Path<Uuid>) -> Result<StatusCode, ApiError> {
    let mut todos = db.lock().map_err(|_| ApiError::Internal("DB lock failed".into()))?;
    let before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() < before {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/todos", post(create).get(list))
        .route("/todos/:id", get(read).put(update).delete(delete))
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

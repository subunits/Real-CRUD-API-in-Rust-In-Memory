use axum::{routing::get, routing::post, routing::put, routing::delete, Router, Json, extract::{Path, State}};
use serde_json::json;
use std::net::SocketAddr;
use real_crud_api::{self as core, Db, Todo};

#[tokio::main]
async fn main() {
    let db: Db = std::sync::Arc::new(std::sync::Mutex::new(vec![]));

    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/:id", get(get_todo).put(update_todo).delete(delete_todo))
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("REST server listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn list_todos(State(db): State<Db>) -> Json<Vec<Todo>> {
    Json(core::list(&db))
}

async fn create_todo(State(db): State<Db>, Json(payload): Json<Todo>) -> Json<serde_json::Value> {
    match core::create(&db, payload) {
        Ok(todo) => Json(json!({ "ok": todo })),
        Err(e) => Json(json!({ "error": e })),
    }
}

async fn get_todo(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    match uuid::Uuid::parse_str(&id) {
        Ok(uuid) => match core::read(&db, uuid) {
            Some(todo) => Json(json!({ "ok": todo })),
            None => Json(json!({ "error": "Not Found" })),
        },
        Err(_) => Json(json!({ "error": "Invalid UUID" })),
    }
}

async fn update_todo(State(db): State<Db>, Path(id): Path<String>, Json(payload): Json<Todo>) -> Json<serde_json::Value> {
    match uuid::Uuid::parse_str(&id) {
        Ok(uuid) => match core::update(&db, uuid, payload.title, payload.done) {
            Ok(todo) => Json(json!({ "ok": todo })),
            Err(e) => Json(json!({ "error": e })),
        },
        Err(_) => Json(json!({ "error": "Invalid UUID" })),
    }
}

async fn delete_todo(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    match uuid::Uuid::parse_str(&id) {
        Ok(uuid) => {
            if core::delete(&db, uuid) {
                Json(json!({ "ok": true }))
            } else {
                Json(json!({ "error": "Not Found" }))
            }
        }
        Err(_) => Json(json!({ "error": "Invalid UUID" })),
    }
}

use tonic::{transport::Server, Request, Response, Status};
use prost::Message;
use real_crud_api::{self as core, Db, Todo};
use uuid::Uuid;
use std::sync::Arc;

pub mod todo_proto {
    tonic::include_proto!("todo");
}

use todo_proto::{TodoService, Todo, TodoId, TodoList};
use todo_proto::todo_service_server::{TodoServiceServer, TodoService as TodoServiceTrait};
use todo_proto::todo_service_client::TodoServiceClient;

#[derive(Clone)]
pub struct MyTodoService {
    db: Db,
}

#[tonic::async_trait]
impl TodoServiceTrait for MyTodoService {
    async fn create(&self, request: Request<Todo>) -> Result<Response<Todo>, Status> {
        let req = request.into_inner();
        let new_todo = core::Todo {
            id: Uuid::new_v4(),
            title: req.title,
            done: req.done,
        };
        match core::create(&self.db, new_todo) {
            Ok(todo) => Ok(Response::new(Todo {
                id: todo.id.to_string(),
                title: todo.title,
                done: todo.done,
            })),
            Err(e) => Err(Status::invalid_argument(e)),
        }
    }

    async fn read(&self, request: Request<TodoId>) -> Result<Response<Todo>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id).map_err(|_| Status::invalid_argument("Invalid UUID"))?;
        match core::read(&self.db, id) {
            Some(todo) => Ok(Response::new(Todo {
                id: todo.id.to_string(),
                title: todo.title,
                done: todo.done,
            })),
            None => Err(Status::not_found("Not Found")),
        }
    }

    async fn list(&self, _: Request<()>) -> Result<Response<TodoList>, Status> {
        let todos = core::list(&self.db);
        let resp = TodoList {
            todos: todos.into_iter().map(|t| Todo {
                id: t.id.to_string(),
                title: t.title,
                done: t.done,
            }).collect(),
        };
        Ok(Response::new(resp))
    }

    async fn update(&self, request: Request<Todo>) -> Result<Response<Todo>, Status> {
        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id).map_err(|_| Status::invalid_argument("Invalid UUID"))?;
        match core::update(&self.db, id, req.title, req.done) {
            Ok(todo) => Ok(Response::new(Todo {
                id: todo.id.to_string(),
                title: todo.title,
                done: todo.done,
            })),
            Err(e) => Err(Status::not_found(e)),
        }
    }

    async fn delete(&self, request: Request<TodoId>) -> Result<Response<()>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id).map_err(|_| Status::invalid_argument("Invalid UUID"))?;
        if core::delete(&self.db, id) {
            Ok(Response::new(()))
        } else {
            Err(Status::not_found("Not Found"))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let db: Db = Arc::new(std::sync::Mutex::new(vec![]));
    let service = MyTodoService { db };

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(TodoServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

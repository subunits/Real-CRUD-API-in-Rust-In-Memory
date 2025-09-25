# Real CRUD API in Rust (In-Memory)

A minimal but realistic CRUD REST API built with [Axum](https://github.com/tokio-rs/axum).
It uses an in-memory store (`Arc<Mutex<Vec<Todo>>>`) with UUIDs for IDs.

## 🚀 Run

```sh
cargo run
```

The server will start on:

```
http://127.0.0.1:3000
```

## 📡 API Endpoints

### Create Todo
```sh
curl -X POST http://127.0.0.1:3000/todos   -H "Content-Type: application/json"   -d '{"id":"00000000-0000-0000-0000-000000000000","title":"Learn Rust","done":false}'
```

### List Todos
```sh
curl http://127.0.0.1:3000/todos
```

### Get One Todo
```sh
curl http://127.0.0.1:3000/todos/<uuid>
```

### Update Todo
```sh
curl -X PUT http://127.0.0.1:3000/todos/<uuid>   -H "Content-Type: application/json"   -d '{"id":"<uuid>","title":"Deep Rust","done":true}'
```

### Delete Todo
```sh
curl -X DELETE http://127.0.0.1:3000/todos/<uuid>
```

## 🛑 Error Handling

Errors return **JSON** with appropriate status codes:

```json
{
  "error": "Not Found"
}
```

- `400 Bad Request` → validation error (e.g., empty title)  
- `404 Not Found` → missing resource  
- `500 Internal Server Error` → DB lock failure (unlikely in memory)  

# Real CRUD API (Rust)

This project provides a **lean, in-memory CRUD API** in Rust, with support for both:

- **REST (JSON over HTTP)** via [Axum](https://crates.io/crates/axum)
- **gRPC (Protocol Buffers)** via [Tonic](https://crates.io/crates/tonic)

All CRUD logic is shared in `lib.rs` (WASM-friendly), making the system portable across runtimes.

---

## 📂 Project Structure

```
real_crud_api/
├── Cargo.toml        # Dependencies
├── build.rs          # Builds gRPC code from proto
├── todo.proto    # Protobuf definition
├── lib.rs        # Core CRUD logic (WASM-ready)
├── main.rs       # REST server (Axum)
└── grpc.rs       # gRPC server (Tonic)
```

---

## 🚀 Running the REST API

Start the REST server:

```sh
cargo run --bin main
```

It will listen on `http://127.0.0.1:3000`.

### Example Requests

Create a todo:
```sh
curl -X POST http://127.0.0.1:3000/todos   -H "Content-Type: application/json"   -d '{"id":"00000000-0000-0000-0000-000000000000","title":"Buy milk","done":false}'
```

List todos:
```sh
curl http://127.0.0.1:3000/todos
```

Get a todo by ID:
```sh
curl http://127.0.0.1:3000/todos/<uuid>
```

Update a todo:
```sh
curl -X PUT http://127.0.0.1:3000/todos/<uuid>   -H "Content-Type: application/json"   -d '{"id":"<uuid>","title":"Buy eggs","done":true}'
```

Delete a todo:
```sh
curl -X DELETE http://127.0.0.1:3000/todos/<uuid>
```

---

## ⚡ Running the gRPC API

Start the gRPC server:

```sh
cargo run --bin grpc
```

It will listen on `http://127.0.0.1:50051`.

### Example with `grpcurl`

Create a todo:
```sh
grpcurl -plaintext -d '{"title":"Buy milk","done":false}'   localhost:50051 todo.TodoService/Create
```

List todos:
```sh
grpcurl -plaintext localhost:50051 todo.TodoService/List
```

Get a todo:
```sh
grpcurl -plaintext -d '{"id":"<uuid>"}'   localhost:50051 todo.TodoService/Read
```

Update a todo:
```sh
grpcurl -plaintext -d '{"id":"<uuid>", "title":"Buy eggs", "done":true}'   localhost:50051 todo.TodoService/Update
```

Delete a todo:
```sh
grpcurl -plaintext -d '{"id":"<uuid>"}'   localhost:50051 todo.TodoService/Delete
```

---

## 🛠️ WASM Compatibility

The core logic in `lib.rs` is **WASM-compatible**.  
You can compile it with `wasm-pack` or use `wasm-bindgen` to expose CRUD functions in the browser or edge runtimes.

---

## ✅ Summary

- **Shared core logic** → portable, WASM-ready
- **REST + gRPC support** → same API, multiple transports
- **In-memory DB** → lean, no external dependencies


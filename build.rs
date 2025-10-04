fn main() {
    tonic_build::configure()
        .compile(&["proto/todo.proto"], &["proto"])
        .unwrap();
}

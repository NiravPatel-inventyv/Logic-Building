use std::{env, path::PathBuf};
use tonic_build;
fn build_proto() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./protos/todos.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("todos_descriptor.bin"))
        .out_dir("./src/tasks/grpc_server")
        .compile(&[file_path], &["proto"])?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./protos/employees.proto")?;
    tonic_build::compile_protos("./protos/students.proto")?;
    tonic_build::compile_protos("./protos/users.proto")?;
    Ok(())
}

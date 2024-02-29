use employee::{ Employee::employees_server::EmployeesServer, EmployeesService};
use student::{ Student::students_server::StudentsServer, StudentsService};
use tonic::transport::Server;
use user::{User, User::users_server::UsersServer, UsersService};

pub mod employee;
pub mod student;
pub mod user;
pub mod utils;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:9000".parse().unwrap();
    let employee_service = EmployeesService::default();
    let student_service = StudentsService::default();
    let user_service = UsersService::default();
    Server::builder()
        .add_service(EmployeesServer::new(employee_service))
        .add_service(StudentsServer::new(student_service))
        .add_service(UsersServer::new(user_service))
        .serve(address)
        .await?;

    Ok(())
}

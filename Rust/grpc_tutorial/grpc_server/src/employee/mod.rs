use tonic::{Request, Response, Status};

use self::Employee::{employees_server::Employees, EmpRequest, EmpRequestId, Response as resp};

#[derive(Debug, Default)]
pub struct EmployeesService {}
pub mod Employee {
    tonic::include_proto!("employees");
}
#[tonic::async_trait]
impl Employees for EmployeesService {
    async fn add_emp(&self, request: Request<EmpRequest>) -> Result<Response<resp>, Status> {
        let req_data = request.into_inner();
        println!("{:?}", req_data);

        let response = resp {
            key: 2000,
            messsage: "data added".to_string(),
            data: "hello im your new friend".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn get_emp(&self, request: Request<EmpRequestId>) -> Result<Response<resp>, Status> {
        let req_data = request.into_inner();
        println!("{:?}", req_data);

        let response = resp {
            key: 200,
            messsage: "data added".to_string(),
            data: "hii im here to meet you".to_string(),
        };

        Ok(Response::new(response))
    }
    async fn remove_emp(&self, request: Request<EmpRequestId>) -> Result<Response<resp>, Status> {
        let req_data = request.into_inner();
        println!("{:?}", req_data);

        let response = resp {
            key: 200,
            messsage: "data added".to_string(),
            data: "hii im going".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn update_emp(&self, request: Request<EmpRequest>) -> Result<Response<resp>, Status> {
        let req_data = request.into_inner();
        println!("{:?}", req_data);
        let response = resp {
            key: 200,
            messsage: "data added".to_string(),
            data: "i changed because u said".to_string(),
        };

        Ok(Response::new(response))
    }
}

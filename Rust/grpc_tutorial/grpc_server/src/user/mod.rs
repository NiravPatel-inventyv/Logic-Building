use tonic::{Request, Response, Status};

use self::User::{users_server::Users, UserRequest, UserRequestId, Response as resp};

#[derive(Debug, Default)]
pub struct UsersService {}
pub mod User {
    tonic::include_proto!("users");
}
#[tonic::async_trait]
impl Users for UsersService {
    async fn add_user(&self, request: Request<UserRequest>) -> Result<Response<resp>, Status> {
        let req_data = request.into_inner();
        println!("{:?}", req_data);

        let response = resp {
            key: 200,
            messsage: "data added".to_string(),
            data: "hello im your new user".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn get_user(&self, request: Request<UserRequestId>) -> Result<Response<resp>, Status> {
        let req_data = request.into_inner();
        println!("{:?}", req_data);

        let response = resp {
            key: 200,
            messsage: "data added".to_string(),
            data: "hii im here to meet you user".to_string(),
        };

        Ok(Response::new(response))
    }  
     async fn remove_user(&self, request: Request<UserRequestId>) -> Result<Response<resp>, Status> {
        let req_data = request.into_inner();
        println!("{:?}", req_data);

        let response = resp {
            key: 200,
            messsage: "data added".to_string(),
            data: "hii im here to meet you user".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn update_user(&self, request: Request<UserRequest>) -> Result<Response<resp>, Status> {
        let req_data = request.into_inner();
        println!("{:?}", req_data);
        let response = resp {
            key: 200,
            messsage: "data added".to_string(),
            data: "i changed because u said user".to_string(),
        };

        Ok(Response::new(response))
    }
}

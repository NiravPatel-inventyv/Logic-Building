use crate::utils::tikv_db::{add_record, delete_record, get_record, scan_data};
use tonic::{Request, Response, Status};

use self::{
    model::map_struct_data,
    Student::{
        students_server::Students, Response as resp, ScanRequest, ScanResponse, StudentRequest,
        StudentRequestId,
    },
};

#[derive(Debug, Default)]
pub struct StudentsService {}
pub mod model;

pub mod Student {
    tonic::include_proto!("students");
}
#[tonic::async_trait]
impl Students for StudentsService {
    async fn add_student(
        &self,
        request: Request<StudentRequest>,
    ) -> Result<Response<resp>, Status> {
        let req_data = request.into_inner();
        let data = map_struct_data(req_data);
        let str_data = serde_json::to_string(&data).unwrap();
        let res = add_record(format!("S-{}", data.id), str_data.to_string()).await;
        if res {
            let response = resp {
                key: 200,
                messsage: "Successfully added".to_string(),
                data: str_data,
            };
            Ok(Response::new(response))
        } else {
            let response = resp {
                key: 401,
                messsage: "failed to add".to_string(),
                data: "".to_string(),
            };
            Ok(Response::new(response))
        }
    }

    async fn get_student(
        &self,
        request: Request<StudentRequestId>,
    ) -> Result<Response<resp>, Status> {
        let req_data = request.into_inner();
        let str_data = get_record(format!("S-{}", req_data.id)).await;
        if str_data != " " {
            let response = resp {
                key: 200,
                messsage: "Successfully got student".to_string(),
                data: str_data,
            };
            Ok(Response::new(response))
        } else {
            let response = resp {
                key: 401,
                messsage: "failed to get record".to_string(),
                data: "".to_string(),
            };
            Ok(Response::new(response))
        }
    }
    async fn remove_student(
        &self,
        request: Request<StudentRequestId>,
    ) -> Result<Response<resp>, Status> {
        let req_data = request.into_inner();
        let str_data = get_record(format!("S-{}", req_data.id)).await;
        let res = delete_record(format!("S-{}", req_data.id)).await;
        if res {
            let response = resp {
                key: 200,
                messsage: "Successfully removed".to_string(),
                data: str_data,
            };
            Ok(Response::new(response))
        } else {
            let response = resp {
                key: 401,
                messsage: "failed to remove".to_string(),
                data: "".to_string(),
            };
            Ok(Response::new(response))
        }
    }

    async fn update_student(
        &self,
        request: Request<StudentRequest>,
    ) -> Result<Response<resp>, Status> {
        let req_data = request.into_inner();
        let data = map_struct_data(req_data);
        let str_data = serde_json::to_string(&data).unwrap();
        let res = add_record(format!("S-{}", data.id), str_data.to_string()).await;
        if res {
            let response = resp {
                key: 200,
                messsage: "Successfully updated".to_string(),
                data: str_data,
            };
            Ok(Response::new(response))
        } else {
            let response = resp {
                key: 401,
                messsage: "failed to update".to_string(),
                data: "".to_string(),
            };
            Ok(Response::new(response))
        }
    }

    async fn scan_student(
        &self,
        request: Request<ScanRequest>,
    ) -> Result<Response<ScanResponse>, Status> {
        let req_data = request.into_inner();
        let res = scan_data(req_data.start, req_data.end, req_data.batch).await;
        let mut result = Vec::new();
        for item in res.into_iter() {
            result.push(String::from_utf8(item.1).unwrap())
        }
        let response = ScanResponse {
            key: 200,
            messsage: "successfully got records".to_string(),
            data: result,
        };

        Ok(Response::new(response))
    }
}

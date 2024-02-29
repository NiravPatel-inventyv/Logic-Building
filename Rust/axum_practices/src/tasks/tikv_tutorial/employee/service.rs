use super::super::utils::structs::Message;
use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};

use super::model::{Employee, ALL_EMPLOYEES};

// pub async fn create_employee(Json(employee): Json<Employee>) -> Response {
//     let u_id = Uuid::new_v4();
// }

pub async fn get_employee(Path(id): Path<u32>) -> Response {
    let employees = ALL_EMPLOYEES.read().await;
    if is_present(id).await {
        let employee = employees.get(&id).unwrap().clone();
        Json(Message {
            status: 3000,
            message_key: String::from("got employee successfully."),
            data: employee,
        })
        .into_response()
    } else {
        Json(Message {
            status: 9000,
            message_key: String::from(format!("employee with id: {} not found", id)),
            data: "",
        })
        .into_response()
    }
}

pub async fn update_employee(Path(id): Path<u32>, Json(employee): Json<Employee>) -> Response {
    let mut employees = ALL_EMPLOYEES.write().await;
    if is_present(id).await {
        let update_employee = employees.get_mut(&id).unwrap();
        update_employee.name = employee.name;
        update_employee.experience = employee.experience;
        update_employee.age = employee.age;
        update_employee.position = employee.position;
        update_employee.skills = employee.skills;

        Json(Message {
            status: 4000,
            message_key: String::from(format!("Update employee with id: {}", id)),
            data: update_employee,
        })
        .into_response()
    } else {
        Json(Message {
            status: 9000,
            message_key: String::from(format!("employee with id: {} not found", id)),
            data: "",
        })
        .into_response()
    }
}

pub async fn delete_employee(Path(id): Path<u32>) -> Response {
    let mut employees = ALL_EMPLOYEES.write().await;
    if is_present(id).await {
        let removed_employee = employees.remove_entry(&id).unwrap().1;
        Json(Message {
            status: 5000,
            message_key: String::from("deleted successfully."),
            data: removed_employee,
        })
        .into_response()
    } else {
        Json(Message {
            status: 9000,
            message_key: String::from(format!("employee with id: {} not found", id)),
            data: "",
        })
        .into_response()
    }
}

pub async fn get_all_employees() -> Response {
    // Return a JSON response with all employees

    Json(Message {
        status: 3000,
        message_key: String::from("success"),
        data: ALL_EMPLOYEES.read().await.clone(),
    })
    .into_response()
}

async fn is_present(id: u32) -> bool {
    let employees = ALL_EMPLOYEES.read().await;
    employees.contains_key(&id)
}

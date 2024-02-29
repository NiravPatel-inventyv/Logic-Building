use axum::{routing::post, Router};

use self::{
    model::ALL_EMPLOYEES,
    service::{create_employee, delete_employee, get_all_employees, get_employee, update_employee},
};
mod model;
mod service;
pub fn get_employee_routes() -> Router {
    Router::new()
        .route("/employees/add/", post(create_employee))
        .route("/employees/get/:id", post(get_employee))
        .route("/employees/update/:id", post(update_employee))
        .route("/employees/delete/:id", post(delete_employee))
        .route("/employees/get_all", post(get_all_employees))
}

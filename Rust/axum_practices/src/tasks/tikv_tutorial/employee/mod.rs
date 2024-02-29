use axum::{routing::post, Router};

mod model;
mod service;
pub fn get_employee_routes() -> Router {
    Router::new()
        .route("/employees/add", post(create_employee))
        .route("/employees/get/:id", post(get_employee))
        .route("/employees/update/:id", post(update_employee))
        .route("/employees/delete/:id", post(delete_employee))
        .route("/employees/get_all", post(get_all_employees))
}

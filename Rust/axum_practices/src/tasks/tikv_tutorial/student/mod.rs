use axum::{routing::post, Router};

use self::service::{
    create_student, delete_student, get_all_students, get_student, update_student,
};
mod model;
mod service;
pub fn get_student_routes() -> Router {
    Router::new()
        .route("/students/add", post(create_student))
        .route("/students/get/:id", post(get_student))
        .route("/students/update/:id", post(update_student))
        .route("/students/delete/:id", post(delete_student))
        .route("/students/get_all", post(get_all_students))
}

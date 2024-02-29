use crate::{
    employee::get_employee_routes, health_check::get_health_check, student::get_student_routes,
    user::get_user_routes,
};
use axum::Router;

pub fn load_routes() -> Router {
    Router::new()
        .merge(get_employee_routes())
        .merge(get_student_routes())
        .merge(get_user_routes())
        .merge(get_health_check())
}

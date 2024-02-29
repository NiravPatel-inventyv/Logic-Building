use axum::{routing::post, Router};

use self::service::{create_user, delete_user, get_all_users, get_user, update_user};
mod model;
mod service;
pub fn get_user_routes() -> Router {
    Router::new()
        .route("/users/add", post(create_user))
        .route("/users/get/:id", post(get_user))
        .route("/users/update/:id", post(update_user))
        .route("/users/delete/:id", post(delete_user))
        .route("/users/get_all", post(get_all_users))
    
}


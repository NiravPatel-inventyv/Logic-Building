use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use rand::Rng;

use crate::tasks::rest_apis::utils::structs::Message;

use super::model::{User, ALL_USERS};

pub async fn create_user(Json(user): Json<User>) -> Response {
    let mut users = ALL_USERS.write().await;
    let id = rand::thread_rng().gen_range(0..=100);
    users.insert(id.clone() as u32, user);
    let added_user = users.get(&id).unwrap().clone();
    Json(Message {
        status: 2000,
        message_key: String::from("created successfully."),
        data: added_user,
    })
    .into_response()
}

pub async fn get_user(Path(id): Path<u32>) -> Response {
    let users = ALL_USERS.read().await;
    if is_present(id).await {
        let user = users.get(&id).unwrap().clone();
        Json(Message {
            status: 3000,
            message_key: String::from("deleted successfully."),
            data: user,
        })
        .into_response()
    } else {
        Json(Message {
            status: 9000,
            message_key: String::from(format!("user with id: {} not found", id)),
            data: "",
        })
        .into_response()
    }
}

pub async fn update_user(Path(id): Path<u32>, Json(user): Json<User>) -> Response {
    let mut users = ALL_USERS.write().await;
    if is_present(id).await {
        let update_user = users.get_mut(&id).unwrap();
        {
            update_user.email = user.email;
            update_user.name = user.name
        }
        Json(Message {
            status: 4000,
            message_key: String::from(format!("Update user with id: {}", id)),
            data: update_user,
        })
        .into_response()
    } else {
        Json(Message {
            status: 9000,
            message_key: String::from(format!("user with id: {} not found", id)),
            data: "",
        })
        .into_response()
    }
}

pub async fn delete_user(Path(id): Path<u32>) -> Response {
    let mut users = ALL_USERS.write().await;
    if is_present(id).await {
        let removed_user = users.remove_entry(&id).unwrap().1;
        Json(Message {
            status: 3000,
            message_key: String::from("deleted successfully."),
            data: removed_user,
        })
        .into_response()
    } else {
        Json(Message {
            status: 9000,
            message_key: String::from(format!("user with id: {} not found", id)),
            data: "",
        })
        .into_response()
    }
}

pub async fn get_all_users() -> Response {
    // Return a JSON response with all users

    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: ALL_USERS.read().await.clone(),
    })
    .into_response()
}
async fn is_present(id: u32) -> bool {
    let students = ALL_USERS.read().await;
    students.contains_key(&id)
}

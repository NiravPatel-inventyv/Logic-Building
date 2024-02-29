use std::fmt::format;

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

use crate::tasks::tikv_tutorial::utils::tikv_db::{add_record, delete_record, get_record};

use super::super::utils::structs::Message;

use super::model::User;

pub async fn create_user(Json(user): Json<User>) -> Response {
    let u_id = Uuid::new_v4();
    let new_user = User {
        id: Some(u_id.to_string()),
        name: user.name,
        email: user.email,
    };
    let str_data = serde_json::to_string(&new_user).unwrap();
    add_record(format!("U-{}", u_id.to_string()), str_data).await;
    Json(Message {
        status: 2000,
        message_key: String::from("user created successfully."),
        data: new_user,
    })
    .into_response()
}

pub async fn get_user(Path(id): Path<String>) -> Response {
    let data = get_record(format!("U-{}", id)).await;
    if (data != "") {
        Json(Message {
            status: 4000,
            message_key: String::from(format!("got user with id: {}", id)),
            data: data,
        })
        .into_response()
    } else {
        Json(Message {
            status: 5000,
            message_key: String::from(format!("user not found")),
            data: "",
        })
        .into_response()
    }
}

pub async fn update_user(Path(id): Path<String>, Json(user): Json<User>) -> Response {
    let data_to_be_updated = User {
        id: Some(id.clone()),
        name: user.name,
        email: user.email,
    };
    let str_data = serde_json::to_string(&data_to_be_updated).unwrap();
    add_record(format!("U-{}", id), str_data).await;
    Json(Message {
        status: 2000,
        message_key: String::from(format!("Update user with id: {}", id)),
        data: data_to_be_updated,
    })
    .into_response()
}

pub async fn delete_user(Path(id): Path<String>) -> Response {
    delete_record(format!("U-{}", id)).await;
    Json(Message {
        status: 2000,
        message_key: String::from("data deleted successfully"),
        data: "",
    })
    .into_response()
}

pub async fn get_all_users() -> Response {
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: "",
    })
    .into_response()
}

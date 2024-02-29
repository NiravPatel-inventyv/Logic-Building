use  super::super::utils::structs::Message;

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use rand::Rng;

use super::model::{Student, ALL_STUDENTS};

pub async fn create_student(Json(student): Json<Student>) -> Response {
    let mut students = ALL_STUDENTS.write().await;
    let id = rand::thread_rng().gen_range(0..=100);
    students.insert(id.clone() as u32, student);
    let added_student = students.get(&id).unwrap().clone();
    Json(Message {
        status: 2000,
        message_key: String::from("created successfully."),
        data: added_student,
    })
    .into_response()
}

pub async fn get_student(Path(id): Path<u32>) -> Response {
    let students = ALL_STUDENTS.read().await;
    if is_present(id).await {
        let student = students.get(&id).unwrap().clone();
        Json(Message {
            status: 3000,
            message_key: String::from("got student successfully."),
            data: student,
        })
        .into_response()
    } else {
        Json(Message {
            status: 9000,
            message_key: String::from(format!("student with id: {} not found", id)),
            data: "",
        })
        .into_response()
    }
}

pub async fn update_student(Path(id): Path<u32>, Json(student): Json<Student>) -> Response {
    let mut students = ALL_STUDENTS.write().await;
    if is_present(id).await {
        let update_student = students.get_mut(&id).unwrap();
        update_student.marks = student.marks;
        update_student.email = student.email;
        update_student.name = student.name;
        update_student.address = student.address;
        update_student.percentage = student.percentage;
        update_student.phone = student.phone;
        update_student.grade = student.grade;
        update_student.city = student.city;

        Json(Message {
            status: 4000,
            message_key: String::from(format!("Update student with id: {}", id)),
            data: update_student,
        })
        .into_response()
    } else {
        Json(Message {
            status: 9000,
            message_key: String::from(format!("student with id: {} not found", id)),
            data: "",
        })
        .into_response()
    }
}

pub async fn delete_student(Path(id): Path<u32>) -> Response {
    let mut students = ALL_STUDENTS.write().await;
    if is_present(id).await {
        let removed_student = students.remove_entry(&id).unwrap().1;
        Json(Message {
            status: 5000,
            message_key: String::from("deleted successfully."),
            data: removed_student,
        })
        .into_response()
    } else {
        Json(Message {
            status: 9000,
            message_key: String::from(format!("student with id: {} not found", id)),
            data: "",
        })
        .into_response()
    }
}

pub async fn get_all_students() -> Response {
    // Return a JSON response with all students

    Json(Message {
        status: 3000,
        message_key: String::from("success"),
        data: ALL_STUDENTS.read().await.clone(),
    })
    .into_response()
}

async fn is_present(id: u32) -> bool {
    let students = ALL_STUDENTS.read().await;
    students.contains_key(&id)
}

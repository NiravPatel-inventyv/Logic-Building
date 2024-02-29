use serde::Serialize;

#[derive(Serialize)]
pub struct Message<T> {
    pub status: u32,
    pub message_key: String,
    pub data: T,
}
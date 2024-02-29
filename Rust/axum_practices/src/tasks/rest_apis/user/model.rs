use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct User {
    pub id : u32,
    pub name: String,
    pub email: String,
}
lazy_static::lazy_static! {
    #[derive( Debug,Clone)]
    pub static ref ALL_USERS: Arc<RwLock<HashMap<u32, User>>> =
        Arc::new(RwLock::new(HashMap::new()));
}
    

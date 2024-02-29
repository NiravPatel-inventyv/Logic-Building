use crate::routes::load_routes;
use middlewares::get_middlewares;
use std::net::SocketAddr;
use tokio::net::TcpListener;
mod config;
mod employee;
mod health_check;
mod middlewares;
pub mod routes;
mod student;
mod user;
mod utils;
#[tokio::main]
async fn main() {
    utils::tikv_db::add_record("dataaa".to_string(), "testing data ise sucessfully stored".to_string()).await;

    utils::tikv_db::get_record("dataaa".to_string()).await;
    utils::tikv_db::get_record("dat".to_string()).await;
    // let app = load_routes();
    // let app = get_middlewares(app);
    // let listener=TcpListener::bind(&"0.0.0.0:3000").await.unwrap();
    // let _server = axum::serve(listener, app).await.unwrap();
}

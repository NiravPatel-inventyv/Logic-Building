use tasks::rest_apis::{middlewares::get_middlewares, routes::load_routes};

// use tasks::tikv_tutorial::{middlewares::get_middlewares, routes::load_routes};
use tokio::net::TcpListener;

pub mod tasks;

/// Run function for http server

pub async fn run() {
    let app = load_routes();
    let app = get_middlewares(app);

    let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();

    if let Ok(server) = axum::serve(listener, app.into_make_service()).await {
        println!("server connected successfully");
    } else {
        println!("failed to connect server");
    }
}

// pub async fn run() {}

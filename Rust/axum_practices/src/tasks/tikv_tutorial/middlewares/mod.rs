use self::cors::get_cors_middleware;
use axum::{middleware, Router};
use tower::{Layer, ServiceBuilder};
use tower_http::trace::TraceLayer;

mod cors;

pub fn get_middlewares(route: Router) -> Router {
    route.layer(ServiceBuilder::new().layer(get_cors_middleware()))
    .layer(TraceLayer::new_for_http())
}

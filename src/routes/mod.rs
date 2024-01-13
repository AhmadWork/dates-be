pub mod hello_world;
mod middleware;
mod users;

use axum::{
    Router,
    routing::{get, post},
    Extension
};

use tower_http::cors::{CorsLayer, Any};
use sea_orm::{DatabaseConnection};
use crate::config::Config;
use std::sync::Arc;

use middleware::auth_required;

use hello_world::hello_world;
use  users::{create_user, logout, sign_in, user_detalis, update_user, add_tree, update_user_overtime};
pub fn create_router(config: Arc<Config>, db: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
    // allow `GET` and `POST` when accessing the resource
    .allow_methods(Any)
    // allow requests from any origin
    .allow_origin(Any);

 Router::new()
     .route("/api/users", get(user_detalis))
     .route("/api/users/update", post(update_user))
     .route("/api/users/update_overtime", post(update_user_overtime))
     .route("/api/users/logout", post(logout))
     .route("/api/tree/new", post(add_tree))
     .layer(axum::middleware::from_fn(auth_required))
     .route("/", get(hello_world))
     .route("/api/users", post(create_user))
     .route("/api/users/login", post(sign_in))
     .layer(Extension(config))
     .layer(Extension(db))
     .layer(cors)


}


use axum::{routing::get, Router};
use crate::handler;

pub fn user_routes() -> Router {
    Router::new().route("/users", get(handler::get_users))
}
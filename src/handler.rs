use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
}

pub async fn get_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];
    Json(users)
}
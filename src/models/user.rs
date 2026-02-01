use axum::{Json};
use uuid::Uuid;

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

impl User {
    pub async fn get_user() -> Json<Uuid> {
        let id = Uuid::new_v4(); 
        Json(id)
    }
}
